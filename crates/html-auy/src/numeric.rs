//! Escrita de inteiros sem `core::fmt`.
//!
//! `write!` devolve `Result` (que seria ignorado) e passa por formatação dinâmica; aqui os dígitos
//! vão para um buffer fixo e daí direto para a saída.

use alloc::string::String;

/// Escreve `value` em decimal.
#[expect(
    clippy::redundant_pub_crate,
    reason = "`pub(crate)` é a visibilidade real: `pub` aqui viraria `unreachable_pub`"
)]
pub(crate) fn write_u64(out: &mut String, value: u64) {
    let mut digits = [0u8; 20];
    let mut cursor = digits.len();
    let mut rest = value;
    loop {
        cursor = cursor.saturating_sub(1);
        let digit = u8::try_from(rest % 10).unwrap_or(0);
        if let Some(slot) = digits.get_mut(cursor) {
            *slot = b'0'.saturating_add(digit);
        }
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    if let Some(tail) = digits.get(cursor..) {
        out.push_str(core::str::from_utf8(tail).unwrap_or(""));
    }
}

/// Escreve `value` em decimal, com sinal quando negativo.
#[expect(
    clippy::redundant_pub_crate,
    reason = "`pub(crate)` é a visibilidade real: `pub` aqui viraria `unreachable_pub`"
)]
pub(crate) fn write_i64(out: &mut String, value: i64) {
    if value < 0 {
        out.push('-');
    }
    write_u64(out, value.unsigned_abs());
}
