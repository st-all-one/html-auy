//! O parser do macro nunca pode entrar em pânico: qualquer token vira `compile_error!`.
//!
//! O alvo alimenta `expand` (parser + validação + codegen) — a única superfície que roda em
//! tempo de compilação do consumidor e não pode abortar o `rustc` com um ICE de biblioteca.

#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str::FromStr;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(tokens) = proc_macro2::TokenStream::from_str(text) else {
        return;
    };
    let _ = html_macros_core::expand(tokens);
});
