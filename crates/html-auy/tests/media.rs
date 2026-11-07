//! Mídia: dimensões e `alt` obrigatórios, `srcset`+`sizes` acoplados, vídeo/áudio com faixas.

#![expect(
    clippy::expect_used,
    reason = "teste: entradas fixas e válidas; o panic é diagnóstico"
)]

use html_auy::hint::FetchPriority;
use html_auy::{
    Alt, Audio, HeightPx, Img, Loading, MediaPreload, Picture, Sizes, Source, Src, SrcSet, Text,
    Track, TrackKind, Variant, Video, WidthPx,
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

fn src(value: &str) -> Src<'_> {
    Src::parse(value).expect("src válido")
}

fn responsive() -> (SrcSet<'static>, Sizes<'static>) {
    static VARIANTS: [V; 2] = [V("/a-640.png", 640), V("/a-1280.png", 1280)];
    let srcset = SrcSet::from_variants(&VARIANTS);
    (srcset, Sizes::new("(min-width: 600px) 50vw, 100vw"))
}

#[test]
fn img_emits_required_attributes_in_canonical_order() {
    let img = Img::new(
        src("/hero.png"),
        Alt::new("Frota azul"),
        WidthPx::new(1200),
        HeightPx::new(630),
    );
    let s = img.render();
    assert!(
        s.as_str().starts_with(
            "<img src=\"/hero.png\" alt=\"Frota azul\" width=\"1200\" height=\"630\">"
        )
    );
}

#[test]
fn img_responsive_emits_both_or_neither() {
    let (srcset, sizes) = responsive();
    let img = Img::new(
        src("/hero.png"),
        Alt::decorative(),
        WidthPx::new(1200),
        HeightPx::new(630),
    )
    .responsive(srcset, sizes);
    let s = img.render();
    assert!(s.as_str().contains(
        "srcset=\"/a-640.png 640w, /a-1280.png 1280w\" sizes=\"(min-width: 600px) 50vw, 100vw\""
    ));

    let empty = SrcSet::default();
    let img = Img::new(
        src("/hero.png"),
        Alt::decorative(),
        WidthPx::new(1),
        HeightPx::new(1),
    )
    .responsive(empty, Sizes::new("100vw"));
    assert!(!img.render().as_str().contains("srcset="));
}

#[test]
fn img_lazy_and_fetchpriority() {
    let img = Img::new(
        src("/below.png"),
        Alt::new(""),
        WidthPx::new(1),
        HeightPx::new(1),
    )
    .lazy()
    .fetchpriority(FetchPriority::Low);
    let s = img.render();
    assert!(s.as_str().contains("loading=\"lazy\""));
    assert!(s.as_str().contains("fetchpriority=\"low\""));
    assert!(s.as_str().contains("alt=\"\""));
}

#[test]
fn img_escapes_alt() {
    let img = Img::new(
        src("/x.png"),
        Alt::new("\"<script>"),
        WidthPx::new(1),
        HeightPx::new(1),
    );
    let s = img.render();
    assert!(!s.as_str().contains("<script>"));
    assert!(!s.as_str().contains("\"\"<"));
}

#[test]
fn picture_keeps_source_order_then_fallback() {
    let (srcset, sizes) = responsive();
    let source = Source::new(srcset, sizes)
        .expect("fonte")
        .media(html_auy::Media::try_new("(min-width: 600px)").expect("media"))
        .mime(html_auy::AttrVal::new("image/png"));
    let fallback = Img::new(
        src("/fallback.png"),
        Alt::new("Frota"),
        WidthPx::new(800),
        HeightPx::new(600),
    );
    let s = Picture::new(fallback).source(source).render();
    let text = s.as_str();
    assert!(text.starts_with("<picture><source srcset=\""));
    assert!(text.contains("media=\"(min-width: 600px)\" type=\"image/png\""));
    assert!(text.contains("</source>") || text.contains("><img"));
    assert!(text.ends_with("</picture>"));
    assert!(text.rfind("<img").expect("img") > text.find("<source").expect("source"));
}

#[test]
fn source_requires_both_srcset_and_sizes() {
    assert!(Source::new(SrcSet::default(), Sizes::new("100vw")).is_none());
    let (srcset, _) = responsive();
    assert!(Source::new(srcset, Sizes::new("")).is_none());
}

#[test]
fn video_requires_captions_and_emits_media_attributes() {
    let captions = Track::captions(src("/legendas.vtt"), Text::new("Português"));
    let extra = Track::subtitles(
        src("/subs-en.vtt"),
        Text::new("English"),
        html_auy::Lang::parse("en").expect("lang"),
    )
    .is_default(true);
    let video = Video::new(src("/video.mp4"), captions)
        .track(extra)
        .poster(src("/poster.jpg"))
        .preload(MediaPreload::Metadata)
        .muted(true)
        .looping(true)
        .playsinline(true)
        .dimensions(WidthPx::new(640), HeightPx::new(360))
        .fallback(Text::new("Seu navegador não suporta vídeo."));
    let s = video.render();
    let text = s.as_str();
    assert!(text.starts_with("<video src=\"/video.mp4\" controls preload=\"metadata\" poster=\"/poster.jpg\" muted loop playsinline width=\"640\" height=\"360\">"));
    assert!(text.contains("<track kind=\"captions\" src=\"/legendas.vtt\" label=\"Português\">"));
    assert!(text.contains("kind=\"subtitles\""));
    assert!(text.contains("srclang=\"en\""));
    assert!(text.contains("default>"));
    assert!(text.ends_with("Seu navegador não suporta vídeo.</video>"));
}

#[test]
fn video_controls_can_be_disabled() {
    let captions = Track::captions(src("/c.vtt"), Text::new("Legendas"));
    let s = Video::new(src("/v.mp4"), captions).controls(false).render();
    assert!(!s.as_str().contains(" controls"));
    assert!(s.as_str().starts_with("<video src=\"/v.mp4\""));
}

#[test]
fn audio_tracks_and_fallback() {
    let track = Track::new(
        src("/audio.vtt"),
        TrackKind::Descriptions,
        Text::new("Descrição"),
    );
    let s = Audio::new(src("/podcast.mp3"))
        .track(track)
        .preload(MediaPreload::Auto)
        .looping(true)
        .fallback(Text::new("Sem áudio."))
        .render();
    let text = s.as_str();
    assert!(text.contains("<audio src=\"/podcast.mp3\" controls preload=\"auto\" loop>"));
    assert!(text.contains("kind=\"descriptions\""));
    assert!(text.ends_with("Sem áudio.</audio>"));
}

#[test]
fn track_subtitles_carries_srclang() {
    let track = Track::subtitles(
        src("/pt.vtt"),
        Text::new("Português"),
        html_auy::Lang::parse("pt-BR").expect("lang"),
    );
    let s = track.render();
    assert!(s.as_str().contains("kind=\"subtitles\""));
    assert!(s.as_str().contains("srclang=\"pt-BR\""));
    assert!(s.as_str().contains("label=\"Português\""));
}

#[test]
fn loading_and_preload_values_are_stable() {
    assert_eq!(Loading::Eager.as_str(), "eager");
    assert_eq!(Loading::Lazy.as_str(), "lazy");
    assert_eq!(MediaPreload::None.as_str(), "none");
    assert_eq!(MediaPreload::Metadata.as_str(), "metadata");
    assert_eq!(MediaPreload::Auto.as_str(), "auto");
}
