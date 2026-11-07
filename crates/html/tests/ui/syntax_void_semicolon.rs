//! syntax void semicolon: o bloco não compila (mensagem em `syntax_void_semicolon.stderr`).

use html::html;

fn main() {
    let _ = html! { br };
}
