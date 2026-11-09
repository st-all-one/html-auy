//! Validação do bloco contra a tabela do spec.
//!
//! A maior parte das regras é **data-driven**: allowlist, obrigatórios, conteúdo-modelo, enums e
//! booleanos vêm de `spec/elements.toml`. O que exige olhar o bloco inteiro (símbolos de `id`,
//! referências acessíveis, estrutura de tabela e mídia) mora em [`crate::rules`].

use crate::ast::{Attr, AttrValue, Block, Element, IfNode, Item, Otherwise};
use crate::attrs::{is_boolean, is_typed};
use crate::diagnostic::Diag;
use crate::rules;
use crate::table::{self, Only, ValueKind};
use core::mem;
use proc_macro2::Span;

/// Atributos que nenhum elemento aceita (docs/design/03 §5, docs/design/08 §2).
const FORBIDDEN_ATTRS: [&str; 12] = [
    "srcdoc",
    "formaction",
    "ping",
    "slot",
    "part",
    "exportparts",
    "shadowrootmode",
    "is",
    "xlink:href",
    "accesskey",
    "draggable",
    "novalidate",
];

/// Categorias que são conteúdo de fluxo: um pai com `children = ["flow"]` as aceita todas
/// (os elementos declaram só a categoria mais específica; `phrasing` e `media` também são fluxo).
const FLOW_ABSORBS: [&str; 5] = ["sectioning", "heading", "form", "phrasing", "interactive"];

/// Atributos que todo controle de formulário aceita além dos do próprio `type` (E21).
const INPUT_BASE_ATTRS: [&str; 7] = [
    "autocomplete",
    "disabled",
    "name",
    "readonly",
    "required",
    "type",
    "value",
];

/// Controle de formulário que precisa de nome acessível resolvido no bloco (E20).
pub(crate) struct Control {
    /// Nome do elemento.
    pub(crate) name: String,
    /// Onde apontar.
    pub(crate) span: Span,
    /// `type` de isenção já reconhecida.
    pub(crate) exempt: bool,
    /// Nome já garantido (label envolvendo ou `aria-labelledby`).
    pub(crate) named: bool,
    /// `id` literal, quando houver.
    pub(crate) id: Option<String>,
}

/// Estado da validação de um bloco.
pub(crate) struct Ctx {
    /// Erros encontrados, na ordem do fonte.
    pub(crate) diags: Vec<Diag>,
    /// `id` literais do bloco (valor, span).
    pub(crate) ids: Vec<(String, Span)>,
    /// Referências literais a id (alvo, span, atributo).
    pub(crate) refs: Vec<(String, Span, String, String)>,
    /// Controles cujo nome acessível depende de outro elemento do bloco.
    pub(crate) controls: Vec<Control>,
}

impl Ctx {
    const fn new() -> Self {
        Self {
            diags: Vec::new(),
            ids: Vec::new(),
            refs: Vec::new(),
            controls: Vec::new(),
        }
    }

    /// Registra um erro.
    pub(crate) fn error(&mut self, diag: Diag) {
        self.diags.push(diag);
    }
}

/// Valida o bloco inteiro. A ordem é a de pré-ordem do fonte; os diagnósticos de escopo de bloco
/// (ids duplicados, referências órfãs, controles sem nome) fecham a lista.
pub(crate) fn check(block: &Block) -> Vec<Diag> {
    let mut ctx = Ctx::new();
    let mut ancestors: Vec<&'static table::Element> = Vec::new();
    check_block(block, &mut ancestors, &mut ctx);
    finish(ctx)
}

fn check_block(block: &Block, ancestors: &mut Vec<&'static table::Element>, ctx: &mut Ctx) {
    for item in &block.items {
        match item {
            Item::Element(element) => check_element(element, ancestors, ctx),
            Item::If(node) => check_if(node, ancestors, ctx),
            Item::For(node) => check_block(&node.body, ancestors, ctx),
            // Os filhos de um primitivo (`Form::open(…) { … }`) viram markup do bloco: passam
            // pela mesma validação, senão `Comp() { script { … } }` fura o vocabulário fechado.
            Item::Primitive(primitive) => check_block(&primitive.children, ancestors, ctx),
            Item::Message(_) | Item::Child(_) => {}
        }
    }
}

fn check_if(node: &IfNode, ancestors: &mut Vec<&'static table::Element>, ctx: &mut Ctx) {
    check_block(&node.then, ancestors, ctx);
    match &node.otherwise {
        None => {}
        Some(Otherwise::Block(block)) => check_block(block, ancestors, ctx),
        Some(Otherwise::If(inner)) => check_if(inner, ancestors, ctx),
    }
}

fn check_element(element: &Element, ancestors: &mut Vec<&'static table::Element>, ctx: &mut Ctx) {
    let Some(spec) = table::element(&element.name) else {
        ctx.error(unknown_element(element));
        return;
    };

    if let Some(only) = spec.only {
        ctx.error(document_element(element, spec, only));
        return;
    }

    check_parent(element, spec, ancestors, ctx);
    check_required(element, spec, ctx);
    check_allowed(element, spec, ctx);
    check_duplicates(element, ctx);
    for attr in &element.attrs {
        check_attr_value(element, spec, attr, ctx);
    }
    check_children(element, spec, ctx);
    check_forbidden_descendants(element, spec, ctx);

    ancestors.push(spec);
    if let Some(body) = &element.body {
        check_block(body, ancestors, ctx);
    }
    ancestors.pop();

    rules::check(element, ancestors.as_slice(), ctx);
}

/// Fecha a validação do bloco: ids repetidos e referências órfãs (E14/E15/E16).
fn finish(ctx: Ctx) -> Vec<Diag> {
    let mut diags = ctx.diags;

    for (index, (id, span)) in ctx.ids.iter().enumerate() {
        let repeated = ctx.ids.iter().take(index).any(|(other, _)| other == id);
        if repeated {
            diags.push(
                Diag::new(
                    *span,
                    format!("`id=\"{id}\"` aparece mais de uma vez no bloco"),
                )
                .rule("escopo local (IDs únicos)")
                .hint("use ids distintos; o `audit` cobra o documento inteiro"),
            );
        }
    }

    for (target, span, attr, source) in &ctx.refs {
        if ctx.ids.iter().any(|(id, _)| id == target) {
            continue;
        }
        diags.push(orphan(*span, attr, target, source));
    }

    for control in &ctx.controls {
        if control.exempt || control.named {
            continue;
        }
        let labelled = control.id.as_ref().is_some_and(|id| {
            ctx.refs
                .iter()
                .any(|(target, _, attr, source)| attr == "for" && source == "label" && target == id)
        });
        if labelled {
            continue;
        }
        diags.push(
            Diag::new(
                control.span,
                format!("`{}` não tem nome acessível", control.name),
            )
            .rule("WCAG 4.1.2 / spec:control_has_accessible_name")
            .hint(control_hint(&control.name)),
        );
    }

    diags
}

fn orphan(span: Span, attr: &str, target: &str, source: &str) -> Diag {
    if attr == "for" {
        let message = if source == "label" {
            format!("`label for=\"{target}\"` não tem `id=\"{target}\"` no mesmo bloco")
        } else {
            format!("`{source} for=\"{target}\"` não tem `id=\"{target}\"` no mesmo bloco")
        };
        return Diag::new(span, message).rule("WCAG 1.3.1").hint(
            "use `Field::text(FieldName::new(\"nome\"), Msg::Name, &state)` (gera label+id+aria) \
                 ou envolva o controle",
        );
    }
    Diag::new(
        span,
        format!("`{attr}=\"{target}\"` referencia `{target}`, que não existe no bloco"),
    )
    .rule("WCAG 1.3.1")
    .hint("use um id presente no mesmo bloco")
}

// ─────────────────────────────── elemento ───────────────────────────────

fn unknown_element(element: &Element) -> Diag {
    let suggestion = nearest(&element.name, table::ELEMENTS.iter().map(|spec| spec.name));
    let message = suggestion.map_or_else(
        || format!("elemento desconhecido `{}`", element.name),
        |name| {
            format!(
                "elemento desconhecido `{}`; você quis dizer `{name}`?",
                element.name
            )
        },
    );
    Diag::new(element.span, message)
        .hint("veja `spec/elements.toml` (CONTRIBUTING.md §3) para o vocabulário fechado")
}

fn document_element(element: &Element, spec: &'static table::Element, only: Only) -> Diag {
    if only == Only::Component {
        return Diag::new(element.span, "`<form>` não é elemento de `html!`")
            .rule("spec:form_requires_csrf")
            .hint("use `Form::open(&action, Method::Post(csrf), children)`");
    }
    let message = match spec.name {
        "main" => String::from("`Page` é dono do `<main>`"),
        "html" | "head" | "body" => format!("`<{}>` só existe no `Page`", spec.tag),
        _ => format!("o shell emite `{}`; `html!` não o constrói", spec.tag),
    };
    Diag::new(element.span, message)
        .rule("spec:shell_only_elements")
        .hint(
            "use `ShellData::new(Text::msg(Msg::Title), heading, lang)` — `<title>`, `<html>`, \
             `<head>` e `<main>` são do shell",
        )
}

fn check_parent(
    element: &Element,
    spec: &'static table::Element,
    ancestors: &[&'static table::Element],
    ctx: &mut Ctx,
) {
    let Some(parent) = ancestors.last().copied() else {
        return;
    };
    // Nome de elemento em `forbidden_children` é sempre direto (`ul > ul` é inválido, mas
    // `ul > li > ul` não é). Categoria é recursiva e fica em `check_forbidden_descendants`.
    let forbidden = parent
        .forbidden_children
        .iter()
        .any(|entry| table::element(entry).is_some() && *entry == spec.name);

    // `<a>` é transparente: herda o content model do ancestral não transparente mais próximo.
    let container = ancestors
        .iter()
        .rev()
        .copied()
        .find(|ancestor| !ancestor.transparent);
    let allowed = container.map_or_else(
        || {
            spec.categories
                .iter()
                .any(|category| *category == "flow" || FLOW_ABSORBS.contains(category))
        },
        |container| child_allowed(container, spec),
    );
    if !forbidden && allowed {
        return;
    }

    let (message, hint, rule) = if forbidden {
        (
            format!(
                "`<{}>` não é permitido dentro de `<{}>`",
                spec.tag, parent.tag
            ),
            format!(
                "use `<span>` (phrasing) ou mova o `<{}>` para fora do `<{}>`",
                spec.tag, parent.tag
            ),
            format!("conteúdo-modelo (spec:element.{})", parent.name),
        )
    } else if parent.transparent {
        let inherited = container.map_or("raiz", |container| container.tag);
        (
            format!(
                "`<{}>` não é permitido aqui: `<{}>` herda o conteúdo de `<{inherited}>`",
                spec.tag, parent.tag
            ),
            format!("use um filho que caiba em `<{inherited}>`"),
            format!("conteúdo-modelo (spec:element.{})", parent.name),
        )
    } else {
        let hint = if parent.children.contains(&"phrasing") {
            format!(
                "use `<span>` (phrasing) ou mova o `<{}>` para fora do `<{}>`",
                spec.tag, parent.tag
            )
        } else {
            format!(
                "filhos permitidos em `<{}>`: {}",
                parent.tag,
                child_list(parent)
            )
        };
        (
            format!(
                "`<{}>` não é permitido dentro de `<{}>`",
                spec.tag, parent.tag
            ),
            hint,
            format!("conteúdo-modelo (spec:element.{})", parent.name),
        )
    };
    ctx.error(Diag::new(element.span, message).rule(rule).hint(hint));
}

/// Categorias proibidas valem em qualquer profundidade (`<a><div><button>` não é um link válido).
fn check_forbidden_descendants(element: &Element, spec: &'static table::Element, ctx: &mut Ctx) {
    let Some(body) = &element.body else {
        return;
    };
    let mut found: Vec<&Element> = Vec::new();
    collect_descendants(body, &mut found);
    for entry in spec.forbidden_children {
        if table::element(entry).is_some() {
            continue;
        }
        for descendant in found.iter().copied() {
            let Some(descendant_spec) = table::element(&descendant.name) else {
                continue;
            };
            if descendant_spec.categories.contains(entry) {
                ctx.error(
                    Diag::new(
                        descendant.span,
                        format!(
                            "`<{}>` não pode estar dentro de `<{}>`",
                            descendant.name, spec.tag
                        ),
                    )
                    .rule(format!("conteúdo-modelo (spec:element.{})", spec.name))
                    .hint(format!(
                        "remova o `<{}>` de dentro do `<{}>`",
                        descendant.name, spec.tag
                    )),
                );
            }
        }
    }
}

fn collect_descendants<'a>(body: &'a Block, found: &mut Vec<&'a Element>) {
    for item in &body.items {
        match item {
            Item::Element(element) => {
                found.push(element);
                if let Some(body) = &element.body {
                    collect_descendants(body, found);
                }
            }
            Item::If(node) => {
                collect_descendants(&node.then, found);
                match &node.otherwise {
                    Some(Otherwise::Block(block)) => collect_descendants(block, found),
                    Some(Otherwise::If(inner)) => collect_descendants_if(inner, found),
                    None => {}
                }
            }
            Item::For(node) => collect_descendants(&node.body, found),
            Item::Primitive(primitive) => collect_descendants(&primitive.children, found),
            Item::Message(_) | Item::Child(_) => {}
        }
    }
}

fn collect_descendants_if<'a>(node: &'a IfNode, found: &mut Vec<&'a Element>) {
    collect_descendants(&node.then, found);
    match &node.otherwise {
        Some(Otherwise::Block(block)) => collect_descendants(block, found),
        Some(Otherwise::If(inner)) => collect_descendants_if(inner, found),
        None => {}
    }
}

fn child_allowed(parent: &'static table::Element, child: &'static table::Element) -> bool {
    if parent.children.contains(&child.name) {
        return true;
    }
    if child
        .categories
        .iter()
        .any(|category| parent.children.contains(category))
    {
        return true;
    }
    parent.children.contains(&"flow")
        && child
            .categories
            .iter()
            .any(|category| FLOW_ABSORBS.contains(category))
}

fn child_list(parent: &'static table::Element) -> String {
    let names: Vec<&str> = parent
        .children
        .iter()
        .copied()
        .filter(|entry| *entry != "none")
        .collect();
    names.join(", ")
}

fn check_required(element: &Element, spec: &'static table::Element, ctx: &mut Ctx) {
    for required in spec.required {
        if element.attr(required).is_some() {
            continue;
        }
        let (rule, hint) = requirement_hint(spec.name, required);
        ctx.error(
            Diag::new(element.span, format!("`{}` exige `{required}`", spec.name))
                .rule(rule)
                .hint(hint),
        );
    }
}

fn requirement_hint(element: &str, attr: &str) -> (String, String) {
    let hint = match (element, attr) {
        ("img", "alt") => String::from("`Alt::new(\"Descrição objetiva\")` ou `Alt::decorative()`"),
        ("img" | "source", "width" | "height") => {
            String::from("`WidthPx::new(720)` / `HeightPx::new(480)` (CLS 0)")
        }
        ("source", "sizes") => String::from("`Sizes::new(\"(min-width: 48em) 50vw, 100vw\")`"),
        ("a", "href") => String::from("`Href::internal(LUrl::…)` ou `Href::external(…)`"),
        ("iframe", "title") => String::from("nome acessível do frame (WCAG 4.1.2)"),
        ("input", "type" | "name") => {
            String::from("use `Field::text(FieldName::new(\"…\"), Msg::…, &state)`")
        }
        ("button", "type") => String::from("use `type=\"button\"` (nunca o submit implícito)"),
        ("label", "for") => {
            String::from("use `for=\"id\"` ou envolva o controle; melhor: `Field::text(…)`")
        }
        ("th", "scope") => String::from("`scope=\"col\"` em `thead`; `scope=\"row\"` no corpo"),
        ("time", "datetime") => String::from("`datetime=(Timestamp::new(…))` (ISO 8601)"),
        _ => format!("adicione `{attr}=(…)`"),
    };
    let rule = if element == "img" && attr == "alt" {
        String::from("WCAG 1.1.1 / spec:element.img")
    } else {
        format!("spec:element.{element}")
    };
    (rule, hint)
}

// ─────────────────────────────── atributos ───────────────────────────────

fn check_allowed(element: &Element, spec: &'static table::Element, ctx: &mut Ctx) {
    for attr in &element.attrs {
        let key = attr.key();
        if key == "role" {
            check_role(spec, attr, ctx);
        } else if is_forbidden(&key) {
            let hint = if key == "accesskey" {
                "atalhos de teclado não são portáveis entre navegador/leitor de tela; use o skip link e `Tab`"
            } else {
                "estilo é token e comportamento é asset declarado"
            };
            ctx.error(
                Diag::new(
                    attr.span,
                    format!("atributo proibido `{key}` em `{}`", spec.name),
                )
                .rule(format!("spec:element.{}", spec.name))
                .hint(hint),
            );
        } else if key.starts_with("data-") {
            if !valid_data_name(&key) {
                ctx.error(
                    Diag::new(attr.span, format!("`data` inválido: `{key}`"))
                        .rule("spec:data")
                        .hint("use `data-*` minúsculo com segmentos `[a-z0-9]`"),
                );
            }
        } else if !allowed_key(spec, &key) {
            let names = allowed_names(spec);
            let mut message = format!("atributo desconhecido `{key}` em `{}`", spec.name);
            if let Some(candidate) = nearest(&key, names.iter().copied()) {
                message.push_str("; você quis dizer `");
                message.push_str(candidate);
                message.push_str("`?");
            }
            ctx.error(Diag::new(attr.span, message).hint(format!(
                "atributos válidos em `{}`: {}",
                spec.name,
                names.join(", ")
            )));
        }
    }
}

fn check_role(spec: &'static table::Element, attr: &Attr, ctx: &mut Ctx) {
    if spec.roles.is_empty() {
        ctx.error(
            Diag::new(attr.span, format!("`role` não existe em `{}`", spec.name))
                .rule("D4")
                .hint("use o papel nativo do elemento"),
        );
        return;
    }
    match &attr.value {
        AttrValue::Literal { text, span } if spec.roles.contains(&text.as_str()) => {}
        AttrValue::Literal { text, span } => ctx.error(
            Diag::new(
                *span,
                format!("`role=\"{text}\"` não é válido em `{}`", spec.name),
            )
            .rule(format!("spec:element.{}", spec.name))
            .hint(format!("papéis aceitos: {}", spec.roles.join(", "))),
        ),
        AttrValue::Computed { span, .. } => ctx.error(
            Diag::new(*span, "`role` não aceita expressão; use um literal")
                .rule("D4")
                .hint("o papel é uma decisão de projeto, não um dado"),
        ),
    }
}

fn is_forbidden(key: &str) -> bool {
    FORBIDDEN_ATTRS.contains(&key) || key.starts_with("on")
}

fn allowed_key(spec: &'static table::Element, key: &str) -> bool {
    if spec.required.contains(&key) || spec.allowed.contains(&key) {
        return true;
    }
    if key.starts_with("data-") {
        return spec.allowed.contains(&"data-*");
    }
    // `aria-*` é allowlist POR ELEMENTO: os nomes aparecem em `GLOBALS` (para o tipo do valor),
    // mas o elemento só os aceita se declarar `aria-*` em `allowed`.
    if key.starts_with("aria-") {
        return spec.allowed.contains(&"aria-*")
            && table::ARIA_VALUES.iter().any(|aria| aria.name == key);
    }
    table::GLOBALS.iter().any(|global| global.name == key)
}

/// Allowlist efetiva impressa na ajuda de E2: `required ++ allowed`, sem curingas.
fn allowed_names(spec: &'static table::Element) -> Vec<&'static str> {
    let mut names: Vec<&'static str> = Vec::new();
    for name in spec.required.iter().chain(spec.allowed) {
        if name.ends_with('*') || names.contains(name) {
            continue;
        }
        names.push(name);
    }
    names
}

fn check_duplicates(element: &Element, ctx: &mut Ctx) {
    let mut seen: Vec<String> = Vec::new();
    for attr in &element.attrs {
        let key = attr.key();
        if seen.contains(&key) {
            ctx.error(
                Diag::new(
                    attr.span,
                    format!("atributo `{key}` repetido em `{}`", element.name),
                )
                .rule(format!("spec:element.{}", element.name))
                .hint("remova a duplicata"),
            );
            continue;
        }
        seen.push(key);
    }
}

fn check_attr_value(element: &Element, spec: &'static table::Element, attr: &Attr, ctx: &mut Ctx) {
    let key = attr.key();
    if key == "role" || is_forbidden(&key) {
        return;
    }

    if is_boolean(&key) {
        if let AttrValue::Literal { span, .. } = attr.value {
            ctx.error(
                Diag::new(
                    span,
                    format!("`{key}` é booleano; use `{key}=(expr)` com `expr: bool`"),
                )
                .rule("spec:boolean_attributes (D19)")
                .hint("presença = true; `disabled=\"false\"` é verdadeiro em HTML"),
            );
        }
        return;
    }

    if key == "aria-label" {
        if let AttrValue::Literal { span, .. } = attr.value {
            ctx.error(
                Diag::new(span, "`aria-label` não aceita literal")
                    .rule("D31")
                    .hint("use `aria_label=(Label::text(Msg::…))`"),
            );
        }
        return;
    }

    if is_typed(spec.name, &key) {
        if let AttrValue::Literal { span, .. } = attr.value {
            ctx.error(literal_rejected(spec.name, &key, span));
        }
        return;
    }

    if key == "for" || key == "list" {
        if let AttrValue::Literal { text, span } = &attr.value {
            ctx.refs
                .push((text.clone(), *span, key.clone(), element.name.clone()));
        }
        return;
    }

    if key == "pattern" {
        check_pattern(attr, ctx);
    }

    let Some((kind, values)) = value_spec(spec.name, &key) else {
        return;
    };

    match &attr.value {
        AttrValue::Literal { text, span } => {
            check_literal(element, spec, &key, text, *span, kind, values, ctx);
        }
        AttrValue::Computed { span, .. } => {
            if kind == ValueKind::Enum {
                ctx.error(
                    Diag::new(
                        *span,
                        format!("`{key}` é enumerado em `{}`; use literal", spec.name),
                    )
                    .rule(format!("spec:element.{}", spec.name))
                    .hint(format!("valores aceitos: {}", values.join(", "))),
                );
            }
        }
    }
}

fn literal_rejected(element: &str, key: &str, span: Span) -> Diag {
    let hint = match key {
        "href" => String::from("use `href=(Href::internal(LUrl::…))`"),
        "src" | "poster" => String::from("use `src=(Src::media(&lurl))` / `Src::asset(…)`"),
        "srcset" => String::from("use `srcset=(SrcSet::from_variants(&[…]))`"),
        "sizes" => String::from("use `sizes=(Sizes::new(\"…\"))`"),
        "alt" => String::from("use `alt=(Alt::new(\"…\"))` ou `Alt::decorative()`"),
        "class" => String::from("use `class=(Class::of(&[Token::…]))`"),
        "datetime" => String::from("use `datetime=(Timestamp::new(segundos, fuso))`"),
        "width" | "height" => String::from("use `WidthPx::new(…)` / `HeightPx::new(…)` (CLS 0)"),
        other => format!("use `{other}=(…)` com o tipo do contexto"),
    };
    Diag::new(span, format!("`{key}` não aceita literal em `{element}`"))
        .rule("contexto de escape (D1)")
        .hint(hint)
}

#[expect(
    clippy::too_many_arguments,
    reason = "um argumento por peça do diagnóstico; agrupar criaria struct de uso único"
)]
fn check_literal(
    element: &Element,
    spec: &'static table::Element,
    key: &str,
    text: &str,
    span: Span,
    kind: ValueKind,
    values: &'static [&'static str],
    ctx: &mut Ctx,
) {
    match kind {
        ValueKind::Enum if !values.is_empty() && !values.contains(&text) => {
            ctx.error(enum_error(spec, key, text, span, values));
        }
        ValueKind::Tokens if !values.is_empty() => {
            for token in text.split_whitespace() {
                if token == "opener" {
                    ctx.error(
                        Diag::new(span, "`rel=\"opener\"` é proibido")
                            .rule("spec:a_rel_enum")
                            .hint("remova `opener`"),
                    );
                } else if !values.contains(&token) {
                    ctx.error(enum_error(spec, key, token, span, values));
                }
            }
        }
        ValueKind::Bcp47 if !valid_language(text) => ctx.error(
            Diag::new(span, format!("`{key}=\"{text}\"` não é BCP-47"))
                .rule(format!("spec:element.{}", spec.name))
                .hint("use a etiqueta canônica, como `pt-BR`"),
        ),
        ValueKind::Mime if !valid_mime(text) => ctx.error(
            Diag::new(span, format!("`{key}=\"{text}\"` não é um tipo MIME"))
                .rule(format!("spec:element.{}", spec.name))
                .hint("use `tipo/subtipo`, como `image/avif`"),
        ),
        ValueKind::Id => ctx.ids.push((text.to_owned(), span)),
        ValueKind::IdRef => {
            ctx.refs
                .push((text.to_owned(), span, key.to_owned(), element.name.clone()));
        }
        ValueKind::IdRefList => {
            for target in text.split([',', ' ']).filter(|part| !part.is_empty()) {
                ctx.refs.push((
                    target.to_owned(),
                    span,
                    key.to_owned(),
                    element.name.clone(),
                ));
            }
        }
        _ => {}
    }
}

fn enum_error(
    spec: &'static table::Element,
    key: &str,
    text: &str,
    span: Span,
    values: &'static [&'static str],
) -> Diag {
    // `aria-*` e globais têm invariante própria (`spec:aria-live`); o resto é do elemento.
    let is_aria = key.starts_with("aria-");
    let is_global = table::GLOBALS.iter().any(|global| global.name == key);
    let rule = if is_aria || is_global {
        format!("spec:{key}")
    } else {
        format!("spec:element.{}", spec.name)
    };
    let message = if is_aria {
        format!("`{key}=\"{text}\"` não é válido")
    } else {
        format!("`{key}=\"{text}\"` não é válido em `{}`", spec.name)
    };
    Diag::new(span, message)
        .rule(rule)
        .hint(format!("valores aceitos: {}", values.join(", ")))
}

/// Tipo de valor declarado para `(elemento, atributo)`, considerando globais e `aria-*`.
fn value_spec(element: &str, key: &str) -> Option<(ValueKind, &'static [&'static str])> {
    if let Some(entry) = table::attr_values(element, key) {
        return Some((entry.kind, entry.values));
    }
    if let Some(global) = table::GLOBALS.iter().find(|global| global.name == key) {
        return Some((global.kind, global.values));
    }
    table::ARIA_VALUES
        .iter()
        .find(|aria| aria.name == key)
        .map(|aria| (aria.kind, aria.values))
}

fn check_pattern(attr: &Attr, ctx: &mut Ctx) {
    let AttrValue::Literal { text, span } = &attr.value else {
        return;
    };
    // `/abc/g` (delimitada), `(?i)…` (flag inline) ou `abc/i` (sufixo de flags). Uma `/` dentro
    // da regex (`[a-z/]`) é legal; só o sufixo de flags é recusado.
    let delimited = text.starts_with('/') || text.starts_with("(?");
    let flagged = text.rsplit_once('/').is_some_and(|(_, flags)| {
        !flags.is_empty()
            && flags
                .chars()
                .all(|ch| matches!(ch, 'i' | 'g' | 'm' | 's' | 'u' | 'y'))
    });
    if delimited || flagged {
        ctx.error(
            Diag::new(*span, "`pattern` não aceita delimitadores nem flags")
                .rule("spec:pattern_flags_forbidden")
                .hint("escreva a regex sem `/…/` e sem flags"),
        );
    }
}

fn valid_data_name(key: &str) -> bool {
    let Some(rest) = key.strip_prefix("data-") else {
        return false;
    };
    !rest.is_empty()
        && rest.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
        })
}

/// BCP-47 do crate `html_auy::Lang`, replicado aqui (o macro não depende do crate `html`).
fn valid_language(text: &str) -> bool {
    let mut parts = text.split('-');
    let Some(language) = parts.next() else {
        return false;
    };
    if !(2..=3).contains(&language.len()) || !language.chars().all(|ch| ch.is_ascii_alphabetic()) {
        return false;
    }
    parts.all(|part| {
        let alpha = part.chars().all(|ch| ch.is_ascii_alphabetic());
        let digit = part.chars().all(|ch| ch.is_ascii_digit());
        let alnum = part.chars().all(|ch| ch.is_ascii_alphanumeric());
        (alpha && (part.len() == 2 || part.len() == 4))
            || (digit && part.len() == 3)
            || (alnum && (5..=8).contains(&part.len()))
    })
}

fn valid_mime(text: &str) -> bool {
    let Some((kind, subtype)) = text.split_once('/') else {
        return false;
    };
    let word = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '.')
    };
    word(kind)
        && !subtype.is_empty()
        && subtype.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '.' || ch == '+'
        })
}

// ─────────────────────────────── filhos ───────────────────────────────

fn check_children(element: &Element, spec: &'static table::Element, ctx: &mut Ctx) {
    let Some(body) = &element.body else {
        if !spec.void {
            ctx.error(
                Diag::new(
                    element.span,
                    format!("`{}` exige corpo: `{} {{ … }}`", spec.name, spec.name),
                )
                .rule(format!("spec:element.{}", spec.name)),
            );
        }
        return;
    };

    if spec.void {
        ctx.error(
            Diag::new(element.span, format!("`{}` não aceita filhos", spec.name))
                .rule(format!("spec:element.{}", spec.name))
                .hint("termine o elemento com `;`"),
        );
        return;
    }

    let accepts_text = spec
        .children
        .iter()
        .any(|entry| matches!(*entry, "text" | "phrasing" | "flow" | "json"));

    let mut children: Vec<&Element> = Vec::new();
    block_elements(body, &mut children);
    let present: Vec<&'static table::Element> = children
        .iter()
        .filter_map(|child| table::element(&child.name))
        .collect();

    let mut text_reported = false;
    for item in &body.items {
        match item {
            Item::Message(_) | Item::Child(_) | Item::Primitive(_)
                if !accepts_text && !text_reported =>
            {
                text_reported = true;
                ctx.error(
                    Diag::new(
                        element.span,
                        format!("texto solto em `{}` não é permitido", spec.name),
                    )
                    .rule(format!("spec:element.{}", spec.name))
                    .hint("o texto vai em `caption`, `th` ou `td`"),
                );
            }
            _ => {}
        }
    }

    for required in spec.required_children {
        if !present
            .iter()
            .any(|child| child.name == *required || child.categories.contains(required))
        {
            let hint = if spec.first_child == *required {
                format!("adicione `{required} {{ … }}` como primeiro filho")
            } else if table::element(required).is_some() {
                format!("adicione `{required} {{ … }}`")
            } else {
                format!("adicione um filho da categoria `{required}`")
            };
            ctx.error(
                Diag::new(element.span, format!("`{}` exige `{required}`", spec.name))
                    .rule(format!("spec:element.{}", spec.name))
                    .hint(hint),
            );
        }
    }

    if !spec.first_child.is_empty()
        && let Some(index) = present
            .iter()
            .position(|child| child.name == spec.first_child)
        && index > 0
    {
        ctx.error(
            Diag::new(
                element.span,
                format!(
                    "`{}` precisa ser o primeiro filho de `{}`",
                    spec.first_child, spec.name
                ),
            )
            .rule(format!("spec:element.{}", spec.name))
            .hint(format!("mova `{}` para o início", spec.first_child)),
        );
    }
}

/// Elementos do nível 0 atravessando `@if`/`@for` (a mesma ordem em que o codegen os emite).
fn block_elements<'a>(block: &'a Block, out: &mut Vec<&'a Element>) {
    for item in &block.items {
        match item {
            Item::Element(element) => out.push(element),
            Item::If(node) => block_elements_if(node, out),
            Item::For(node) => block_elements(&node.body, out),
            Item::Primitive(primitive) => block_elements(&primitive.children, out),
            Item::Message(_) | Item::Child(_) => {}
        }
    }
}

fn block_elements_if<'a>(node: &'a IfNode, out: &mut Vec<&'a Element>) {
    block_elements(&node.then, out);
    match &node.otherwise {
        Some(Otherwise::Block(inner)) => block_elements(inner, out),
        Some(Otherwise::If(inner)) => block_elements_if(inner, out),
        None => {}
    }
}

// ─────────────────────────────── utilidades ───────────────────────────────

/// Sugestão por distância de edição (≤ 2) para nomes de elemento e atributo.
pub(crate) fn nearest<'a>(
    name: &str,
    candidates: impl Iterator<Item = &'a str>,
) -> Option<&'a str> {
    let mut best: Option<(&str, usize)> = None;
    for candidate in candidates {
        let distance = edit_distance(name, candidate);
        if distance > 2 {
            continue;
        }
        if best.is_none_or(|(_, current)| distance < current) {
            best = Some((candidate, distance));
        }
    }
    best.map(|(candidate, _)| candidate)
}

fn edit_distance(left: &str, right: &str) -> usize {
    let right: Vec<char> = right.chars().collect();
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current: Vec<usize> = vec![0; right.len().saturating_add(1)];
    for (row, left_char) in left.chars().enumerate() {
        if let Some(slot) = current.first_mut() {
            *slot = row.saturating_add(1);
        }
        for (column, right_char) in right.iter().enumerate() {
            let deletion = previous
                .get(column.saturating_add(1))
                .copied()
                .unwrap_or(0)
                .saturating_add(1);
            let insertion = current.get(column).copied().unwrap_or(0).saturating_add(1);
            let substitution = previous
                .get(column)
                .copied()
                .unwrap_or(0)
                .saturating_add(usize::from(left_char != *right_char));
            let value = deletion.min(insertion).min(substitution);
            if let Some(slot) = current.get_mut(column.saturating_add(1)) {
                *slot = value;
            }
        }
        mem::swap(&mut previous, &mut current);
    }
    previous.get(right.len()).copied().unwrap_or(0)
}

/// Ajuda de E20 por espécie de controle: campo de formulário tem construtor próprio.
fn control_hint(name: &str) -> &'static str {
    match name {
        "input" | "select" | "textarea" => {
            "use `Field::text(FieldName::new(\"…\"), Msg::…, &state)` ou `aria-labelledby`"
        }
        _ => "use texto, `Label::text(Msg::…)` ou `aria-labelledby`",
    }
}

/// `true` quando o atributo é aceito por qualquer `input` (base de `attribute_by_input_type`).
pub(crate) fn input_base_attr(key: &str) -> bool {
    INPUT_BASE_ATTRS.contains(&key) || matches!(key, "class" | "id")
}
