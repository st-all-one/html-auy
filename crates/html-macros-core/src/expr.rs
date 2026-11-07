//! Scanner de expressão: sem `syn`, o macro não interpreta Rust — ele copia tokens.
//!
//! A regra é uma só: `&`/`&mut` opcional, uma base (`Ident`, literal, grupo `(…)`, grupo `[…]`) e a
//! cadeia de pós-fixos (`::ident`, `::<…>`, `.campo`, grupo imediatamente seguinte, `?`). Tudo o que
//! não for capturável assim precisa vir entre parênteses — e é isso que torna `href=x rel=y`
//! impossível de escrever.

use crate::parser::Cursor;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};

/// O que o scanner consumiu.
pub(crate) struct Scanned {
    /// Os tokens da expressão.
    pub(crate) tokens: TokenStream,
    /// Span do primeiro token.
    pub(crate) span: Span,
    /// `true` quando é só um caminho (`Msg::X`): vira mensagem, não `Render`.
    pub(crate) pure_path: bool,
}

/// Consome uma expressão do cursor.
pub(crate) fn scan(cursor: &mut Cursor) -> Option<Scanned> {
    let start_span = cursor.peek()?.span();
    let mut out: Vec<TokenTree> = Vec::new();
    let mut pure_path = true;

    if cursor.check_punct('&') {
        out.push(cursor.bump()?);
        pure_path = false;
        if cursor.check_ident("mut") {
            out.push(cursor.bump()?);
        }
    }

    let base = cursor.peek()?;
    let base_ok = match base {
        TokenTree::Ident(_) | TokenTree::Literal(_) => true,
        TokenTree::Group(group) => {
            matches!(
                group.delimiter(),
                Delimiter::Parenthesis | Delimiter::Bracket
            )
        }
        TokenTree::Punct(_) => false,
    };
    if !base_ok {
        return None;
    }
    if !matches!(base, TokenTree::Ident(_)) {
        pure_path = false;
    }
    out.push(cursor.bump()?);

    loop {
        if cursor.check_punct_pair(':', ':') {
            out.push(cursor.bump()?);
            out.push(cursor.bump()?);
            if cursor.check_punct('<') {
                consume_angles(cursor, &mut out)?;
                pure_path = false;
                continue;
            }
            if matches!(cursor.peek(), Some(TokenTree::Ident(_))) {
                out.push(cursor.bump()?);
                continue;
            }
            break;
        }

        if cursor.check_punct('.') {
            out.push(cursor.bump()?);
            pure_path = false;
            if matches!(
                cursor.peek(),
                Some(TokenTree::Ident(_) | TokenTree::Literal(_))
            ) {
                out.push(cursor.bump()?);
            }
            continue;
        }

        match cursor.peek() {
            Some(TokenTree::Group(group))
                if matches!(
                    group.delimiter(),
                    Delimiter::Parenthesis | Delimiter::Bracket
                ) =>
            {
                out.push(cursor.bump()?);
                pure_path = false;
                continue;
            }
            Some(TokenTree::Group(_)) => break,
            _ => {}
        }

        if cursor.check_punct('?') {
            out.push(cursor.bump()?);
            pure_path = false;
            continue;
        }

        break;
    }

    Some(Scanned {
        tokens: out.into_iter().collect(),
        span: start_span,
        pure_path,
    })
}

/// Consome `<…>` de um turbofish, contando profundidade (aceita `>>` fechando dois níveis).
fn consume_angles(cursor: &mut Cursor, out: &mut Vec<TokenTree>) -> Option<()> {
    let mut depth = 0i32;
    loop {
        let token = cursor.bump()?;
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == '<' => {
                depth = depth.saturating_add(1);
            }
            TokenTree::Punct(punct) if punct.as_char() == '>' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
        out.push(token);
        if depth <= 0 {
            return Some(());
        }
    }
}
