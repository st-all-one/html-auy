//! Aceite: página com imagem responsiva, `picture`, vídeo e áudio passa no audit e mantém contratos.

#![expect(
    clippy::expect_used,
    reason = "teste de aceite: entradas fixas e válidas; o panic é diagnóstico"
)]

use html_auy::hint::FetchPriority;
use html_auy::{
    Alt, Audio, HeightPx, Img, Lang, MediaPreload, Page, Picture, ShellData, Sizes, Source, Src,
    SrcSet, Text, Track, Variant, Video, View, WidthPx, html,
};

struct V(&'static str, u32);

impl Variant for V {
    fn src(&self) -> &str {
        self.0
    }

    fn width(&self) -> u32 {
        self.1
    }
}

static VARIANTS: [V; 2] = [
    V("/media/hero-640.avif", 640),
    V("/media/hero-1280.avif", 1280),
];

fn src(value: &str) -> Src<'_> {
    Src::parse(value).expect("src válido")
}

fn lang() -> Lang {
    Lang::parse("pt-BR").expect("lang")
}

fn page() -> Page {
    let srcset = SrcSet::from_variants(&VARIANTS);
    let sizes = Sizes::new("(min-width: 48em) 50vw, 100vw");
    let hero = Img::new(
        src("/media/hero-1280.avif"),
        Alt::new("Frota no pátio"),
        WidthPx::new(1280),
        HeightPx::new(720),
    )
    .responsive(srcset.clone(), sizes)
    .lazy()
    .fetchpriority(FetchPriority::Low);

    let source = Source::new(srcset, sizes)
        .expect("fonte")
        .media(html_auy::Media::try_new("(min-width: 48em)").expect("media"))
        .mime(html_auy::AttrVal::new("image/avif"));
    let fallback = Img::new(
        src("/media/hero-640.avif"),
        Alt::new("Frota no pátio"),
        WidthPx::new(640),
        HeightPx::new(360),
    );
    let picture = Picture::new(fallback).source(source);

    let captions = Track::captions(src("/media/aula.vtt"), Text::new("Legendas"));
    let video = Video::new(src("/media/aula.mp4"), captions)
        .poster(src("/media/aula.jpg"))
        .preload(MediaPreload::Metadata)
        .dimensions(WidthPx::new(1280), HeightPx::new(720))
        .fallback(Text::new("Seu navegador não suporta vídeo."));
    let audio = Audio::new(src("/media/podcast.mp3"))
        .preload(MediaPreload::None)
        .fallback(Text::new("Seu navegador não suporta áudio."));

    let view = View::new(html! {
        section {
            h2 { Text::new("Mídia") }
            for media in [hero.render(), picture.render(), video.render(), audio.render()] {
                (media)
            }
        }
    });
    Page::new(
        &ShellData::new(Text::new("Mídia"), Text::new("Mídia"), lang()),
        view,
    )
}

#[test]
fn media_page_keeps_every_contract() {
    let drawn = page();
    let body = drawn.markup().as_str();
    assert!(body.contains("<img src=\"/media/hero-1280.avif\""));
    assert!(body.contains("srcset=\"/media/hero-640.avif 640w, /media/hero-1280.avif 1280w\""));
    assert!(body.contains("sizes=\"(min-width: 48em) 50vw, 100vw\""));
    assert!(body.contains("loading=\"lazy\""));
    assert!(body.contains("fetchpriority=\"low\""));
    assert!(body.contains("<picture><source"));
    let picture_start = body.find("<picture>").expect("picture");
    let picture_end = picture_start + body[picture_start..].find("</picture>").expect("fim");
    let picture = &body[picture_start..picture_end];
    assert!(picture.find("<source").expect("source") < picture.find("<img").expect("img"));
    assert!(body.contains("<video src=\"/media/aula.mp4\" controls"));
    assert!(body.contains("<track kind=\"captions\""));
    assert!(body.contains("<audio src=\"/media/podcast.mp3\" controls preload=\"none\">"));
}

#[cfg(feature = "audit")]
#[test]
fn media_page_passes_the_document_audit() {
    assert_eq!(html_auy::audit::check(page().markup().as_str()), Vec::new());
}
