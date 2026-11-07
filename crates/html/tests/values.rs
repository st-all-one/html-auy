//! Matrizes de valores: cada tipo recusa o que não pode emitir e emite o que promete.

use html::{
    Alt, AttrValue, Class, ClassToken, Dir, ExternalUrl, FieldName, Fragment, HeightPx, Href, Id,
    Integrity, JsonGraph, JsonLd, LUrl, Label, Lang, MailAddr, Media, Offset, PhoneNumber, Render,
    Sizes, Src, SrcSet, Timestamp, Tone, Variant, WidthPx,
};

/// Escreve um valor de atributo como o crate faria.
fn attr_value(value: &impl AttrValue) -> String {
    let mut out = String::new();
    value.write_attr(&mut out);
    out
}

#[test]
fn lur_rejects_hostile_paths() {
    assert_eq!(LUrl::new("/noticias").unwrap().as_str(), "/noticias");
    assert_eq!(LUrl::new("/a?b=c#d").unwrap().as_str(), "/a?b=c#d");
    for bad in [
        "",
        "noticias",
        "//evil.example",
        "/a:b",
        "/a\\b",
        "/a b",
        "/a\tb",
        "/a\0b",
        "/a\"b",
        "/a<b",
        "/a>b",
        "/a`b",
        "/../etc/passwd",
        "/a/../b",
    ] {
        assert!(LUrl::new(bad).is_none(), "{bad:?} deveria ser recusado");
    }
    assert_eq!(
        LUrl::from_string(String::from("/ok")).unwrap().as_str(),
        "/ok"
    );
    assert!(LUrl::from_string(String::from("nope")).is_none());
}

#[test]
fn lur_with_segments_encodes() {
    let url = LUrl::with_segments("/noticias", &["a frota", "ônibus", "100%"]).unwrap();
    assert_eq!(url.as_str(), "/noticias/a%20frota/%C3%B4nibus/100%25");
    assert_eq!(LUrl::with_segments("/", &[]).unwrap().as_str(), "/");
    assert!(LUrl::with_segments("relative", &[]).is_none());
    // Segmento de dado não escapa da raiz: `..` é recusado, como em `new`.
    assert!(LUrl::with_segments("/noticias", &[".."]).is_none());
    assert!(LUrl::with_segments("/", &["..", ".."]).is_none());
    assert!(LUrl::with_segments("/a", &["ok", "..", "b"]).is_none());
    // O parser de URL normaliza `%2e` como `.`: o tipo também recusa.
    assert!(LUrl::new("/%2e%2e/etc").is_none());
    assert!(LUrl::new("/a/.%2E").is_none());
    assert!(LUrl::new("/a/%2E.").is_none());
    assert!(LUrl::new("/a/%2e%2e").is_none());
}

#[test]
fn fragment_is_the_only_same_page_destination() {
    assert_eq!(
        attr_value(&Href::fragment(Fragment::new("#").unwrap())),
        "#"
    );
    assert_eq!(
        attr_value(&Href::fragment(Fragment::new("#conteudo").unwrap())),
        "#conteudo"
    );
    assert_eq!(
        attr_value(&Href::fragment(Fragment::new("#a-b_c").unwrap())),
        "#a-b_c"
    );
    for bad in [
        "", "conteudo", "#a b", "#a\"b", "#a<b", "#a>b", "#a`b", "#../x",
    ] {
        assert!(Fragment::new(bad).is_none(), "{bad:?} deveria ser recusado");
    }
}

#[test]
fn external_url_only_accepts_http_and_https() {
    assert!(ExternalUrl::parse("https://example.org/a").is_some());
    assert!(ExternalUrl::parse("HTTP://example.org").is_some());
    for bad in [
        "javascript:alert(1)",
        "JavaScript:alert(1)",
        "data:text/html,<script>",
        "vbscript:msgbox",
        "file:///etc/passwd",
        "https://example.org/a b",
        "https://example.org/\"",
        "example.org",
    ] {
        assert!(
            ExternalUrl::parse(bad).is_none(),
            "{bad:?} deveria ser recusado"
        );
    }
}

#[test]
fn href_emits_escaped_and_normalized_scheme() {
    let external = ExternalUrl::parse("HTTP://example.org/a?b=1&c=2").unwrap();
    let href = Href::external(external);
    assert_eq!(attr_value(&href), "http://example.org/a?b=1&amp;c=2");

    let internal = Href::internal(LUrl::new("/noticias").unwrap());
    assert_eq!(attr_value(&internal), "/noticias");

    let mail = Href::mailto(MailAddr::parse("a@b.org").unwrap());
    assert_eq!(attr_value(&mail), "mailto:a@b.org");

    let tel = Href::tel(PhoneNumber::parse("+5511999998888").unwrap());
    assert_eq!(attr_value(&tel), "tel:+5511999998888");
}

#[test]
fn mail_and_phone_grammars() {
    assert!(MailAddr::parse("a@b.org").is_some());
    assert!(MailAddr::parse("nome.sobrenome@prefeitura.sp.gov.br").is_some());
    for bad in [
        "a@b",
        "a@@b.org",
        "@b.org",
        "a@.org",
        "a@b.",
        "a b@c.org",
        "a<b@c.org",
    ] {
        assert!(
            MailAddr::parse(bad).is_none(),
            "{bad:?} deveria ser recusado"
        );
    }
    assert!(PhoneNumber::parse("+5511").is_some());
    assert!(PhoneNumber::parse("+551199999888812345").is_none());
    for bad in ["+", "5511999998888", "+55 11", "+55-11"] {
        assert!(
            PhoneNumber::parse(bad).is_none(),
            "{bad:?} deveria ser recusado"
        );
    }
}

#[test]
fn sizes_grammar() {
    assert_eq!(
        Sizes::new("(min-width: 48em) 33vw, 100vw").as_str(),
        "(min-width: 48em) 33vw, 100vw"
    );
    assert_eq!(Sizes::new("100vw").as_str(), "100vw");
    assert_eq!(Sizes::new(".5em").as_str(), ".5em");
    assert_eq!(
        Sizes::new("calc(100vw - 2rem)").as_str(),
        "calc(100vw - 2rem)"
    );
    assert_eq!(Sizes::new("auto").as_str(), "auto");
    assert_eq!(Sizes::new("AUTO").as_str(), "AUTO");
    assert_eq!(
        Sizes::new("(min-width: 48em) calc(50vw - 1rem), auto").as_str(),
        "(min-width: 48em) calc(50vw - 1rem), auto"
    );
    assert_eq!(
        Sizes::new("calc(calc(100vw - 1rem) / 2)").as_str(),
        "calc(calc(100vw - 1rem) / 2)"
    );
    assert_eq!(
        Sizes::new("min(100vw, 30rem)").as_str(),
        "min(100vw, 30rem)"
    );
    assert_eq!(Sizes::new("max(50vw, 20rem)").as_str(), "max(50vw, 20rem)");
    assert_eq!(
        Sizes::new("clamp(20rem, 50vw, 80rem)").as_str(),
        "clamp(20rem, 50vw, 80rem)"
    );
    assert_eq!(
        Sizes::new("(min-width: 48em) clamp(20rem, 50vw, 80rem), 100vw").as_str(),
        "(min-width: 48em) clamp(20rem, 50vw, 80rem), 100vw"
    );
    for bad in [
        "",
        "  ",
        "100",
        "abc",
        "(min-width: 48em)",
        "100vw,",
        ",100vw",
        "(min-width: 48em 33vw",
        ".px",
        "..px",
        "1.px",
        "1.2.3px",
        "min()",
        "max(1px",
        "clamp(1px, 2px)",
        "clamp(, 2px, 3px)",
        "clamp(1px, , 3px)",
        "clamp(1px, 2px, )",
    ] {
        assert_eq!(Sizes::new(bad).as_str(), "", "{bad:?} deveria virar vazio");
        assert!(Sizes::try_new(bad).is_none());
    }
    assert_ne!(Sizes::new("100vw").as_str(), "");
}

struct Fixed {
    src: &'static str,
    width: u32,
}

impl Variant for Fixed {
    fn src(&self) -> &str {
        self.src
    }
    fn width(&self) -> u32 {
        self.width
    }
}

#[test]
fn srcset_orders_filters_and_deduplicates() {
    let variants = [
        Fixed {
            src: "/a-720.avif",
            width: 720,
        },
        Fixed {
            src: "/a-360.avif",
            width: 360,
        },
        Fixed {
            src: "/a-0.avif",
            width: 0,
        },
        Fixed {
            src: "/a-bad.avif",
            width: 360,
        },
        Fixed {
            src: "/a,b.avif",
            width: 540,
        },
        Fixed {
            src: "javascript:alert(1)",
            width: 800,
        },
        Fixed {
            src: "/a-1200.avif",
            width: 1200,
        },
        Fixed {
            src: "https://cdn.example.com/a-1600.avif",
            width: 1600,
        },
    ];
    let set = SrcSet::from_variants(&variants);
    assert_eq!(
        attr_value(&set),
        "/a-360.avif 360w, /a-720.avif 720w, /a-1200.avif 1200w, \
         https://cdn.example.com/a-1600.avif 1600w"
    );
    assert!(!set.is_empty());

    let empty = SrcSet::from_variants::<Fixed>(&[]);
    assert!(empty.is_empty());
    assert_eq!(attr_value(&empty), "");
}

#[test]
fn class_set_is_sorted_and_deduplicated() {
    #[derive(Clone, Copy)]
    enum Token {
        Card,
        List,
        Featured,
    }

    impl ClassToken for Token {
        fn as_str(self) -> &'static str {
            match self {
                Self::Card => "card",
                Self::List => "list",
                Self::Featured => "featured",
            }
        }
    }

    let class = Class::of(&[Token::List, Token::Card, Token::List]);
    assert_eq!(class.tokens(), ["card", "list"]);
    assert_eq!(attr_value(&class), "card list");

    let merged = class.extend(Class::of(&[Token::Featured, Token::Card]));
    assert_eq!(merged.tokens(), ["card", "featured", "list"]);
    assert_eq!(attr_value(&merged), "card featured list");

    assert!(Class::of::<Token>(&[]).is_empty());
    assert_eq!(attr_value(&Class::of(&[Tone::Danger])), "tone-danger");
}

#[test]
fn lang_canonicalizes_and_validates() {
    assert_eq!(Lang::parse("pt-BR").unwrap().as_str(), "pt-BR");
    assert_eq!(Lang::parse("PT-br").unwrap().as_str(), "pt-BR");
    assert_eq!(Lang::parse("en").unwrap().as_str(), "en");
    assert_eq!(Lang::parse("zh-Hant-TW").unwrap().as_str(), "zh-Hant-TW");
    assert_eq!(Lang::parse("zh-hant-tw").unwrap().as_str(), "zh-Hant-TW");
    assert_eq!(Lang::parse("es-419").unwrap().as_str(), "es-419");
    assert_eq!(Lang::EN.as_str(), "en");
    assert_eq!(Lang::parse("pt-BR").unwrap(), Lang::parse("pt-br").unwrap());
    for bad in [
        "",
        "e",
        "portugues-do-brasil-extra",
        "pt-",
        "-pt",
        "pt-1234",
        "pt_BR",
        "pt BR",
    ] {
        assert!(Lang::parse(bad).is_none(), "{bad:?} deveria ser recusado");
    }
}

#[test]
fn dir_values() {
    assert_eq!(Dir::Ltr.as_str(), "ltr");
    assert_eq!(Dir::Rtl.as_str(), "rtl");
    assert_eq!(Dir::Auto.as_str(), "auto");
}

#[test]
fn id_and_field_name_normalize() {
    assert_eq!(Id::new("main").as_str(), "main");
    assert_eq!(Id::new("a b.c").as_str(), "a-b-c");
    assert_eq!(Id::new("--x--").as_str(), "x");
    assert_eq!(Id::new("").as_str(), "id");
    assert_eq!(Id::new("not$ciado").as_str(), "not-ciado");
    assert_eq!(Id::try_new("main").unwrap().as_str(), "main");
    assert!(Id::try_new("").is_none());
    assert!(Id::try_new("a b").is_none());

    assert_eq!(FieldName::new("email").as_str(), "email");
    assert_eq!(
        FieldName::new("sections[0][campo]").as_str(),
        "sections[0][campo]"
    );
    assert_eq!(FieldName::new("").as_str(), "field");

    let mut out = String::new();
    FieldName::new("sections[0][campo]").id_into(&mut out);
    assert_eq!(out, "f-sections-0-campo");

    let mut plain = String::new();
    FieldName::new("email").id_into(&mut plain);
    assert_eq!(plain, "f-email");
}

#[test]
fn dimensions_saturate() {
    assert_eq!(WidthPx::new(0).get(), 1);
    assert_eq!(WidthPx::new(720).get(), 720);
    assert_eq!(HeightPx::new(0).get(), 1);
    assert_eq!(attr_value(&WidthPx::new(720)), "720");
}

#[test]
fn alt_has_no_implicit_empty() {
    assert_eq!(Alt::new("Ônibus no pátio").as_str(), "Ônibus no pátio");
    assert_eq!(Alt::new(""), Alt::decorative());
    assert_eq!(attr_value(&Alt::new("")), "");
    assert_eq!(attr_value(&Alt::new("a\"b")), "a&quot;b");
}

#[test]
fn labels_render_text_or_sr_only() {
    #[derive(Clone, Copy)]
    enum Message {
        Close,
    }

    impl html::Msg for Message {
        fn as_str(self) -> &'static str {
            "Fechar"
        }
    }

    let visible = Label::text(Message::Close);
    assert_eq!(attr_value(&visible), "Fechar");
    assert_eq!(visible.render().as_str(), "Fechar");

    let hidden = Label::sr_only(Message::Close);
    assert_eq!(attr_value(&hidden), "Fechar");
    assert_eq!(
        hidden.render().as_str(),
        r#"<span class="sr-only">Fechar</span>"#
    );
}

#[test]
fn offset_parsing_and_bounds() {
    assert_eq!(Offset::UTC.as_seconds(), 0);
    assert_eq!(Offset::from_hms(-3, 0).unwrap().as_seconds(), -10800);
    assert_eq!(Offset::from_hms(5, 30).unwrap().as_seconds(), 19800);
    assert!(Offset::from_hms(24, 0).is_none());
    assert!(Offset::from_hms(0, 60).is_none());
    assert!(Offset::from_seconds(86_341).is_none());
    // RFC 3339 não tem offset em segundos: 90 s truncaria para `+00:01` e denotaria outro instante.
    assert!(Offset::from_seconds(90).is_none());
    assert!(Offset::from_seconds(-90).is_none());
    assert!(Offset::from_seconds(3_600).is_some());
    assert_eq!(Offset::parse("Z").unwrap(), Offset::UTC);
    assert_eq!(Offset::parse("-03:00").unwrap().as_seconds(), -10800);
    assert_eq!(Offset::parse("+05:30").unwrap().as_seconds(), 19800);
    assert!(Offset::parse("03:00").is_none());
    assert!(Offset::parse("+3:00").is_none());
    assert!(Offset::parse("+24:00").is_none());
}

#[test]
fn timestamp_formats_rfc3339() {
    assert_eq!(
        Timestamp::new(0, Offset::UTC).as_str(),
        "1970-01-01T00:00:00Z"
    );
    assert_eq!(
        Timestamp::new(86_400, Offset::UTC).as_str(),
        "1970-01-02T00:00:00Z"
    );
    assert_eq!(
        Timestamp::new(-1, Offset::UTC).as_str(),
        "1969-12-31T23:59:59Z"
    );
    assert_eq!(
        Timestamp::new(951_782_400, Offset::UTC).as_str(),
        "2000-02-29T00:00:00Z"
    );
    assert_eq!(
        Timestamp::new(0, Offset::from_hms(5, 30).unwrap()).as_str(),
        "1970-01-01T05:30:00+05:30"
    );
    assert_eq!(
        Timestamp::new(0, Offset::from_hms(-3, 0).unwrap()).as_str(),
        "1969-12-31T21:00:00-03:00"
    );
    assert_eq!(
        Timestamp::new(i64::MAX, Offset::UTC).as_str(),
        "9999-12-31T23:59:59Z"
    );
    assert_eq!(
        Timestamp::new(i64::MIN, Offset::UTC).as_str(),
        "0001-01-01T00:00:00Z"
    );
}

struct Graph;

impl JsonGraph for Graph {
    fn write_json(&self, out: &mut String) {
        out.push_str(r#"{"@context":"https://schema.org","name":"</script><b>x</b>"}"#);
    }
}

#[test]
fn jsonld_escapes_at_construction() {
    let json = JsonLd::graph(&Graph);
    assert!(!json.as_str().contains('<'));
    assert!(json.as_str().contains("\\u003c"));
    let rendered = json.render();
    assert!(
        rendered
            .as_str()
            .starts_with(r#"<script type="application/ld+json">"#)
    );
    assert!(rendered.as_str().ends_with("</script>"));
    assert!(!rendered.as_str().contains("</script><b>"));
}

struct MediaPath(LUrl<'static>);

impl html::MediaPath for MediaPath {
    fn path(&self) -> &LUrl<'_> {
        &self.0
    }
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: caminho interno fixo"
)]
fn media_path() -> MediaPath {
    MediaPath(LUrl::from_string(String::from("/media/a.avif")).expect("caminho interno"))
}

#[test]
fn media_src_reads_the_path() {
    let media = media_path();
    let source = Src::media(&media);
    assert_eq!(source.as_str(), "/media/a.avif");
    assert_eq!(attr_value(&source), "/media/a.avif");
}

#[test]
fn src_accepts_internal_paths_and_absolute_urls() {
    assert_eq!(
        Src::parse("/media/a.avif").map(|s| s.as_str()),
        Some("/media/a.avif")
    );
    assert_eq!(
        Src::parse("https://cdn.example.com/a.avif").map(|s| s.as_str()),
        Some("https://cdn.example.com/a.avif")
    );
    for bad in [
        "javascript:alert(1)",
        "data:image/png;base64,xx",
        "//cdn.example.com/a.avif",
        "media/a.avif",
        " /media/a.avif",
        "http://",
    ] {
        assert!(Src::parse(bad).is_none(), "{bad:?} não deveria construir");
    }
}

#[test]
fn media_validates_shape_and_ignores_invalid() {
    assert_eq!(Media::print().as_str(), "print");
    assert_eq!(Media::screen().as_str(), "screen");
    assert_eq!(
        Media::new("screen and (min-width: 48em)").as_str(),
        "screen and (min-width: 48em)"
    );
    assert_eq!(Media::new("print").as_str(), "print");
    for bad in ["", "  ", "(min-width: 48em", "min-width)", "print\u{1}"] {
        assert!(Media::new(bad).is_empty(), "{bad:?} deveria virar vazio");
        assert!(Media::try_new(bad).is_none());
    }
    assert!(Media::try_new("(prefers-color-scheme: dark)").is_some());
}

#[test]
fn integrity_validates_sri_metadata() {
    assert_eq!(
        Integrity::parse("sha384-AbC123+/=").map(|i| i.as_str()),
        Some("sha384-AbC123+/=")
    );
    assert!(Integrity::parse("sha256-abc sha512-def").is_some());
    for bad in [
        "",
        "sha1-abc",
        "md5-abc",
        "sha256",
        "sha256-",
        "sha256-abc def",
        "sha256-abc;",
    ] {
        assert!(
            Integrity::parse(bad).is_none(),
            "{bad:?} deveria ser recusado"
        );
    }
}
