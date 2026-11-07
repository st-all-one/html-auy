//! Propriedades do escape: por contexto, nada perigoso atravessa.

use proptest::prelude::*;

/// Fora do `prop_assert!`: `\u{0}` dentro da macro vira referência a argumento de formatação.
const NUL: char = '\0';
const LINE_SEPARATOR: char = '\u{2028}';

/// `&` só aparece iniciando uma entidade conhecida.
fn ampersands_are_entities(output: &str) -> bool {
    let mut rest = output;
    while let Some(index) = rest.find('&') {
        let Some(tail) = rest.get(index..) else {
            return false;
        };
        let known = [
            "&amp;", "&lt;", "&gt;", "&quot;", "&#x27;", "&#13;", "&#10;", "&#9;", "&#xFFFD;",
        ];
        let Some(entity) = known.iter().find(|entity| tail.starts_with(**entity)) else {
            return false;
        };
        let Some(next) = rest.get(index.saturating_add(entity.len())..) else {
            return false;
        };
        rest = next;
    }
    true
}

proptest! {
    #[test]
    fn text_output_has_no_raw_specials(input in ".*") {
        let mut out = String::new();
        html_auy::escape::text(&mut out, &input);
        prop_assert!(!out.contains('<'));
        prop_assert!(!out.contains('>'));
        prop_assert!(!out.contains('"'));
        prop_assert!(!out.contains('\''));
        prop_assert!(!out.contains(NUL));
        prop_assert!(ampersands_are_entities(&out));
    }

    #[test]
    fn text_is_identity_without_specials(input in "[^&<>\"'\\x00]*") {
        let mut out = String::new();
        html_auy::escape::text(&mut out, &input);
        prop_assert_eq!(out, input);
    }

    #[test]
    fn attr_output_has_no_raw_whitespace_or_specials(input in ".*") {
        let mut out = String::new();
        html_auy::escape::attr(&mut out, &input);
        prop_assert!(!out.contains('<'));
        prop_assert!(!out.contains('>'));
        prop_assert!(!out.contains('"'));
        prop_assert!(!out.contains('\''));
        prop_assert!(!out.contains('\n'));
        prop_assert!(!out.contains('\r'));
        prop_assert!(!out.contains('\t'));
        prop_assert!(!out.contains(NUL));
        prop_assert!(ampersands_are_entities(&out));
    }

    #[test]
    fn json_script_round_trips_through_a_json_parser(input in ".*") {
        let value = serde_json::json!({
            "text": input,
            "hostile": "</script><script>alert(1)</script>",
            "separators": "a\u{2028}b\u{2029}c",
        });
        let raw = serde_json::to_string(&value).unwrap();
        let mut escaped = String::new();
        html_auy::escape::json_script(&mut escaped, &raw);
        prop_assert!(!escaped.contains("</script"));
        prop_assert!(!escaped.contains('<'));
        prop_assert!(!escaped.contains('>'));
        prop_assert!(!escaped.contains(LINE_SEPARATOR));
        let parsed: serde_json::Value = serde_json::from_str(&escaped).unwrap();
        prop_assert_eq!(parsed, value);
    }
}
