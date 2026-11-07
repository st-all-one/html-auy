//! srcset without sizes: o bloco não compila (mensagem em `srcset_without_sizes.stderr`).

use html::html;

fn main() {
    let _ = html! { img src=(s) alt=(a) width=(w) height=(h) srcset=(ss); };
}
