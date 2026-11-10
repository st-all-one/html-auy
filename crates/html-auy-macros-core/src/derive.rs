//! Derive `Msg`: o catálogo fica explícito no `enum`, a resolução continua no consumidor.

use crate::parser::literal_text;
use proc_macro2::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};

type Error = (Span, String);

struct Variant {
    name: Ident,
    text: String,
}

struct Cursor {
    tokens: Vec<TokenTree>,
    index: usize,
}

impl Cursor {
    const fn new(tokens: Vec<TokenTree>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&TokenTree> {
        self.tokens.get(self.index)
    }

    fn bump(&mut self) -> Option<TokenTree> {
        let token = self.tokens.get(self.index).cloned()?;
        self.index = self.index.saturating_add(1);
        Some(token)
    }

    fn check_punct(&self, want: char) -> bool {
        matches!(self.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == want)
    }
}

/// Expande `#[derive(Msg)]`; entrada inválida vira `compile_error!`, nunca pânico.
#[must_use]
pub(crate) fn derive(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok((name, variants)) => expand(&name, &variants),
        Err((span, message)) => compile_error(span, &message),
    }
}

fn parse(input: TokenStream) -> Result<(Ident, Vec<Variant>), Error> {
    let mut cursor = Cursor::new(input.into_iter().collect());
    attributes(&mut cursor)?;
    if matches!(cursor.peek(), Some(TokenTree::Ident(ident)) if ident == "pub") {
        cursor.bump();
        if matches!(cursor.peek(), Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis)
        {
            cursor.bump();
        }
    }

    let Some(TokenTree::Ident(kind)) = cursor.bump() else {
        return Err((Span::call_site(), String::from("`Msg` exige um `enum`")));
    };
    if kind != "enum" {
        return Err((
            kind.span(),
            String::from("`Msg` só pode ser derivado em `enum`"),
        ));
    }

    let Some(TokenTree::Ident(name)) = cursor.bump() else {
        return Err((kind.span(), String::from("`Msg` exige um nome de enum")));
    };
    if cursor.peek().is_some_and(|token| {
        matches!(token, TokenTree::Punct(punct) if punct.as_char() == '<')
            || matches!(token, TokenTree::Ident(ident) if ident == "where")
    }) {
        return Err((
            name.span(),
            String::from("`Msg` não aceita generics nem `where`"),
        ));
    }

    let Some(TokenTree::Group(body)) = cursor.bump() else {
        return Err((name.span(), String::from("`Msg` exige `{ … }`")));
    };
    if body.delimiter() != Delimiter::Brace {
        return Err((body.span(), String::from("`Msg` exige `{ … }`")));
    }

    let variants = variants(body.stream())?;
    if variants.is_empty() {
        return Err((
            name.span(),
            String::from("`Msg` exige ao menos uma variante"),
        ));
    }
    Ok((name, variants))
}

fn variants(tokens: TokenStream) -> Result<Vec<Variant>, Error> {
    let mut cursor = Cursor::new(tokens.into_iter().collect());
    let mut out = Vec::new();

    while cursor.peek().is_some() {
        if cursor.check_punct(',') {
            cursor.bump();
            continue;
        }
        let attrs = attributes(&mut cursor)?;
        let Some(TokenTree::Ident(name)) = cursor.bump() else {
            return Err((cursor_span(&cursor), String::from("variante esperada")));
        };
        if cursor.peek().is_some_and(|token| {
            matches!(
                token,
                TokenTree::Group(group)
                    if matches!(group.delimiter(), Delimiter::Parenthesis | Delimiter::Brace)
            )
        }) {
            return Err((
                name.span(),
                String::from("`Msg` exige variantes sem campos"),
            ));
        }

        let mut text = None;
        for attr in &attrs {
            if let Some(value) = msg_text(attr)?
                && text.replace(value).is_some()
            {
                return Err((attr.span(), String::from("`#[msg]` repetido")));
            }
        }
        let Some(text) = text else {
            return Err((name.span(), String::from("variante sem `#[msg(\"…\")]`")));
        };
        out.push(Variant { name, text });

        if cursor.check_punct(',') {
            cursor.bump();
        } else if cursor.peek().is_some() {
            return Err((
                cursor_span(&cursor),
                String::from("separe as variantes com `,`"),
            ));
        }
    }
    Ok(out)
}

/// Consome `#[…]` e devolve os grupos de atributo.
fn attributes(cursor: &mut Cursor) -> Result<Vec<Group>, Error> {
    let mut out = Vec::new();
    while cursor.check_punct('#') {
        cursor.bump();
        let Some(TokenTree::Group(group)) = cursor.bump() else {
            return Err((Span::call_site(), String::from("atributo malformado")));
        };
        if group.delimiter() != Delimiter::Bracket {
            return Err((group.span(), String::from("atributo malformado")));
        }
        out.push(group);
    }
    Ok(out)
}

/// Extrai o texto de `#[msg("…")]`; atributos de outra natureza são ignorados.
fn msg_text(group: &Group) -> Result<Option<String>, Error> {
    let mut tokens = group.stream().into_iter();
    let Some(TokenTree::Ident(name)) = tokens.next() else {
        return Ok(None);
    };
    if name != "msg" {
        return Ok(None);
    }
    let malformed = || {
        (
            group.span(),
            String::from("use `#[msg(\"texto\")]` numa string literal"),
        )
    };
    let Some(literal) = tokens.next() else {
        return Err(malformed());
    };
    let literal = match literal {
        TokenTree::Group(args) if args.delimiter() == Delimiter::Parenthesis => {
            let mut args = args.stream().into_iter();
            let Some(TokenTree::Literal(literal)) = args.next() else {
                return Err(malformed());
            };
            if args.next().is_some() {
                return Err(malformed());
            }
            literal
        }
        TokenTree::Punct(punct) if punct.as_char() == '=' => match tokens.next() {
            Some(TokenTree::Literal(literal)) => literal,
            _ => return Err(malformed()),
        },
        _ => return Err(malformed()),
    };
    if tokens.next().is_some() {
        return Err(malformed());
    }
    literal_text(&literal).map(Some).ok_or_else(malformed)
}

fn cursor_span(cursor: &Cursor) -> Span {
    cursor.peek().map_or_else(Span::call_site, TokenTree::span)
}

fn expand(name: &Ident, variants: &[Variant]) -> TokenStream {
    let arms = variants.iter().map(|variant| {
        let name = &variant.name;
        let text = Literal::string(&variant.text);
        quote! { Self::#name => #text, }
    });
    quote! {
        impl ::html_auy::Msg for #name {
            fn as_str(self) -> &'static str {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

fn compile_error(span: Span, message: &str) -> TokenStream {
    let message = Literal::string(message);
    quote_spanned! {span=> ::core::compile_error!(#message); }
}
