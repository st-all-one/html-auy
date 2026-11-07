//! Propriedades dos valores: as invariantes que sobrevivem a qualquer entrada.

use html_auy::{Class, ClassToken, Lang, Offset, SrcSet, Timestamp, Variant};
use proptest::prelude::*;

#[derive(Clone, Copy)]
enum Token {
    Card,
    List,
    Featured,
}

impl ClassToken for Token {
    fn as_str(self) -> &'static str {
        match self {
            Self::Card => "card",
            Self::List => "list",
            Self::Featured => "featured",
        }
    }
}

fn tokens(choices: &[u8]) -> Vec<Token> {
    choices
        .iter()
        .map(|choice| match choice % 3 {
            0 => Token::Card,
            1 => Token::List,
            _ => Token::Featured,
        })
        .collect()
}

struct Fixed {
    src: &'static str,
    width: u32,
}

impl Variant for Fixed {
    fn src(&self) -> &str {
        self.src
    }

    fn width(&self) -> u32 {
        self.width
    }
}

const VARIANT_POOL: [(&str, u32); 5] = [
    ("/a-360.avif", 360),
    ("/a-720.avif", 720),
    ("/a-1200.avif", 1200),
    ("/z-720.avif", 720),
    ("/a-zero.avif", 0),
];

proptest! {
    #[test]
    fn class_extend_is_commutative_and_idempotent(left in prop::collection::vec(0u8..3, 0..6), right in prop::collection::vec(0u8..3, 0..6)) {
        let left_class = Class::of(&tokens(&left));
        let right_class = Class::of(&tokens(&right));
        let merged = left_class.clone().extend(right_class.clone());
        let flipped = right_class.extend(left_class.clone());
        prop_assert_eq!(merged.tokens(), flipped.tokens());
        let doubled = left_class.clone().extend(left_class.clone());
        prop_assert_eq!(doubled.tokens(), left_class.tokens());
    }

    #[test]
    fn class_tokens_are_sorted_and_unique(choices in prop::collection::vec(0u8..3, 0..8)) {
        let class = Class::of(&tokens(&choices));
        let list = class.tokens();
        prop_assert!(list.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn srcset_is_idempotent(picks in prop::collection::vec(0usize..5, 0..6)) {
        let variants: Vec<Fixed> = picks
            .iter()
            .map(|pick| {
                let (src, width) = VARIANT_POOL[pick.checked_rem(VARIANT_POOL.len()).unwrap_or(0)];
                Fixed { src, width }
            })
            .collect();
        let first = SrcSet::from_variants(&variants);
        let widths: Vec<u32> = first.entries().iter().map(|(width, _)| *width).collect();
        prop_assert!(widths.windows(2).all(|pair| pair[0] < pair[1]));
        prop_assert!(widths.iter().all(|width| *width > 0));

        let reordered: Vec<Fixed> = variants.iter().rev().map(|variant| Fixed { src: variant.src, width: variant.width }).collect();
        let second = SrcSet::from_variants(&reordered);
        prop_assert_eq!(first.entries(), second.entries());
    }

    #[test]
    fn timestamp_shape_is_always_rfc3339(seconds in any::<i64>(), hours in -23i8..=23, minutes in 0u8..60) {
        let offset = Offset::from_hms(hours, minutes).unwrap();
        let stamp = Timestamp::new(seconds, offset);
        prop_assert!(looks_like_rfc3339(stamp.as_str()), "{}", stamp.as_str());
        prop_assert_eq!(Offset::from_seconds(offset.as_seconds()).unwrap(), offset);
    }

    #[test]
    fn lang_canonicalization_is_idempotent(input in "[A-Za-z]{1,4}(-[A-Za-z0-9]{1,8}){0,2}") {
        if let Some(lang) = Lang::parse(&input) {
            let canonical = lang.as_str().to_owned();
            let again = Lang::parse(&canonical).unwrap();
            prop_assert_eq!(again.as_str(), canonical.as_str());
            prop_assert_eq!(again, lang);
        }
    }
}

/// `YYYY-MM-DDTHH:MM:SS` + (`Z` | `±HH:MM`).
fn looks_like_rfc3339(text: &str) -> bool {
    let bytes = text.as_bytes();
    let digit = |at: usize| bytes.get(at).is_some_and(u8::is_ascii_digit);
    let separator = |at: usize, want: u8| bytes.get(at) == Some(&want);
    let digits_ok = [0usize, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18]
        .iter()
        .all(|at| digit(*at));
    let separators_ok = separator(4, b'-')
        && separator(7, b'-')
        && separator(10, b'T')
        && separator(13, b':')
        && separator(16, b':');
    if !digits_ok || !separators_ok {
        return false;
    }
    match bytes.get(19) {
        Some(b'Z') => bytes.len() == 20,
        Some(b'+' | b'-') => {
            bytes.len() == 25
                && separator(22, b':')
                && [20usize, 21, 23, 24].iter().all(|at| digit(*at))
        }
        _ => false,
    }
}
