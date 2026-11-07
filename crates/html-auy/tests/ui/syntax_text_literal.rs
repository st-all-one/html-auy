//! syntax text literal: o bloco não compila (mensagem em `syntax_text_literal.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div { "texto" } };
}
