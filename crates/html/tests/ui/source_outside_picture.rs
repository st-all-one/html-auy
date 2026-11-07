//! `source` fora de `picture`: o perfil de mídia não permite.

use html::html;

fn main() {
    let _ = html! {
        div {
            source srcset=(ss) sizes=(sz);
        }
    };
}
