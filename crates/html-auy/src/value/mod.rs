//! Valores tipados: cada contexto de escrita tem um tipo, e o tipo é a garantia.

mod autocomplete;
mod class;
mod image;
mod integrity;
mod jsonld;
mod lang;
mod media;
mod text;
mod timestamp;
pub(crate) mod url;

use alloc::string::String;

pub use autocomplete::Autocomplete;
pub use class::{Class, ClassList, ClassToken, Tone};
pub use image::{Alt, HeightPx, Sizes, SrcSet, Variant, WidthPx};
pub use integrity::Integrity;
pub use jsonld::{JsonGraph, JsonLd};
pub use lang::{Dir, Hreflang, Lang};
pub use media::Media;
pub use text::{AttrVal, FieldName, Id, Label};
pub use timestamp::{Date, DateTimeValue, Offset, Timestamp};
pub(crate) use url::valid_path;
pub use url::{ExternalUrl, Fragment, Href, LUrl, MailAddr, MediaPath, PhoneNumber, Src};

use crate::escape;
use crate::msg::Text;
use crate::numeric;

/// Escreve o valor de um atributo **já escapado**, sem aspas.
///
/// Implementar este trait é assumir a responsabilidade pelo escape; por isso `&str` e `String`
/// **não** o implementam: string livre entra por [`Text`]/[`AttrVal`], que escapam.
pub trait AttrValue {
    /// Escreve o valor escapado em `out`.
    fn write_attr(&self, out: &mut String);

    /// `true` quando o atributo deve ser omitido (lista vazia).
    fn is_empty_attr(&self) -> bool {
        false
    }
}

/// Implementa [`AttrValue`] para um tipo que já garante o próprio vocabulário.
macro_rules! attr_value_via_escape {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl AttrValue for $ty {
                fn write_attr(&self, out: &mut String) {
                    escape::attr(out, self.as_str());
                }
            }
        )+
    };
}

attr_value_via_escape!(
    Text<'_>,
    AttrVal<'_>,
    Label<'_>,
    Src<'_>,
    Sizes<'_>,
    Id<'_>,
    FieldName<'_>,
    Timestamp,
    Lang,
    Media<'_>,
    Integrity<'_>,
    Autocomplete<'_>,
);

impl AttrValue for Class {
    fn write_attr(&self, out: &mut String) {
        let mut tokens = self.tokens().iter();
        if let Some(first) = tokens.next() {
            escape::attr(out, first);
        }
        for token in tokens {
            out.push(' ');
            escape::attr(out, token);
        }
    }

    fn is_empty_attr(&self) -> bool {
        self.is_empty()
    }
}

impl AttrValue for ClassList {
    fn write_attr(&self, out: &mut String) {
        let mut tokens = self.tokens().iter();
        if let Some(first) = tokens.next() {
            escape::attr(out, first);
        }
        for token in tokens {
            out.push(' ');
            escape::attr(out, token);
        }
    }

    fn is_empty_attr(&self) -> bool {
        self.is_empty()
    }
}

impl AttrValue for SrcSet<'_> {
    fn write_attr(&self, out: &mut String) {
        let mut entries = self.entries().iter();
        if let Some((width, src)) = entries.next() {
            write_srcset_entry(out, *width, src);
        }
        for (width, src) in entries {
            out.push_str(", ");
            write_srcset_entry(out, *width, src);
        }
    }
}

fn write_srcset_entry(out: &mut String, width: u32, src: &str) {
    escape::attr(out, src);
    out.push(' ');
    numeric::write_u64(out, u64::from(width));
    out.push('w');
}

impl AttrValue for Alt<'_> {
    fn write_attr(&self, out: &mut String) {
        escape::attr(out, self.as_str());
    }
}

impl AttrValue for Href<'_> {
    fn write_attr(&self, out: &mut String) {
        self.write_href(out);
    }
}

impl AttrValue for WidthPx {
    fn write_attr(&self, out: &mut String) {
        numeric::write_u64(out, u64::from(self.get()));
    }
}

impl AttrValue for HeightPx {
    fn write_attr(&self, out: &mut String) {
        numeric::write_u64(out, u64::from(self.get()));
    }
}

macro_rules! attr_value_unsigned {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl AttrValue for $ty {
                fn write_attr(&self, out: &mut String) {
                    numeric::write_u64(out, u64::from(*self));
                }
            }
        )+
    };
}

macro_rules! attr_value_signed {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl AttrValue for $ty {
                fn write_attr(&self, out: &mut String) {
                    numeric::write_i64(out, i64::from(*self));
                }
            }
        )+
    };
}

attr_value_unsigned!(u8, u16, u32, u64);
attr_value_signed!(i8, i16, i32, i64);
