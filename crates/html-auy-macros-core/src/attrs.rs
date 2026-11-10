//! Predicados de atributo que a validação e o codegen **precisam** responder igual.

use crate::table;

/// Atributos cujo valor é uma expressão tipada — literal é erro (E9/E41, §5.0).
pub(crate) const TYPED: [&str; 9] = [
    "href", "src", "poster", "srcset", "sizes", "alt", "datetime", "width", "height",
];

/// Presença como booleano: os `BOOLEAN_ATTRIBUTES` da spec e os globais `Presence` (`inert`).
pub(crate) fn is_boolean(name: &str) -> bool {
    table::BOOLEAN_ATTRIBUTES.contains(&name)
        || table::GLOBALS
            .iter()
            .any(|global| global.name == name && matches!(global.kind, table::ValueKind::Presence))
}

/// Valor tipado: `width`/`height` só onde o plano tipa (`img`/`video`/`iframe`); em SVG são
/// genéricos (`rect` exige literal `width`/`height`).
pub(crate) fn is_typed(element: &str, name: &str) -> bool {
    if matches!(name, "width" | "height") {
        return matches!(element, "img" | "video" | "iframe" | "source");
    }
    TYPED.contains(&name)
}

/// Caminho interno literal aceito pelo macro; espelha a gramática de `LUrl::new`.
///
/// A cópia existe porque o macro não depende do crate `html-auy` (seria um ciclo). Os testes de
/// `tests/values.rs` e `tests/ergonomics.rs` fixam os dois lados.
pub(crate) fn literal_path(input: &str) -> bool {
    if !input.starts_with('/') || input.starts_with("//") {
        return false;
    }
    let rejected = input.chars().any(|ch| {
        ch.is_control()
            || ch.is_ascii_whitespace()
            || matches!(ch, ':' | '\\' | '"' | '<' | '>' | '`')
    });
    if rejected {
        return false;
    }
    !input.split('/').any(|segment| {
        segment == "."
            || segment == ".."
            || segment.eq_ignore_ascii_case(".%2e")
            || segment.eq_ignore_ascii_case("%2e.")
            || segment.eq_ignore_ascii_case("%2e%2e")
    })
}

/// Fragmento literal aceito pelo macro; espelha `Fragment::new`.
pub(crate) fn literal_fragment(input: &str) -> bool {
    input.strip_prefix('#').is_some_and(|name| {
        name.is_empty()
            || name
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    })
}
