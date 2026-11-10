//! Import único para o uso cotidiano.
//!
//! O prelude não amplia garantias: ele só reúne os nomes que quase toda página usa. Quem precisa
//! de um tipo menos comum continua importando explicitamente.

pub use crate::{
    Alt, Asset, Assets, Class, Date, Dir, ExternalUrl, HeightPx, Href, JsonGraph, JsonLd, LUrl,
    Label, Lang, Markup, Msg, Offset, Page, Regions, Render, ShellData, Text, Timestamp, View,
    WidthPx, html,
};
