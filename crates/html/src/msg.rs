//! Mensagens e texto: o crate não conhece locale (D31).

use crate::markup::Render;
use alloc::string::String;

/// Texto vindo do consumidor: o `enum` (ou `&'static str`) é dele, a resolução é dele.
///
/// Implementar este trait é o que permite escrever `Msg::Titulo` dentro de `html!` sem que o crate
/// saiba onde está o catálogo de traduções.
pub trait Msg: Copy {
    /// O texto já resolvido no locale da requisição.
    fn as_str(self) -> &'static str;
}

/// Texto destinado a conteúdo: sempre escapado na escrita.
///
/// Dois construtores, dois mundos: [`Text::msg`] para mensagem estática (o compilador exige um
/// [`Msg`]) e [`Text::new`] para dado dinâmico (escapado na saída). Não há terceiro caminho.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text<'a>(&'a str);

impl<'a> Text<'a> {
    /// Texto dinâmico (título vindo do banco, por exemplo).
    #[must_use]
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }

    /// Texto de mensagem: `M: Msg`.
    #[must_use]
    pub fn msg<M: Msg>(message: M) -> Text<'static> {
        Text(message.as_str())
    }

    /// O texto cru (ainda **não** escapado).
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

impl Render for Text<'_> {
    fn render_to(&self, out: &mut String) {
        crate::escape::text(out, self.0);
    }
}
