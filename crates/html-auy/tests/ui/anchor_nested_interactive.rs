//! `<a>` não pode conter interativo em nenhuma profundidade.

use html_auy::{Fragment, Href, Text, html};

fn main() {
    let href = Href::fragment(Fragment::new("#x").expect("fragmento"));
    let text = Text::new("Abrir");
    let _ = html! {
        div {
            a href=(href) {
                div {
                    button type="button" { (text) }
                }
            }
        }
    };
}
