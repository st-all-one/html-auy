//! `<a>` transparente herda `p` (phrasing): `<div>` não cabe.

use html::{Fragment, Href, Text, html};

fn main() {
    let href = Href::fragment(Fragment::new("#x").expect("fragmento"));
    let text = Text::new("x");
    let _ = html! { p { a href=(href) { div { (text) } } } };
}
