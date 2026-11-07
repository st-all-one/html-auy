//! rel opener: o bloco não compila (mensagem em `rel_opener.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { a href=(h) rel="opener" { m } };
}
