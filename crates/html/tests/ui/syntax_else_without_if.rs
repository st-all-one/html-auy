//! syntax else without if: o bloco não compila (mensagem em `syntax_else_without_if.stderr`).

use html::html;

fn main() {
    let _ = html! { div { @else { } } };
}
