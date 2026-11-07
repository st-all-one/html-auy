//! forbidden onclick: o bloco não compila (mensagem em `forbidden_onclick.stderr`).

use html::html;

fn main() {
    let _ = html! { div onclick="go()" { } };
}
