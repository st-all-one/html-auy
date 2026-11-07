//! dup attribute: o bloco não compila (mensagem em `dup_attribute.stderr`).

use html::html;

fn main() {
    let _ = html! { div id="a" id="b" { } };
}
