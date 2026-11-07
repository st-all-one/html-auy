//! syntax else without if: o bloco não compila (mensagem em `syntax_else_without_if.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div { @else { } } };
}
