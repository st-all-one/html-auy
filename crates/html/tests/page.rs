//! Shell: o documento tem uma ordem e ela não é negociável (docs/06, docs/09).

use html::{
    Alternate, Asset, Dir, ExternalUrl, HtmxConfig, JsonGraph, JsonLd, LUrl, Lang, Msg, OpenGraph,
    OpenGraphKind, Page, Regions, ShellData, Text, View, html,
};

#[derive(Clone, Copy)]
struct M;

impl Msg for M {
    fn as_str(self) -> &'static str {
        "Rótulo"
    }
}

struct Graph;

impl JsonGraph for Graph {
    fn write_json(&self, out: &mut String) {
        out.push_str(r#"{"@type":"WebSite"}"#);
    }
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: etiqueta fixa e válida"
)]
fn lang() -> Lang {
    Lang::parse("pt-BR").expect("etiqueta BCP-47 válida")
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: entradas fixas e válidas"
)]
fn shell() -> ShellData<'static> {
    let mut regions = Regions::new();
    regions.header = Some(html! { header { (Text::new("topo")) } });
    regions.footer = Some(html! { footer { (Text::new("base")) } });
    ShellData::new(
        Text::new("Notícias — exemplo"),
        Text::new("Notícias"),
        lang(),
    )
    .description(Text::new("as últimas"))
    .canonical(ExternalUrl::parse("https://example.com/noticias").expect("URL absoluta"))
    .dir(Dir::Ltr)
    .schemas(vec![JsonLd::graph(&Graph)])
    .assets(
        [
            Asset::style("/assets/a.css").expect("css"),
            Asset::script("/assets/a.js").expect("js"),
        ]
        .into_iter()
        .collect(),
    )
    .regions(regions)
}

#[test]
fn document_starts_with_doctype_and_lang() {
    let page = Page::new(&shell(), View::new(html! { p { (Text::new("corpo")) } }));
    assert!(page.markup().as_str().starts_with("<!DOCTYPE html>"));
    assert!(
        page.markup()
            .as_str()
            .contains("<html lang=\"pt-BR\" dir=\"ltr\">")
    );
}

#[test]
fn head_order() {
    let page = Page::new(&shell(), View::new(html! { p { (Text::new("corpo")) } }));
    let drawn = page.markup().as_str();
    let head_end = drawn.find("</head>").expect("fim do head");
    let positions = [
        "<meta charset",
        "<title>",
        "rel=\"canonical\"",
        "rel=\"stylesheet\"",
        "type=\"module\"",
        "application/ld+json",
    ]
    .map(|needle| drawn.find(needle).expect("marcador no head"));
    assert!(
        positions.windows(2).all(|pair| pair[0] < pair[1]),
        "ordem do head: {positions:?} (fim em {head_end})"
    );
    assert!(positions.iter().all(|position| *position < head_end));
    assert!(head_end <= 14_000, "prefixo grande demais: {head_end}");
}

#[test]
fn head_prefix_is_exact() {
    let data = ShellData::new(Text::new("Título"), Text::new("Título"), lang());
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(page.markup().as_str().starts_with(
        "<!DOCTYPE html><html lang=\"pt-BR\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>"
    ));
}

#[test]
fn exactly_one_main_and_one_h1() {
    let page = Page::new(&shell(), View::new(html! { h2 { (Text::new("seção")) } }));
    let drawn = page.markup().as_str();
    assert_eq!(drawn.matches("<main").count(), 1);
    assert_eq!(drawn.matches("<h1>").count(), 1);
    assert!(drawn.contains("<main id=\"main\" tabindex=\"-1\"><h1>Notícias</h1>"));
}

#[test]
fn regions_and_modals_keep_order() {
    let view = View::new(html! { p { (Text::new("corpo")) } })
        .modal(html! { dialog aria_label=(html::Label::text(M)) { (Text::new("modal a")) } })
        .modal(html! { dialog aria_label=(html::Label::text(M)) { (Text::new("modal b")) } });
    let page = Page::new(&shell(), view);
    let drawn = page.markup().as_str();
    let header = drawn.find("topo").expect("header");
    let main = drawn.find("<main").expect("main");
    let footer = drawn.find("base").expect("footer");
    let first_modal = drawn.find("modal a").expect("modal a");
    let second_modal = drawn.find("modal b").expect("modal b");
    assert!(header < main && main < footer && footer < first_modal);
    assert!(first_modal < second_modal);
}

#[test]
fn region_slots_bracket_main() {
    let mut regions = Regions::new();
    regions.header = Some(html! { header { (Text::new("topo")) } });
    regions.before_main =
        Some(html! { nav aria_label=(html::Label::text(M)) { (Text::new("trilha")) } });
    regions.after_main = Some(html! { aside { (Text::new("lateral")) } });
    regions.footer = Some(html! { footer { (Text::new("base")) } });
    let data = ShellData::new(
        Text::new("Notícias — exemplo"),
        Text::new("Notícias"),
        lang(),
    )
    .regions(regions);
    let drawn = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let s = drawn.markup().as_str();
    let header = s.find("topo").expect("header");
    let before = s.find("trilha").expect("before_main");
    let main = s.find("<main").expect("main");
    let after = s.find("lateral").expect("after_main");
    let footer = s.find("base").expect("footer");
    assert!(
        header < before && before < main,
        "header < before_main < main"
    );
    assert!(main < after && after < footer, "main < after_main < footer");
    assert_eq!(s.matches("<main").count(), 1);
}

#[test]
fn noscript_keeps_skip_link_first_and_precedes_header() {
    let data = shell()
        .skip(Text::new("Pular para o conteúdo"))
        .noscript(html! { p { (Text::new("ative o JavaScript")) } });
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let s = page.markup().as_str();
    let skip = s.find("class=\"skip-link\"").expect("skip link");
    let status = s.find("role=\"status\"").expect("status");
    let noscript = s.find("<noscript>").expect("noscript");
    let header = s.find("topo").expect("header");
    assert!(
        skip < status && status < noscript && noscript < header,
        "{s}"
    );
    assert!(s.contains("<noscript><p>ative o JavaScript</p></noscript>"));
}

#[test]
fn styles_before_scripts_and_deduplicated() {
    let css = Asset::style("/assets/a.css").expect("css");
    let js = Asset::script("/assets/a.js").expect("js");
    let view = View::new(html! { p { (Text::new("x")) } })
        .asset(js)
        .asset(css);
    let page = Page::new(&shell(), view);
    let drawn = page.markup().as_str();
    let style = drawn.find("rel=\"stylesheet\"").expect("estilo");
    let script = drawn.find("type=\"module\"").expect("script");
    assert!(style < script, "estilo antes de script");
    assert_eq!(drawn.matches("/assets/a.css").count(), 1);
}

#[test]
fn view_merge_sums_modals_and_dedups_assets() {
    let css = Asset::style("/assets/a.css").expect("css");
    let left = View::new(html! { p { (Text::new("um")) } })
        .modal(html! { dialog aria_label=(html::Label::text(M)) { (Text::new("m1")) } })
        .asset(css);
    let right = View::new(html! { p { (Text::new("dois")) } })
        .modal(html! { dialog aria_label=(html::Label::text(M)) { (Text::new("m2")) } })
        .asset(css);
    let merged = left.merge(right);
    assert_eq!(merged.modals().len(), 2);
    assert_eq!(merged.assets().len(), 1);
    assert!(merged.main().as_str().contains("um"));
}

#[test]
fn skip_link_is_first_focusable_and_live_region_is_persistent() {
    let data = shell().skip(Text::new("Pular para o conteúdo"));
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let drawn = page.markup().as_str();
    let skip = drawn.find("class=\"skip-link\"").expect("skip link");
    let header = drawn.find("topo").expect("header");
    assert!(skip < header, "skip link vem antes de qualquer focável");
    assert!(drawn.contains("<a class=\"skip-link\" href=\"#main\">Pular para o conteúdo</a>"));
    assert_eq!(drawn.matches("role=\"status\"").count(), 1);
    assert!(drawn.contains("aria-atomic=\"true\""));
}

#[test]
fn without_skip_text_there_is_no_anchor() {
    let page = Page::new(&shell(), View::new(html! { p { (Text::new("corpo")) } }));
    assert!(!page.markup().as_str().contains("skip-link"));
}

#[test]
fn alternates_follow_canonical_and_keep_consumer_order() {
    let data = shell().alternates(vec![
        Alternate::new(
            lang(),
            ExternalUrl::parse("https://example.com/en/noticias").expect("URL absoluta"),
        ),
        Alternate::x_default(
            ExternalUrl::parse("https://example.com/noticias").expect("URL absoluta"),
        ),
    ]);
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let drawn = page.markup().as_str();
    let canonical = drawn.find("rel=\"canonical\"").expect("canonical");
    let english = drawn
        .find(
            "<link rel=\"alternate\" hreflang=\"pt-BR\" href=\"https://example.com/en/noticias\">",
        )
        .expect("alternate pt-BR");
    let fallback_default = drawn
        .find(
            "<link rel=\"alternate\" hreflang=\"x-default\" href=\"https://example.com/noticias\">",
        )
        .expect("x-default");
    assert!(canonical < english && english < fallback_default);
}

#[test]
fn without_alternates_there_is_no_alternate_link() {
    let page = Page::new(&shell(), View::new(html! { p { (Text::new("corpo")) } }));
    assert!(!page.markup().as_str().contains("rel=\"alternate\""));
}

#[test]
fn open_graph_derives_from_document_data() {
    let image = ExternalUrl::parse("https://example.com/media/capa.png").expect("URL absoluta");
    let data = shell().open_graph(OpenGraph::new().image(image));
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let drawn = page.markup().as_str();
    assert!(drawn.contains("<meta property=\"og:type\" content=\"website\">"));
    assert!(drawn.contains("<meta property=\"og:title\" content=\"Notícias — exemplo\">"));
    assert!(drawn.contains("<meta property=\"og:description\" content=\"as últimas\">"));
    assert!(drawn.contains("<meta property=\"og:url\" content=\"https://example.com/noticias\">"));
    assert!(
        drawn.contains(
            "<meta property=\"og:image\" content=\"https://example.com/media/capa.png\">"
        )
    );
    let description = drawn.find("name=\"description\"").expect("description");
    let open_graph = drawn.find("property=\"og:type\"").expect("og:type");
    assert!(description < open_graph, "OG vem depois da description");
}

#[test]
fn open_graph_without_image_omits_og_image() {
    let data = shell().open_graph(OpenGraph::new());
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(!page.markup().as_str().contains("og:image"));
}

#[test]
fn open_graph_escapes_title() {
    let data = ShellData::new(Text::new("a \"b\" <c>"), Text::new("Título"), lang())
        .open_graph(OpenGraph::new());
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(
        page.markup()
            .as_str()
            .contains("property=\"og:title\" content=\"a &quot;b&quot; &lt;c&gt;\"")
    );
}

#[test]
fn open_graph_kind_is_data_not_a_hardcoded_website() {
    let data = shell().open_graph(OpenGraph::new().kind(OpenGraphKind::Article));
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(
        page.markup()
            .as_str()
            .contains("<meta property=\"og:type\" content=\"article\">")
    );
    let data = shell().open_graph(OpenGraph::new().kind(OpenGraphKind::Product));
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(
        page.markup()
            .as_str()
            .contains("<meta property=\"og:type\" content=\"product\">")
    );
}

#[test]
fn manifest_is_emitted_between_canonical_and_alternates() {
    let manifest = LUrl::new("/manifest.webmanifest").expect("caminho válido");
    let data = shell()
        .alternates(vec![Alternate::x_default(
            ExternalUrl::parse("https://example.com/noticias").expect("URL absoluta"),
        )])
        .manifest(manifest);
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let drawn = page.markup().as_str();
    assert!(drawn.contains("<link rel=\"manifest\" href=\"/manifest.webmanifest\">"));
    let canonical = drawn.find("rel=\"canonical\"").expect("canonical");
    let manifest = drawn.find("rel=\"manifest\"").expect("manifest");
    let alternate = drawn.find("rel=\"alternate\"").expect("alternate");
    assert!(canonical < manifest && manifest < alternate);
}

#[test]
fn manifest_escapes_the_path() {
    let manifest = LUrl::new("/manifest?x=1&y=2").expect("caminho válido");
    let data = shell().manifest(manifest);
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    assert!(
        page.markup()
            .as_str()
            .contains("href=\"/manifest?x=1&amp;y=2\"")
    );
}

#[test]
fn htmx_config_is_safe_and_precedes_assets() {
    let data = shell()
        .assets(
            [
                Asset::style("/assets/a.css").expect("css"),
                Asset::script("/assets/a.js").expect("js"),
            ]
            .into_iter()
            .collect(),
        )
        .htmx_config(HtmxConfig::safe());
    let page = Page::new(&data, View::new(html! { p { (Text::new("corpo")) } }));
    let s = page.markup().as_str();
    let config = s.find("name=\"htmx-config\"").expect("config");
    let css = s.find("rel=\"stylesheet\"").expect("css");
    let js = s.find("type=\"module\"").expect("js");
    assert!(config < css && css < js, "{s}");
    assert!(s.contains("allowEval&quot;:false"));
    assert!(s.contains("includeIndicatorStyles&quot;:false"));
    assert!(s.contains("selfRequestsOnly&quot;:true"));
}
