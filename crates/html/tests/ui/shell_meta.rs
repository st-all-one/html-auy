//! shell meta: o bloco não compila (mensagem em `shell_meta.stderr`).

use html::html;

fn main() {
    let _ = html! { meta charset="utf-8"; };
}
