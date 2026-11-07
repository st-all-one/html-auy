//! JSON-LD: serialização do consumidor, escape do crate.

use crate::markup::Render;
use alloc::string::String;

/// O consumidor serializa o grafo (ele conhece `serde`/`schemars`; o crate não).
pub trait JsonGraph {
    /// Escreve o JSON em `out`.
    fn write_json(&self, out: &mut String);
}

impl<T: JsonGraph + ?Sized> JsonGraph for &T {
    fn write_json(&self, out: &mut String) {
        (*self).write_json(out);
    }
}

/// Bloco JSON-LD já serializado e escapado.
///
/// O escape acontece na construção: `<` vira `\u003c`, então `</script` é impossível mesmo que o
/// dado contenha essa sequência.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JsonLd {
    json: String,
}

impl JsonLd {
    /// Serializa e escapa o grafo.
    #[must_use]
    pub fn graph<G: JsonGraph>(graph: &G) -> Self {
        let mut raw = String::new();
        graph.write_json(&mut raw);
        let mut json = String::with_capacity(raw.len());
        crate::escape::json_script(&mut json, &raw);
        Self { json }
    }

    /// O corpo já escapado.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.json
    }
}

impl Render for JsonLd {
    fn render_to(&self, out: &mut String) {
        out.push_str(r#"<script type="application/ld+json">"#);
        out.push_str(&self.json);
        out.push_str("</script>");
    }
}
