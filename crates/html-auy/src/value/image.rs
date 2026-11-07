//! Imagem responsiva: `alt` explícito, dimensões intrínsecas, `srcset` ordenado.

use alloc::vec::Vec;
use core::cmp::Ordering;

/// Texto alternativo: descrição ou declaração explícita de que a imagem é decorativa.
///
/// Não há default. `Alt::new("")` equivale a [`Alt::decorative`] — o vazio só existe quando o
/// autor disse que é decorativa, nunca por esquecimento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alt<'a> {
    /// Imagem decorativa (`alt=""`), ignorada por leitores de tela.
    Decorative,
    /// Descrição objetiva.
    Text(&'a str),
}

impl<'a> Alt<'a> {
    /// Descrição da imagem (vazio = decorativa).
    #[must_use]
    pub const fn new(description: &'a str) -> Self {
        if description.is_empty() {
            Self::Decorative
        } else {
            Self::Text(description)
        }
    }

    /// Imagem decorativa: `alt=""`.
    #[must_use]
    pub const fn decorative() -> Self {
        Self::Decorative
    }

    /// O texto que vai no atributo (`""` quando decorativa).
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        match self {
            Self::Decorative => "",
            Self::Text(text) => text,
        }
    }
}

/// Largura intrínseca em pixels CSS: **nunca zero** (é o que garante `CLS 0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WidthPx(u32);

impl WidthPx {
    /// Largura em pixels; `0` satura para `1` (dimensão zero é ausência de dimensão).
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value.max(1))
    }

    /// O valor emitido.
    #[must_use]
    pub const fn get(&self) -> u32 {
        self.0
    }
}

/// Altura intrínseca em pixels CSS: **nunca zero**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeightPx(u32);

impl HeightPx {
    /// Altura em pixels; `0` satura para `1`.
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value.max(1))
    }

    /// O valor emitido.
    #[must_use]
    pub const fn get(&self) -> u32 {
        self.0
    }
}

/// Variante de imagem do consumidor (o crate não conhece storage nem transformação).
pub trait Variant {
    /// Caminho interno validado ou URL `http(s)` absoluta.
    fn src(&self) -> &str;

    /// Largura intrínseca em pixels.
    fn width(&self) -> u32;
}

/// Conjunto `srcset` ordenado por largura e deduplicado.
///
/// A ordem é a garantia de bytes: a mesma lista de variantes gera a mesma string, e o `ETag` do
/// consumidor é estável por construção.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SrcSet<'a> {
    entries: Vec<(u32, &'a str)>,
}

impl<'a> SrcSet<'a> {
    /// Constrói a partir das variantes: descarta largura zero e `src` inválido, ordena por
    /// `(largura, src)` e mantém uma entrada por largura (empate = menor `src`).
    #[must_use]
    pub fn from_variants<V: Variant>(variants: &'a [V]) -> Self {
        let mut entries: Vec<(u32, &'a str)> = variants
            .iter()
            .filter(|variant| variant.width() > 0)
            .map(|variant| (variant.width(), variant.src()))
            .filter(|(_, src)| valid_src(src))
            .collect();
        entries.sort_unstable_by(|left, right| match left.0.cmp(&right.0) {
            Ordering::Equal => left.1.cmp(right.1),
            other => other,
        });
        entries.dedup_by_key(|(width, _)| *width);
        Self { entries }
    }

    /// `true` quando não há variante aproveitável.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// As entradas `(largura, src)` na ordem canônica.
    #[must_use]
    pub fn entries(&self) -> &[(u32, &'a str)] {
        &self.entries
    }
}

/// Lista `sizes`: gramática validada, inválida vira vazia e o atributo não é emitido.
///
/// Aceita `auto`, `calc(...)`, `min(...)`, `max(...)` e `clamp(...)` como `<source-size-value>`;
/// o `auto` só é útil com `loading="lazy"` — a restrição semântica é do consumidor, não da
/// gramática.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sizes<'a>(&'a str);

impl<'a> Sizes<'a> {
    /// Aceita `input` só se a gramática vale; caso contrário, devolve vazio.
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        if valid_sizes(input) {
            Self(input)
        } else {
            Self("")
        }
    }

    /// `Some` só se a gramática vale.
    #[must_use]
    pub fn try_new(input: &'a str) -> Option<Self> {
        if valid_sizes(input) {
            Some(Self(input))
        } else {
            None
        }
    }

    /// A lista (vazia quando inválida).
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    /// `true` quando não há lista (o atributo não é emitido).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// `src` de variante: a mesma gramática de [`Src::parse`] (caminho interno ou `http(s)`).
///
/// Vírgula crua é separador de `srcset`: fica fora (use `%2C` na URL).
fn valid_src(input: &str) -> bool {
    !input.contains(',') && crate::value::Src::parse(input).is_some()
}

/// `sizes := item ("," item)*`; `item := length | media length`.
fn valid_sizes(input: &str) -> bool {
    if input.trim().is_empty() {
        return false;
    }
    let mut items = 0usize;
    let mut depth = 0i32;
    let mut start = 0usize;
    for (index, ch) in input.char_indices() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth < 0 {
                    return false;
                }
            }
            ',' if depth == 0 => {
                if !valid_item(input.get(start..index).unwrap_or("")) {
                    return false;
                }
                items = items.saturating_add(1);
                start = index.saturating_add(1);
            }
            _ => {}
        }
    }
    if depth != 0 {
        return false;
    }
    if !valid_item(input.get(start..).unwrap_or("")) {
        return false;
    }
    items.saturating_add(1) > 0
}

fn valid_item(item: &str) -> bool {
    let item = item.trim();
    if item.is_empty() {
        return false;
    }
    let Some(after) = item.strip_prefix('(') else {
        return valid_length(item);
    };
    let mut depth = 1i32;
    let mut close = None;
    for (index, ch) in after.char_indices() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    close = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    let Some(close) = close else {
        return false;
    };
    let condition = after.get(..close).unwrap_or("");
    if condition.trim().is_empty() {
        return false;
    }
    let tail = after.get(close.saturating_add(1)..).unwrap_or("").trim();
    !tail.is_empty() && valid_length(tail)
}

fn valid_length(input: &str) -> bool {
    const UNITS: [&str; 16] = [
        "px", "em", "rem", "ex", "ch", "vw", "vh", "vmin", "vmax", "%", "pt", "cm", "mm", "in",
        "pc", "q",
    ];
    let input = input.trim();
    if input.eq_ignore_ascii_case("auto") {
        return true;
    }
    if let Some((name, inner)) = math_inner(input) {
        if inner.trim().is_empty() || !balanced_parens(inner) {
            return false;
        }
        return match name {
            "clamp" => clamp_args(inner),
            _ => true,
        };
    }
    let Some(split) = input.find(|ch: char| !ch.is_ascii_digit() && ch != '.') else {
        return false;
    };
    let (number, unit) = input.split_at(split);
    let digits = number.bytes().filter(u8::is_ascii_digit).count();
    let dots = number.bytes().filter(|byte| *byte == b'.').count();
    UNITS.contains(&unit) && digits > 0 && dots <= 1 && !number.ends_with('.')
}

/// Função matemática exata (`calc`/`min`/`max`/`clamp`): prefixo case-insensitive e um único
/// parêntese externo.
fn math_inner(input: &str) -> Option<(&'static str, &str)> {
    for name in ["calc", "min", "max", "clamp"] {
        let prefix_len = name.len().saturating_add(1);
        let Some(rest) = input.get(..prefix_len) else {
            continue;
        };
        let Some(head) = rest.get(..name.len()) else {
            continue;
        };
        if head.eq_ignore_ascii_case(name) && rest.ends_with('(') {
            return input
                .get(prefix_len..)?
                .strip_suffix(')')
                .map(|inner| (name, inner));
        }
    }
    None
}

/// `clamp(a, b, c)`: exatamente três argumentos não vazios no nível externo.
fn clamp_args(inner: &str) -> bool {
    let mut depth = 0i32;
    let mut start = 0usize;
    let mut args = 0usize;
    for (index, ch) in inner.char_indices() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                if inner.get(start..index).unwrap_or("").trim().is_empty() {
                    return false;
                }
                args = args.saturating_add(1);
                start = index.saturating_add(1);
            }
            _ => {}
        }
    }
    !inner.get(start..).unwrap_or("").trim().is_empty() && args == 2
}

/// Parênteses internos do `calc(...)` casados (não valida a aritmética, só a forma).
fn balanced_parens(input: &str) -> bool {
    let mut depth = 0i32;
    for ch in input.chars() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}
