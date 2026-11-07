//! input attr by type: o bloco não compila (mensagem em `input_attr_by_type.stderr`).

use html::html;

fn main() {
    let _ = html! { label { input type="email" name=(n) min="1"; } };
}
