//! aria enum: o bloco não compila (mensagem em `aria_enum.stderr`).

use html::html;

fn main() {
    let _ = html! { div aria_live="loud" { } };
}
