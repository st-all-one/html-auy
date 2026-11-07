//! missing alt: o bloco não compila (mensagem em `missing_alt.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { img src=(s) width=(w) height=(h); };
}
