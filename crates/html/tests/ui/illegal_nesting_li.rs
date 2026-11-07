//! illegal nesting li: o bloco não compila (mensagem em `illegal_nesting_li.stderr`).

use html::html;

fn main() {
    let _ = html! { div { li { } } };
}
