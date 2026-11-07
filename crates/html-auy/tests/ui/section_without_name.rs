//! section without name: o bloco não compila (mensagem em `section_without_name.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { section { m } };
}
