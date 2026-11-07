#![expect(
    clippy::redundant_pub_crate,
    reason = "constante de teste compartilhada entre dois binários de teste"
)]

//! Bytes esperados do `<main>` de aceitação (M1 e M2 comparam com a mesma constante).

/// Lista de notícias: heading + dois itens, sem indentação.
pub(crate) const MAIN: &str = "<h2>Notícias</h2><ul><li>Primeira</li><li>Segunda</li></ul>";
