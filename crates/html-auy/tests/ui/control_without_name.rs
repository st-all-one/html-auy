//! control without name: o bloco não compila (mensagem em `control_without_name.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { button type="button" { } };
}
