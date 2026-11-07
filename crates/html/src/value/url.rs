//! URLs: cada destino tem um tipo, e o tipo recusa o que não pode ser emitido.

use crate::shell::Asset;
use alloc::borrow::Cow;
use alloc::string::String;

/// Caminho interno do site (`/noticias/a-frota-nova`), validado na construção.
///
/// Garantias: começa com `/`, não é `//` (protocol-relative), não tem `:`, `\`, espaço, controle,
/// `"`, `<`, `>`, crase, e nenhum segmento é `..` (nada de escapar da raiz). `?` e `#` são aceitos.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LUrl<'a>(Cow<'a, str>);

impl<'a> LUrl<'a> {
    /// Valida um caminho emprestado.
    #[must_use]
    pub fn new(path: &'a str) -> Option<Self> {
        if valid_path(path) {
            Some(Self(Cow::Borrowed(path)))
        } else {
            None
        }
    }

    /// Valida um caminho já alocado (sobrecusto zero na emissão).
    #[must_use]
    pub fn from_string(path: String) -> Option<Self> {
        if valid_path(&path) {
            Some(Self(Cow::Owned(path)))
        } else {
            None
        }
    }

    /// Junta `base` e `segments`, percent-encoding cada segmento (`%XX` maiúsculo).
    ///
    /// ```
    /// let url = html::LUrl::with_segments("/noticias", &["a frota", "ônibus"]).unwrap();
    /// assert_eq!(url.as_str(), "/noticias/a%20frota/%C3%B4nibus");
    /// ```
    #[must_use]
    pub fn with_segments(base: &str, segments: &[&str]) -> Option<LUrl<'static>> {
        // Segmentos entram no caminho, não na query/fragmento da base.
        if !valid_path(base) || base.contains(['?', '#']) {
            return None;
        }
        let mut path = String::from(base.trim_end_matches('/'));
        for segment in segments {
            path.push('/');
            percent_encode(&mut path, segment);
        }
        if path.is_empty() || path == "/" {
            path.push('/');
        }
        // O caminho montado passa pela MESMA validação de `new`: segmento `..` não escapa da raiz.
        if !valid_path(&path) {
            return None;
        }
        Some(LUrl(Cow::Owned(path)))
    }

    /// O caminho.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// URL absoluta (`http`/`https`), o único destino externo aceito.
///
/// `javascript:`, `data:`, `vbscript:` e `file:` **não** constroem: o esquema é decide antes do
/// parse, não depois do escape. A emissão normaliza o esquema para minúsculas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalUrl<'a>(&'a str);

impl<'a> ExternalUrl<'a> {
    /// Valida uma URL externa.
    #[must_use]
    pub fn parse(input: &'a str) -> Option<Self> {
        let (scheme, rest) = split_scheme(input)?;
        if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
            return None;
        }
        if rest.is_empty() || rest.contains('\u{0}') {
            return None;
        }
        if input
            .chars()
            .any(|ch| ch.is_control() || ch == ' ' || ch == '"')
        {
            return None;
        }
        Some(Self(input))
    }

    /// A URL como veio.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    /// Escreve como valor de atributo, com o esquema normalizado para minúsculas.
    pub(crate) fn write_attr(&self, out: &mut String) {
        write_external(out, self.0);
    }
}

/// Endereço de e-mail (`mailto:`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MailAddr<'a>(&'a str);

impl<'a> MailAddr<'a> {
    /// Valida `local@dominio.tld` (conservador: um `@`, domínio com ponto, sem controle).
    #[must_use]
    pub fn parse(input: &'a str) -> Option<Self> {
        if input.chars().any(|ch| {
            ch.is_control() || ch.is_ascii_whitespace() || matches!(ch, '<' | '>' | '"' | '\\')
        }) {
            return None;
        }
        let (local, domain) = input.split_once('@')?;
        if local.is_empty() || domain.contains('@') {
            return None;
        }
        let (head, tail) = domain.split_once('.')?;
        if head.is_empty() || tail.is_empty() || tail.ends_with('.') {
            return None;
        }
        Some(Self(input))
    }

    /// O endereço.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Telefone em E.164 (`+5511999998888`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhoneNumber<'a>(&'a str);

impl<'a> PhoneNumber<'a> {
    /// Valida `^\+[0-9]{1,15}$`.
    #[must_use]
    pub fn parse(input: &'a str) -> Option<Self> {
        let digits = input.strip_prefix('+')?;
        if digits.is_empty() || digits.len() > 15 || !digits.chars().all(|ch| ch.is_ascii_digit()) {
            return None;
        }
        Some(Self(input))
    }

    /// O número.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Fragmento de documento (`#conteudo`), o único destino que não sai da página.
///
/// Garantias: começa com `#`; o resto é vazio ou usa só `[A-Za-z0-9_-]` (o mesmo alfabeto de
/// [`crate::Id`]). Isso cobre âncoras de skip link e de índice sem abrir URL arbitrária.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fragment<'a>(&'a str);

impl<'a> Fragment<'a> {
    /// Valida um fragmento (`#`, `#conteudo`).
    #[must_use]
    pub fn new(input: &'a str) -> Option<Self> {
        let name = input.strip_prefix('#')?;
        let valid = name.is_empty()
            || name
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-');
        if valid { Some(Self(input)) } else { None }
    }

    /// O fragmento com o `#`.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Destino de um `href`: só as formas que o crate sabe emitir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Href<'a> {
    /// Caminho interno.
    Internal(LUrl<'a>),
    /// Fragmento da própria página (`#conteudo`).
    Fragment(Fragment<'a>),
    /// URL absoluta `http`/`https`.
    External(ExternalUrl<'a>),
    /// `mailto:`.
    Mailto(MailAddr<'a>),
    /// `tel:`.
    Tel(PhoneNumber<'a>),
}

impl<'a> Href<'a> {
    /// Link interno.
    #[must_use]
    pub const fn internal(url: LUrl<'a>) -> Self {
        Self::Internal(url)
    }

    /// Fragmento da própria página, já validado.
    #[must_use]
    pub const fn fragment(fragment: Fragment<'a>) -> Self {
        Self::Fragment(fragment)
    }

    /// Link externo já validado.
    #[must_use]
    pub const fn external(url: ExternalUrl<'a>) -> Self {
        Self::External(url)
    }

    /// Link `mailto:`.
    #[must_use]
    pub const fn mailto(address: MailAddr<'a>) -> Self {
        Self::Mailto(address)
    }

    /// Link `tel:`.
    #[must_use]
    pub const fn tel(number: PhoneNumber<'a>) -> Self {
        Self::Tel(number)
    }

    /// Escreve o valor já escapado.
    pub(crate) fn write_href(&self, out: &mut String) {
        match self {
            Self::Internal(url) => crate::escape::attr(out, url.as_str()),
            Self::Fragment(fragment) => crate::escape::attr(out, fragment.as_str()),
            Self::External(url) => write_external(out, url.as_str()),
            Self::Mailto(address) => {
                out.push_str("mailto:");
                crate::escape::attr(out, address.as_str());
            }
            Self::Tel(number) => {
                out.push_str("tel:");
                crate::escape::attr(out, number.as_str());
            }
        }
    }
}

/// Fonte de `src`/`poster`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Src<'a>(&'a str);

impl<'a> Src<'a> {
    /// Fonte de mídia do consumidor: o caminho já é `LUrl` (validado na construção).
    #[must_use]
    pub fn media(media: &'a impl MediaPath) -> Self {
        Self(media.path().as_str())
    }

    /// Caminho de asset declarado no shell.
    #[must_use]
    pub const fn asset(asset: &'a Asset) -> Self {
        Self(asset.href())
    }

    /// Fonte de mídia a partir de caminho interno validado **ou** URL absoluta `http(s)`.
    ///
    /// Caminho interno segue a mesma gramática de [`LUrl`]; URL externa passa por
    /// [`ExternalUrl`]. `javascript:`, `data:` e `//host` não constroem.
    #[must_use]
    pub fn parse(input: &'a str) -> Option<Self> {
        if ExternalUrl::parse(input).is_some() || valid_path(input) {
            Some(Self(input))
        } else {
            None
        }
    }

    /// O caminho.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Objeto de mídia do consumidor (chave de storage, caminho de CDN).
///
/// O caminho é [`LUrl`], não `&str`: `src` não escapa do tipo validado, e um caminho hostil
/// (`javascript:`, `data:`) não atravessa a fronteira.
pub trait MediaPath {
    /// O caminho público do objeto, já validado.
    fn path(&self) -> &LUrl<'_>;
}

impl<T: MediaPath + ?Sized> MediaPath for &T {
    fn path(&self) -> &LUrl<'_> {
        (*self).path()
    }
}

/// `true` se `input` é um caminho interno aceitável (mesma regra de [`LUrl`]).
#[expect(
    clippy::redundant_pub_crate,
    reason = "`pub(crate)` é a visibilidade real: `pub` aqui viraria `unreachable_pub`"
)]
pub(crate) fn valid_path(input: &str) -> bool {
    if !input.starts_with('/') || input.starts_with("//") {
        return false;
    }
    let rejected = input.chars().any(|ch| {
        ch.is_control()
            || ch.is_ascii_whitespace()
            || matches!(ch, ':' | '\\' | '"' | '<' | '>' | '`')
    });
    if rejected {
        return false;
    }
    !input.split('/').any(is_dot_segment)
}

/// Segmento que o parser de URL do navegador normaliza para `.`/`..` (inclusive `%2e`).
fn is_dot_segment(segment: &str) -> bool {
    segment == "."
        || segment == ".."
        || segment.eq_ignore_ascii_case(".%2e")
        || segment.eq_ignore_ascii_case("%2e.")
        || segment.eq_ignore_ascii_case("%2e%2e")
}

/// `(esquema, resto sem "://")`.
fn split_scheme(input: &str) -> Option<(&str, &str)> {
    let (scheme, rest) = input.split_once("://")?;
    if scheme.is_empty()
        || !scheme
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '+' || ch == '-')
    {
        return None;
    }
    Some((scheme, rest))
}

/// Escreve a URL externa com o esquema em minúsculas.
fn write_external(out: &mut String, input: &str) {
    let Some((scheme, rest)) = split_scheme(input) else {
        return;
    };
    for byte in scheme.bytes() {
        out.push(char::from(byte.to_ascii_lowercase()));
    }
    out.push_str("://");
    crate::escape::attr(out, rest);
}

/// Percent-encoding de um segmento de caminho (`%XX` maiúsculo, não-escapáveis preservados).
fn percent_encode(out: &mut String, segment: &str) {
    const UNRESERVED: &[u8] = b"-._~";
    for byte in segment.bytes() {
        if byte.is_ascii_alphanumeric() || UNRESERVED.contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            let high = byte >> 4;
            let low = byte & 0x0f;
            out.push(hex_digit(high));
            out.push(hex_digit(low));
        }
    }
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => char::from(b'0'.saturating_add(value)),
        _ => char::from(b'A'.saturating_add(value.saturating_sub(10))),
    }
}
