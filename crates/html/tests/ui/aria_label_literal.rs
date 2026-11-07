//! aria label literal: o bloco não compila (mensagem em `aria_label_literal.stderr`).

use html::html;

fn main() {
    let _ = html! { div aria_label="texto" { } };
}
