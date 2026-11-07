//! dup id block: o bloco não compila (mensagem em `dup_id_block.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div id="a" { } div id="a" { } };
}
