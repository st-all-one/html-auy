//! Idioma e direção: `Lang` é etiqueta BCP-47 validada, não string livre.

/// Maior etiqueta BCP-47 que o crate aceita (`language-script-region-variant` cabe em 35 bytes).
const LANG_CAPACITY: usize = 35;

/// Etiqueta BCP-47 canônica (`pt-BR`, `zh-Hant-TW`, `en`).
///
/// A representação é um buffer fixo terminado em NUL: `Copy`, `Ord`, sem alocação. A
/// canonicalização (`language` minúsculo, `script` capitalizado, `region` maiúsculo) é o que faz
/// `pt-br` e `PT-BR` gerarem os **mesmos bytes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lang {
    buf: [u8; LANG_CAPACITY],
}

/// Como a posição do subtipo decide a caixa.
#[derive(Clone, Copy)]
enum Case {
    /// `language`, `variant`.
    Lower,
    /// `region`.
    Upper,
    /// `script`.
    Title,
}

impl Case {
    /// `true` quando o caractere desta posição vai em maiúscula.
    const fn wants_upper(self, first: bool) -> bool {
        match self {
            Self::Upper => true,
            Self::Title => first,
            Self::Lower => false,
        }
    }
}

impl Lang {
    /// `en`.
    pub const EN: Self = Self::from_bytes(b"en");

    /// Valida e canonicaliza uma etiqueta BCP-47 (language, script, region, variants).
    #[must_use]
    pub fn parse(input: &str) -> Option<Self> {
        let mut buf = [0u8; LANG_CAPACITY];
        let mut cursor = 0;
        for (position, part) in input.split('-').enumerate() {
            if part.is_empty() {
                return None;
            }
            if position > 0 && !push_byte(&mut buf, &mut cursor, b'-') {
                return None;
            }
            let case = classify(part, position)?;
            if !push_part(&mut buf, &mut cursor, part, case) {
                return None;
            }
        }
        if cursor == 0 {
            None
        } else {
            Some(Self { buf })
        }
    }

    /// A etiqueta canônica.
    #[must_use]
    pub fn as_str(&self) -> &str {
        let end = self
            .buf
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(LANG_CAPACITY);
        self.buf
            .get(..end)
            .and_then(|slice| core::str::from_utf8(slice).ok())
            .unwrap_or("")
    }

    /// Buffer zero-preenchido a partir de bytes ASCII (uso interno const).
    #[expect(
        clippy::indexing_slicing,
        reason = "índices limitados por `LANG_CAPACITY` no laço; é a construção de uma const"
    )]
    const fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = [0u8; LANG_CAPACITY];
        let mut index = 0;
        while index < bytes.len() && index < LANG_CAPACITY {
            buf[index] = bytes[index];
            index = index.saturating_add(1);
        }
        Self { buf }
    }
}

/// Alvo de `link rel="alternate"`: uma etiqueta BCP-47 ou o curinga `x-default`.
///
/// `x-default` não é etiqueta de idioma ([`Lang`] recusa), mas é o valor que o `hreflang` usa para
/// apontar a versão de fallback (seletor de idioma, página inicial).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hreflang {
    /// Versão no idioma indicado.
    Lang(Lang),
    /// Versão de fallback (`x-default`).
    XDefault,
}

impl Hreflang {
    /// O valor emitido em `hreflang`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Lang(lang) => lang.as_str(),
            Self::XDefault => "x-default",
        }
    }
}

impl From<Lang> for Hreflang {
    fn from(lang: Lang) -> Self {
        Self::Lang(lang)
    }
}

fn classify(part: &str, position: usize) -> Option<Case> {
    match position {
        0 if is_language(part) => Some(Case::Lower),
        0 => None,
        1 if is_script(part) => Some(Case::Title),
        _ if is_region(part) => Some(Case::Upper),
        _ if is_variant(part) => Some(Case::Lower),
        _ => None,
    }
}

fn is_language(part: &str) -> bool {
    (2..=3).contains(&part.len()) && part.chars().all(|ch| ch.is_ascii_alphabetic())
}

fn is_script(part: &str) -> bool {
    part.len() == 4 && part.chars().all(|ch| ch.is_ascii_alphabetic())
}

fn is_region(part: &str) -> bool {
    (part.len() == 2 && part.chars().all(|ch| ch.is_ascii_alphabetic()))
        || (part.len() == 3 && part.chars().all(|ch| ch.is_ascii_digit()))
}

fn is_variant(part: &str) -> bool {
    (5..=8).contains(&part.len()) && part.chars().all(|ch| ch.is_ascii_alphanumeric())
}

fn push_part(buf: &mut [u8; LANG_CAPACITY], cursor: &mut usize, part: &str, case: Case) -> bool {
    for (index, ch) in part.chars().enumerate() {
        let byte = if case.wants_upper(index == 0) {
            ch.to_ascii_uppercase()
        } else {
            ch.to_ascii_lowercase()
        };
        let Ok(raw) = u8::try_from(byte) else {
            return false;
        };
        if !push_byte(buf, cursor, raw) {
            return false;
        }
    }
    true
}

fn push_byte(buf: &mut [u8; LANG_CAPACITY], cursor: &mut usize, byte: u8) -> bool {
    let Some(slot) = buf.get_mut(*cursor) else {
        return false;
    };
    *slot = byte;
    *cursor = cursor.saturating_add(1);
    true
}

/// Direção de escrita (`html[dir]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    /// Esquerda para a direita.
    Ltr,
    /// Direita para a esquerda.
    Rtl,
    /// Decidido pelo primeiro caractere forte.
    Auto,
}

impl Dir {
    /// O valor do atributo `dir`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
            Self::Auto => "auto",
        }
    }
}
