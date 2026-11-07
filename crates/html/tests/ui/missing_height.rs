//! missing height: o bloco não compila (mensagem em `missing_height.stderr`).

use html::html;

fn main() {
    let _ = html! { img src=(s) alt=(a) width=(w); };
}
