//! AST do `html!`: o que o parser entrega e o validador percorre.

use proc_macro2::{Span, TokenStream};

/// Sequência de itens de um bloco `{ … }`.
#[derive(Debug, Default)]
pub(crate) struct Block {
    /// Itens na ordem do fonte.
    pub(crate) items: Vec<Item>,
}

/// Um item de bloco.
#[derive(Debug)]
pub(crate) enum Item {
    /// Elemento do vocabulário: `div attr=(…) { … }` ou `img attr=(…);`.
    Element(Element),
    /// `@if` / `@else`.
    If(IfNode),
    /// `for padrao in iteravel { … }`.
    For(ForNode),
    /// Caminho puro: vira `Msg` (texto escapado).
    Message(Message),
    /// Qualquer outra expressão: vira `Render` (filho composto).
    Child(Child),
    /// Primitivo do consumidor com filhos: `Form::open(a, m, c) { … }`.
    Primitive(Primitive),
}

/// Mensagem estática (`Msg::X`).
#[derive(Debug)]
pub(crate) struct Message {
    /// O caminho.
    pub(crate) path: TokenStream,
    /// Span do primeiro token.
    pub(crate) span: Span,
}

/// Filho que implementa `Render`.
#[derive(Debug)]
pub(crate) struct Child {
    /// A expressão.
    pub(crate) expr: TokenStream,
    /// Span do primeiro token.
    pub(crate) span: Span,
}

/// Primitivo que recebe o bloco filho como último argumento.
#[derive(Debug)]
pub(crate) struct Primitive {
    /// A chamada, sem os filhos.
    pub(crate) callee: TokenStream,
    /// Os filhos.
    pub(crate) children: Block,
    /// Span do primeiro token.
    pub(crate) span: Span,
}

/// Um elemento do vocabulário.
#[derive(Debug)]
pub(crate) struct Element {
    /// Nome como escrito no fonte (`svg_title`, `h1`).
    pub(crate) name: String,
    /// Span do nome.
    pub(crate) span: Span,
    /// Atributos na ordem do fonte.
    pub(crate) attrs: Vec<Attr>,
    /// `Some` quando há `{ … }`; `None` quando o elemento é fechado com `;`.
    pub(crate) body: Option<Block>,
}

/// Um atributo escrito no elemento.
#[derive(Debug)]
pub(crate) struct Attr {
    /// Nome como escrito (`data_section`, `viewBox`).
    pub(crate) name: String,
    /// Span do nome.
    pub(crate) span: Span,
    /// Valor.
    pub(crate) value: AttrValue,
    /// `nome?=(expr)`: `Option<T>`, `None` não emite.
    pub(crate) optional: bool,
}

impl Attr {
    /// Nome HTML do atributo: `data_section` → `data-section`; camelCase do SVG (`viewBox`)
    /// passa intacto porque não tem `_`.
    #[must_use]
    pub(crate) fn key(&self) -> String {
        self.name.replace('_', "-")
    }
}

impl Element {
    /// O atributo pelo nome HTML já normalizado.
    #[must_use]
    pub(crate) fn attr(&self, key: &str) -> Option<&Attr> {
        self.attrs.iter().find(|attr| attr.key() == key)
    }

    /// O valor literal de um atributo (só quando escrito como `nome="…"`).
    #[must_use]
    pub(crate) fn literal(&self, key: &str) -> Option<&str> {
        match self.attr(key)?.value {
            AttrValue::Literal { ref text, .. } => Some(text),
            AttrValue::Computed { .. } => None,
        }
    }
}

/// Valor de atributo.
#[derive(Debug)]
pub(crate) enum AttrValue {
    /// `nome=(expr)`.
    Computed {
        /// A expressão entre parênteses.
        expr: TokenStream,
        /// Span da expressão.
        span: Span,
    },
    /// `nome="literal"` (só para atributo enumerado/estático).
    Literal {
        /// O texto do literal, já desescapado.
        text: String,
        /// Span do literal.
        span: Span,
    },
}

/// `@if cond { … } @else { … }`.
#[derive(Debug)]
pub(crate) struct IfNode {
    /// Condição, capturada verbatim.
    pub(crate) condition: TokenStream,
    /// Span do `@if`.
    pub(crate) span: Span,
    /// Corpo do `if`.
    pub(crate) then: Block,
    /// `@else` — bloco ou `@else if`.
    pub(crate) otherwise: Option<Otherwise>,
}

/// Continuação de um `@if`.
#[derive(Debug)]
pub(crate) enum Otherwise {
    /// `@else { … }`.
    Block(Block),
    /// `@else if …`.
    If(Box<IfNode>),
}

/// `for padrao in iteravel { … }`.
#[derive(Debug)]
pub(crate) struct ForNode {
    /// O padrão (verbatim, sem o `in`).
    pub(crate) pattern: TokenStream,
    /// O iterável (verbatim, sem o corpo).
    pub(crate) iterable: TokenStream,
    /// Span do `for`.
    pub(crate) span: Span,
    /// Corpo.
    pub(crate) body: Block,
}
