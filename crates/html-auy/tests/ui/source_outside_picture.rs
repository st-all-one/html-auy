//! `source` fora de `picture`: o perfil de mídia não permite.

use html_auy::html;

fn main() {
    let _ = html! {
        div {
            source srcset=(ss) sizes=(sz);
        }
    };
}
