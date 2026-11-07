//! missing input name: o bloco não compila (mensagem em `missing_input_name.stderr`).

use html::html;

fn main() {
    let _ = html! { label { input type="text" id="n"; } };
}
