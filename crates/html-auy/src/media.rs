//! Mídia: imagem, `picture`, vídeo, áudio e faixas.
//!
//! Os construtores recebem valores já validados pelo crate (`Src`, `Alt`, `SrcSet`, `Sizes`) e
//! devolvem `Markup` canônico. `srcset` e `sizes` andam juntos: [`Img::responsive`] só emite os
//! dois quando ambos têm conteúdo, tal como o hint `ImagePreload`.

use crate::hint::FetchPriority;
use crate::msg::Text;
use crate::value::{
    Alt, AttrVal, AttrValue, HeightPx, Href, Lang, Media, Sizes, Src, SrcSet, WidthPx,
};
use crate::{Markup, escape};
use alloc::string::String;
use alloc::vec::Vec;

/// Estratégia de carregamento da imagem (`loading`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Loading {
    /// `loading="eager"`.
    Eager,
    /// `loading="lazy"`.
    Lazy,
}

impl Loading {
    /// O valor do atributo.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eager => "eager",
            Self::Lazy => "lazy",
        }
    }
}

/// Imagem com `alt` e dimensões intrínsecas obrigatórios.
///
/// A ordem de atributos segue `src`, `srcset`, `sizes`, `alt`, `width`, `height`; `srcset` só é
/// emitido com `sizes` (a mesma instância usada no hint, quando houver).
#[derive(Debug, Clone)]
pub struct Img<'a> {
    src: Src<'a>,
    alt: Alt<'a>,
    width: WidthPx,
    height: HeightPx,
    srcset: Option<SrcSet<'a>>,
    sizes: Option<Sizes<'a>>,
    loading: Option<Loading>,
    fetchpriority: Option<FetchPriority>,
}

impl<'a> Img<'a> {
    /// Imagem obrigatoriamente descrita e dimensionada.
    #[must_use]
    pub const fn new(src: Src<'a>, alt: Alt<'a>, width: WidthPx, height: HeightPx) -> Self {
        Self {
            src,
            alt,
            width,
            height,
            srcset: None,
            sizes: None,
            loading: None,
            fetchpriority: None,
        }
    }

    /// `srcset` + `sizes`: só vale quando ambos têm conteúdo.
    #[must_use]
    pub fn responsive(mut self, srcset: SrcSet<'a>, sizes: Sizes<'a>) -> Self {
        if !srcset.is_empty() && !sizes.is_empty() {
            self.srcset = Some(srcset);
            self.sizes = Some(sizes);
        }
        self
    }

    /// `loading`.
    #[must_use]
    pub const fn loading(mut self, loading: Loading) -> Self {
        self.loading = Some(loading);
        self
    }

    /// `loading="lazy"` para imagens fora da dobra.
    #[must_use]
    pub const fn lazy(mut self) -> Self {
        self.loading = Some(Loading::Lazy);
        self
    }

    /// `fetchpriority`.
    #[must_use]
    pub const fn fetchpriority(mut self, priority: FetchPriority) -> Self {
        self.fetchpriority = Some(priority);
        self
    }

    /// O HTML da imagem.
    #[must_use]
    pub fn render(&self) -> Markup {
        let mut out = String::with_capacity(256);
        out.push_str("<img src=\"");
        self.src.write_attr(&mut out);
        out.push('"');
        if let (Some(srcset), Some(sizes)) = (&self.srcset, &self.sizes) {
            out.push_str(" srcset=\"");
            srcset.write_attr(&mut out);
            out.push_str("\" sizes=\"");
            sizes.write_attr(&mut out);
            out.push('"');
        }
        out.push_str(" alt=\"");
        self.alt.write_attr(&mut out);
        out.push_str("\" width=\"");
        self.width.write_attr(&mut out);
        out.push_str("\" height=\"");
        self.height.write_attr(&mut out);
        out.push('"');
        if let Some(loading) = self.loading {
            out.push_str(" loading=\"");
            out.push_str(loading.as_str());
            out.push('"');
        }
        if let Some(priority) = self.fetchpriority {
            out.push_str(" fetchpriority=\"");
            out.push_str(priority.as_str());
            out.push('"');
        }
        out.push('>');
        Markup(out)
    }
}

/// `<source>` de `picture`: `srcset` e `sizes` obrigatórios.
#[derive(Debug, Clone)]
pub struct Source<'a> {
    srcset: SrcSet<'a>,
    sizes: Sizes<'a>,
    media: Option<Media<'a>>,
    mime: Option<AttrVal<'a>>,
}

impl<'a> Source<'a> {
    /// Só existe com `srcset` e `sizes` não vazios.
    #[must_use]
    pub fn new(srcset: SrcSet<'a>, sizes: Sizes<'a>) -> Option<Self> {
        if srcset.is_empty() || sizes.is_empty() {
            None
        } else {
            Some(Self {
                srcset,
                sizes,
                media: None,
                mime: None,
            })
        }
    }

    /// Condição `media`.
    #[must_use]
    pub const fn media(mut self, media: Media<'a>) -> Self {
        self.media = Some(media);
        self
    }

    /// Tipo MIME (`type="image/avif"`).
    #[must_use]
    pub const fn mime(mut self, mime: AttrVal<'a>) -> Self {
        self.mime = Some(mime);
        self
    }

    /// O HTML do `<source>`.
    #[must_use]
    pub fn render(&self) -> Markup {
        let mut out = String::with_capacity(192);
        out.push_str("<source srcset=\"");
        self.srcset.write_attr(&mut out);
        out.push_str("\" sizes=\"");
        self.sizes.write_attr(&mut out);
        out.push('"');
        if let Some(media) = self.media {
            out.push_str(" media=\"");
            media.write_attr(&mut out);
            out.push('"');
        }
        if let Some(mime) = self.mime {
            out.push_str(" type=\"");
            mime.write_attr(&mut out);
            out.push('"');
        }
        out.push('>');
        Markup(out)
    }
}

/// `<picture>`: fontes na ordem declarada e um `<img>` obrigatório como fallback.
#[derive(Debug, Clone)]
pub struct Picture<'a> {
    sources: Vec<Source<'a>>,
    fallback: Img<'a>,
}

impl<'a> Picture<'a> {
    /// Fallback obrigatório (quem não escolher um `<source>` usa esta imagem).
    #[must_use]
    pub const fn new(fallback: Img<'a>) -> Self {
        Self {
            sources: Vec::new(),
            fallback,
        }
    }

    /// Acrescenta uma fonte, na ordem de avaliação.
    #[must_use]
    pub fn source(mut self, source: Source<'a>) -> Self {
        self.sources.push(source);
        self
    }

    /// O HTML do `<picture>`.
    #[must_use]
    pub fn render(&self) -> Markup {
        let fallback = self.fallback.render();
        let mut out = String::with_capacity(
            fallback
                .as_str()
                .len()
                .saturating_add(self.sources.len().saturating_mul(192))
                .saturating_add(24),
        );
        out.push_str("<picture>");
        for source in &self.sources {
            out.push_str(source.render().as_str());
        }
        out.push_str(fallback.as_str());
        out.push_str("</picture>");
        Markup(out)
    }
}

/// `preload` de mídia.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaPreload {
    /// `none`.
    None,
    /// `metadata`.
    Metadata,
    /// `auto`.
    Auto,
}

impl MediaPreload {
    /// O valor do atributo.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Metadata => "metadata",
            Self::Auto => "auto",
        }
    }
}

/// `kind` de `<track>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrackKind {
    /// `subtitles`.
    Subtitles,
    /// `captions`.
    Captions,
    /// `descriptions`.
    Descriptions,
    /// `chapters`.
    Chapters,
    /// `metadata`.
    Metadata,
}

impl TrackKind {
    /// O valor do atributo.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Subtitles => "subtitles",
            Self::Captions => "captions",
            Self::Descriptions => "descriptions",
            Self::Chapters => "chapters",
            Self::Metadata => "metadata",
        }
    }
}

/// Link para a transcrição textual de uma mídia falada.
///
/// A transcrição não é um `<track>`: ela é conteúdo alternativo, normalmente uma página ou uma
/// seção com `id`. O escape é o do link; a mídia continua sendo emitida no mesmo `figure`.
#[derive(Debug, Clone)]
pub struct Transcript<'a> {
    href: Href<'a>,
    label: Text<'a>,
}

impl<'a> Transcript<'a> {
    /// Link de transcrição para `href`, com nome acessível em `label`.
    #[must_use]
    pub const fn new(href: Href<'a>, label: Text<'a>) -> Self {
        Self { href, label }
    }

    /// O HTML do link.
    #[must_use]
    pub fn render(&self) -> Markup {
        let mut out = String::with_capacity(self.label.as_str().len().saturating_add(32));
        out.push_str("<a href=\"");
        self.href.write_attr(&mut out);
        out.push_str("\">");
        escape::text(&mut out, self.label.as_str());
        out.push_str("</a>");
        Markup(out)
    }
}

/// Faixa de texto (`<track>`), com `srclang` obrigatório em legendas.
#[derive(Debug, Clone)]
pub struct Track<'a> {
    src: Src<'a>,
    kind: TrackKind,
    label: Text<'a>,
    srclang: Option<Lang>,
    default: bool,
}

impl<'a> Track<'a> {
    /// Faixa sem `srclang` (só use em `captions`/`descriptions`/`chapters`/`metadata`).
    #[must_use]
    pub const fn new(src: Src<'a>, kind: TrackKind, label: Text<'a>) -> Self {
        Self {
            src,
            kind,
            label,
            srclang: None,
            default: false,
        }
    }

    /// Legendas (`kind="captions"`).
    #[must_use]
    pub const fn captions(src: Src<'a>, label: Text<'a>) -> Self {
        Self::new(src, TrackKind::Captions, label)
    }

    /// Audiodescrição (`kind="descriptions"`).
    ///
    /// É o caminho tipado para WCAG 1.2.3/1.2.5 em vídeo; uma transcrição completa
    /// ([`Transcript`]) é a alternativa de 1.2.3 quando não há faixa descritiva.
    #[must_use]
    pub const fn descriptions(src: Src<'a>, label: Text<'a>) -> Self {
        Self::new(src, TrackKind::Descriptions, label)
    }

    /// Tradução (`kind="subtitles"`), com o idioma de origem.
    #[must_use]
    pub const fn subtitles(src: Src<'a>, label: Text<'a>, lang: Lang) -> Self {
        Self {
            src,
            kind: TrackKind::Subtitles,
            label,
            srclang: Some(lang),
            default: false,
        }
    }

    /// `srclang` (obrigatório quando `kind="subtitles"`).
    #[must_use]
    pub const fn srclang(mut self, lang: Lang) -> Self {
        self.srclang = Some(lang);
        self
    }

    /// `default`.
    #[must_use]
    pub const fn is_default(mut self, yes: bool) -> Self {
        self.default = yes;
        self
    }

    /// O HTML do `<track>`.
    #[must_use]
    pub fn render(&self) -> Markup {
        let mut out = String::with_capacity(160);
        out.push_str("<track kind=\"");
        out.push_str(self.kind.as_str());
        out.push_str("\" src=\"");
        self.src.write_attr(&mut out);
        out.push('"');
        if let Some(lang) = self.srclang {
            out.push_str(" srclang=\"");
            lang.write_attr(&mut out);
            out.push('"');
        }
        out.push_str(" label=\"");
        escape::attr(&mut out, self.label.as_str());
        out.push('"');
        if self.default {
            out.push_str(" default");
        }
        out.push('>');
        Markup(out)
    }
}

/// Vídeo com legendas obrigatórias na construção (WCAG 1.2.2).
#[expect(
    clippy::struct_excessive_bools,
    reason = "atributos HTML independentes (`controls`, `muted`, `loop`, `playsinline`); um enum por combinação criaria estados que o HTML aceita mas o builder recusaria"
)]
#[derive(Debug, Clone)]
pub struct Video<'a> {
    src: Src<'a>,
    tracks: Vec<Track<'a>>,
    controls: bool,
    preload: Option<MediaPreload>,
    poster: Option<Src<'a>>,
    muted: bool,
    looping: bool,
    playsinline: bool,
    width: Option<WidthPx>,
    height: Option<HeightPx>,
    transcript: Option<Transcript<'a>>,
    fallback: Option<Text<'a>>,
}

impl<'a> Video<'a> {
    /// Vídeo com a faixa de legendas obrigatória; outras faixas entram por [`Self::track`].
    #[must_use]
    pub fn new(src: Src<'a>, captions: Track<'a>) -> Self {
        let tracks = alloc::vec![captions];
        Self {
            src,
            tracks,
            controls: true,
            preload: None,
            poster: None,
            muted: false,
            looping: false,
            playsinline: false,
            width: None,
            height: None,
            transcript: None,
            fallback: None,
        }
    }

    /// Faixa adicional (traduções, descrições, capítulos).
    #[must_use]
    pub fn track(mut self, track: Track<'a>) -> Self {
        self.tracks.push(track);
        self
    }

    /// `controls` (padrão `true`; desligue só com controles próprios acessíveis).
    #[must_use]
    pub const fn controls(mut self, yes: bool) -> Self {
        self.controls = yes;
        self
    }

    /// `preload`.
    #[must_use]
    pub const fn preload(mut self, preload: MediaPreload) -> Self {
        self.preload = Some(preload);
        self
    }

    /// `poster`.
    #[must_use]
    pub const fn poster(mut self, poster: Src<'a>) -> Self {
        self.poster = Some(poster);
        self
    }

    /// `muted`.
    #[must_use]
    pub const fn muted(mut self, yes: bool) -> Self {
        self.muted = yes;
        self
    }

    /// `loop`.
    #[must_use]
    pub const fn looping(mut self, yes: bool) -> Self {
        self.looping = yes;
        self
    }

    /// `playsinline`.
    #[must_use]
    pub const fn playsinline(mut self, yes: bool) -> Self {
        self.playsinline = yes;
        self
    }

    /// Dimensões intrínsecas (as duas, para evitar layout shift).
    #[must_use]
    pub const fn dimensions(mut self, width: WidthPx, height: HeightPx) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Transcrição completa; quando presente, o `<video>` é envolvido em `<figure>` com um link.
    ///
    /// Use como alternativa de 1.2.3 quando o vídeo não tem faixa `descriptions`; para 1.2.5,
    /// prefira [`Track::descriptions`].
    #[must_use]
    pub fn transcript(mut self, transcript: Transcript<'a>) -> Self {
        self.transcript = Some(transcript);
        self
    }

    /// Texto de fallback para navegador sem suporte.
    #[must_use]
    pub const fn fallback(mut self, text: Text<'a>) -> Self {
        self.fallback = Some(text);
        self
    }

    /// O HTML do `<video>` (ou do `<figure>` quando há transcrição).
    #[must_use]
    pub fn render(&self) -> Markup {
        let mut out = String::with_capacity(256);
        out.push_str("<video src=\"");
        self.src.write_attr(&mut out);
        out.push('"');
        if self.controls {
            out.push_str(" controls");
        }
        if let Some(preload) = self.preload {
            out.push_str(" preload=\"");
            out.push_str(preload.as_str());
            out.push('"');
        }
        if let Some(poster) = self.poster {
            out.push_str(" poster=\"");
            poster.write_attr(&mut out);
            out.push('"');
        }
        if self.muted {
            out.push_str(" muted");
        }
        if self.looping {
            out.push_str(" loop");
        }
        if self.playsinline {
            out.push_str(" playsinline");
        }
        if let (Some(width), Some(height)) = (self.width, self.height) {
            out.push_str(" width=\"");
            width.write_attr(&mut out);
            out.push_str("\" height=\"");
            height.write_attr(&mut out);
            out.push('"');
        }
        out.push('>');
        for track in &self.tracks {
            out.push_str(track.render().as_str());
        }
        if let Some(text) = &self.fallback {
            escape::text(&mut out, text.as_str());
        }
        out.push_str("</video>");
        with_transcript(Markup(out), self.transcript.as_ref())
    }
}

/// Áudio com controles e faixas opcionais.
#[derive(Debug, Clone)]
pub struct Audio<'a> {
    src: Src<'a>,
    tracks: Vec<Track<'a>>,
    controls: bool,
    preload: Option<MediaPreload>,
    looping: bool,
    transcript: Option<Transcript<'a>>,
    fallback: Option<Text<'a>>,
}

impl<'a> Audio<'a> {
    /// Áudio com controles (padrão `true`).
    #[must_use]
    pub const fn new(src: Src<'a>) -> Self {
        Self {
            src,
            tracks: Vec::new(),
            controls: true,
            preload: None,
            looping: false,
            transcript: None,
            fallback: None,
        }
    }

    /// Faixa de texto (legendas, descrições, capítulos).
    #[must_use]
    pub fn track(mut self, track: Track<'a>) -> Self {
        self.tracks.push(track);
        self
    }

    /// `controls`.
    #[must_use]
    pub const fn controls(mut self, yes: bool) -> Self {
        self.controls = yes;
        self
    }

    /// `preload`.
    #[must_use]
    pub const fn preload(mut self, preload: MediaPreload) -> Self {
        self.preload = Some(preload);
        self
    }

    /// `loop`.
    #[must_use]
    pub const fn looping(mut self, yes: bool) -> Self {
        self.looping = yes;
        self
    }

    /// Transcrição completa; quando presente, o `<audio>` é envolvido em `<figure>` com um link.
    ///
    /// É o caminho tipado para 1.2.1 em áudio falado. `Track` não substitui transcrição: áudio não
    /// tem área visual para legenda.
    #[must_use]
    pub fn transcript(mut self, transcript: Transcript<'a>) -> Self {
        self.transcript = Some(transcript);
        self
    }

    /// Texto de fallback.
    #[must_use]
    pub const fn fallback(mut self, text: Text<'a>) -> Self {
        self.fallback = Some(text);
        self
    }

    /// O HTML do `<audio>` (ou do `<figure>` quando há transcrição).
    #[must_use]
    pub fn render(&self) -> Markup {
        let mut out = String::with_capacity(192);
        out.push_str("<audio src=\"");
        self.src.write_attr(&mut out);
        out.push('"');
        if self.controls {
            out.push_str(" controls");
        }
        if let Some(preload) = self.preload {
            out.push_str(" preload=\"");
            out.push_str(preload.as_str());
            out.push('"');
        }
        if self.looping {
            out.push_str(" loop");
        }
        out.push('>');
        for track in &self.tracks {
            out.push_str(track.render().as_str());
        }
        if let Some(text) = &self.fallback {
            escape::text(&mut out, text.as_str());
        }
        out.push_str("</audio>");
        with_transcript(Markup(out), self.transcript.as_ref())
    }
}

/// Envolve a mídia em `<figure>` apenas quando há transcrição declarada.
fn with_transcript(media: Markup, transcript: Option<&Transcript<'_>>) -> Markup {
    let Some(transcript) = transcript else {
        return media;
    };
    let link = transcript.render();
    let mut out = String::with_capacity(
        media
            .as_str()
            .len()
            .saturating_add(link.as_str().len())
            .saturating_add(48),
    );
    out.push_str("<figure>");
    out.push_str(media.as_str());
    out.push_str("<figcaption>");
    out.push_str(link.as_str());
    out.push_str("</figcaption></figure>");
    Markup(out)
}
