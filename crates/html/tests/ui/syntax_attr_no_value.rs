//! syntax attr no value: o bloco não compila (mensagem em `syntax_attr_no_value.stderr`).

use html::html;

fn main() {
    let _ = html! { div class { } };
}
