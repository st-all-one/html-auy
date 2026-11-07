//! Determinismo: mesmos dados ⇒ mesmos bytes, independentemente de como o fonte está escrito.

use html_auy::{Class, LUrl, Markup, MediaPath, Render, Text, Tone, html};

/// Um bloco renderizado duas vezes devolve os mesmos bytes.
#[test]
fn same_block_renders_the_same_twice() {
    let first = html! {
        section class=(Class::of(&[Tone::Primary])) {
            h2 { (Text::new("Título")) }
            p { (Text::new("corpo")) }
        }
    };
    let second = html! {
        section class=(Class::of(&[Tone::Primary])) {
            h2 { (Text::new("Título")) }
            p { (Text::new("corpo")) }
        }
    };
    assert_eq!(first.as_str(), second.as_str());
}

/// A marcação do fonte (espaço, quebra de linha) não entra na saída.
#[test]
fn whitespace() {
    let compact = html! {div class=(Class::of(&[Tone::Primary])){span{(Text::new("x"))}}};
    let spread = html! {
        div class=(Class::of(&[Tone::Primary])) {
            span {
                (Text::new("x"))
            }
        }
    };
    let wide = html! {
        div class=(Class::of(&[Tone::Primary])) {
                span {
                        (Text::new("x"))
                }
        }
    };
    assert_eq!(compact.as_str(), spread.as_str());
    assert_eq!(compact.as_str(), wide.as_str());
}

/// A ordem do fonte dos atributos não entra na saída: vale a ordem canônica.
#[test]
fn attribute_order_is_canonical() {
    let markup = html! {
        img width=(html_auy::WidthPx::new(1)) src=(html_auy::Src::media(&media()))
            alt=(html_auy::Alt::new("a")) height=(html_auy::HeightPx::new(1));
    };
    assert_eq!(
        markup.as_str(),
        r#"<img src="/media/a.png" alt="a" width="1" height="1">"#
    );
}

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
    Media(LUrl::from_string(String::from("/media/a.png")).expect("caminho interno"))
}

/// Nenhuma fixture emite atributo vazio, `style=` nem handler inline.
#[test]
fn no_forbidden_attribute_shape() {
    let markup = html! {
        div class=(Class::of(&[Tone::Neutral])) id="a" {
            img src=(html_auy::Src::media(&media())) alt=(html_auy::Alt::decorative())
                width=(html_auy::WidthPx::new(16)) height=(html_auy::HeightPx::new(16));
        }
    };
    let drawn = markup.as_str();
    // `alt=""` é a marca de imagem decorativa (WCAG 1.1.1); qualquer outro atributo vazio é acidente.
    let without_decorative_alt = drawn.replace("alt=\"\"", "");
    assert!(
        !without_decorative_alt.contains("=\"\""),
        "atributo vazio em {drawn}"
    );
    assert!(!drawn.contains("style="), "`style=` em {drawn}");
    assert!(!drawn.contains(" on"), "handler inline em {drawn}");
}

/// `Render` de um tipo do consumidor mantém o escape de texto.
#[test]
fn custom_render_is_escaped() {
    struct Warning<'a>(&'a str);

    impl Render for Warning<'_> {
        fn render_to(&self, out: &mut String) {
            out.push_str("<p class=\"warning\">");
            html_auy::escape::text(out, self.0);
            out.push_str("</p>");
        }
    }

    let markup = Markup::from_render(&Warning("atenção <agora>"));
    assert_eq!(
        markup.as_str(),
        "<p class=\"warning\">atenção &lt;agora&gt;</p>"
    );
}
