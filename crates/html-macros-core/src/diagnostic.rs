//! Diagnóstico: cada erro é um `compile_error!` com span, regra e ajuda.

use proc_macro2::{Literal, Span, TokenStream};
use quote::quote_spanned;

/// Um erro de `html!`.
///
/// O texto é o contrato (docs/10 §2): primeira linha curta, depois `= regra:` (a invariante que
/// fundamenta) e `= ajuda:` (o que fazer). Uma mensagem por arquivo em `tests/ui/`.
#[derive(Debug)]
pub(crate) struct Diag {
    /// Onde apontar.
    pub(crate) span: Span,
    /// Primeira linha.
    pub(crate) message: String,
    /// Invariante de `spec/elements.toml` (docs/04 §4.1).
    pub(crate) rule: Option<String>,
    /// Como corrigir.
    pub(crate) hint: Option<String>,
}

impl Diag {
    /// Erro sem regra nem ajuda.
    #[must_use]
    pub(crate) fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            rule: None,
            hint: None,
        }
    }

    /// Erro com a invariante que o fundamenta.
    #[must_use]
    pub(crate) fn rule(mut self, rule: impl Into<String>) -> Self {
        self.rule = Some(rule.into());
        self
    }

    /// Erro com a instrução de correção.
    #[must_use]
    pub(crate) fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// O texto que o `compile_error!` vai imprimir.
    #[must_use]
    pub(crate) fn render(&self) -> String {
        let mut text = self.message.clone();
        if let Some(rule) = &self.rule {
            text.push_str("\n  = regra: ");
            text.push_str(rule);
        }
        if let Some(hint) = &self.hint {
            text.push_str("\n  = ajuda: ");
            text.push_str(hint);
        }
        text
    }

    /// O `compile_error!` correspondente.
    fn to_tokens(&self) -> TokenStream {
        let literal = Literal::string(&self.render());
        let span = self.span;
        quote_spanned! { span => ::core::compile_error!(#literal); }
    }
}

/// Emite todos os diagnósticos de uma vez (o autor corrige em lote, não um por compilação).
///
/// O bloco termina em `unreachable!()` para que a expansão ainda tenha tipo `Markup` e não gere
/// erro em cascata: o `compile_error!` já garante que o código não compila.
#[must_use]
pub(crate) fn emit(diags: &[Diag]) -> TokenStream {
    let errors = diags.iter().map(Diag::to_tokens);
    quote_spanned! { diags.first().map_or_else(Span::call_site, |diag| diag.span) =>
        {
            #(#errors)*
            ::core::unreachable!()
        }
    }
}
