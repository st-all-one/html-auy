//! forbidden style: o bloco não compila (mensagem em `forbidden_style.stderr`).

use html::html;

fn main() {
    let _ = html! { div style="color: red" { } };
}
