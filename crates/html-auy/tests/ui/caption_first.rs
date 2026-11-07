//! caption first: o bloco não compila (mensagem em `caption_first.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { table { colgroup { } caption { m } tbody { tr { td { m } } } } };
}
