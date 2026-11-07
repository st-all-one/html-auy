//! Núcleo do `html!`: parser, validação e codegen, sem tocar em `proc_macro`.
//!
//! Vive fora do crate proc-macro para ser testável e fuzzável como biblioteca comum: `cargo
//! fuzz` alimenta tokens, não expande macros. O fluxo é HTML, não Rust — `proc-macro2` e `quote`
//! bastam, e `syn` nunca entra.

#![forbid(unsafe_code)]
#![expect(
    clippy::redundant_pub_crate,
    reason = "crate interna: os módulos são privados e `pub(crate)` é a maior visibilidade possível"
)]

mod table;

mod ast;
mod attrs;
mod codegen;
mod diagnostic;
mod expr;
mod parser;
mod rules;
mod validate;

#[cfg(test)]
mod tests;

/// Expande um bloco `html!` em tokens; entrada inválida vira `compile_error!`, nunca pânico.
#[must_use]
pub fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let block = match parser::parse(input) {
        Ok(block) => block,
        Err(diags) => return diagnostic::emit(&diags),
    };
    let diags = validate::check(&block);
    if diags.is_empty() {
        codegen::expand(&block)
    } else {
        diagnostic::emit(&diags)
    }
}
