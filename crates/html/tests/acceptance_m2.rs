//! M2: a mesma página com `html!`; os bytes têm de bater com os de M1.

mod common;

use html::{Lang, Page, ShellData, Text, View, html};

#[test]
fn macro_matches_primitive_bytes() {
    let drawn = html! {
        h2 { (Text::new("Notícias")) }
        ul {
            for title in ["Primeira", "Segunda"] {
                li { (Text::new(title)) }
            }
        }
    };
    assert_eq!(drawn.as_str(), common::MAIN);
}

#[test]
fn page_from_view_is_stable() {
    let data = ShellData::new(
        Text::new("Notícias"),
        Text::new("Notícias"),
        Lang::parse("pt-BR").expect("etiqueta BCP-47"),
    );
    let main = html! {
        h2 { (Text::new("Notícias")) }
        ul {
            for title in ["Primeira", "Segunda"] {
                li { (Text::new(title)) }
            }
        }
    };
    let page = Page::new(&data, View::new(main));
    insta::assert_snapshot!("acceptance_m2_page", page.markup().as_str());
}
