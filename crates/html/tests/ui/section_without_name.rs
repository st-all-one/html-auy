//! section without name: o bloco não compila (mensagem em `section_without_name.stderr`).

use html::html;

fn main() {
    let _ = html! { section { m } };
}
