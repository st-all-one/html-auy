//! syntax attr no value: o bloco não compila (mensagem em `syntax_attr_no_value.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div class { } };
}
