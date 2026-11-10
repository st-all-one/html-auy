# html-auy

Typed, deterministic HTML generation for Rust. The HTML is the artifact: one escape per context,
attributes validated before compilation, and accessibility enforced by the types that emit markup.

```toml
[dependencies]
html-auy = "0.2"
```

```rust
use html_auy::prelude::*;

#[derive(Clone, Copy, Msg)]
enum M {
    #[msg("News")]
    News,
}

let link = html! {
    a href="/news" { M::News }
};
```

`prelude` gathers the everyday vocabulary. Internal literal routes are validated by the macro;
external URLs remain typed (`Href::external`, `Src::parse`).

`html!` is a compile-time checked vocabulary, not a general-purpose string template. Errors such as
an unknown attribute, an `img` without `alt`, or a `button` with a bad accessible name are emitted
by the macro with the source span.

## Guarantees

- **Typed escaping**: `Text`, `AttrVal`, `Href`, `Src`, `SrcSet`, `JsonLd` carry the context.
- **No raw HTML escape hatch**: `Markup` has no public constructor from `String`.
- **Accessibility by construction**: components such as `Img`, `Form`, and `Field` require the
  semantics they emit.
- **Deterministic bytes**: attribute, class, `srcset`, and asset order are canonical, so `ETag` is
  stable across renders.
- **`no_std` core**: no I/O, clock, environment, or runtime. An optional `audit` feature adds a
  final-document check through `html5ever`.

## Shell

`Page` and `ShellData` emit the document scaffold (`html`, `head`, `body`) and keep view composition
out of the macro. `View` carries the main content, modals, and assets; assets preserve declaration
order and deduplicate without moving.

## Features

- `audit` — enables `html_auy::audit::check`, a final-document pass for page-level rules.

## MSRV

Rust 1.92, edition 2024. The development toolchain is pinned; the MSRV is tested in CI.

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.

Repository: <https://github.com/st-all-one/html-auy>
