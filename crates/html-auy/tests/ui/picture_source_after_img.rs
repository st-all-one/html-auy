//! `source` depois do `img`: a ordem do `picture` não compila.

use html_auy::html;

fn main() {
    let _ = html! {
        picture {
            img src=(s) alt=(a) width=(w) height=(h);
            source srcset=(ss) sizes=(sz);
        }
    };
}
