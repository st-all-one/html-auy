//! syntax text literal: o bloco não compila (mensagem em `syntax_text_literal.stderr`).

use html::html;

fn main() {
    let _ = html! { div { "texto" } };
}
