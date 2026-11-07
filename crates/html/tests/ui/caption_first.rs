//! caption first: o bloco não compila (mensagem em `caption_first.stderr`).

use html::html;

fn main() {
    let _ = html! { table { colgroup { } caption { m } tbody { tr { td { m } } } } };
}
