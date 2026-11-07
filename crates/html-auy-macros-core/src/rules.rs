//! Regras que exigem olhar além de um atributo: estrutura, símbolos do bloco e campos irmãos.
//!
//! Tudo o que a tabela já responde (allowlist, obrigatórios, conteúdo-modelo, enums, booleanos)
//! fica em [`crate::validate`]; aqui só entra a invariante que precisa de código.

use crate::ast::{AttrValue, Block, Element, Item};
use crate::diagnostic::Diag;
use crate::table::{self, Element as Spec};
use crate::validate::Ctx;

/// `input type` que dispensa nome acessível próprio (`label_exempt_types`).
const EXEMPT_INPUT_TYPES: [&str; 5] = ["hidden", "submit", "reset", "button", "image"];

/// Elementos que um `label` pode rotular quando os envolve.
const LABELABLE: [&str; 7] = [
    "button", "input", "meter", "output", "progress", "select", "textarea",
];

/// Elementos que exigem nome acessível (`control_has_accessible_name`).
const NAMED_CONTROLS: [&str; 2] = ["a", "button"];

/// Invariantes de escopo `block` que a tabela já decide (lista explícita, coberta por teste).
#[cfg(test)]
const DATA_DRIVEN: &[&str] = &[
    "a_rel_enum",
    "alt_empty_means_decorative",
    "autocomplete_enum",
    "caption_required",
    "closedby_enum",
    "datetime_profile",
    "href_is_typed",
    "intrinsic_dimensions_required",
    "no_flow_inside_phrasing",
    "no_text_children",
    "preserve_whitespace",
    "referrer_policy_enum",
    "scope_enum",
    "shell_only_elements",
    "source_profile_media_only",
    "source_profile_picture_only",
    "svg_icon_default_decorative",
    "svg_informative_has_name",
    "tbody_explicit",
    "type_enum",
];

/// Invariantes de escopo `block` cujo elemento é `shell`/`component`: o construtor do crate as
/// garante e a macro nunca chega ao elemento (`only`).
#[cfg(test)]
const BY_CONSTRUCTION: &[&str] = &[
    "action_required_unless_dialog",
    "head_charset_first",
    "link_rel_enum",
    "main_tabindex_minus_one",
    "meta_form_exclusive",
    "meta_name_enum",
    "method_enum",
    "named_is_landmark",
    "no_http_equiv",
    "noscript_context",
    "novalidate_forbidden",
    "preload_requires_as_and_crossorigin",
    "script_form_exclusive",
    "script_is_jsonld_only",
    "script_src_requires_defer",
];

/// Invariantes de escopo `block` implementadas neste arquivo.
#[cfg(test)]
const CODED: &[&str] = &[
    "attribute_by_input_type",
    "blank_target_implies_noopener",
    "caption_not_empty",
    "captions_kind_required",
    "control_has_accessible_name",
    "cross_origin_requires_sandbox",
    "dialog_modal_semantics",
    "for_matches_id_in_block",
    "label_exempt_types",
    "label_or_labelledby_in_block",
    "li_only_in_list",
    "legend_not_empty",
    "media_requires_captions",
    "media_source_xor_src",
    "no_aria_hidden_on_focusable",
    "no_aria_label_over_visible_text",
    "no_aria_modal",
    "pattern_flags_forbidden",
    "scope_matches_section",
    "section_has_heading_or_name",
    "select_has_selectable_option",
    "srclang_required_for_subtitles",
    "srcset_requires_sizes_and_dims",
    "summary_single_heading",
];

/// Cobra as invariantes que precisam de código para o elemento.
pub(crate) fn check(element: &Element, ancestors: &[&'static Spec], ctx: &mut Ctx) {
    check_focusable(element, ctx);
    check_blank_target(element, ctx);
    check_named_control(element, ancestors, ctx);
    check_input_attributes(element, ctx);
    check_srcset(element, ctx);
    check_dialog(element, ctx);
    check_section(element, ctx);
    check_table(element, ancestors, ctx);
    check_media(element, ctx);
    check_frame(element, ctx);
    check_label(element, ctx);
    check_list_item(element, ancestors, ctx);
}

// ─────────────────────────────── foco e nome ───────────────────────────────

fn check_focusable(element: &Element, ctx: &mut Ctx) {
    if !matches!(element.name.as_str(), "a" | "button" | "input") {
        return;
    }
    if let Some(attr) = element.attr("aria-hidden")
        && element.literal("aria-hidden") == Some("true")
    {
        ctx.error(
            Diag::new(
                attr.span,
                format!(
                    "`{}` é focável; `aria-hidden=\"true\"` remove o foco da árvore",
                    element.name
                ),
            )
            .rule("WCAG 4.1.2 / spec:no_aria_hidden_on_focusable")
            .hint("remova o `aria-hidden`"),
        );
    }
}

fn check_named_control(element: &Element, ancestors: &[&'static Spec], ctx: &mut Ctx) {
    let named = NAMED_CONTROLS.contains(&element.name.as_str());
    // `optgroup` tem `label` próprio (nome acessível pelo atributo), não por `<label for>`.
    let labelable = matches!(element.name.as_str(), "input" | "select" | "textarea");

    if named {
        let has_visible_text = has_visible_text(element);
        let has_aria =
            element.attr("aria-label").is_some() || element.attr("aria-labelledby").is_some();
        if has_visible_text && element.attr("aria-label").is_some() {
            let attr = element.attr("aria-label");
            let span = attr.map_or(element.span, |attr| attr.span);
            ctx.error(
                Diag::new(
                    span,
                    format!(
                        "`{}` tem texto visível; o nome acessível já é o texto",
                        element.name
                    ),
                )
                .rule("WCAG 2.5.3 / spec:no_aria_label_over_visible_text")
                .hint("remova `aria-label`"),
            );
        }
        if !has_visible_text && !has_aria {
            ctx.error(
                Diag::new(
                    element.span,
                    format!("`{}` não tem nome acessível", element.name),
                )
                .rule("WCAG 4.1.2 / spec:control_has_accessible_name")
                .hint("use texto, `Label::text(Msg::…)` ou `aria-labelledby`"),
            );
        }
        return;
    }

    if !labelable {
        return;
    }

    let exempt = element
        .literal("type")
        .is_some_and(|kind| EXEMPT_INPUT_TYPES.contains(&kind));
    if exempt {
        return;
    }

    let wrapped = ancestors.iter().any(|ancestor| ancestor.name == "label");
    let has_aria =
        element.attr("aria-label").is_some() || element.attr("aria-labelledby").is_some();
    let id = element.literal("id").map(str::to_owned);
    ctx.controls.push(crate::validate::Control {
        name: element.name.clone(),
        span: element.span,
        exempt: false,
        named: wrapped || has_aria,
        id,
    });
}

// ─────────────────────────────── links e mídia ───────────────────────────────

fn check_blank_target(element: &Element, ctx: &mut Ctx) {
    if element.name != "a" || element.literal("target") != Some("_blank") {
        return;
    }
    let Some(attr) = element.attr("rel") else {
        return;
    };
    match &attr.value {
        AttrValue::Literal { text, span } => {
            if !text.split_whitespace().any(|token| token == "noopener") {
                ctx.error(
                    Diag::new(
                        *span,
                        format!("`target=\"_blank\"` com `rel=\"{text}\"` não inclui `noopener`"),
                    )
                    .rule("spec:blank_target_implies_noopener")
                    .hint("use `rel=\"noopener\"`"),
                );
            }
        }
        // `rel` computado não pode ser inspecionado na compilação: o codegen também não sintetiza
        // (o atributo já existe), então a única saída segura é recusar.
        AttrValue::Computed { span, .. } => {
            ctx.error(
                Diag::new(
                    *span,
                    "`rel` computado não garante `noopener` com `target=\"_blank\"`",
                )
                .rule("spec:blank_target_implies_noopener")
                .hint("use `rel=\"noopener noreferrer\"` literal"),
            );
        }
    }
}

fn check_srcset(element: &Element, ctx: &mut Ctx) {
    if element.name != "img" || element.attr("srcset").is_none() {
        return;
    }
    let complete = element.attr("sizes").is_some()
        && element.attr("width").is_some()
        && element.attr("height").is_some();
    if !complete {
        ctx.error(
            Diag::new(element.span, "`srcset` exige `sizes` e `width`/`height`")
                .rule("CLS 0 / spec:srcset_requires_sizes_and_dims")
                .hint("adicione `sizes=(Sizes::new(\"…\"))`"),
        );
    }
}

fn check_media(element: &Element, ctx: &mut Ctx) {
    let plays = matches!(element.name.as_str(), "video" | "audio");
    if plays
        && element.attr("src").is_some()
        && descendants(element).any(|child| child.name == "media_source")
    {
        ctx.error(
            Diag::new(
                element.span,
                format!(
                    "`{}` tem `src` e `media_source`; use um dos dois",
                    element.name
                ),
            )
            .rule("spec:media_source_xor_src")
            .hint("remova o `src` ou os `media_source`"),
        );
    }

    match element.name.as_str() {
        "video" => {
            let has_captions = descendants(element)
                .any(|child| child.name == "track" && child.literal("kind") == Some("captions"));
            if !has_captions {
                ctx.error(
                    Diag::new(element.span, "`video` exige `track kind=\"captions\"`")
                        .rule("WCAG 1.2.2 / spec:media_requires_captions")
                        .hint(
                            "adicione `track kind=\"captions\" src=(Src::asset(…)) srclang=\"pt\" \
                             label=\"Português\";`",
                        ),
                );
            }
        }
        "track"
            if element.literal("kind") == Some("subtitles")
                && element.attr("srclang").is_none() =>
        {
            ctx.error(
                Diag::new(element.span, "`track kind=\"subtitles\"` exige `srclang`")
                    .rule("spec:srclang_required_for_subtitles")
                    .hint("adicione `srclang=\"pt\"`"),
            );
        }
        _ => {}
    }
}

fn check_frame(element: &Element, ctx: &mut Ctx) {
    if element.name != "iframe" {
        return;
    }
    let complete = element.attr("sandbox").is_some() && element.attr("referrerpolicy").is_some();
    if !complete {
        ctx.error(
            Diag::new(element.span, "`iframe` exige `sandbox` e `referrerpolicy`")
                .rule("spec:cross_origin_requires_sandbox")
                .hint("adicione `sandbox=\"allow-scripts\"` e `referrerpolicy=\"no-referrer\"`"),
        );
    }
}

// ─────────────────────────────── input ───────────────────────────────

fn check_input_attributes(element: &Element, ctx: &mut Ctx) {
    if element.name != "input" {
        return;
    }
    let Some(kind) = element.literal("type") else {
        return;
    };
    let Some(extras) = table::input_attrs(kind) else {
        return;
    };
    for attr in &element.attrs {
        let key = attr.key();
        let global = table::GLOBALS.iter().any(|global| global.name == key);
        let aria = key.starts_with("aria-");
        let data = key.starts_with("data-");
        let base = crate::validate::input_base_attr(&key);
        let extra = extras.contains(&key.as_str());
        if !(global || aria || data || base || extra) {
            ctx.error(
                Diag::new(
                    attr.span,
                    format!("`{key}` não existe em `input type=\"{kind}\"`"),
                )
                .rule("spec:attribute_by_input_type")
                .hint(format!("remova `{key}` ou use outro `type`")),
            );
        }
    }
}

// ──────────────────────────── diálogo e estrutura ────────────────────────────

fn check_dialog(element: &Element, ctx: &mut Ctx) {
    if element.name != "dialog" {
        return;
    }
    if let Some(attr) = element.attr("aria-modal") {
        ctx.error(
            Diag::new(
                attr.span,
                "`aria-modal` é derivado de `showModal()`; remova",
            )
            .rule("spec:no_aria_modal")
            .hint("remova o atributo"),
        );
    }
    let named = element.attr("aria-labelledby").is_some() || element.attr("aria-label").is_some();
    if !named {
        ctx.error(
            Diag::new(element.span, "`dialog` exige nome acessível")
                .rule("spec:dialog_modal_semantics")
                .hint("`aria-labelledby=\"…\"` apontando para o título no bloco"),
        );
    }
}

fn check_section(element: &Element, ctx: &mut Ctx) {
    if element.name != "section" {
        return;
    }
    let has_heading = descendants(element).any(|child| is_heading(&child.name));
    let named = element.attr("aria-label").is_some() || element.attr("aria-labelledby").is_some();
    if !has_heading && !named {
        ctx.error(
            Diag::new(element.span, "`section` sem heading e sem nome acessível")
                .rule("spec:section_has_heading_or_name")
                .hint("use `div` ou adicione um heading"),
        );
    }
}

fn check_table(element: &Element, ancestors: &[&'static Spec], ctx: &mut Ctx) {
    match element.name.as_str() {
        "caption" => {
            if element
                .body
                .as_ref()
                .is_none_or(|body| body.items.is_empty())
            {
                ctx.error(
                    Diag::new(element.span, "`caption` não pode ser vazio")
                        .rule("spec:caption_not_empty")
                        .hint("escreva o nome acessível da tabela"),
                );
            }
        }
        "summary" => {
            let headings = descendants(element).filter(|child| is_heading(&child.name));
            if headings.count() > 1 {
                ctx.error(
                    Diag::new(element.span, "`summary` aceita no máximo um heading")
                        .rule("spec:summary_single_heading")
                        .hint("remova o heading extra"),
                );
            }
        }
        "legend" => {
            if element
                .body
                .as_ref()
                .is_none_or(|body| body.items.is_empty())
            {
                ctx.error(
                    Diag::new(element.span, "`legend` não pode ser vazio")
                        .rule("spec:legend_not_empty")
                        .hint("escreva o nome do grupo"),
                );
            }
        }
        "select" => {
            let options: Vec<&Element> = descendant_named(element, "option");
            let selectable = options.iter().any(|option| {
                option.literal("disabled") != Some("true") && option.attr("disabled").is_none()
            });
            if !selectable {
                ctx.error(
                    Diag::new(
                        element.span,
                        "`select` exige ao menos uma `option` selecionável",
                    )
                    .rule("spec:select_has_selectable_option")
                    .hint("adicione `option value=\"…\"` sem `disabled`"),
                );
            }
        }
        "th" => check_scope(element, ancestors, ctx),
        _ => {}
    }
}

fn check_scope(element: &Element, ancestors: &[&'static Spec], ctx: &mut Ctx) {
    let Some(scope) = element.literal("scope") else {
        return;
    };
    let section = ancestors
        .iter()
        .rev()
        .find(|ancestor| matches!(ancestor.name, "thead" | "tbody" | "tfoot"))
        .map(|ancestor| ancestor.name);
    let Some(section) = section else {
        return;
    };
    let matches_section = match section {
        "thead" => matches!(scope, "col" | "colgroup"),
        _ => matches!(scope, "row" | "rowgroup"),
    };
    if matches_section {
        return;
    }
    let span = element.attr("scope").map_or(element.span, |attr| attr.span);
    ctx.error(
        Diag::new(
            span,
            format!("`th scope=\"{scope}\"` não corresponde à seção `{section}`"),
        )
        .rule("spec:scope_matches_section")
        .hint(if section == "thead" {
            "use `scope=\"col\"` em `thead`"
        } else {
            "use `scope=\"row\"` no corpo"
        }),
    );
}

fn check_label(element: &Element, ctx: &mut Ctx) {
    if element.name != "label" {
        return;
    }
    if element.attr("for").is_some() {
        return;
    }
    let wraps = descendants(element).any(|child| LABELABLE.contains(&child.name.as_str()));
    if !wraps {
        ctx.error(
            Diag::new(element.span, "`label` exige `for` ou um controle envolvido")
                .rule("WCAG 1.3.1")
                .hint(
                    "`Field::text(FieldName::new(\"nome\"), Msg::Name, &state)` gera label+id+aria",
                ),
        );
    }
}

fn check_list_item(element: &Element, ancestors: &[&'static Spec], ctx: &mut Ctx) {
    if element.name != "li" {
        return;
    }
    let parent = ancestors.last().map(|ancestor| ancestor.name);
    if matches!(parent, Some("ul" | "ol")) {
        return;
    }
    let message = parent.map_or_else(
        || String::from("`li` só existe dentro de `ul`/`ol`"),
        |name| format!("`<li>` não é permitido dentro de `<{name}>`"),
    );
    ctx.error(
        Diag::new(element.span, message)
            .rule("spec:li_only_in_list")
            .hint("use `<ul>` ou `<ol>`"),
    );
}

// ─────────────────────────────── utilidades ───────────────────────────────

fn is_heading(name: &str) -> bool {
    matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
}

/// Todos os elementos descendentes, atravessando `@if`/`for` e corpos aninhados.
pub(crate) fn descendants(element: &Element) -> impl Iterator<Item = &Element> {
    element.body.iter().flat_map(|body| block_elements(body))
}

fn block_elements(block: &Block) -> Vec<&Element> {
    let mut found = Vec::new();
    collect(block, &mut found);
    found
}

fn collect<'a>(block: &'a Block, found: &mut Vec<&'a Element>) {
    for item in &block.items {
        match item {
            Item::Element(element) => {
                found.push(element);
                if let Some(body) = &element.body {
                    collect(body, found);
                }
            }
            Item::If(node) => {
                collect(&node.then, found);
                if let Some(crate::ast::Otherwise::Block(block)) = &node.otherwise {
                    collect(block, found);
                }
                if let Some(crate::ast::Otherwise::If(inner)) = &node.otherwise {
                    collect_if(inner, found);
                }
            }
            Item::For(node) => collect(&node.body, found),
            Item::Primitive(primitive) => collect(&primitive.children, found),
            Item::Message(_) | Item::Child(_) => {}
        }
    }
}

fn collect_if<'a>(node: &'a crate::ast::IfNode, found: &mut Vec<&'a Element>) {
    collect(&node.then, found);
    match &node.otherwise {
        Some(crate::ast::Otherwise::Block(block)) => collect(block, found),
        Some(crate::ast::Otherwise::If(inner)) => collect_if(inner, found),
        None => {}
    }
}

fn descendant_named<'a>(element: &'a Element, name: &str) -> Vec<&'a Element> {
    descendants(element)
        .filter(|child| child.name == name)
        .collect()
}

/// Há texto/interpolação em qualquer profundidade (o nome acessível visível).
fn has_visible_text(element: &Element) -> bool {
    let Some(body) = &element.body else {
        return false;
    };
    block_has_text(body)
}

fn block_has_text(block: &Block) -> bool {
    block.items.iter().any(|item| match item {
        Item::Primitive(primitive) => block_has_text(&primitive.children),
        Item::Message(_) | Item::Child(_) => true,
        Item::Element(element) => element.body.as_ref().is_some_and(block_has_text),
        Item::If(node) => {
            block_has_text(&node.then) || node.otherwise.as_ref().is_some_and(block_has_text_if)
        }
        Item::For(node) => block_has_text(&node.body),
    })
}

fn block_has_text_if(node: &crate::ast::Otherwise) -> bool {
    match node {
        crate::ast::Otherwise::Block(block) => block_has_text(block),
        crate::ast::Otherwise::If(inner) => {
            block_has_text(&inner.then) || inner.otherwise.as_ref().is_some_and(block_has_text_if)
        }
    }
}

/// `true` se o atributo é universal em qualquer `input` (base de `attribute_by_input_type`).
#[cfg(test)]
mod tests {
    use super::{BY_CONSTRUCTION, CODED, DATA_DRIVEN};
    use crate::table::{self, Element, Scope};

    #[test]
    fn every_block_invariant_is_classified() {
        for (name, scope) in table::INVARIANT_SCOPES {
            if *scope != Scope::Block {
                continue;
            }
            let buckets = usize::from(DATA_DRIVEN.contains(name))
                + usize::from(CODED.contains(name))
                + usize::from(BY_CONSTRUCTION.contains(name));
            assert_eq!(
                buckets, 1,
                "invariante `{name}` precisa estar em exatamente uma lista de rules.rs"
            );
        }
    }

    #[test]
    fn classified_names_are_block_invariants() {
        for name in DATA_DRIVEN.iter().chain(CODED).chain(BY_CONSTRUCTION) {
            assert_eq!(
                table::invariant_scope(name),
                Some(Scope::Block),
                "`{name}` listada em rules.rs mas sem escopo block no spec"
            );
        }
    }

    #[test]
    fn by_construction_invariants_live_in_document_elements() {
        for (name, scope) in table::INVARIANT_SCOPES {
            if *scope != Scope::Block || !BY_CONSTRUCTION.contains(name) {
                continue;
            }
            let carrier: Vec<&Element> = table::ELEMENTS
                .iter()
                .filter(|element| element.invariants.contains(name))
                .collect();
            assert!(
                !carrier.is_empty(),
                "`{name}` em BY_CONSTRUCTION mas nenhum elemento a carrega"
            );
            for element in carrier {
                assert!(
                    element.only.is_some(),
                    "`{name}` em BY_CONSTRUCTION mas `{}` é construído por `html!`",
                    element.name
                );
            }
        }
    }

    #[test]
    fn document_elements_have_only_document_invariants() {
        for element in table::ELEMENTS {
            if element.only.is_none() {
                continue;
            }
            for invariant in element.invariants {
                if table::invariant_scope(invariant) != Some(Scope::Block) {
                    continue;
                }
                let classified = DATA_DRIVEN.contains(invariant)
                    || CODED.contains(invariant)
                    || BY_CONSTRUCTION.contains(invariant);
                assert!(
                    classified,
                    "`{}` carrega `{invariant}` e precisa estar classificado em rules.rs",
                    element.name
                );
            }
        }
    }
}
