//! aria label visible: o bloco não compila (mensagem em `aria_label_visible.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { button type="button" aria_label=(l) { m } };
}
