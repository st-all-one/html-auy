//! Resource hints: ordem fixa, atributos tipados e deduplicação (docs/06 §2).

use html::{
    Asset, CrossOrigin, ExternalUrl, FetchPriority, Hint, Integrity, Lang, Media, Page, Preload,
    PreloadAs, ReferrerPolicy, ShellData, Sizes, Src, SrcSet, Text, Variant, View, html,
};

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: etiqueta BCP-47 fixa e válida"
)]
fn lang() -> Lang {
    Lang::parse("pt-BR").expect("etiqueta BCP-47 válida")
}

struct VariantPx {
    width: u32,
    src: &'static str,
}

impl Variant for VariantPx {
    fn src(&self) -> &str {
        self.src
    }

    fn width(&self) -> u32 {
        self.width
    }
}

#[test]
fn hints_follow_fixed_kind_order_and_precede_stylesheets() {
    let hints = vec![
        Hint::Prefetch(Src::parse("/noticias").expect("caminho interno")),
        Hint::ModulePreload(Src::parse("/assets/app.js").expect("caminho interno")),
        Hint::Preload(Preload::font(
            Src::parse("/fonts/inter.woff2").expect("caminho interno"),
        )),
        Hint::DnsPrefetch(ExternalUrl::parse("https://cdn.example.com").expect("URL absoluta")),
        Hint::Preconnect(ExternalUrl::parse("https://fonts.example.com").expect("URL absoluta")),
    ];
    let data = ShellData::new(Text::new("Notícias"), Text::new("Notícias"), lang())
        .hints(hints)
        .assets(std::iter::once(Asset::style("/assets/app.css").expect("css")).collect());
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let s = drawn.markup().as_str();

    let preconnect = s.find("<link rel=\"preconnect\"").expect("preconnect");
    let dns = s.find("<link rel=\"dns-prefetch\"").expect("dns-prefetch");
    let preload = s.find("<link rel=\"preload\"").expect("preload");
    let module = s
        .find("<link rel=\"modulepreload\"")
        .expect("modulepreload");
    let prefetch = s.find("<link rel=\"prefetch\"").expect("prefetch");
    let stylesheet = s
        .find("<link rel=\"stylesheet\" href=\"/assets/app.css\">")
        .expect("stylesheet");
    assert!(preconnect < dns, "preconnect antes de dns-prefetch");
    assert!(dns < preload, "dns-prefetch antes de preload");
    assert!(preload < module, "preload antes de modulepreload");
    assert!(module < prefetch, "modulepreload antes de prefetch");
    assert!(prefetch < stylesheet, "hints antes do CSS");

    assert!(s.contains(
        "<link rel=\"preload\" href=\"/fonts/inter.woff2\" as=\"font\" type=\"font/woff2\" crossorigin=\"anonymous\">"
    ));
    assert!(s.contains("<link rel=\"modulepreload\" href=\"/assets/app.js\">"));
    assert!(s.contains("<link rel=\"prefetch\" href=\"/noticias\">"));
    assert!(s.contains("<link rel=\"dns-prefetch\" href=\"https://cdn.example.com\">"));
}

#[test]
fn duplicate_hints_are_emitted_once() {
    let url = ExternalUrl::parse("https://cdn.example.com").expect("URL absoluta");
    let data = ShellData::new(Text::new("A"), Text::new("A"), lang())
        .hints(vec![Hint::Preconnect(url), Hint::Preconnect(url)]);
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert_eq!(
        drawn
            .markup()
            .as_str()
            .matches("rel=\"preconnect\"")
            .count(),
        1
    );
}

#[test]
fn responsive_image_preload_carries_same_srcset_and_sizes() {
    let variants = [
        VariantPx {
            width: 360,
            src: "/media/hero-360.avif",
        },
        VariantPx {
            width: 720,
            src: "/media/hero-720.avif",
        },
    ];
    let preload = Preload::image(Src::parse("/media/hero.avif").expect("caminho interno"))
        .responsive(
            SrcSet::from_variants(&variants),
            Sizes::new("(min-width: 48em) 33vw, 100vw"),
        )
        .fetchpriority(FetchPriority::High);
    let data =
        ShellData::new(Text::new("A"), Text::new("A"), lang()).hint(Hint::ImagePreload(preload));
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let s = drawn.markup().as_str();
    assert!(s.contains(
        "<link rel=\"preload\" href=\"/media/hero.avif\" as=\"image\" fetchpriority=\"high\" \
         imagesrcset=\"/media/hero-360.avif 360w, /media/hero-720.avif 720w\" \
         imagesizes=\"(min-width: 48em) 33vw, 100vw\">"
    ));
}

#[test]
fn generic_preload_keeps_as_and_type() {
    let preload = Preload::new(
        Src::parse("/dados.json").expect("caminho interno"),
        PreloadAs::Fetch,
    )
    .with_type("application/json");
    let data = ShellData::new(Text::new("A"), Text::new("A"), lang()).hint(Hint::Preload(preload));
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(drawn.markup().as_str().contains(
        "<link rel=\"preload\" href=\"/dados.json\" as=\"fetch\" type=\"application/json\">"
    ));
}

#[test]
fn stylesheet_media_is_emitted() {
    let data = ShellData::new(Text::new("A"), Text::new("A"), lang()).assets(
        std::iter::once(Asset::style_media("/print.css", Media::print()).expect("css")).collect(),
    );
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(
        drawn
            .markup()
            .as_str()
            .contains("<link rel=\"stylesheet\" href=\"/print.css\" media=\"print\">")
    );
}

#[test]
fn invalid_media_never_becomes_an_empty_attribute() {
    assert!(
        Asset::style_media("/print.css", Media::new("(min-width: 48em")).is_none(),
        "consulta inválida não constrói o asset"
    );
    let preload = Preload::style(Src::parse("/print.css").expect("caminho interno"))
        .media(Media::new("(min-width: 48em"));
    let data = ShellData::new(Text::new("A"), Text::new("A"), lang()).hint(Hint::Preload(preload));
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(!drawn.markup().as_str().contains("media=\"\""));
}

#[test]
fn style_preload_carries_media() {
    let preload =
        Preload::style(Src::parse("/print.css").expect("caminho interno")).media(Media::print());
    let data = ShellData::new(Text::new("A"), Text::new("A"), lang()).hint(Hint::Preload(preload));
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(
        drawn
            .markup()
            .as_str()
            .contains("<link rel=\"preload\" href=\"/print.css\" as=\"style\" media=\"print\">")
    );
}

#[test]
fn empty_responsive_set_does_not_emit_empty_attributes() {
    let empty: [VariantPx; 0] = [];
    let preload = Preload::image(Src::parse("/media/hero.avif").expect("caminho interno"))
        .responsive(SrcSet::from_variants(&empty), Sizes::new("100vw"));
    let data =
        ShellData::new(Text::new("A"), Text::new("A"), lang()).hint(Hint::ImagePreload(preload));
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let s = drawn.markup().as_str();
    assert!(!s.contains("imagesrcset"));
    assert!(!s.contains("imagesizes"));
}

#[test]
fn preload_carries_integrity_and_referrerpolicy() {
    let integrity = Integrity::parse("sha384-AbC123+/=").expect("SRI válido");
    let preload = Preload::style(Src::parse("/app.css").expect("caminho interno"))
        .crossorigin(CrossOrigin::Anonymous)
        .integrity(integrity)
        .referrerpolicy(ReferrerPolicy::StrictOriginWhenCrossOrigin);
    let data = ShellData::new(Text::new("A"), Text::new("A"), lang()).hint(Hint::Preload(preload));
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(drawn.markup().as_str().contains(concat!(
        "<link rel=\"preload\" href=\"/app.css\" as=\"style\" crossorigin=\"anonymous\" ",
        "integrity=\"sha384-AbC123+/=\" referrerpolicy=\"strict-origin-when-cross-origin\">"
    )));
}
