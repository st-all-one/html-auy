//! missing href: o bloco não compila (mensagem em `missing_href.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { a { m } };
}
