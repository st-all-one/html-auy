//! Contrato de resposta parcial: multi-alvo, seleção e política de concorrência.
//!
//! O crate não faz I/O nem escolhe o transporte. [`Partial`] é o modelo tipado que o consumidor
//! serializa no seu mecanismo de swap (OOB, `<hx-partial>` ou um adaptador próprio). Nenhum
//! `hx-*` é emitido aqui: os tipos existem para que a decisão de "o que trocar, onde e em que
//! ordem" seja dado, não string de atributo.

use crate::markup::Markup;
use crate::value::Id;
use alloc::vec::Vec;

/// Estratégia de troca de uma região, equivalente tipado de `hx-swap`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Swap {
    /// Substitui o conteúdo interno do alvo.
    Inner,
    /// Substitui o próprio elemento alvo.
    Outer,
    /// Insere antes do alvo.
    BeforeBegin,
    /// Insere como primeiro filho do alvo.
    AfterBegin,
    /// Insere como último filho do alvo.
    BeforeEnd,
    /// Insere depois do alvo.
    AfterEnd,
    /// Remove o alvo, independentemente do corpo.
    Delete,
    /// Não troca o alvo; atualizações OOB continuam valendo.
    None,
}

impl Swap {
    /// O nome estável do contrato (não é um atributo `hx-*`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inner => "inner",
            Self::Outer => "outer",
            Self::BeforeBegin => "before-begin",
            Self::AfterBegin => "after-begin",
            Self::BeforeEnd => "before-end",
            Self::AfterEnd => "after-end",
            Self::Delete => "delete",
            Self::None => "none",
        }
    }
}

/// O que acontece com o alvo principal da requisição.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainSwap {
    /// O corpo principal substitui o alvo.
    Replace(Markup),
    /// A resposta só atualiza regiões; o alvo principal fica intacto.
    Keep,
}

impl MainSwap {
    /// O markup principal, quando há substituição.
    #[must_use]
    pub const fn markup(&self) -> Option<&Markup> {
        match self {
            Self::Replace(markup) => Some(markup),
            Self::Keep => None,
        }
    }

    /// `true` quando o alvo principal é preservado.
    #[must_use]
    pub const fn keeps(&self) -> bool {
        matches!(self, Self::Keep)
    }
}

/// Atualização de uma região secundária.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionUpdate<'a> {
    target: Id<'a>,
    swap: Swap,
    markup: Markup,
}

impl<'a> RegionUpdate<'a> {
    /// Cria uma atualização para `target`.
    #[must_use]
    pub fn new(target: Id<'a>, swap: Swap, markup: impl Into<Markup>) -> Self {
        Self {
            target,
            swap,
            markup: markup.into(),
        }
    }

    /// O `id` da região.
    #[must_use]
    pub const fn target(&self) -> &Id<'a> {
        &self.target
    }

    /// A estratégia de troca.
    #[must_use]
    pub const fn swap(&self) -> Swap {
        self.swap
    }

    /// O conteúdo da região.
    #[must_use]
    pub const fn markup(&self) -> &Markup {
        &self.markup
    }
}

/// Resposta parcial composta pelo servidor.
///
/// A ordem de [`Partial::updates`] é a ordem de aplicação; o consumidor a preserva. O `focus`
/// aponta para o título/controle que deve receber foco depois do swap, nunca para o gatilho.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partial<'a> {
    main: MainSwap,
    updates: Vec<RegionUpdate<'a>>,
    focus: Option<Id<'a>>,
    select: Option<Id<'a>>,
    reselect: Option<Id<'a>>,
    select_oob: Vec<Id<'a>>,
}

impl<'a> Partial<'a> {
    /// Resposta com um corpo principal substitutivo.
    #[must_use]
    pub fn new(main: impl Into<Markup>) -> Self {
        Self {
            main: MainSwap::Replace(main.into()),
            updates: Vec::new(),
            focus: None,
            select: None,
            reselect: None,
            select_oob: Vec::new(),
        }
    }

    /// Resposta que não toca o alvo principal.
    #[must_use]
    pub const fn keep_main() -> Self {
        Self {
            main: MainSwap::Keep,
            updates: Vec::new(),
            focus: None,
            select: None,
            reselect: None,
            select_oob: Vec::new(),
        }
    }

    /// Acrescenta uma atualização de região, preservando a ordem de chamada.
    #[must_use]
    pub fn update(mut self, target: Id<'a>, swap: Swap, markup: impl Into<Markup>) -> Self {
        self.updates.push(RegionUpdate::new(target, swap, markup));
        self
    }

    /// Define o foco pós-swap (normalmente o título da região).
    #[must_use]
    pub fn focus(mut self, target: Id<'a>) -> Self {
        self.focus = Some(target);
        self
    }

    /// Define o alvo de seleção do corpo (`hx-select`/`HX-Reselect`).
    #[must_use]
    pub fn select(mut self, target: Id<'a>) -> Self {
        self.select = Some(target);
        self
    }

    /// Sobrescreve a seleção no servidor (`HX-Reselect`).
    #[must_use]
    pub fn reselect(mut self, target: Id<'a>) -> Self {
        self.reselect = Some(target);
        self
    }

    /// Define as regiões selecionadas do corpo para atualização OOB (`hx-select-oob`).
    #[must_use]
    pub fn select_oob(mut self, targets: Vec<Id<'a>>) -> Self {
        self.select_oob = targets;
        self
    }

    /// O que acontece com o alvo principal.
    #[must_use]
    pub const fn main(&self) -> &MainSwap {
        &self.main
    }

    /// Atualizações secundárias, na ordem de aplicação.
    #[must_use]
    pub fn updates(&self) -> &[RegionUpdate<'a>] {
        &self.updates
    }

    /// Alvo de foco pós-swap.
    #[must_use]
    pub const fn focus_target(&self) -> Option<&Id<'a>> {
        self.focus.as_ref()
    }

    /// Alvo de seleção do corpo.
    #[must_use]
    pub const fn select_target(&self) -> Option<&Id<'a>> {
        self.select.as_ref()
    }

    /// Alvo de reseleção do servidor.
    #[must_use]
    pub const fn reselect_target(&self) -> Option<&Id<'a>> {
        self.reselect.as_ref()
    }

    /// Regiões selecionadas para OOB.
    #[must_use]
    pub fn select_oob_targets(&self) -> &[Id<'a>] {
        &self.select_oob
    }
}

impl Default for Partial<'_> {
    fn default() -> Self {
        Self::keep_main()
    }
}

/// Política de concorrência do cliente para a mesma região.
///
/// `Queue` não existe de propósito: fila pode aplicar uma resposta antiga depois de uma nova.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sync {
    /// Ignora o novo pedido enquanto houver um em voo.
    Drop,
    /// Aborta o pedido em voo e começa o novo.
    Replace,
    /// Ignora o novo pedido se já houver um em voo; caso contrário, aborta este quando outro chegar.
    Abort,
}

impl Sync {
    /// O nome estável do contrato.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Drop => "drop",
            Self::Replace => "replace",
            Self::Abort => "abort",
        }
    }
}

/// Classe de requisição que muda a resposta e, portanto, o header `Vary`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestClass {
    /// Navegação completa.
    Full,
    /// Fragmento básico.
    Partial,
    /// Fragmento que também varia pelo alvo.
    Targeted,
    /// Restauração de histórico.
    HistoryRestore,
    /// Navegação potencializada (`hx-boost`).
    Boosted,
}

impl RequestClass {
    /// O valor de `Vary` que o handler deve emitir.
    #[must_use]
    pub const fn vary(self) -> &'static str {
        match self {
            Self::Full => "",
            Self::Partial => "HX-Request",
            Self::Targeted => "HX-Request, HX-Target",
            Self::HistoryRestore => "HX-Request, HX-History-Restore-Request",
            Self::Boosted => "HX-Request, HX-Boosted",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MainSwap, Partial, RequestClass, Swap, Sync};
    use crate::Text;
    use crate::markup::Render;
    use crate::value::Id;
    use alloc::vec;

    #[test]
    fn swap_names_are_stable() {
        assert_eq!(Swap::Inner.as_str(), "inner");
        assert_eq!(Swap::Outer.as_str(), "outer");
        assert_eq!(Swap::BeforeEnd.as_str(), "before-end");
        assert_eq!(Swap::Delete.as_str(), "delete");
        assert_eq!(Swap::None.as_str(), "none");
    }

    #[test]
    fn partial_keeps_update_order_and_main_choice() {
        let main = Text::new("principal").render();
        let first = Id::new("a");
        let second = Id::new("b");
        let partial = Partial::new(main)
            .update(first.clone(), Swap::Inner, Text::new("A").render())
            .update(second.clone(), Swap::Outer, Text::new("B").render())
            .focus(second);
        assert_eq!(
            partial.main().markup(),
            Some(&Text::new("principal").render())
        );
        assert_eq!(partial.updates().len(), 2);
        assert_eq!(partial.updates()[0].target().as_str(), "a");
        assert_eq!(partial.updates()[1].target().as_str(), "b");
        assert_eq!(partial.focus_target().map(Id::as_str), Some("b"));
        assert!(!partial.main().keeps());
    }

    #[test]
    fn keep_main_and_selections_are_explicit() {
        let target = Id::new("cart");
        let partial = Partial::keep_main()
            .select(target.clone())
            .reselect(Id::new("content"))
            .select_oob(vec![Id::new("count"), Id::new("total")]);
        assert!(matches!(partial.main(), MainSwap::Keep));
        assert_eq!(partial.select_target().map(Id::as_str), Some("cart"));
        assert_eq!(partial.reselect_target().map(Id::as_str), Some("content"));
        assert_eq!(partial.select_oob_targets().len(), 2);
    }

    #[test]
    fn sync_and_request_class_are_fixed() {
        assert_eq!(Sync::Drop.as_str(), "drop");
        assert_eq!(Sync::Replace.as_str(), "replace");
        assert_eq!(Sync::Abort.as_str(), "abort");
        assert_eq!(RequestClass::Full.vary(), "");
        assert_eq!(RequestClass::Partial.vary(), "HX-Request");
        assert_eq!(RequestClass::Targeted.vary(), "HX-Request, HX-Target");
        assert_eq!(
            RequestClass::HistoryRestore.vary(),
            "HX-Request, HX-History-Restore-Request"
        );
        assert_eq!(RequestClass::Boosted.vary(), "HX-Request, HX-Boosted");
    }

    #[test]
    fn default_is_keep_main() {
        let partial = Partial::default();
        assert!(matches!(partial.main(), MainSwap::Keep));
        assert_eq!(partial.updates().len(), 0);
    }
}
