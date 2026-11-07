//! label without for: o bloco não compila (mensagem em `label_without_for.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { label { m } };
}
