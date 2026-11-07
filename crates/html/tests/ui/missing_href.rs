//! missing href: o bloco não compila (mensagem em `missing_href.stderr`).

use html::html;

fn main() {
    let _ = html! { a { m } };
}
