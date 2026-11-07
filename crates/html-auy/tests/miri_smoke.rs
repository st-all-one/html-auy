//! Caminho puro exercitado sob miri: sem `unsafe`, a expectativa é "nenhum UB, nenhum vazamento".

use html_auy::{Markup, Offset, SrcSet, Text, Timestamp, Variant, escape};

struct Fixed(&'static str, u32);

impl Variant for Fixed {
    fn src(&self) -> &str {
        self.0
    }

    fn width(&self) -> u32 {
        self.1
    }
}

#[test]
fn core_paths_are_sound() {
    let mut out = String::new();
    escape::text(&mut out, "<a href=\"x\">&'\0 fim");
    escape::attr(&mut out, "a\r\n\tb");
    escape::json_script(&mut out, "</script>\u{2028}\u{2029}");
    assert_ne!(out, "");

    let markup = Markup::from_render(&Text::new("<a>"));
    assert_eq!(markup.as_str(), "&lt;a&gt;");
    assert_eq!(markup.len(), 9);

    let variants = [Fixed("/a-720.avif", 720), Fixed("/a-360.avif", 360)];
    let set = SrcSet::from_variants(&variants);
    assert_eq!(set.entries().len(), 2);

    let stamp = Timestamp::new(1_760_000_000, Offset::from_hms(-3, 0).unwrap());
    assert_eq!(stamp.as_str().len(), 25);
}
