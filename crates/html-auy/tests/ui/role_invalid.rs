//! role invalid: o bloco não compila (mensagem em `role_invalid.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { dialog role="foo" aria_label=(l) { m } };
}
