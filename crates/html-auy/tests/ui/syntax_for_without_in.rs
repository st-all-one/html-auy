//! syntax for without in: o bloco não compila (mensagem em `syntax_for_without_in.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div { for x { } } };
}
