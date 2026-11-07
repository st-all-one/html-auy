//! Geração de HTML tipada: o vocabulário é uma tabela, o escape é único, a saída é canônica.
//!
//! O crate não faz I/O, não lê ambiente e não conhece locale: ele recebe **dados** já decididos e
//! devolve **bytes** determinísticos. As garantias são de três espécies, e cada uma mora num lugar
//! que pode ser testado:
//!
//! - **tipo** — não existe `Markup::from(String)`, nem `style=`, nem `<form>` sem CSRF; o que não
//!   compila não precisa ser revisado ([`Markup`], [`form`]);
//! - **bloco** — `html!` valida o fragmento contra [`spec`] e recusa com `compile_error!`;
//! - **página** — [`audit`] reprocessa o HTML final quando o consumidor liga a feature.
//!
//! ```
//! use html::{escape, Markup, Render};
//!
//! struct Warning<'a>(&'a str);
//!
//! impl Render for Warning<'_> {
//!     const SIZE_HINT: usize = 48;
//!
//!     fn render_to(&self, out: &mut String) {
//!         out.push_str(r#"<p class="warning">"#);
//!         escape::text(out, self.0);
//!         out.push_str("</p>");
//!     }
//! }
//!
//! let markup = Markup::from_render(&Warning("atenção <agora>"));
//! assert_eq!(markup.as_str(), r#"<p class="warning">atenção &lt;agora&gt;</p>"#);
//! ```

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(test)]
extern crate std;

// Faz `::html::…` resolver dentro do próprio crate (a expansão do macro usa caminho absoluto).
extern crate self as html;

#[cfg(feature = "audit")]
pub mod audit;

#[doc(hidden)]
pub mod __private;
pub mod escape;
pub mod field;
pub mod form;
pub mod hint;
mod markup;
pub mod media;
mod msg;
pub(crate) mod numeric;
pub mod response;
pub mod shell;
pub mod spec;
pub mod value;
pub mod view;

pub use field::{ChoiceOption, Field, FieldState, InputType, SelectOption};
pub use form::{Action, CsrfToken, Form, Method, SubmitKey};
pub use hint::{
    CrossOrigin, FetchPriority, Hint, ImagePreload, Preload, PreloadAs, ReferrerPolicy,
};
pub use html_macros::html;
pub use markup::{Markup, Render};
pub use media::{Audio, Img, Loading, MediaPreload, Picture, Source, Track, TrackKind, Video};
pub use msg::{Msg, Text};
pub use response::{MainSwap, Partial, RegionUpdate, RequestClass, Swap, Sync};
pub use shell::{
    Alternate, Asset, AssetKind, Assets, HtmxConfig, OpenGraph, OpenGraphKind, Page, Regions,
    ShellData,
};
pub use value::{
    Alt, AttrVal, AttrValue, Autocomplete, Class, ClassToken, Dir, ExternalUrl, FieldName,
    Fragment, HeightPx, Href, Hreflang, Id, Integrity, JsonGraph, JsonLd, LUrl, Label, Lang,
    MailAddr, Media, MediaPath, Offset, PhoneNumber, Sizes, Src, SrcSet, Timestamp, Tone, Variant,
    WidthPx,
};
pub use view::View;
