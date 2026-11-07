//! aria ref orphan: o bloco não compila (mensagem em `aria_ref_orphan.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div aria_labelledby="nope" { } };
}
