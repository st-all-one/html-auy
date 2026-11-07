//! Auditoria do HTML **final** (N4): o que só o documento inteiro responde.
//!
//! A macro cobra o bloco e os tipos cobram os dados; sobram as invariantes de documento — quantos
//! `<main>`, se os níveis de título saltam, se os ids repetem, se uma referência ARIA resolve. Elas
//! dependem de montagem (shell + views + modais) e por isso só são verificáveis na saída.
//!
//! A ordem da saída é a pré-ordem do documento (`RcDom` não guarda offset de origem), com a ordem
//! fixa de `RULES` desempatando dois problemas no mesmo nó — determinística por construção.
//!
//! Feature `audit`: sem ela o crate não tem dependência normal nenhuma.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use html5ever::tendril::TendrilSink;
use html5ever::{ParseOpts, parse_document};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

/// Ordem fixa das regras na saída (desempate quando duas caem no mesmo nó).
const RULES: [&str; 13] = [
    "main_unique",
    "single_h1",
    "heading_levels_no_skips",
    "ids_unique",
    "landmark_names_distinct",
    "landmark_names_required",
    "aria_current_unique_per_nav",
    "aria_refs_resolve",
    "popover_target_resolves",
    "no_content_derived_identifiers",
    "no_inline_script",
    "no_style_attribute",
    "no_event_handler",
];

/// Um problema encontrado no documento.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Nome da invariante (`spec/elements.toml`).
    pub rule: &'static str,
    /// O que está errado, em uma linha.
    pub message: String,
}

/// Um elemento visitado, na ordem de documento.
struct Snapshot {
    tag: String,
    attrs: Vec<(String, String)>,
    handle: Handle,
    /// `<nav>` ancestral que contém o elemento, se houver.
    nav: Option<usize>,
}

impl Snapshot {
    fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    fn has(&self, name: &str) -> bool {
        self.attr(name).is_some()
    }
}

struct Emit {
    at: usize,
    order: usize,
    finding: Finding,
}

/// Audita o documento e devolve os problemas em ordem determinística (pré-ordem, regra fixa).
#[must_use]
pub fn check(html: &str) -> Vec<Finding> {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).one(html);
    let elements = collect(&dom.document);

    let mut emits: Vec<Emit> = Vec::new();
    let ids = identifiers(&elements);
    let landmark_names = landmarks(&elements, &ids);

    check_counts(&elements, &mut emits);
    check_headings(&elements, &mut emits);
    check_duplicate_ids(&elements, &mut emits);
    check_landmark_names(&landmark_names, &mut emits);
    check_aria_current(&elements, &mut emits);
    check_aria_refs(&elements, &ids, &mut emits);
    check_popover(&elements, &ids, &mut emits);
    check_identifiers(&elements, &mut emits);
    check_conformance(&elements, &mut emits);

    emits.sort_by_key(|emit| (emit.at, emit.order));
    emits.into_iter().map(|emit| emit.finding).collect()
}

fn rule_order(rule: &str) -> usize {
    RULES
        .iter()
        .position(|known| *known == rule)
        .unwrap_or(RULES.len())
}

fn push(emits: &mut Vec<Emit>, at: usize, rule: &'static str, message: String) {
    emits.push(Emit {
        at,
        order: rule_order(rule),
        finding: Finding { rule, message },
    });
}

/// Pré-ordem iterativa (pilha explícita: documento hostil não derruba a auditoria).
fn collect(document: &Handle) -> Vec<Snapshot> {
    let mut elements = Vec::new();
    let mut stack: Vec<(Handle, Option<usize>)> = Vec::new();
    let mut next_nav = 0usize;
    push_children_with_nav(document, &mut stack, None);
    while let Some((handle, nav)) = stack.pop() {
        let mut child_nav = nav;
        if let NodeData::Element { name, attrs, .. } = &handle.data {
            let tag = name.local.as_ref().to_string();
            if tag == "nav" {
                child_nav = Some(next_nav);
                next_nav = next_nav.saturating_add(1);
            }
            let attrs = attrs
                .borrow()
                .iter()
                .map(|attr| (attr.name.local.as_ref().to_string(), attr.value.to_string()))
                .collect();
            elements.push(Snapshot {
                tag,
                attrs,
                handle: handle.clone(),
                nav,
            });
        }
        push_children_with_nav(&handle, &mut stack, child_nav);
    }
    elements
}

fn push_children_with_nav(
    handle: &Handle,
    stack: &mut Vec<(Handle, Option<usize>)>,
    nav: Option<usize>,
) {
    let children = handle.children.borrow();
    for child in children.iter().rev() {
        stack.push((child.clone(), nav));
    }
}

/// Empilha os filhos em ordem inversa para que a pré-ordem saia da esquerda para a direita.
fn push_children(handle: &Handle, stack: &mut Vec<Handle>) {
    let children = handle.children.borrow();
    for child in children.iter().rev() {
        stack.push(child.clone());
    }
}

fn identifiers(elements: &[Snapshot]) -> BTreeMap<&str, usize> {
    elements
        .iter()
        .enumerate()
        .filter_map(|(index, element)| Some((element.attr("id")?, index)))
        .collect()
}

fn find_id(ids: &BTreeMap<&str, usize>, id: &str) -> Option<usize> {
    ids.get(id).copied()
}

fn text_content(handle: &Handle) -> String {
    let mut text = String::new();
    let mut stack: Vec<Handle> = Vec::new();
    push_children(handle, &mut stack);
    while let Some(node) = stack.pop() {
        if let NodeData::Text { contents } = &node.data {
            text.push_str(contents.borrow().as_ref());
        }
        push_children(&node, &mut stack);
    }
    text
}

/// Landmark que aceita nome (`nav`, `aside`, `section`): posição, tag e nome calculado.
struct Landmark {
    at: usize,
    tag: &'static str,
    name: Option<String>,
}

fn landmarks(elements: &[Snapshot], ids: &BTreeMap<&str, usize>) -> Vec<Landmark> {
    let mut found = Vec::new();
    for (index, element) in elements.iter().enumerate() {
        let tag: &'static str = match element.tag.as_str() {
            "nav" => "nav",
            "aside" => "aside",
            "section" => "section",
            _ => continue,
        };
        found.push(Landmark {
            at: index,
            tag,
            name: accessible_name(element, elements, ids),
        });
    }
    found
}

fn accessible_name(
    element: &Snapshot,
    elements: &[Snapshot],
    ids: &BTreeMap<&str, usize>,
) -> Option<String> {
    if let Some(label) = element.attr("aria-label") {
        return Some(label.to_string());
    }
    let labelledby = element.attr("aria-labelledby")?;
    let mut name = String::new();
    for token in labelledby.split_whitespace() {
        if let Some(index) = find_id(ids, token)
            && let Some(target) = elements.get(index)
        {
            // Calculado sob demanda: só o alvo nomeado paga pelo percurso.
            name.push_str(&text_content(&target.handle));
        }
    }
    if name.is_empty() { None } else { Some(name) }
}

// ─────────────────────────────── regras ───────────────────────────────

fn check_counts(elements: &[Snapshot], emits: &mut Vec<Emit>) {
    for (rule, tag) in [("main_unique", "main"), ("single_h1", "h1")] {
        let found: Vec<usize> = elements
            .iter()
            .enumerate()
            .filter(|(_, element)| element.tag == tag)
            .map(|(index, _)| index)
            .collect();
        if found.len() == 1 {
            continue;
        }
        let at = found.get(1).copied().unwrap_or(0);
        push(
            emits,
            at,
            rule,
            format!(
                "documento tem {} elementos `<{tag}>` (exige exatamente 1)",
                found.len()
            ),
        );
    }
}

fn heading_level(tag: &str) -> Option<u8> {
    match tag {
        "h1" => Some(1),
        "h2" => Some(2),
        "h3" => Some(3),
        "h4" => Some(4),
        "h5" => Some(5),
        "h6" => Some(6),
        _ => None,
    }
}

fn check_headings(elements: &[Snapshot], emits: &mut Vec<Emit>) {
    let mut previous: Option<u8> = None;
    for (index, element) in elements.iter().enumerate() {
        let Some(level) = heading_level(&element.tag) else {
            continue;
        };
        if let Some(previous) = previous
            && level > previous.saturating_add(1)
        {
            push(
                emits,
                index,
                "heading_levels_no_skips",
                format!(
                    "`{}` depois de `h{previous}` pula o nível {}",
                    element.tag,
                    previous.saturating_add(1)
                ),
            );
        }
        previous = Some(level);
    }
}

fn check_duplicate_ids(elements: &[Snapshot], emits: &mut Vec<Emit>) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (index, element) in elements.iter().enumerate() {
        let Some(id) = element.attr("id") else {
            continue;
        };
        if !seen.insert(id) {
            push(
                emits,
                index,
                "ids_unique",
                format!("`id=\"{id}\"` aparece mais de uma vez"),
            );
        }
    }
}

fn check_landmark_names(landmarks: &[Landmark], emits: &mut Vec<Emit>) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for landmark in landmarks {
        let Some(name) = &landmark.name else {
            continue;
        };
        if !seen.insert(name.as_str()) {
            push(
                emits,
                landmark.at,
                "landmark_names_distinct",
                format!("dois landmarks com o mesmo nome acessível `{name}`"),
            );
        }
    }
    for tag in ["nav", "aside"] {
        let count = landmarks
            .iter()
            .filter(|landmark| landmark.tag == tag)
            .count();
        if count < 2 {
            continue;
        }
        for landmark in landmarks {
            if landmark.tag == tag && landmark.name.is_none() {
                push(
                    emits,
                    landmark.at,
                    "landmark_names_required",
                    format!("`{tag}` sem nome acessível: há {count} `{tag}` no documento"),
                );
            }
        }
    }
}

fn check_aria_current(elements: &[Snapshot], emits: &mut Vec<Emit>) {
    let mut seen: BTreeMap<usize, usize> = BTreeMap::new();
    for (index, element) in elements.iter().enumerate() {
        let Some(current) = element.attr("aria-current") else {
            continue;
        };
        if current.eq_ignore_ascii_case("false") {
            continue;
        }
        let Some(nav) = element.nav else {
            continue;
        };
        if seen.insert(nav, index).is_some() {
            push(
                emits,
                index,
                "aria_current_unique_per_nav",
                "mais de um `aria-current` no mesmo `<nav>`".to_string(),
            );
        }
    }
}

fn check_aria_refs(elements: &[Snapshot], ids: &BTreeMap<&str, usize>, emits: &mut Vec<Emit>) {
    for (index, element) in elements.iter().enumerate() {
        for attr in ["aria-labelledby", "aria-describedby", "aria-controls"] {
            let Some(value) = element.attr(attr) else {
                continue;
            };
            for token in value.split_whitespace() {
                if find_id(ids, token).is_none() {
                    push(
                        emits,
                        index,
                        "aria_refs_resolve",
                        format!("`{attr}=\"{token}\"` referencia um id que não existe"),
                    );
                }
            }
        }
    }
}

fn check_popover(elements: &[Snapshot], ids: &BTreeMap<&str, usize>, emits: &mut Vec<Emit>) {
    for (index, element) in elements.iter().enumerate() {
        let Some(target) = element.attr("popovertarget") else {
            continue;
        };
        let resolves = find_id(ids, target)
            .and_then(|target_index| elements.get(target_index))
            .is_some_and(Snapshot::has_popover);
        if !resolves {
            push(
                emits,
                index,
                "popover_target_resolves",
                format!("`popovertarget=\"{target}\"` não resolve para um elemento com `popover`"),
            );
        }
    }
}

/// Extensão de `Snapshot` para a checagem de `popover`.
impl Snapshot {
    fn has_popover(&self) -> bool {
        self.has("popover")
    }
}

fn check_identifiers(elements: &[Snapshot], emits: &mut Vec<Emit>) {
    for (index, element) in elements.iter().enumerate() {
        for attr in ["id", "name"] {
            let Some(value) = element.attr(attr) else {
                continue;
            };
            let valid = if attr == "id" {
                valid_identifier(value)
            } else {
                valid_name(value)
            };
            if !valid {
                push(
                    emits,
                    index,
                    "no_content_derived_identifiers",
                    format!("`{attr}=\"{value}\"` não é identificador estável"),
                );
            }
        }
    }
}

fn valid_identifier(value: &str) -> bool {
    let bytes = value.as_bytes();
    let Some(first) = bytes.first() else {
        return false;
    };
    if !first.is_ascii_alphabetic() || bytes.len() > 64 {
        return false;
    }
    bytes
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

/// O `name` de campo aceita a gramática de `FieldName` (`sections[0][campo]`), não a de `id`.
fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '[' | ']' | '.'))
}

fn check_conformance(elements: &[Snapshot], emits: &mut Vec<Emit>) {
    for (index, element) in elements.iter().enumerate() {
        if element.tag == "script" && !element.has("src") {
            let json_ld = element
                .attr("type")
                .is_some_and(|kind| kind == "application/ld+json");
            if !json_ld {
                push(
                    emits,
                    index,
                    "no_inline_script",
                    String::from("`<script>` sem `src` e sem `application/ld+json`"),
                );
            }
        }
        if element.has("style") {
            push(
                emits,
                index,
                "no_style_attribute",
                String::from("`style=` em elemento; estilo é token"),
            );
        }
        for (name, _) in &element.attrs {
            if is_event_handler(name) {
                push(
                    emits,
                    index,
                    "no_event_handler",
                    format!("handler inline `{name}`; comportamento é asset declarado"),
                );
            }
        }
    }
}

fn is_event_handler(name: &str) -> bool {
    name.len() > 2
        && name.starts_with("on")
        && name
            .bytes()
            .skip(2)
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}
