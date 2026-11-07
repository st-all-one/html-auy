//! Tabela do vocabulário: uma linha por elemento, uma por atributo global.
//!
//! A tabela **não é escrita à mão**: `spec/elements.toml` é normativo e `tools/gen_spec.py` gera
//! `table.rs`. O arquivo gerado é versionado e o crate `html-macros` o inclui por `#[path]`, de modo
//! que validação de tipo (aqui) e validação de bloco (no macro) leem a **mesma** tabela.
//!
//! O teste `spec_parity` compara esta tabela com o TOML campo a campo: esquecer de rodar o gerador
//! deixa o CI vermelho, nunca o produto errado.

mod table;

pub use table::{
    ARIA_VALUES, ATTR_VALUES, AriaValue, AttrValues, BOOLEAN_ATTRIBUTES, ELEMENTS, Element,
    GLOBALS, GlobalValue, INPUT_ATTRS, INVARIANT_SCOPES, NameKind, Only, Scope, ValueKind,
    attr_values, element, input_attrs, invariant_scope,
};
