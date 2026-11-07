//! form in macro: o bloco não compila (mensagem em `form_in_macro.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { form { } };
}
