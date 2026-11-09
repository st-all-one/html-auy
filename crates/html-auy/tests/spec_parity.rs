//! Paridade entre `spec/elements.toml` (normativo) e a tabela gerada.
//!
//! Cada teste aqui falha quando alguém edita o TOML e esquece `tools/gen_spec.py`, ou quando a
//! tabela promete algo que a spec não diz. É a prova de que a tabela não está velha — e a razão de
//! não existir `build.rs`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use html_auy::spec::{
    ARIA_VALUES, ATTR_VALUES, ELEMENTS, GLOBALS, INPUT_ATTRS, INVARIANT_SCOPES, Only, Scope,
    ValueKind, element,
};

/// Categorias de conteúdo-modelo declaradas em `[meta] categories`.
const CATEGORIES: [&str; 14] = [
    "flow",
    "phrasing",
    "sectioning",
    "heading",
    "form",
    "table",
    "media",
    "metadata",
    "interactive",
    "text",
    "ruby_text",
    "svg_shape",
    "json",
    "none",
];

/// Categorias que, em HTML, são conteúdo de fluxo. O spec declara só a categoria mais específica
/// (`main` é `sectioning`, `table` é `table`), então a alcançabilidade a partir de `body` fecha a
/// transitividade aqui — o vocabulário não muda, só a leitura do conteúdo-modelo.
const FLOW_ABSORBS: [&str; 4] = ["sectioning", "heading", "table", "form"];

/// Atributos que nenhum elemento pode exigir (docs/design/03 §5, docs/design/08 §2).
const FORBIDDEN_ATTRS: [&str; 13] = [
    "style",
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

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[expect(
    clippy::panic,
    reason = "auxiliar de teste: falha com o caminho do arquivo"
)]
fn read(relative: &str) -> String {
    let path = root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: TOML inválido é falha imediata"
)]
fn spec() -> toml::Table {
    read("spec/elements.toml").parse().expect("TOML inválido")
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: spec sem elementos é falha"
)]
fn spec_elements(doc: &toml::Table) -> &toml::Table {
    doc["element"].as_table().expect("[element.*] ausente")
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: tipo errado é falha imediata"
)]
fn text_of(value: &toml::Value) -> String {
    value.as_str().expect("esperava string").to_owned()
}

/// Lista declarada: ausente = vazia; string sozinha = lista de um.
fn list(table: &toml::Table, key: &str) -> Vec<String> {
    match table.get(key) {
        None => Vec::new(),
        Some(toml::Value::Array(items)) => items.iter().map(text_of).collect(),
        Some(other) => vec![text_of(other)],
    }
}

fn sorted(mut items: Vec<String>) -> Vec<String> {
    items.sort();
    items
}

fn as_refs(items: &[String]) -> Vec<&str> {
    items.iter().map(String::as_str).collect()
}

fn flag(table: &toml::Table, key: &str) -> bool {
    table
        .get(key)
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
}

/// Atributos cujo nome coincide com o de um elemento deste vocabulário (`abbr title`
/// convive com `<title>`): a ambiguidade é resolvida pela posição, não pelo nome.
const AMBIGUOUS: [&str; 5] = ["cite", "form", "label", "span", "title"];

fn expected_tag(name: &str) -> &str {
    match name {
        "svg_title" => "title",
        "media_source" => "source",
        other => other,
    }
}

/// Forma dos atributos globais: `required`/`allowed` de um elemento + `GLOBALS` + `aria-*`.
fn effective_attrs(spec_el: &toml::Table) -> Vec<String> {
    let mut attrs = list(spec_el, "required");
    attrs.extend(list(spec_el, "allowed"));
    attrs.extend(GLOBALS.iter().map(|global| global.name.to_owned()));
    attrs.extend(ARIA_VALUES.iter().map(|aria| aria.name.to_owned()));
    attrs
}

#[test]
fn elements_match_toml_field_by_field() {
    let doc = spec();
    let els = spec_elements(&doc);
    assert_eq!(els.len(), ELEMENTS.len(), "quantidade de elementos");
    assert_eq!(els.len(), 111, "o vocabulário declarado é de 111 elementos");

    for (name, value) in els {
        let spec_el = value.as_table().expect("elemento não é tabela");
        let got = element(name).unwrap_or_else(|| panic!("`{name}` não está em ELEMENTS"));

        assert_eq!(got.name, name);
        assert_eq!(got.tag, expected_tag(name), "tag de `{name}`");
        assert_eq!(got.categories, as_refs(&sorted(list(spec_el, "category"))));
        // Ordem declarada é ordem de emissão (docs/design/09 §1): a paridade é sensível a ela.
        assert_eq!(got.required, as_refs(&list(spec_el, "required")));
        assert_eq!(got.allowed, as_refs(&list(spec_el, "allowed")));
        assert_eq!(
            got.order,
            as_refs(&list(spec_el, "order")),
            "order de `{name}`"
        );
        assert_eq!(got.children, as_refs(&sorted(list(spec_el, "children"))));
        assert_eq!(
            got.required_children,
            as_refs(&sorted(list(spec_el, "required_children")))
        );
        let first_child = spec_el
            .get("first_child")
            .map_or("", |value| value.as_str().expect("first_child string"));
        assert_eq!(got.first_child, first_child);
        assert_eq!(
            got.forbidden_children,
            as_refs(&sorted(list(spec_el, "forbidden_children")))
        );
        let only = spec_el.get("only").map(text_of);
        let expected_only = match only.as_deref() {
            None => None,
            Some("shell") => Some(Only::Shell),
            Some("component") => Some(Only::Component),
            Some(other) => panic!("only = {other:?} é inválido"),
        };
        assert_eq!(got.only, expected_only, "only de `{name}`");
        assert_eq!(got.void, flag(spec_el, "void"), "void de `{name}`");
        assert_eq!(
            got.accessible_name,
            spec_el.get("accessible_name").map_or("none", |value| value
                .as_str()
                .expect("accessible_name string"))
        );
        assert_eq!(got.roles, as_refs(&sorted(list(spec_el, "roles"))));
        assert_eq!(
            got.invariants,
            as_refs(&sorted(list(spec_el, "invariants")))
        );
    }
}

#[test]
fn invariant_scopes_match_toml() {
    let doc = spec();
    let declared = doc["invariants"].as_table().expect("[invariants] ausente");
    assert_eq!(
        declared.len(),
        INVARIANT_SCOPES.len(),
        "quantidade de invariantes"
    );

    for (name, scope) in declared {
        let scope = text_of(scope);
        let expected = match scope.as_str() {
            "block" => Scope::Block,
            "type" => Scope::Type,
            "output" => Scope::Output,
            other => panic!("escopo `{other}` inválido"),
        };
        let got = INVARIANT_SCOPES
            .iter()
            .find(|(candidate, _)| *candidate == name)
            .unwrap_or_else(|| panic!("`{name}` não está em INVARIANT_SCOPES"))
            .1;
        assert_eq!(got, expected, "escopo de `{name}`");
    }

    for spec_el in spec_elements(&doc).values() {
        let table = spec_el.as_table().expect("elemento não é tabela");
        for invariant in list(table, "invariants") {
            assert!(
                INVARIANT_SCOPES.iter().any(|(name, _)| *name == invariant),
                "`{invariant}` é usada e não declara escopo em [invariants]"
            );
        }
    }
}

#[test]
fn only_implies_its_invariant() {
    for spec_el in ELEMENTS {
        let Some(only) = spec_el.only else { continue };
        let needed = match only {
            Only::Shell => "shell_only_elements",
            Only::Component => "form_requires_csrf",
        };
        assert!(
            spec_el.invariants.contains(&needed),
            "`{}` é `{only:?}` sem a invariante `{needed}`",
            spec_el.name
        );
    }
}

#[test]
fn structural_shapes_hold() {
    let mut names = BTreeSet::new();
    for spec_el in ELEMENTS {
        assert!(names.insert(spec_el.name), "`{}` duplicado", spec_el.name);
        assert!(
            !spec_el.name.contains('-'),
            "`{}` tem hífen (custom element é proibido)",
            spec_el.name
        );
        assert_eq!(spec_el.name, spec_el.name.to_lowercase(), "nome maiúsculo");
        assert!(
            !spec_el.categories.is_empty(),
            "`{}` sem categoria",
            spec_el.name
        );
        for category in spec_el.categories {
            assert!(
                CATEGORIES.contains(category),
                "`{}`: categoria `{category}` desconhecida",
                spec_el.name
            );
        }
        if spec_el.void {
            assert_eq!(
                spec_el.children,
                ["none"],
                "`{}` é void e declara filhos",
                spec_el.name
            );
            assert!(
                spec_el.required_children.is_empty(),
                "`{}` é void e exige filho",
                spec_el.name
            );
            assert_eq!(spec_el.first_child, "");
        } else {
            assert_ne!(
                spec_el.children,
                ["none"],
                "`{}` não é void e declara children = [\"none\"]",
                spec_el.name
            );
        }
    }
}

#[test]
fn child_references_resolve() {
    for spec_el in ELEMENTS {
        for child in spec_el
            .children
            .iter()
            .chain(spec_el.required_children)
            .chain(core::iter::once(&spec_el.first_child).filter(|name| !name.is_empty()))
        {
            let child: &str = child;
            assert!(
                CATEGORIES.contains(&child) || element(child).is_some(),
                "`{}` referencia `{child}`, que não é categoria nem elemento",
                spec_el.name
            );
            assert!(
                element(child)
                    .is_none_or(|target| target.only.is_none() || target.only == spec_el.only),
                "`{}` permite o elemento de documento `{child}` como filho",
                spec_el.name
            );
        }
        for child in spec_el.forbidden_children {
            assert!(
                CATEGORIES.contains(child) || element(child).is_some(),
                "`{}` proíbe `{child}`, que não é categoria nem elemento",
                spec_el.name
            );
        }
        if !spec_el.first_child.is_empty() {
            assert!(
                child_ok(spec_el, spec_el.first_child),
                "`{}`: first_child `{}` não casa com children {:?}",
                spec_el.name,
                spec_el.first_child,
                spec_el.children
            );
        }
        for child in spec_el.required_children {
            assert!(
                CATEGORIES.contains(child) || element(child).is_some(),
                "`{}`: required_children `{child}` não é categoria nem elemento",
                spec_el.name
            );
            assert!(
                child_ok(spec_el, child),
                "`{}`: required_children `{child}` não casa com children {:?}",
                spec_el.name,
                spec_el.children
            );
        }
    }
}

#[test]
fn required_attributes_are_attributes() {
    for spec_el in ELEMENTS {
        for attr in spec_el.required {
            assert!(
                !ELEMENTS.iter().any(|candidate| candidate.name == *attr)
                    || AMBIGUOUS.contains(attr),
                "`{}`: required contém o ELEMENTO `{attr}` (use required_children)",
                spec_el.name
            );
            assert!(
                !FORBIDDEN_ATTRS.contains(attr),
                "`{}`: required contém atributo proibido `{attr}`",
                spec_el.name
            );
        }
        for attr in spec_el.allowed {
            assert!(
                !FORBIDDEN_ATTRS.contains(attr),
                "`{}`: allowed contém atributo proibido `{attr}`",
                spec_el.name
            );
            assert!(
                !attr.starts_with("on"),
                "`{}`: allowed contém handler `{attr}`",
                spec_el.name
            );
            assert_ne!(
                attr, &"role",
                "`{}`: `role` livre (use roles = [...])",
                spec_el.name
            );
        }
    }
}

#[test]
fn roles_are_closed_and_typed() {
    for spec_el in ELEMENTS {
        if spec_el.roles.is_empty() {
            assert_ne!(
                spec_el.name, "div",
                "`div` precisa declarar os papéis de região viva"
            );
            continue;
        }
        assert!(
            matches!(spec_el.name, "div" | "dialog"),
            "`{}` declara roles; só `div`/`dialog` podem (D4)",
            spec_el.name
        );
        for role in spec_el.roles {
            let mut chars = role.chars();
            let first = chars.next().expect("papel vazio");
            assert!(first.is_ascii_lowercase(), "papel `{role}` inválido");
            assert!(
                chars.all(|c| c.is_ascii_lowercase() || c == '-'),
                "papel `{role}` inválido"
            );
        }
    }
}

#[test]
fn boolean_attributes_are_exactly_the_used_ones() {
    let doc = spec();
    let meta = doc["meta"].as_table().expect("[meta] ausente");
    let declared: BTreeSet<String> = list(meta, "boolean_attributes").into_iter().collect();
    assert_eq!(declared.len(), 15, "os 15 booleanos de [meta]");

    let used: BTreeSet<String> = ELEMENTS
        .iter()
        .flat_map(|spec_el| spec_el.required.iter().chain(spec_el.allowed))
        .filter(|attr| declared.iter().any(|name| name == *attr))
        .map(|attr| (*attr).to_owned())
        .collect();
    assert_eq!(
        used, declared,
        "booleanos declarados e não usados (ou vice-versa)"
    );
}

#[test]
fn attr_values_reference_real_attributes() {
    let doc = spec();
    let els = spec_elements(&doc);
    for entry in ATTR_VALUES {
        let owners: Vec<&toml::Table> = if entry.element.is_empty() {
            els.values().filter_map(toml::Value::as_table).collect()
        } else {
            let owner = els
                .get(entry.element)
                .unwrap_or_else(|| panic!("attr_values cita `{}`, que não existe", entry.element))
                .as_table()
                .expect("elemento não é tabela");
            vec![owner]
        };
        for owner in owners {
            let attrs = effective_attrs(owner);
            assert!(
                attrs.iter().any(|candidate| candidate == entry.attr),
                "attr_values `{}`: `{}` não está em required ∪ allowed ∪ globals",
                entry.element,
                entry.attr
            );
        }
        if entry.kind == ValueKind::Tokens {
            assert!(
                !entry.values.is_empty(),
                "attr_values `{}`: lista de tokens vazia",
                entry.attr
            );
        }
        for value in entry.values {
            assert!(
                !value.contains(char::is_whitespace) || entry.kind == ValueKind::Tokens,
                "attr_values `{}`: valor `{value}` com espaço fora de lista de tokens",
                entry.attr
            );
        }
    }

    for spec_el in ELEMENTS {
        for attr in spec_el.order {
            let declared = spec_el
                .required
                .iter()
                .chain(spec_el.allowed)
                .any(|c| c == attr);
            assert!(
                declared,
                "`{}`: order contém `{attr}` fora de required ∪ allowed",
                spec_el.name
            );
        }
        assert_eq!(
            spec_el.order.iter().collect::<BTreeSet<_>>().len(),
            spec_el.order.len(),
            "`{}`: order repete um nome",
            spec_el.name
        );
    }
}

#[test]
fn input_attributes_cover_the_input_types() {
    let types = ATTR_VALUES
        .iter()
        .find(|entry| entry.element == "input" && entry.attr == "type")
        .expect("input.type sem enum");
    for input_type in types.values {
        assert!(
            INPUT_ATTRS.iter().any(|(name, _)| name == input_type),
            "`{input_type}` não declara atributos em [input_attributes]"
        );
    }
    assert_eq!(INPUT_ATTRS.len(), types.values.len());
    assert_eq!(INPUT_ATTRS.len(), 22, "os 22 tipos de input");
}

#[test]
fn vocabulary_is_reachable_from_body() {
    let root = element("html").expect("`html` ausente");
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut queue: Vec<&'static html_auy::spec::Element> = vec![root];
    while let Some(current) = queue.pop() {
        if seen.insert(current.name) {
            queue.extend(
                ELEMENTS
                    .iter()
                    .filter(|child| child.name != current.name && reaches(current, child)),
            );
        }
    }

    for needed in ["main", "section", "p", "ul", "table", "form"] {
        assert!(
            seen.contains(needed),
            "`{needed}` não é alcançável a partir de `html` (o `body` deveria alcançá-lo)"
        );
    }
    for spec_el in ELEMENTS {
        assert!(
            seen.contains(spec_el.name),
            "`{}` não é alcançável a partir de `body`",
            spec_el.name
        );
    }
}

/// O filho casa `children` do pai por nome ou por categoria (é a regra do conteúdo-modelo).
fn child_ok(parent: &html_auy::spec::Element, child: &str) -> bool {
    if parent.children.contains(&child) {
        return true;
    }
    element(child).is_some_and(|target| {
        target
            .categories
            .iter()
            .any(|category| parent.children.contains(category))
    })
}

/// Um pai alcança o filho se o filho casa `children` por nome ou categoria, e não está em
/// `forbidden_children`.
fn reaches(parent: &html_auy::spec::Element, child: &html_auy::spec::Element) -> bool {
    let forbidden = parent
        .forbidden_children
        .iter()
        .any(|entry| *entry == child.name || child.categories.contains(entry));
    if forbidden {
        return false;
    }
    if parent.children.contains(&child.name) {
        return true;
    }
    let by_category = child
        .categories
        .iter()
        .any(|category| parent.children.contains(category));
    let by_flow = child
        .categories
        .iter()
        .any(|category| FLOW_ABSORBS.contains(category))
        && parent.children.contains(&"flow");
    by_category || by_flow
}

#[test]
fn tags_only_differ_where_documented() {
    for spec_el in ELEMENTS {
        match spec_el.name {
            "svg_title" => assert_eq!(spec_el.tag, "title"),
            "media_source" => assert_eq!(spec_el.tag, "source"),
            name => assert_eq!(spec_el.tag, name, "`{name}` não pode renomear a tag"),
        }
    }
}

#[test]
fn docs_declare_the_same_counts() {
    let vocab: Vec<u32> = read("docs/design/03-vocabulario-de-elementos.md")
        .lines()
        .filter(|line| line.contains("declara **"))
        .filter_map(declared_number)
        .collect();
    assert_eq!(
        vocab.first().copied(),
        Some(111),
        "docs/design/03 declara outro número de elementos"
    );

    let scoped = docs_scope_counts();
    for (scope, real) in [
        ("block", count(Scope::Block)),
        ("type", count(Scope::Type)),
        ("output", count(Scope::Output)),
    ] {
        let declared = scoped
            .iter()
            .find(|(name, _)| *name == scope)
            .map_or(0, |(_, number)| *number);
        assert_eq!(declared, real, "docs/design/04 §4.0: contagem de `{scope}`");
    }
}

/// Contagens de docs/design/04 §4.0: o último número em negrito de cada linha de escopo.
fn docs_scope_counts() -> Vec<(&'static str, u32)> {
    let doc4 = read("docs/design/04-validacao.md");
    let mut scoped = Vec::new();
    for scope in ["block", "type", "output"] {
        let row = doc4
            .lines()
            .find(|line| line.starts_with("| `") && line.contains(&format!("`{scope}`")));
        if let Some(number) = row.and_then(declared_number) {
            scoped.push((scope, number));
        }
    }
    scoped
}

fn count(scope: Scope) -> u32 {
    u32::try_from(
        INVARIANT_SCOPES
            .iter()
            .filter(|(_, candidate)| *candidate == scope)
            .count(),
    )
    .unwrap_or(0)
}

/// Último número entre `**` da linha (`declara **111**` → 111).
fn declared_number(line: &str) -> Option<u32> {
    let mut rest = line;
    let mut found = None;
    while let Some(marker) = rest.find("**") {
        let opened = rest.get(marker..)?.strip_prefix("**")?;
        let close = opened.find("**")?;
        if let Some(number) = opened.get(..close).and_then(|body| body.parse().ok()) {
            found = Some(number);
        }
        rest = opened.get(close..)?.strip_prefix("**")?;
    }
    found
}

#[test]
fn every_used_invariant_is_documented() {
    let doc4 = read("docs/design/04-validacao.md");
    for (name, _) in INVARIANT_SCOPES {
        assert!(
            doc4.contains(&format!("`{name}`")),
            "invariante `{name}` não está documentada em docs/design/04 §4.1"
        );
    }
}
