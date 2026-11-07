//! srcset without sizes: o bloco não compila (mensagem em `srcset_without_sizes.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { img src=(s) alt=(a) width=(w) height=(h) srcset=(ss); };
}
