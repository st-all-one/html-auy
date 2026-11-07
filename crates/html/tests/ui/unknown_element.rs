//! unknown element: o bloco não compila (mensagem em `unknown_element.stderr`).

use html::html;

fn main() {
    let _ = html! { dev { } };
}
