//! unknown attribute: o bloco não compila (mensagem em `unknown_attribute.stderr`).

use html::html;

fn main() {
    let _ = html! { p hreff="x" { } };
}
