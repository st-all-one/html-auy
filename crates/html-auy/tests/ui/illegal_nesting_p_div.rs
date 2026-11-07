//! illegal nesting p div: o bloco não compila (mensagem em `illegal_nesting_p_div.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { p { div { } } };
}
