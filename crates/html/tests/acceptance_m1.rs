//! M1: a página é montada com os primitivos do crate, sem macro.

mod common;

use html::__private::Writer;
use html::Text;

#[test]
fn page_without_macro_matches_contract() {
    let mut writer = Writer::with_capacity(common::MAIN.len());
    writer.open("h2");
    writer.close_open();
    writer.child(&Text::new("Notícias"));
    writer.close("h2");
    writer.open("ul");
    writer.close_open();
    for title in ["Primeira", "Segunda"] {
        writer.open("li");
        writer.close_open();
        writer.child(&Text::new(title));
        writer.close("li");
    }
    writer.close("ul");
    assert_eq!(writer.finish().as_str(), common::MAIN);
}
