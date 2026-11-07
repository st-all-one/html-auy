//! Integridade de subrecurso (SRI): `sha256-…`, `sha384-…`, `sha512-…`.

/// Metadados SRI validados.
///
/// O valor é uma lista separada por espaços de tokens `algoritmo-base64`; só `sha256`, `sha384` e
/// `sha512` são aceitos. A digest é validada como base64 conservador (`A-Za-z0-9+/=`); a
/// verificação criptográfica é do navegador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integrity<'a>(&'a str);

impl<'a> Integrity<'a> {
    /// `Some` só se todos os tokens são SRI válidos.
    #[must_use]
    pub fn parse(input: &'a str) -> Option<Self> {
        let mut any = false;
        for token in input.split_ascii_whitespace() {
            any = true;
            let (algorithm, digest) = token.split_once('-')?;
            if !matches!(algorithm, "sha256" | "sha384" | "sha512") {
                return None;
            }
            if digest.is_empty()
                || !digest
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '+' | '/' | '='))
            {
                return None;
            }
        }
        any.then_some(Self(input))
    }

    /// O valor como veio.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}
