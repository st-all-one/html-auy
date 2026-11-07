//! summary two headings: o bloco não compila (mensagem em `summary_two_headings.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { details { summary { h2 { m } h3 { m } } } };
}
