//! Token de `autocomplete` para campos de formulário.
//!
//! Além dos tokens simples, aceita o par `username webauthn` (passkey) e `webauthn`.

/// Valor de `autocomplete` validado contra o vocabulário HTML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Autocomplete<'a>(&'a str);

const VALUES: &[&str] = &[
    "off",
    "on",
    "name",
    "given-name",
    "additional-name",
    "family-name",
    "username",
    "username webauthn",
    "webauthn",
    "email",
    "current-password",
    "new-password",
    "one-time-code",
    "street-address",
    "address-line1",
    "address-line2",
    "address-level1",
    "address-level2",
    "postal-code",
    "country",
    "country-name",
    "tel",
    "tel-national",
    "tel-area-code",
    "tel-local",
    "bday",
    "bday-day",
    "bday-month",
    "bday-year",
    "sex",
    "url",
    "photo",
    "organization",
    "organization-title",
    "cc-name",
    "cc-number",
    "cc-exp",
    "cc-exp-month",
    "cc-exp-year",
    "cc-csc",
    "transaction-currency",
    "transaction-amount",
    "language",
    "impp",
    "shipping",
    "billing",
];

impl<'a> Autocomplete<'a> {
    /// `Some` só para um token do vocabulário.
    #[must_use]
    pub fn parse(value: &'a str) -> Option<Self> {
        VALUES.contains(&value).then_some(Self(value))
    }

    /// O token como veio.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}
