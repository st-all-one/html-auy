//! bool literal: o bloco não compila (mensagem em `bool_literal.stderr`).

use html::html;

fn main() {
    let _ = html! { button type="button" disabled="false" { m } };
}
