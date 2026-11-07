//! select without option: o bloco não compila (mensagem em `select_without_option.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { label { select name=(n) { option value="a" disabled=(d) { } } } };
}
