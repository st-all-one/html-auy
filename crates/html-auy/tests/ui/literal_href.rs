//! literal href: o bloco não compila (mensagem em `literal_href.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { a href="/x" { m } };
}
