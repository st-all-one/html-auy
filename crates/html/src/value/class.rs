//! Classes CSS: o enum de tokens é do consumidor, o conjunto é do crate.

use alloc::vec::Vec;

/// Token de classe gerado do vocabulário de design (`tokens.css`) do consumidor.
pub trait ClassToken: Copy {
    /// O nome da classe, sem o ponto.
    fn as_str(self) -> &'static str;
}

/// Conjunto de classes **ordenado e deduplicado**: a mesma entrada dá os mesmos bytes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Class {
    tokens: Vec<&'static str>,
}

impl Class {
    /// Conjunto a partir dos tokens do consumidor (ordena e deduplica).
    #[must_use]
    pub fn of<T: ClassToken>(tokens: &[T]) -> Self {
        let mut list: Vec<&'static str> = tokens.iter().map(|token| (*token).as_str()).collect();
        list.sort_unstable();
        list.dedup();
        Self { tokens: list }
    }

    /// Funde dois conjuntos mantendo a ordem canônica.
    #[must_use]
    pub fn extend(mut self, other: Self) -> Self {
        self.tokens.extend(other.tokens);
        self.tokens.sort_unstable();
        self.tokens.dedup();
        self
    }

    /// `true` se não há token (o atributo `class` então não é emitido).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Os tokens na ordem canônica.
    #[must_use]
    pub fn tokens(&self) -> &[&'static str] {
        &self.tokens
    }
}

/// Tons semânticos do vocabulário de design (cada um é o token `tone-*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tone {
    /// Ação principal.
    Primary,
    /// Ação secundária.
    Secondary,
    /// Neutro.
    Neutral,
    /// Confirmação.
    Success,
    /// Atenção.
    Warning,
    /// Erro ou ação destrutiva.
    Danger,
    /// Informação.
    Info,
}

impl ClassToken for Tone {
    fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "tone-primary",
            Self::Secondary => "tone-secondary",
            Self::Neutral => "tone-neutral",
            Self::Success => "tone-success",
            Self::Warning => "tone-warning",
            Self::Danger => "tone-danger",
            Self::Info => "tone-info",
        }
    }
}
