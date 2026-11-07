//! Casos de erro do `html!`: cada arquivo em `ui/` fixa uma mensagem de contrato (docs/10 §2).

#[test]
fn ui() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}
