//! aria hidden focusable: o bloco não compila (mensagem em `aria_hidden_focusable.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { button type="button" aria_hidden="true" { m } };
}
