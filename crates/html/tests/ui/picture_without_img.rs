//! `picture` sem `img`: o fallback é obrigatório.

use html::html;

fn main() {
    let _ = html! {
        picture {
            source srcset=(ss) sizes=(sz);
        }
    };
}
