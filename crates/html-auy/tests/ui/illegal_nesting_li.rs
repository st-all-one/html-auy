//! illegal nesting li: o bloco não compila (mensagem em `illegal_nesting_li.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div { li { } } };
}
