//! Bytes canônicos: ordem de atributo, presença, omissão e escape (docs/09, docs/25 §7).
//!
//! Cada snapshot é um contrato de saída: mudou a string, mudou o produto.

use html::{
    Alt, Class, ExternalUrl, FieldName, HeightPx, Href, Id, JsonGraph, JsonLd, LUrl, Label, Markup,
    MediaPath, Msg, Offset, Sizes, Src, SrcSet, Text, Timestamp, Tone, Variant, WidthPx, html,
};

struct Media(LUrl<'static>);

impl MediaPath for Media {
    fn path(&self) -> &LUrl<'_> {
        &self.0
    }
}

/// O `Media` do teste: caminho fixo, validado uma vez.
#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: caminho interno fixo"
)]
fn media() -> Media {
    Media(LUrl::from_string(String::from("/media/a.avif")).expect("caminho interno"))
}

struct Avif {
    width: u32,
    src: &'static str,
}

impl Variant for Avif {
    fn src(&self) -> &str {
        self.src
    }

    fn width(&self) -> u32 {
        self.width
    }
}

#[test]
fn img_order() {
    let variants = [
        Avif {
            width: 360,
            src: "/media/a-360.avif",
        },
        Avif {
            width: 720,
            src: "/media/a-720.avif",
        },
    ];
    let markup = html! {
        img src=(Src::media(&media()))
            srcset=(SrcSet::from_variants(&variants))
            sizes=(Sizes::new("(min-width: 48em) 33vw, 100vw"))
            alt=(Alt::new("Gráfico de vendas"))
            width=(WidthPx::new(720))
            height=(HeightPx::new(480));
    };
    insta::assert_snapshot!("img_order", markup.as_str());
}

#[test]
fn blank_target_gets_noopener() {
    let url = ExternalUrl::parse("https://example.org").expect("URL https válida");
    let markup = html! {
        a href=(Href::external(url)) target="_blank" { (Text::new("Exemplo")) }
    };
    insta::assert_snapshot!("blank_target_gets_noopener", markup.as_str());
}

#[test]
fn boolean_is_presence() {
    let on = html! { button type="button" disabled=(true) { (Text::new("Enviar")) } };
    let off = html! { button type="button" disabled=(false) { (Text::new("Enviar")) } };
    insta::assert_snapshot!(
        "boolean_is_presence",
        format!("{}\n{}", on.as_str(), off.as_str())
    );
}

#[test]
fn optional_attribute_is_omitted_when_none() {
    let present: Option<Id<'_>> = Some(Id::new("relatorio"));
    let absent: Option<Id<'_>> = None;
    let with = html! { div id?=(present) { } };
    let without = html! { div id?=(absent) { } };
    insta::assert_snapshot!(
        "optional_attribute_is_omitted_when_none",
        format!("{}\n{}", with.as_str(), without.as_str())
    );
}

#[test]
fn empty_class_is_omitted() {
    let markup = html! { div class=(Class::default()) { } };
    insta::assert_snapshot!("empty_class_is_omitted", markup.as_str());
}

#[test]
fn data_attributes_are_sorted() {
    let markup = html! { div data_zeta="z" data_alpha="a" data_mid="m" { } };
    insta::assert_snapshot!("data_attributes_are_sorted", markup.as_str());
}

#[test]
fn unnamed_svg_is_decorative() {
    let markup = html! { svg { } };
    insta::assert_snapshot!("unnamed_svg_is_decorative", markup.as_str());
}

#[test]
fn time_renders_timestamp() {
    let stamp = Timestamp::new(1_704_164_645, Offset::UTC);
    let markup = html! { time datetime=(stamp) { (Text::new("2 de janeiro")) } };
    insta::assert_snapshot!("time_renders_timestamp", markup.as_str());
}

struct Injection;

impl JsonGraph for Injection {
    fn write_json(&self, out: &mut String) {
        out.push_str(r#"{"name":"</script><script>alert(1)</script>"}"#);
    }
}

#[test]
fn jsonld_escapes_script_close() {
    let markup = Markup::from_render(&JsonLd::graph(&Injection));
    insta::assert_snapshot!("jsonld_escapes_script_close", markup.as_str());
}

#[test]
fn class_tokens_are_canonical() {
    let markup = html! { div class=(Class::of(&[Tone::Warning, Tone::Primary])) { } };
    insta::assert_snapshot!("class_tokens_are_canonical", markup.as_str());
}

#[test]
fn inert_is_presence() {
    let markup = html! { div inert=(true) { (Text::new("x")) } };
    insta::assert_snapshot!("inert_presence", markup.as_str());
}

#[test]
fn svg_explicit_attribute_wins_over_synthesis() {
    let markup = html! {
        svg aria_hidden="false" {
            circle cx="1" cy="1" r="1";
        }
    };
    insta::assert_snapshot!("svg_explicit_wins", markup.as_str());
    assert_eq!(
        markup.as_str().matches("aria-hidden").count(),
        1,
        "síntese não duplica atributo explícito"
    );
}

#[test]
fn svg_shape_accepts_literal_dimensions() {
    let markup = html! {
        svg viewBox="0 0 10 10" width="10" height="10" {
            rect x="0" y="0" width="10" height="10";
        }
    };
    insta::assert_snapshot!("svg_literal_dims", markup.as_str());
}

#[derive(Clone, Copy)]
struct Busca;

impl Msg for Busca {
    fn as_str(self) -> &'static str {
        "Busca"
    }
}

#[test]
fn select_with_optgroup_compiles() {
    let markup = html! {
        label for="uf" { (Text::new("UF")) }
        select id="uf" name=(FieldName::new("uf")) {
            optgroup label="Sudeste" {
                option value="sp" { (Text::new("SP")) }
            }
        }
    };
    insta::assert_snapshot!("select_optgroup", markup.as_str());
}

#[test]
fn caption_inside_if_satisfies_table() {
    let markup = html! {
        table {
            @if true {
                caption { (Text::new("Frota")) }
            }
            tbody { tr { td { (Text::new("x")) } } }
        }
    };
    insta::assert_snapshot!("table_caption_in_if", markup.as_str());
}

#[test]
fn input_aria_label_is_a_name() {
    let markup = html! {
        input type="text" name=(FieldName::new("q")) aria_label=(Label::text(Busca));
    };
    insta::assert_snapshot!("input_aria_label", markup.as_str());
}

#[test]
fn pattern_without_flags_compiles() {
    let markup = html! {
        input type="text" name=(FieldName::new("codigo")) pattern="[A-Za-z]+"
            aria_label=(Label::text(Busca));
    };
    insta::assert_snapshot!("pattern_plain", markup.as_str());
}

#[test]
fn svg_shapes_are_self_closing() {
    let markup = html! {
        svg viewBox="0 0 4 1" {
            circle cx="1" cy="1" r="1";
            circle cx="3" cy="1" r="1";
        }
    };
    // Sem `/>`, o parser HTML aninha o segundo `circle` dentro do primeiro.
    assert_eq!(markup.as_str().matches("/>").count(), 2);
    assert!(!markup.as_str().contains("</circle>"));
}
