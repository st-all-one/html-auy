//! Proc-macro `html!`: a superfície pública do núcleo [`html_macros_core`].

use proc_macro::TokenStream;

/// Constrói um [`Markup`] a partir de um bloco de marcação validado.
///
/// [`Markup`]: https://docs.rs/html
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    html_macros_core::expand(input.into()).into()
}
