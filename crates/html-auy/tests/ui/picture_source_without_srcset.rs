//! `source` sem `srcset`: não há variante a escolher.

use html_auy::html;

fn main() {
    let _ = html! {
        picture {
            source type="image/avif" sizes=(sz);
            img src=(s) alt=(a) width=(w) height=(h);
        }
    };
}
