//! datalist sem option: o bloco não compila (mensagem em `datalist_without_option.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { datalist { } };
}
