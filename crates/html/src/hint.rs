//! Resource hints: o que o `<head>` pode dizer antes de o parser chegar ao recurso.
//!
//! `link` é `only = "shell"`: `html!` recusa o elemento e o consumidor não tem como escrever
//! `<link rel="preconnect">` à mão. Este módulo é o único caminho — e ele codifica as
//! invariantes que o `Page` promete: `preload` sempre com `as`, fonte sempre com `crossorigin`,
//! `ImagePreload` com `imagesrcset`/`imagesizes` só quando ambos existem, ordem fixa e
//! deduplicação.
//!
//! A ordem de emissão é fixa por espécie (`preconnect` → `dns-prefetch` → `preload` →
//! `modulepreload` → `prefetch`), com a ordem de declaração preservada dentro da espécie.

use crate::escape;
use crate::value::{AttrValue, ExternalUrl, Integrity, Media, Sizes, Src, SrcSet};
use alloc::string::String;

/// Espécie de hint emitida pelo [`crate::shell::Page`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hint<'a> {
    /// `<link rel="preconnect">`: DNS + TCP + TLS antecipados para uma origem externa.
    Preconnect(ExternalUrl<'a>),
    /// `<link rel="dns-prefetch">`: só a resolução DNS, mais barata que `preconnect`.
    DnsPrefetch(ExternalUrl<'a>),
    /// `<link rel="preload">` genérico.
    Preload(Preload<'a>),
    /// `<link rel="preload" as="image">` com `imagesrcset`/`imagesizes` tipados.
    ImagePreload(ImagePreload<'a>),
    /// `<link rel="modulepreload">`: módulo ES baixado, analisado e compilado.
    ModulePreload(Src<'a>),
    /// `<link rel="prefetch">`: recurso provavelmente necessário na navegação seguinte.
    Prefetch(Src<'a>),
}

/// Valor de `as` em [`Preload`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreloadAs {
    /// Áudio.
    Audio,
    /// Documento HTML.
    Document,
    /// Conteúdo embutido.
    Embed,
    /// Requisição `fetch`.
    Fetch,
    /// Fonte.
    Font,
    /// Imagem.
    Image,
    /// Objeto.
    Object,
    /// Script clássico.
    Script,
    /// Folha de estilo.
    Style,
    /// Faixa de mídia.
    Track,
    /// Vídeo.
    Video,
    /// Worker.
    Worker,
}

impl PreloadAs {
    /// O valor do atributo `as`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Document => "document",
            Self::Embed => "embed",
            Self::Fetch => "fetch",
            Self::Font => "font",
            Self::Image => "image",
            Self::Object => "object",
            Self::Script => "script",
            Self::Style => "style",
            Self::Track => "track",
            Self::Video => "video",
            Self::Worker => "worker",
        }
    }
}

/// Prioridade relativa de download (`fetchpriority`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FetchPriority {
    /// Prioridade alta: a imagem LCP ou um script crítico.
    High,
    /// Prioridade baixa.
    Low,
    /// Deixa o navegador decidir.
    Auto,
}

impl FetchPriority {
    /// O valor do atributo `fetchpriority`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Low => "low",
            Self::Auto => "auto",
        }
    }
}

/// Modo CORS de um recurso pré-carregado.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossOrigin {
    /// `crossorigin="anonymous"`.
    Anonymous,
    /// `crossorigin="use-credentials"`.
    UseCredentials,
}

impl CrossOrigin {
    /// O valor do atributo `crossorigin`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anonymous => "anonymous",
            Self::UseCredentials => "use-credentials",
        }
    }
}

/// Política de referrer (`referrerpolicy`) aceita em `link`.
///
/// `unsafe-url` fica de fora: vaza a URL completa para origens de terceiros.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferrerPolicy {
    /// `no-referrer`.
    NoReferrer,
    /// `no-referrer-when-downgrade`.
    NoReferrerWhenDowngrade,
    /// `origin`.
    Origin,
    /// `origin-when-cross-origin`.
    OriginWhenCrossOrigin,
    /// `same-origin`.
    SameOrigin,
    /// `strict-origin`.
    StrictOrigin,
    /// `strict-origin-when-cross-origin`.
    StrictOriginWhenCrossOrigin,
}

impl ReferrerPolicy {
    /// O valor do atributo `referrerpolicy`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReferrer => "no-referrer",
            Self::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            Self::Origin => "origin",
            Self::OriginWhenCrossOrigin => "origin-when-cross-origin",
            Self::SameOrigin => "same-origin",
            Self::StrictOrigin => "strict-origin",
            Self::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
        }
    }
}

/// Um `<link rel="preload">` genérico.
///
/// Os construtores cobrem os casos que mais erram na mão; [`Preload::new`] aceita qualquer
/// combinação de `as`. `font` sempre emite `type="font/woff2"` e `crossorigin="anonymous"` (sem
/// `crossorigin` o navegador baixa a fonte duas vezes). Para imagem responsiva, use
/// [`Preload::image`], que devolve [`ImagePreload`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preload<'a> {
    href: Src<'a>,
    as_: PreloadAs,
    type_: Option<&'static str>,
    crossorigin: Option<CrossOrigin>,
    integrity: Option<Integrity<'a>>,
    referrerpolicy: Option<ReferrerPolicy>,
    media: Option<Media<'a>>,
    fetchpriority: Option<FetchPriority>,
}

impl<'a> Preload<'a> {
    /// Preload com `as` explícito; os demais atributos entram pelos builders.
    #[must_use]
    pub const fn new(href: Src<'a>, as_: PreloadAs) -> Self {
        Self {
            href,
            as_,
            type_: None,
            crossorigin: None,
            integrity: None,
            referrerpolicy: None,
            media: None,
            fetchpriority: None,
        }
    }

    /// Fonte: `as="font"`, `type="font/woff2"` e `crossorigin="anonymous"` sempre juntos.
    #[must_use]
    pub const fn font(href: Src<'a>) -> Self {
        Self::new(href, PreloadAs::Font)
            .with_type("font/woff2")
            .crossorigin(CrossOrigin::Anonymous)
    }

    /// Imagem: `as="image"`; o `imagesrcset`/`imagesizes` só existe em [`ImagePreload`].
    #[must_use]
    pub const fn image(href: Src<'a>) -> ImagePreload<'a> {
        ImagePreload {
            preload: Self::new(href, PreloadAs::Image),
            imagesrcset: None,
            imagesizes: None,
        }
    }

    /// Folha de estilo: `as="style"`; [`Preload::media`] cobre `print`/consulta condicional.
    #[must_use]
    pub const fn style(href: Src<'a>) -> Self {
        Self::new(href, PreloadAs::Style)
    }

    /// Script: `as="script"`; prefira [`Hint::ModulePreload`] para módulos.
    #[must_use]
    pub const fn script(href: Src<'a>) -> Self {
        Self::new(href, PreloadAs::Script)
    }

    /// `type`: MIME do recurso (por exemplo `font/woff2`, `image/avif`).
    #[must_use]
    pub const fn with_type(mut self, mime: &'static str) -> Self {
        self.type_ = Some(mime);
        self
    }

    /// `crossorigin`.
    #[must_use]
    pub const fn crossorigin(mut self, mode: CrossOrigin) -> Self {
        self.crossorigin = Some(mode);
        self
    }

    /// `integrity`: SRI validado (`sha256-…`, `sha384-…`, `sha512-…`).
    #[must_use]
    pub const fn integrity(mut self, integrity: Integrity<'a>) -> Self {
        self.integrity = Some(integrity);
        self
    }

    /// `referrerpolicy`: política de referrer do recurso.
    #[must_use]
    pub const fn referrerpolicy(mut self, policy: ReferrerPolicy) -> Self {
        self.referrerpolicy = Some(policy);
        self
    }

    /// `media`: consulta vazia é ignorada (o atributo não é emitido).
    #[must_use]
    pub const fn media(mut self, media: Media<'a>) -> Self {
        self.media = if media.is_empty() { None } else { Some(media) };
        self
    }

    /// `fetchpriority`.
    #[must_use]
    pub const fn fetchpriority(mut self, priority: FetchPriority) -> Self {
        self.fetchpriority = Some(priority);
        self
    }
}

/// Imagem de [`Preload`]: `as="image"` com `imagesrcset`/`imagesizes` acoplados.
///
/// [`ImagePreload::responsive`] é a única porta para os dois atributos; não existe em [`Preload`],
/// então `Preload::font(...).responsive(...)` é erro de compilação.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImagePreload<'a> {
    preload: Preload<'a>,
    imagesrcset: Option<SrcSet<'a>>,
    imagesizes: Option<Sizes<'a>>,
}

impl<'a> ImagePreload<'a> {
    /// `imagesrcset`/`imagesizes`: use as **mesmas** instâncias do `<img>` para não baixar duas vezes.
    ///
    /// Sem os dois valores não vazios, nenhum dos atributos é emitido.
    #[must_use]
    pub fn responsive(mut self, srcset: SrcSet<'a>, sizes: Sizes<'a>) -> Self {
        if !srcset.is_empty() && !sizes.is_empty() {
            self.imagesrcset = Some(srcset);
            self.imagesizes = Some(sizes);
        }
        self
    }

    /// `type`: MIME da imagem.
    #[must_use]
    pub const fn with_type(mut self, mime: &'static str) -> Self {
        self.preload = self.preload.with_type(mime);
        self
    }

    /// `crossorigin`.
    #[must_use]
    pub const fn crossorigin(mut self, mode: CrossOrigin) -> Self {
        self.preload = self.preload.crossorigin(mode);
        self
    }

    /// `integrity`.
    #[must_use]
    pub const fn integrity(mut self, integrity: Integrity<'a>) -> Self {
        self.preload = self.preload.integrity(integrity);
        self
    }

    /// `referrerpolicy`.
    #[must_use]
    pub const fn referrerpolicy(mut self, policy: ReferrerPolicy) -> Self {
        self.preload = self.preload.referrerpolicy(policy);
        self
    }

    /// `media`.
    #[must_use]
    pub const fn media(mut self, media: Media<'a>) -> Self {
        self.preload = self.preload.media(media);
        self
    }

    /// `fetchpriority`.
    #[must_use]
    pub const fn fetchpriority(mut self, priority: FetchPriority) -> Self {
        self.preload = self.preload.fetchpriority(priority);
        self
    }
}

/// Emite os hints na ordem fixa do `<head>`.
pub(crate) fn write(out: &mut String, hints: &[Hint<'_>]) {
    for hint in hints {
        if let Hint::Preconnect(url) = hint {
            write_url_hint(out, "preconnect", url);
        }
    }
    for hint in hints {
        if let Hint::DnsPrefetch(url) = hint {
            write_url_hint(out, "dns-prefetch", url);
        }
    }
    for hint in hints {
        match hint {
            Hint::Preload(preload) => write_preload(out, preload, None),
            Hint::ImagePreload(image) => write_preload(
                out,
                &image.preload,
                image.imagesrcset.as_ref().zip(image.imagesizes.as_ref()),
            ),
            _ => {}
        }
    }
    for hint in hints {
        if let Hint::ModulePreload(src) = hint {
            write_src_hint(out, "modulepreload", *src);
        }
    }
    for hint in hints {
        if let Hint::Prefetch(src) = hint {
            write_src_hint(out, "prefetch", *src);
        }
    }
}

fn write_url_hint(out: &mut String, rel: &str, url: &ExternalUrl<'_>) {
    out.push_str("<link rel=\"");
    out.push_str(rel);
    out.push_str("\" href=\"");
    url.write_attr(out);
    out.push_str("\">");
}

fn write_src_hint(out: &mut String, rel: &str, src: Src<'_>) {
    out.push_str("<link rel=\"");
    out.push_str(rel);
    out.push_str("\" href=\"");
    src.write_attr(out);
    out.push_str("\">");
}

fn write_preload(
    out: &mut String,
    preload: &Preload<'_>,
    images: Option<(&SrcSet<'_>, &Sizes<'_>)>,
) {
    out.push_str("<link rel=\"preload\" href=\"");
    preload.href.write_attr(out);
    out.push_str("\" as=\"");
    out.push_str(preload.as_.as_str());
    out.push('"');
    if let Some(mime) = preload.type_ {
        out.push_str(" type=\"");
        escape::attr(out, mime);
        out.push('"');
    }
    if let Some(mode) = preload.crossorigin {
        out.push_str(" crossorigin=\"");
        out.push_str(mode.as_str());
        out.push('"');
    }
    if let Some(integrity) = &preload.integrity {
        out.push_str(" integrity=\"");
        integrity.write_attr(out);
        out.push('"');
    }
    if let Some(policy) = preload.referrerpolicy {
        out.push_str(" referrerpolicy=\"");
        out.push_str(policy.as_str());
        out.push('"');
    }
    if let Some(media) = &preload.media {
        out.push_str(" media=\"");
        media.write_attr(out);
        out.push('"');
    }
    if let Some(priority) = preload.fetchpriority {
        out.push_str(" fetchpriority=\"");
        out.push_str(priority.as_str());
        out.push('"');
    }
    if let Some((srcset, sizes)) = images {
        out.push_str(" imagesrcset=\"");
        srcset.write_attr(out);
        out.push('"');
        out.push_str(" imagesizes=\"");
        sizes.write_attr(out);
        out.push('"');
    }
    out.push('>');
}
