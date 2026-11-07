//! aria modal: o bloco não compila (mensagem em `aria_modal.stderr`).

use html::html;

fn main() {
    let _ = html! { dialog aria_modal="true" aria_label=(l) { m } };
}
