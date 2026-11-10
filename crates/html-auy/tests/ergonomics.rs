//! Ergonomic additions: prelude, concat, builders, derive, literals and dates.

use html_auy::prelude::*;

#[derive(Clone, Copy, Msg)]
enum Derived {
    #[msg("Início")]
    Home,
    #[msg("Serviços")]
    Services,
}

/// Public enum to exercise `pub` parsing in the derive.
#[derive(Clone, Copy, Msg)]
pub enum PublicDerived {
    /// Public variant.
    #[msg("Público")]
    Public,
}

#[test]
fn msg_derive_and_text() {
    assert_eq!(Derived::Home.as_str(), "Início");
    assert_eq!(Text::msg(Derived::Services).as_str(), "Serviços");
    assert_eq!(PublicDerived::Public.as_str(), "Público");
}

#[test]
fn markup_concat_preserves_order() {
    let first = html! { p { Text::new("a") } };
    let second = html! { p { Text::new("b") } };
    let joined = Markup::concat([first, second]);
    assert_eq!(joined.as_str(), "<p>a</p><p>b</p>");
}

#[test]
fn regions_builder_and_option_assets() {
    let regions = Regions::new()
        .header(html! { header { Text::new("topo") } })
        .before_main(html! { p { Text::new("aviso") } })
        .after_main(html! { aside { Text::new("depois") } })
        .footer(html! { footer { Text::new("fim") } });
    let assets: Assets = [
        Asset::style("/a.css"),
        Asset::style("/a.css"),
        Asset::script("/a.js"),
    ]
    .into_iter()
    .collect();
    assert_eq!(assets.len(), 2);

    let data = ShellData::new(
        Text::new("Título"),
        Text::new("Heading"),
        Lang::parse("pt-BR").unwrap(),
    )
    .assets(assets)
    .regions(regions);
    let page = Page::new(&data, View::new(html! { p { Text::new("corpo") } }));
    let html = page.markup().as_str();
    assert!(html.contains("<header>topo</header>"));
    assert!(html.contains("<p>aviso</p><main"));
    assert!(html.contains("</main><aside>depois</aside>"));
    assert!(html.contains("<footer>fim</footer>"));
    assert!(html.contains("<link rel=\"stylesheet\" href=\"/a.css\">"));
}

#[test]
fn literal_href_and_src_are_validated_and_typed() {
    let markup = html! {
        a href="/noticias?a=1&b=2#topo" { Text::new("link") }
        img src="/media/a.png"
            alt=(Alt::decorative())
            width=(WidthPx::new(1))
            height=(HeightPx::new(1));
        a href="#conteudo" { Text::new("âncora") }
    };
    let text = markup.as_str();
    assert!(text.contains("<a href=\"/noticias?a=1&amp;b=2#topo\">link</a>"));
    assert!(text.contains("<img src=\"/media/a.png\" alt=\"\" width=\"1\" height=\"1\">"));
    assert!(text.contains("<a href=\"#conteudo\">âncora</a>"));
}

#[test]
fn date_is_date_only_and_timestamp_parses_rfc3339() {
    let date = Date::new(2026, 11, 5).unwrap();
    assert_eq!(date.as_str(), "2026-11-05");
    assert!(Date::new(2026, 2, 29).is_none());
    assert!(Date::parse("2026-02-29").is_none());
    assert!(Date::new(2024, 2, 29).is_some());
    assert_eq!(Date::parse("2026-11-05").unwrap().as_str(), "2026-11-05");

    let instant = Timestamp::parse_rfc3339("2026-11-05T09:00:00-03:00").unwrap();
    assert_eq!(instant.as_str(), "2026-11-05T09:00:00-03:00");
    assert_eq!(
        Timestamp::parse_rfc3339("2026-11-05T12:00:00Z")
            .unwrap()
            .as_str(),
        "2026-11-05T12:00:00Z"
    );
    assert!(Timestamp::parse_rfc3339("2026-11-05T09:00:00.123Z").is_none());
    assert!(Timestamp::parse_rfc3339("2026-11-05 09:00:00Z").is_none());

    let markup = html! { time datetime=(date) { Text::new("5 de novembro") } };
    assert_eq!(
        markup.as_str(),
        "<time datetime=\"2026-11-05\">5 de novembro</time>"
    );
}
