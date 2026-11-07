//! Consulta de mídia: `link media` e preload condicionado.

/// Consulta de mídia validada (gramática CSS completa fica com o navegador).
///
/// A validação é estrutural: não vazia, sem caracteres de controle e com parênteses casados.
/// `print` e `screen` são os casos que o shell usa; o restante entra por [`Media::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Media<'a>(&'a str);

impl<'a> Media<'a> {
    /// `media="print"` — não bloqueia a renderização de tela.
    #[must_use]
    pub const fn print() -> Media<'static> {
        Media("print")
    }

    /// `media="screen"`.
    #[must_use]
    pub const fn screen() -> Media<'static> {
        Media("screen")
    }

    /// Consulta arbitrária; inválida vira vazia e o atributo não é emitido.
    #[must_use]
    pub fn new(query: &'a str) -> Self {
        let query = query.trim();
        if valid_media(query) {
            Self(query)
        } else {
            Self("")
        }
    }

    /// `Some` só se a consulta é estruturalmente válida.
    #[must_use]
    pub fn try_new(query: &'a str) -> Option<Self> {
        let query = query.trim();
        valid_media(query).then_some(Self(query))
    }

    /// A consulta como veio.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    /// `true` quando não há consulta (o atributo não é emitido).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn valid_media(input: &str) -> bool {
    let input = input.trim();
    if input.is_empty() {
        return false;
    }
    let mut depth = 0u32;
    for ch in input.chars() {
        if ch.is_control() {
            return false;
        }
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                if depth == 0 {
                    return false;
                }
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    depth == 0
}
