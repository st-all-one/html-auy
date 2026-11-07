//! `Markup`/`Render`: um buffer, duas portas, nenhuma conversão implícita.

use html_auy::{Markup, Render, escape};

struct Fixed(&'static str);

impl Render for Fixed {
    const SIZE_HINT: usize = 3;

    fn render_to(&self, out: &mut String) {
        escape::text(out, self.0);
    }
}

#[test]
fn render_equals_render_to() {
    let value = Fixed("<x>");
    let mut out = String::new();
    value.render_to(&mut out);
    assert_eq!(value.render().as_str(), out);
    assert_eq!(value.render().as_str(), "&lt;x&gt;");
    assert_eq!(<Fixed as Render>::SIZE_HINT, 3);
    assert_eq!(value.size_hint(), 3);
}

#[test]
fn markup_is_render_and_from_render() {
    let markup = Markup::from_render(&Fixed("oi"));
    assert_eq!(markup.as_str(), "oi");
    assert_eq!(markup.len(), 2);
    assert!(!markup.is_empty());

    let again = Markup::from_render(&markup);
    assert_eq!(again, markup);
    assert_eq!(markup.clone().into_string(), "oi");
    assert_eq!(markup.clone().render(), markup);
}

#[test]
fn references_render_without_copy() {
    let value = Fixed("x");
    let reference = &value;
    assert_eq!(Markup::from_render(&reference).as_str(), "x");
    assert_eq!(Markup::from_render(&&value).as_str(), "x");
}

#[test]
fn empty_markup() {
    let empty = Markup::from_render(&Fixed(""));
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.as_str(), "");
}
