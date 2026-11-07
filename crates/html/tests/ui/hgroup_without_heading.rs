//! hgroup sem heading: o bloco não compila (mensagem em `hgroup_without_heading.stderr`).

use html::html;

fn main() {
    let _ = html! { hgroup { p { m } } };
}
