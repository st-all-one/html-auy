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
