//! literal alt: o bloco não compila (mensagem em `literal_alt.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { img src=(s) width=(w) height=(h) alt="x"; };
}
