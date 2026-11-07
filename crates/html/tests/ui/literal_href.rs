//! literal href: o bloco não compila (mensagem em `literal_href.stderr`).

use html::html;

fn main() {
    let _ = html! { a href="/x" { m } };
}
