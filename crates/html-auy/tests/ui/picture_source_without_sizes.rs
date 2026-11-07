//! `source` sem `sizes`: o descritor `w` do `SrcSet` exige a lista.

use html_auy::html;

fn main() {
    let _ = html! {
        picture {
            source type="image/avif" srcset=(ss);
            img src=(s) alt=(a) width=(w) height=(h);
        }
    };
}
