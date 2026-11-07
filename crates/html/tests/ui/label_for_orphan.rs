//! label for orphan: o bloco não compila (mensagem em `label_for_orphan.stderr`).

use html::html;

fn main() {
    let _ = html! { label for="nope" { m } };
}
