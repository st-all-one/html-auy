//! Parser: formas válidas parseiam, formas inválidas viram diagnóstico com a mensagem certa.

use crate::ast::{AttrValue, Block, Item};
use crate::diagnostic::Diag;
use crate::parser::parse;
use proc_macro2::TokenStream;
use proptest::prelude::*;
use std::str::FromStr;

fn parse_str(source: &str) -> Result<Block, Vec<Diag>> {
    let tokens = TokenStream::from_str(source).expect("fonte recusada pelo tokenizador");
    parse(tokens)
}

fn first_error(source: &str) -> Diag {
    let mut diags = parse_str(source).expect_err("esperava erro");
    assert!(!diags.is_empty(), "erro sem diagnóstico: {source}");
    diags.swap_remove(0)
}

#[test]
fn valid_forms_parse() {
    let cases = [
        "div { }",
        "br;",
        r#"img src=(Src::media(&m)) alt=(Alt::new("x")) width=(WidthPx::new(1)) height=(HeightPx::new(1));"#,
        "a href=(Href::internal(url)) { Msg::Home }",
        "div class=(Class::of(&[Token::Card])) { }",
        "div hidden=(true) { }",
        r#"div data_section="x" { }"#,
        "div aria_label=(Label::text(Msg::X)) { }",
        "div { @if cond { p { Msg::A } } }",
        "div { @if cond { } @else { } }",
        "div { @if a { } @else if b { } @else { } }",
        "ul { for item in &data.items { li { Text::new(&item.title) } } }",
        "div { Msg::Title }",
        "div { (Text::new(&s)) }",
        "div { Text::new(&s) }",
        "div { Path::f(1, 2) }",
        "div { Form::open(a, b, c) { Msg::X } }",
        "div { @if &mut flag { } }",
        r#"svg viewBox="0 0 24 24" { }"#,
        "div { div { div { span { Msg::X } } } }",
        "div hidden?=(maybe) { }",
        "div title=\"literal\" { }",
    ];
    for source in cases {
        assert!(parse_str(source).is_ok(), "deveria parsear: {source}");
    }
}

#[test]
fn children_are_classified_by_shape() {
    let block = parse_str("div { Msg::X span { } (minha) }").unwrap();
    let [Item::Element(element)] = block.items.as_slice() else {
        panic!("esperava um elemento na raiz");
    };
    let body = element.body.as_ref().expect("corpo");
    assert!(matches!(body.items.first(), Some(Item::Message(_))));
    assert!(matches!(body.items.get(1), Some(Item::Element(_))));
    assert!(matches!(body.items.get(2), Some(Item::Child(_))));

    let block = parse_str("div { Text::new(&s) }").unwrap();
    let [Item::Element(element)] = block.items.as_slice() else {
        panic!("esperava um elemento na raiz");
    };
    let body = element.body.as_ref().expect("corpo");
    assert!(matches!(body.items.first(), Some(Item::Child(_))));
}

#[test]
fn primitive_takes_children() {
    let block = parse_str("Form::open(a, b, c) { Msg::X }").unwrap();
    let [Item::Primitive(primitive)] = block.items.as_slice() else {
        panic!("esperava primitivo");
    };
    assert_eq!(primitive.children.items.len(), 1);
}

#[test]
fn attributes_keep_source_name_and_shape() {
    let block = parse_str("div data_section=\"a b\" hidden?=(opt) class=(c) { }").unwrap();
    let [Item::Element(element)] = block.items.as_slice() else {
        panic!("esperava elemento");
    };
    assert_eq!(element.attrs.len(), 3);
    assert_eq!(
        element.attrs.first().map(|attr| attr.name.as_str()),
        Some("data_section")
    );
    assert!(matches!(
        element.attrs.first().map(|attr| &attr.value),
        Some(AttrValue::Literal { text, .. }) if text == "a b"
    ));
    assert!(element.attrs.get(1).is_some_and(|attr| attr.optional));
    assert!(matches!(
        element.attrs.get(2).map(|attr| &attr.value),
        Some(AttrValue::Computed { .. })
    ));
}

#[test]
fn if_chain_nests() {
    let block = parse_str("@if a { } @else if b { } @else { }").unwrap();
    let [Item::If(node)] = block.items.as_slice() else {
        panic!("esperava @if");
    };
    let Some(crate::ast::Otherwise::If(inner)) = &node.otherwise else {
        panic!("esperava @else if");
    };
    assert!(matches!(
        inner.otherwise,
        Some(crate::ast::Otherwise::Block(_))
    ));
}

#[test]
fn invalid_forms_report_their_code() {
    let cases: [(&str, &str); 12] = [
        ("div", "`div` exige corpo: `div { … }`"),
        ("img src=(Src::media(&m))", "`img` é void: termine com `;`"),
        (
            "div class { }",
            "atributo `class` sem valor; use `class=(…)`",
        ),
        (
            "div { \"texto\" }",
            "texto literal não é permitido; use uma variante de `Msg`",
        ),
        ("div { @when cond { } }", "diretiva desconhecida `@when`"),
        ("@else { }", "`@else` sem `@if`"),
        ("div { } ;", "token inesperado em `html!`"),
        ("for item { }", "`for` exige `in` e um corpo `{ … }`"),
        ("Div { }", "nome de elemento inválido"),
        ("@if cond", "`@if` exige um corpo `{ … }`"),
        ("for x in y", "`for` exige um corpo `{ … }`"),
        ("div class=\"a\"", "`div` exige corpo: `div { … }`"),
    ];
    for (source, expected) in cases {
        let diag = first_error(source);
        assert_eq!(diag.message, expected, "fonte: {source}");
    }
}

#[test]
fn errors_carry_position_and_hint() {
    let diag = first_error("Div { }");
    assert_eq!(diag.span.start().line, 1);
    assert_eq!(diag.span.start().column, 0);

    let diag = first_error("div { \"texto\" }");
    assert!(diag.hint.is_some(), "texto literal precisa de ajuda");
    assert_eq!(diag.span.start().line, 1);

    let diag = first_error("for item { }");
    assert_eq!(diag.span.start().column, 0);
}

proptest! {
    #[test]
    fn arbitrary_tokens_never_panic(source in "[a-z0-9{}();=@.,:\"'<>+&?!#\\[\\]]{0,48}") {
        if let Ok(tokens) = TokenStream::from_str(&source) {
            let _result = parse(tokens);
        }
    }
}

#[test]
fn deep_nesting_is_an_error_not_a_stack_overflow() {
    let depth = 200;
    let mut source = String::new();
    for _ in 0..depth {
        source.push_str("div { ");
    }
    for _ in 0..depth {
        source.push_str(" }");
    }
    let diags = parse_str(&source).expect_err("aninhamento excessivo é erro");
    assert!(
        diags.iter().any(|diag| diag.message.contains("excede 64")),
        "mensagem de profundidade: {:?}",
        diags.iter().map(|diag| &diag.message).collect::<Vec<_>>()
    );
}

#[test]
fn expand_never_panics_on_hostile_tokens() {
    let cases = [
        "",
        "div",
        "div {",
        "}",
        "@if",
        "div { @else }",
        "div { for x in }",
        "div { @if { } }",
        "img;",
        "div { ( ) }",
        "div { m }",
        "a href= { }",
        "div { div { div { } } }",
        "🙂 { }",
        "'lifetime { }",
    ];
    for source in cases {
        let Ok(tokens) = TokenStream::from_str(source) else {
            continue;
        };
        drop(crate::expand(tokens));
    }
}
