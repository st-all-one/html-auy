//! dup id block: o bloco não compila (mensagem em `dup_id_block.stderr`).

use html::html;

fn main() {
    let _ = html! { div id="a" { } div id="a" { } };
}
