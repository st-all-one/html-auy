//! label for orphan: o bloco não compila (mensagem em `label_for_orphan.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { label for="nope" { m } };
}
