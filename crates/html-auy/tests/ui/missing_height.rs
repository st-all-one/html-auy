//! missing height: o bloco não compila (mensagem em `missing_height.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { img src=(s) alt=(a) width=(w); };
}
