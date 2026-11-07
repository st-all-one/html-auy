//! aria modal: o bloco não compila (mensagem em `aria_modal.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { dialog aria_modal="true" aria_label=(l) { m } };
}
