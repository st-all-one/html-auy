//! role not allowed: o bloco não compila (mensagem em `role_not_allowed.stderr`).

use html::html;

fn main() {
    let _ = html! { p role="note" { m } };
}
