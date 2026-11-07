//! width must be pixels: `width` em `img`/`video`/`iframe` exige `WidthPx`
//! (mensagem em `width_must_be_pixels.stderr`).

use html_auy::{Alt, AttrVal, HeightPx, LUrl, MediaPath, Src, html};

struct Media(LUrl<'static>);

impl MediaPath for Media {
    fn path(&self) -> &LUrl<'_> {
        &self.0
    }
}

fn main() {
    let media = Media(LUrl::new("/m.png").unwrap());
    let _ = html! {
        img src=(Src::media(&media)) alt=(Alt::new("x"))
            width=(AttrVal::new("100%")) height=(HeightPx::new(1));
    };
}
