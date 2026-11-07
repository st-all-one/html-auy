//! target blank rel: o bloco não compila (mensagem em `target_blank_rel.stderr`).

use html::html;

fn main() {
    let _ = html! { a href=(h) target="_blank" rel="external" { m } };
}
