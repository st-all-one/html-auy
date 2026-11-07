//! dialog without name: o bloco não compila (mensagem em `dialog_without_name.stderr`).

use html::html;

fn main() {
    let _ = html! { dialog { m } };
}
