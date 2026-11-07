//! Porte de componentes de `docs/20` §5: `picture` e `select` em `html!`.

use html_auy::{
    Alt, Asset, FieldName, HeightPx, LUrl, MediaPath, Sizes, Src, SrcSet, Variant, WidthPx, html,
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
    Media(LUrl::from_string(String::from("/media/capa.avif")).expect("caminho interno"))
}

struct Avif;

impl Variant for Avif {
    fn src(&self) -> &'static str {
        "/media/capa-720.avif"
    }

    fn width(&self) -> u32 {
        720
    }
}

#[test]
fn picture_with_source_and_sizes() {
    let asset = Asset::style("/assets/a.css").expect("asset");
    let variants = [Avif];
    let drawn = html! {
        picture {
            source type="image/avif" srcset=(SrcSet::from_variants(&variants)) sizes=(Sizes::new("100vw"));
            img src=(Src::media(&media())) alt=(Alt::new("capa")) width=(WidthPx::new(720)) height=(HeightPx::new(480));
        }
    };
    let _ = asset;
    insta::assert_snapshot!("port_picture", drawn.as_str());
}

#[test]
fn select_with_named_field() {
    let drawn = html! {
        label {
            select name=(FieldName::new("secao")) {
                option value="a" { (html_auy::Text::new("A")) }
                option value="b" selected=(true) { (html_auy::Text::new("B")) }
            }
        }
    };
    insta::assert_snapshot!("port_select", drawn.as_str());
}

struct Webp;

enum Capa {
    Avif720,
    Webp1080,
}

impl Variant for Capa {
    fn src(&self) -> &'static str {
        match self {
            Self::Avif720 => "/media/capa-720.avif",
            Self::Webp1080 => "/media/capa-1080.webp",
        }
    }

    fn width(&self) -> u32 {
        match self {
            Self::Avif720 => 720,
            Self::Webp1080 => 1080,
        }
    }
}

impl Variant for Webp {
    fn src(&self) -> &'static str {
        "/media/capa-1080.webp"
    }

    fn width(&self) -> u32 {
        1080
    }
}

#[test]
fn picture_art_direction_with_media_and_sizes() {
    let avif = [Avif];
    let webp = [Webp];
    let fallback = [Capa::Avif720, Capa::Webp1080];
    let drawn = html! {
        picture {
            source type="image/avif" media="(min-width: 48em)"
                srcset=(SrcSet::from_variants(&avif))
                sizes=(Sizes::new("(min-width: 48em) 50vw, 100vw"))
                width=(WidthPx::new(1080)) height=(HeightPx::new(720));
            source type="image/webp" media="(min-width: 48em)"
                srcset=(SrcSet::from_variants(&webp))
                sizes=(Sizes::new("(min-width: 48em) 50vw, 100vw"));
            img src=(Src::media(&media()))
                srcset=(SrcSet::from_variants(&fallback))
                sizes=(Sizes::new("(min-width: 48em) 50vw, 100vw"))
                alt=(Alt::new("capa"))
                width=(WidthPx::new(720)) height=(HeightPx::new(480))
                loading="lazy" decoding="async" fetchpriority="low";
        }
    };
    insta::assert_snapshot!("port_picture_art_direction", drawn.as_str());
}

#[test]
fn picture_source_behind_if_keeps_order_and_fallback() {
    let with_avif = false;
    let avif = [Avif];
    let drawn = html! {
        picture {
            @if with_avif {
                source type="image/avif"
                    srcset=(SrcSet::from_variants(&avif))
                    sizes=(Sizes::new("100vw"));
            }
            img src=(Src::media(&media()))
                alt=(Alt::new("capa"))
                width=(WidthPx::new(720)) height=(HeightPx::new(480))
                loading="lazy" decoding="async";
        }
    };
    insta::assert_snapshot!("port_picture_if", drawn.as_str());
}
