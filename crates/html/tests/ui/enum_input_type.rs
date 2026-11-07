//! enum input type: o bloco não compila (mensagem em `enum_input_type.stderr`).

use html::html;

fn main() {
    let _ = html! { label { input type="texto" name=(n); } };
}
