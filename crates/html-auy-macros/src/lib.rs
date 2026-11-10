//! Proc-macros `html!` e `Msg`: a superfície pública do núcleo [`html_auy_macros_core`].

use proc_macro::TokenStream;

/// Constrói um [`Markup`] a partir de um bloco de marcação validado.
///
/// [`Markup`]: https://docs.rs/html-auy
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    html_auy_macros_core::expand(input.into()).into()
}

/// Deriva [`Msg`] para um `enum` de variantes unitárias anotadas com `#[msg("…")]`.
///
/// [`Msg`]: https://docs.rs/html-auy/latest/html_auy/trait.Msg.html
#[proc_macro_derive(Msg, attributes(msg))]
pub fn derive_msg(input: TokenStream) -> TokenStream {
    html_auy_macros_core::expand_msg_derive(input.into()).into()
}
