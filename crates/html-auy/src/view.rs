//! A view: o conteúdo do `<main>` mais os modais e os assets que ele exige.

use crate::markup::Markup;
use crate::shell::{Asset, Assets};
use alloc::vec::Vec;

/// Conteúdo de uma página: `main`, modais (na ordem declarada) e assets.
///
/// `View::merge` compõe duas views somando modais e deduplicando assets; o `main` de `self` vence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    main: Markup,
    modals: Vec<Markup>,
    assets: Assets,
}

impl Default for View {
    fn default() -> Self {
        Self {
            main: Markup(alloc::string::String::new()),
            modals: Vec::new(),
            assets: Assets::new(),
        }
    }
}

impl View {
    /// View com um `main`.
    #[must_use]
    pub fn new(main: impl Into<Markup>) -> Self {
        Self {
            main: main.into(),
            modals: Vec::new(),
            assets: Assets::new(),
        }
    }

    /// Acrescenta um modal, que o `Page` emite depois do rodapé.
    #[must_use]
    pub fn modal(mut self, modal: impl Into<Markup>) -> Self {
        self.modals.push(modal.into());
        self
    }

    /// Declara um asset exigido pela view.
    #[must_use]
    pub fn asset(mut self, asset: Asset) -> Self {
        self.assets.insert(asset);
        self
    }

    /// Funde `other` nesta view: soma modais, deduplica assets e mantém o `main` de `self`.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.modals.extend(other.modals);
        self.assets.extend(other.assets);
        self
    }

    /// O conteúdo do `<main>`.
    #[must_use]
    pub const fn main(&self) -> &Markup {
        &self.main
    }

    /// Os modais, na ordem declarada.
    #[must_use]
    pub fn modals(&self) -> &[Markup] {
        &self.modals
    }

    /// Os assets exigidos pela view.
    #[must_use]
    pub const fn assets(&self) -> &Assets {
        &self.assets
    }
}
