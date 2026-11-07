//! Proc-macro `html!`: a superfície pública do núcleo [`html_auy_macros_core`].

use proc_macro::TokenStream;

/// Constrói um [`Markup`] a partir de um bloco de marcação validado.
///
/// [`Markup`]: https://docs.rs/html-auy
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    html_auy_macros_core::expand(input.into()).into()
}
