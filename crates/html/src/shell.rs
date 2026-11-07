//! Shell da página: o que pertence ao documento, não ao fragmento.
//!
//! Aqui vivem os únicos elementos que `html!` recusa: `<html>`, `<head>`, `<title>`, `<link>`,
//! `<script>`, `<h1>` e `<main>` (invariante `shell_only_elements`). O `Page` os emite numa ordem
//! fixa — é essa ordem que mantém o prefixo do `<head>` pequeno e o CSS antes do JS.

use crate::value::AttrValue;
use alloc::collections::BTreeSet;
use core::cmp::Ordering;

/// Espécie de asset declarado pelo consumidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetKind {
    /// `link rel="stylesheet"`.
    Style,
    /// `script type="module" defer`.
    Script,
}

impl AssetKind {
    /// Rank de emissão: estilo antes de script (sem FOUC).
    const fn rank(self) -> u8 {
        match self {
            Self::Style => 0,
            Self::Script => 1,
        }
    }
}

/// Asset declarado: caminho validado e ordem canônica por `(espécie, caminho)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Asset {
    kind: AssetKind,
    href: &'static str,
    media: Option<crate::value::Media<'static>>,
}

impl Asset {
    /// Folha de estilo do próprio site (`/assets/app.css`).
    #[must_use]
    pub fn style(href: &'static str) -> Option<Self> {
        if crate::value::valid_path(href) {
            Some(Self {
                kind: AssetKind::Style,
                href,
                media: None,
            })
        } else {
            None
        }
    }

    /// Folha de estilo condicionada por `media` (por exemplo `print`), sem bloquear a tela.
    #[must_use]
    pub fn style_media(href: &'static str, media: crate::value::Media<'static>) -> Option<Self> {
        if crate::value::valid_path(href) && !media.is_empty() {
            Some(Self {
                kind: AssetKind::Style,
                href,
                media: Some(media),
            })
        } else {
            None
        }
    }

    /// Módulo JavaScript do próprio site (`/assets/app.js`).
    #[must_use]
    pub fn script(href: &'static str) -> Option<Self> {
        if crate::value::valid_path(href) {
            Some(Self {
                kind: AssetKind::Script,
                href,
                media: None,
            })
        } else {
            None
        }
    }

    /// A espécie.
    #[must_use]
    pub const fn kind(&self) -> AssetKind {
        self.kind
    }

    /// O caminho emitido.
    #[must_use]
    pub const fn href(&self) -> &'static str {
        self.href
    }

    /// O `media` (só faz sentido em folha de estilo).
    #[must_use]
    pub const fn media(&self) -> Option<crate::value::Media<'static>> {
        self.media
    }
}

impl PartialOrd for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Asset {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.rank().cmp(&other.kind.rank()) {
            Ordering::Equal => match self.href.cmp(other.href) {
                Ordering::Equal => self.media.cmp(&other.media),
                other => other,
            },
            other => other,
        }
    }
}

/// Assets declarados, sem repetição e em ordem canônica.
pub type Assets = BTreeSet<Asset>;

/// Versão do documento em outro idioma (`<link rel="alternate" hreflang="…">`).
///
/// O `href` é uma URL absoluta (`http`/`https`): o Google exige `hreflang` totalmente qualificado.
/// A ordem de emissão é a do `Vec`, como em [`ShellData::schemas`] — quem decide é o consumidor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternate<'a> {
    hreflang: crate::value::Hreflang,
    href: crate::value::ExternalUrl<'a>,
}

impl<'a> Alternate<'a> {
    /// Versão no idioma `lang`.
    #[must_use]
    pub const fn new(lang: crate::value::Lang, href: crate::value::ExternalUrl<'a>) -> Self {
        Self {
            hreflang: crate::value::Hreflang::Lang(lang),
            href,
        }
    }

    /// Versão de fallback (`x-default`).
    #[must_use]
    pub const fn x_default(href: crate::value::ExternalUrl<'a>) -> Self {
        Self {
            hreflang: crate::value::Hreflang::XDefault,
            href,
        }
    }

    /// O `hreflang` emitido.
    #[must_use]
    pub const fn hreflang(&self) -> crate::value::Hreflang {
        self.hreflang
    }

    /// O `href` emitido.
    #[must_use]
    pub const fn href(&self) -> &crate::value::ExternalUrl<'a> {
        &self.href
    }
}

/// `og:type` aceito pelo crate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum OpenGraphKind {
    /// `website` (padrão).
    #[default]
    Website,
    /// `article`.
    Article,
    /// `product`.
    Product,
}

impl OpenGraphKind {
    /// O valor emitido em `og:type`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Website => "website",
            Self::Article => "article",
            Self::Product => "product",
        }
    }
}

/// Metadados Open Graph derivados do documento.
///
/// `og:type`, `og:title`, `og:description`, `og:url` e `og:image` saem dos dados que o shell já
/// tem; só a imagem precisa ser declarada. Sem [`ShellData::open_graph`], nenhuma tag `og:*` é
/// emitida.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OpenGraph<'a> {
    kind: OpenGraphKind,
    image: Option<crate::value::ExternalUrl<'a>>,
}

impl<'a> OpenGraph<'a> {
    /// Sem imagem (`og:image` não é emitido); `og:type` é `website`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            kind: OpenGraphKind::Website,
            image: None,
        }
    }

    /// `og:type`.
    #[must_use]
    pub const fn kind(mut self, kind: OpenGraphKind) -> Self {
        self.kind = kind;
        self
    }

    /// `og:image`.
    #[must_use]
    pub const fn image(mut self, image: crate::value::ExternalUrl<'a>) -> Self {
        self.image = Some(image);
        self
    }

    /// O `og:type` declarado.
    #[must_use]
    pub const fn get_kind(&self) -> OpenGraphKind {
        self.kind
    }

    /// A imagem declarada.
    #[must_use]
    pub const fn get_image(&self) -> Option<&crate::value::ExternalUrl<'a>> {
        self.image.as_ref()
    }
}

/// Regiões fixas do shell.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Regions {
    /// Cabeçalho do site (`<header>`), emitido antes do `<main>`.
    pub header: Option<crate::Markup>,
    /// Conteúdo entre o header e o `<main>`: breadcrumb (`<nav>`), `<aside>`, avisos.
    pub before_main: Option<crate::Markup>,
    /// Conteúdo entre o `<main>` e o footer: `<aside>`, navegação secundária.
    pub after_main: Option<crate::Markup>,
    /// Rodapé do site (`<footer>`), emitido depois do `<main>`.
    pub footer: Option<crate::Markup>,
}

impl Regions {
    /// Construtor vazio (o mesmo que `Default`).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Configuração segura do htmx, emitida como `<meta name="htmx-config">`.
///
/// O crate não emite `hx-*`; este tipo só fixa as opções que impedem eval, scripts injetados,
/// `<style>` inline e requests cross-origin. Quem não vendoriza htmx não chama o builder.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct HtmxConfig;

impl HtmxConfig {
    /// Configuração fixa e segura.
    #[must_use]
    pub const fn safe() -> Self {
        Self
    }

    /// O JSON emitido no `content`.
    #[must_use]
    pub const fn json(&self) -> &'static str {
        r#"{"allowEval":false,"allowScriptTags":false,"includeIndicatorStyles":false,"selfRequestsOnly":true,"withCredentials":false,"historyCacheSize":0,"timeout":10000}"#
    }
}

/// Dados do documento: tudo o que o `Page` precisa e nada que ele decida por você.
///
/// O `<h1>` é **aqui**, não na view: o shell é dono do documento e da hierarquia de títulos
/// (`single_h1`, `main_unique`). O `h1` é emitido aqui por padrão; uma view pode declarar o seu
/// quando o consumidor monta o shell, e o `audit` cobra a contagem final. A view começa no conteúdo
/// do `<main>`.
#[derive(Debug, Clone)]
pub struct ShellData<'a> {
    pub(crate) title: crate::msg::Text<'a>,
    pub(crate) heading: crate::msg::Text<'a>,
    pub(crate) lang: crate::value::Lang,
    pub(crate) description: Option<crate::msg::Text<'a>>,
    pub(crate) canonical: Option<crate::value::ExternalUrl<'a>>,
    pub(crate) manifest: Option<crate::value::LUrl<'a>>,
    pub(crate) dir: Option<crate::value::Dir>,
    pub(crate) schemas: alloc::vec::Vec<crate::value::JsonLd>,
    pub(crate) assets: Assets,
    pub(crate) regions: Regions,
    pub(crate) skip: Option<crate::msg::Text<'a>>,
    pub(crate) noscript: Option<crate::Markup>,
    pub(crate) alternates: alloc::vec::Vec<Alternate<'a>>,
    pub(crate) open_graph: Option<OpenGraph<'a>>,
    pub(crate) htmx_config: Option<HtmxConfig>,
    pub(crate) hints: alloc::vec::Vec<crate::hint::Hint<'a>>,
}

impl<'a> ShellData<'a> {
    /// Documento mínimo: título, `h1` e idioma.
    #[must_use]
    pub fn new(
        title: crate::msg::Text<'a>,
        heading: crate::msg::Text<'a>,
        lang: crate::value::Lang,
    ) -> Self {
        Self {
            title,
            heading,
            lang,
            description: None,
            canonical: None,
            manifest: None,
            dir: None,
            schemas: alloc::vec::Vec::new(),
            assets: Assets::new(),
            regions: Regions::new(),
            skip: None,
            noscript: None,
            alternates: alloc::vec::Vec::new(),
            open_graph: None,
            htmx_config: None,
            hints: alloc::vec::Vec::new(),
        }
    }

    /// Texto do skip link (o crate não conhece locale: sem texto, o link não é emitido).
    ///
    /// O link é o **primeiro focável** do documento e aponta para o `<main id="main">` (SC 2.4.1).
    #[must_use]
    pub const fn skip(mut self, text: crate::msg::Text<'a>) -> Self {
        self.skip = Some(text);
        self
    }

    /// `<meta name="description">`.
    #[must_use]
    pub const fn description(mut self, text: crate::msg::Text<'a>) -> Self {
        self.description = Some(text);
        self
    }

    /// `<link rel="canonical">`.
    #[must_use]
    pub const fn canonical(mut self, url: crate::value::ExternalUrl<'a>) -> Self {
        self.canonical = Some(url);
        self
    }

    /// `<link rel="manifest">`.
    #[must_use]
    pub fn manifest(mut self, manifest: crate::value::LUrl<'a>) -> Self {
        self.manifest = Some(manifest);
        self
    }

    /// Resource hints do documento (`preconnect`, `preload`, ...), deduplicados.
    #[must_use]
    pub fn hints(mut self, hints: alloc::vec::Vec<crate::hint::Hint<'a>>) -> Self {
        for hint in hints {
            if !self.hints.contains(&hint) {
                self.hints.push(hint);
            }
        }
        self
    }

    /// Um hint, para encadear casos isolados.
    #[must_use]
    pub fn hint(mut self, hint: crate::hint::Hint<'a>) -> Self {
        if !self.hints.contains(&hint) {
            self.hints.push(hint);
        }
        self
    }

    /// Conteúdo de fallback em `<noscript>`, emitido no início do `<body>`.
    ///
    /// O conteúdo é markup pronto: use-o para uma mensagem ou navegação essencial que funcione
    /// sem JavaScript. O `<noscript>` fica depois do skip link e da região viva, preservando o
    /// skip link como primeiro focável.
    #[must_use]
    pub fn noscript(mut self, content: crate::Markup) -> Self {
        self.noscript = Some(content);
        self
    }

    /// `dir` do `<html>`.
    #[must_use]
    pub const fn dir(mut self, dir: crate::value::Dir) -> Self {
        self.dir = Some(dir);
        self
    }

    /// Blocos JSON-LD, emitidos no fim do `<head>`.
    #[must_use]
    pub fn schemas(mut self, schemas: alloc::vec::Vec<crate::value::JsonLd>) -> Self {
        self.schemas = schemas;
        self
    }

    /// Versões do documento em outros idiomas, após o `canonical`.
    ///
    /// Sem chamada, nenhum `link rel="alternate"` é emitido: i18n é opcional.
    #[must_use]
    pub fn alternates(mut self, alternates: alloc::vec::Vec<Alternate<'a>>) -> Self {
        self.alternates = alternates;
        self
    }

    /// Metadados Open Graph derivados do documento.
    #[must_use]
    pub const fn open_graph(mut self, open_graph: OpenGraph<'a>) -> Self {
        self.open_graph = Some(open_graph);
        self
    }

    /// Configuração segura do htmx, emitida antes de qualquer asset.
    #[must_use]
    pub const fn htmx_config(mut self, config: HtmxConfig) -> Self {
        self.htmx_config = Some(config);
        self
    }

    /// Assets do documento (a view soma os dela).
    #[must_use]
    pub fn assets(mut self, assets: Assets) -> Self {
        self.assets = assets;
        self
    }

    /// Cabeçalho e rodapé.
    #[must_use]
    pub fn regions(mut self, regions: Regions) -> Self {
        self.regions = regions;
        self
    }
}

/// Documento completo: shell + `View`, em bytes na ordem fixa.
///
/// O `Page` não carrega status HTTP nem cabeçalhos: ele é o `<html>` e nada além.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    markup: crate::Markup,
}

impl Page {
    /// Monta o documento.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "contrato: o `Page` consome a `View`"
    )]
    #[expect(
        clippy::too_many_lines,
        reason = "a ordem do documento é a garantia; dividir esconde a sequência"
    )]
    #[must_use]
    pub fn new(data: &ShellData<'_>, view: crate::view::View) -> Self {
        use crate::markup::Render;

        let mut out = alloc::string::String::with_capacity(
            1_024usize
                .saturating_add(data.title.as_str().len())
                .saturating_add(view.main().len()),
        );

        out.push_str("<!DOCTYPE html><html lang=\"");
        crate::escape::attr(&mut out, data.lang.as_str());
        out.push('"');
        if let Some(dir) = data.dir {
            out.push_str(" dir=\"");
            crate::escape::attr(&mut out, dir.as_str());
            out.push('"');
        }
        out.push_str("><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>");
        crate::escape::text(&mut out, data.title.as_str());
        out.push_str("</title>");
        if let Some(description) = &data.description {
            out.push_str("<meta name=\"description\" content=\"");
            crate::escape::attr(&mut out, description.as_str());
            out.push_str("\">");
        }
        write_open_graph(&mut out, data);
        if let Some(canonical) = &data.canonical {
            out.push_str("<link rel=\"canonical\" href=\"");
            canonical.write_attr(&mut out);
            out.push_str("\">");
        }
        if let Some(manifest) = &data.manifest {
            out.push_str("<link rel=\"manifest\" href=\"");
            crate::escape::attr(&mut out, manifest.as_str());
            out.push_str("\">");
        }
        write_alternates(&mut out, &data.alternates);
        crate::hint::write(&mut out, &data.hints);
        if let Some(config) = data.htmx_config {
            out.push_str("<meta name=\"htmx-config\" content=\"");
            crate::escape::attr(&mut out, config.json());
            out.push_str("\">");
        }

        let mut assets = data.assets.clone();
        assets.extend(view.assets().iter().copied());
        for asset in &assets {
            match asset.kind() {
                AssetKind::Style => {
                    out.push_str("<link rel=\"stylesheet\" href=\"");
                    crate::escape::attr(&mut out, asset.href());
                    out.push('"');
                    if let Some(media) = asset.media() {
                        out.push_str(" media=\"");
                        media.write_attr(&mut out);
                        out.push('"');
                    }
                    out.push('>');
                }
                AssetKind::Script => {
                    out.push_str("<script type=\"module\" defer src=\"");
                    crate::escape::attr(&mut out, asset.href());
                    out.push_str("\"></script>");
                }
            }
        }
        for schema in &data.schemas {
            schema.render_to(&mut out);
        }

        out.push_str("</head><body>");
        if let Some(skip) = &data.skip {
            out.push_str("<a class=\"skip-link\" href=\"#main\">");
            crate::escape::text(&mut out, skip.as_str());
            out.push_str("</a>");
        }
        // Região viva persistente (SC 4.1.3): vazia no primeiro render, preenchida por swap.
        out.push_str("<div role=\"status\" aria-atomic=\"true\"></div>");
        if let Some(noscript) = &data.noscript {
            out.push_str("<noscript>");
            out.push_str(noscript.as_str());
            out.push_str("</noscript>");
        }
        if let Some(header) = &data.regions.header {
            out.push_str(header.as_str());
        }
        if let Some(before_main) = &data.regions.before_main {
            out.push_str(before_main.as_str());
        }
        out.push_str("<main id=\"main\" tabindex=\"-1\"><h1>");
        crate::escape::text(&mut out, data.heading.as_str());
        out.push_str("</h1>");
        out.push_str(view.main().as_str());
        out.push_str("</main>");
        if let Some(after_main) = &data.regions.after_main {
            out.push_str(after_main.as_str());
        }
        if let Some(footer) = &data.regions.footer {
            out.push_str(footer.as_str());
        }
        for modal in view.modals() {
            out.push_str(modal.as_str());
        }
        out.push_str("</body></html>");

        Self {
            markup: crate::Markup(out),
        }
    }

    /// O HTML do documento.
    #[must_use]
    pub const fn markup(&self) -> &crate::Markup {
        &self.markup
    }

    /// O HTML do documento, consumindo o `Page`.
    #[must_use]
    pub fn into_markup(self) -> crate::Markup {
        self.markup
    }
}

/// `og:*` derivado do documento; só a imagem é dado próprio do [`OpenGraph`].
fn write_open_graph(out: &mut alloc::string::String, data: &ShellData<'_>) {
    let Some(open_graph) = &data.open_graph else {
        return;
    };
    out.push_str("<meta property=\"og:type\" content=\"");
    out.push_str(open_graph.get_kind().as_str());
    out.push_str("\"><meta property=\"og:title\" content=\"");
    crate::escape::attr(out, data.title.as_str());
    out.push_str("\">");
    if let Some(description) = &data.description {
        out.push_str("<meta property=\"og:description\" content=\"");
        crate::escape::attr(out, description.as_str());
        out.push_str("\">");
    }
    if let Some(canonical) = &data.canonical {
        out.push_str("<meta property=\"og:url\" content=\"");
        canonical.write_attr(out);
        out.push_str("\">");
    }
    if let Some(image) = open_graph.get_image() {
        out.push_str("<meta property=\"og:image\" content=\"");
        image.write_attr(out);
        out.push_str("\">");
    }
}

/// `link rel="alternate"` na ordem do consumidor.
fn write_alternates(out: &mut alloc::string::String, alternates: &[Alternate<'_>]) {
    for alternate in alternates {
        let hreflang = alternate.hreflang();
        out.push_str("<link rel=\"alternate\" hreflang=\"");
        crate::escape::attr(out, hreflang.as_str());
        out.push_str("\" href=\"");
        alternate.href().write_attr(out);
        out.push_str("\">");
    }
}
