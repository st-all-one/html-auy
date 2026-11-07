//! Vocabulário semântico: marcação de texto, grupos de títulos e `datalist`.

use html::{Fragment, Href, Text, html};

#[test]
fn inline_semantics_emit_native_tags() {
    let drawn = html! {
        p {
            (Text::new("H"))
            sub { (Text::new("2")) }
            (Text::new("O"))
            sup { (Text::new("2")) }
            br;
            data value="42" { (Text::new("resposta")) }
            dfn { (Text::new("termo")) }
            samp { (Text::new("saída")) }
            var { (Text::new("x")) }
            bdi { (Text::new("مرحبا")) }
            bdo dir="rtl" { (Text::new("abc")) }
            wbr;
        }
    };
    let s = drawn.as_str();
    for tag in [
        "<sub>",
        "<sup>",
        "<br>",
        "<data value=\"42\">",
        "<dfn>",
        "<samp>",
        "<var>",
        "<bdi>",
        "<bdo dir=\"rtl\">",
        "<wbr>",
    ] {
        assert!(s.contains(tag), "faltou {tag}: {s}");
    }
}

#[test]
fn ruby_annotations_are_phrasing_content() {
    let drawn = html! {
        p {
            ruby {
                (Text::new("漢"))
                rt { (Text::new("kan")) }
                rp { (Text::new("(")) }
            }
        }
    };
    let s = drawn.as_str();
    assert!(s.contains("<ruby>漢<rt>kan</rt><rp>(</rp></ruby>"), "{s}");
}

#[test]
fn hgroup_requires_a_heading_and_accepts_a_subtitle() {
    let drawn = html! {
        hgroup {
            h2 { (Text::new("Título")) }
            p { (Text::new("Subtítulo")) }
        }
    };
    let s = drawn.as_str();
    assert!(
        s.starts_with("<hgroup><h2>Título</h2><p>Subtítulo</p></hgroup>"),
        "{s}"
    );
}

#[test]
fn datalist_options_and_input_list_resolve() {
    let drawn = html! {
        label for="cidade" { (Text::new("Cidade")) }
        input type="text" name="cidade" id="cidade" list="cidades";
        datalist id="cidades" {
            option value="recife" { (Text::new("Recife")) }
            option value="olinda" { (Text::new("Olinda")) }
        }
    };
    let s = drawn.as_str();
    assert!(s.contains("list=\"cidades\""), "{s}");
    assert!(s.contains("<datalist id=\"cidades\">"), "{s}");
    assert!(
        s.contains("<option value=\"recife\">Recife</option>"),
        "{s}"
    );
}

#[test]
fn h1_is_ordinary_markup_for_custom_shells() {
    let drawn = html! {
        h1 { (Text::new("Portal")) }
    };
    assert_eq!(drawn.as_str(), "<h1>Portal</h1>");
}

#[test]
fn title_is_a_global_attribute() {
    let drawn = html! {
        div title="Marcador" {
            button type="button" title="Alto Contraste" { (Text::new("Contraste")) }
        }
    };
    let s = drawn.as_str();
    assert!(s.contains("title=\"Marcador\""), "{s}");
    assert!(s.contains("title=\"Alto Contraste\""), "{s}");
}

#[test]
fn transparent_anchor_inherits_flow_from_its_container() {
    let href = Href::fragment(Fragment::new("#servicos").expect("fragmento"));
    let drawn = html! {
        div {
            a href=(href) {
                div {
                    h3 { (Text::new("Serviço")) }
                }
            }
        }
    };
    let s = drawn.as_str();
    assert!(s.contains("<a href=\"#servicos\"><div><h3>"), "{s}");
}

#[test]
fn nested_list_is_allowed_inside_li() {
    let drawn = html! {
        ul {
            li {
                (Text::new("pai"))
                ul {
                    li { (Text::new("filho")) }
                }
            }
        }
    };
    assert!(
        drawn
            .as_str()
            .contains("<ul><li>pai<ul><li>filho</li></ul></li></ul>")
    );
}
