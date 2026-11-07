//! aria ref orphan: o bloco não compila (mensagem em `aria_ref_orphan.stderr`).

use html::html;

fn main() {
    let _ = html! { div aria_labelledby="nope" { } };
}
