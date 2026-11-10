---
name: html-auy
description: >
  Produce correct Rust HTML with html-auy: typed escape contexts, deterministic
  Markup, compile-time vocabulary validation, accessible shell/forms/media, and
  final-document audit. Load when writing, reviewing, or migrating html-auy code.
category: rust
version: "0.2.0"
tags: [rust, html, accessibility, a11y, templates, security]
license: "MIT OR Apache-2.0"
---

# html-auy — correct usage skill

Use this skill when writing, reviewing, or migrating Rust code that emits HTML with `html-auy`. The objective is code that compiles against the real API, preserves the crate's semantics, and does not reintroduce the problems the library exists to remove.

## Goals

- Produce idiomatic, compiling `html-auy` code using only the public API.
- Keep HTML as an artifact: deterministic `Markup`, no VDOM, hydration, runtime templates, or raw string concatenation.
- Use typed values for text, URLs, attributes, classes, media, dates, and JSON-LD.
- Start from `html_auy::prelude::*` for the everyday vocabulary; the prelude is import-only.
- Use `Page`/`ShellData`/`View` for documents, `Form`/`Field` for forms, and the media primitives for images/audio/video.
- Preserve accessibility by construction: required alt text, labels, captions, landmarks, skip link, help slot, status region, and typed media alternatives.
- Keep security boundaries: no raw HTML sink, no arbitrary attribute names, no `on*`, no inline `<style>`, and `style=` only as an audited escape hatch.
- Validate changes with the crate's tests, `trybuild`, `audit`, spec checks, and the commands in this skill.

## Non-goals

Do not use `html-auy` as, or pretend it is:

- a web framework, HTTP server, router, session store, ORM, or `IntoResponse` implementation;
- a reactive UI runtime, VDOM, hydration system, or client-state container;
- a runtime template engine or a sanitizer for HTML from a database;
- a replacement for `axe`, `sniffCSS`, keyboard tests, or computed-style checks;
- a way to emit arbitrary HTML, arbitrary attributes, custom elements, or inline JavaScript;
- a way to bypass accessibility by writing `aria-*` by hand;
- a reason to modify `spec/elements.toml` unless the task explicitly asks for a vocabulary change.

## Hard constraints

1. `Markup` has no public constructor from `String`. Never write `Markup::from(String)`, `Raw`, `PreEscaped`, or `|safe`.
2. Do not emit document elements inside `html!`: `html`, `head`, `body`, `title`, `meta`, `link`, `script`, `noscript`, and `main` are `Page` territory.
3. Do not emit `<form>` inside `html!`; use `Form::open` or `Form::open_with_submit`.
4. Do not put literal text inside `html!`; use `M::X` for static UI text, `Text::msg(M)` where the API requires `Text`, `Text::new` only for dynamic data, or `(markup)`.
5. Do not use `&str`/`String` for typed attribute contexts; use `Href`, `Src`, `Alt`, `Class`, `Label`, `AttrVal`, `Date`, `Timestamp`, etc.
6. Do not add `on*` attributes, `<style>`, `srcdoc`, custom element attributes, or arbitrary names.
7. `style=` is an allowed escape hatch but not a styling strategy; it is escaped as an attribute, is not CSS-sanitized, and is reported by `audit`.
8. `accesskey` is forbidden. The crate does not provide `Alt+1`; use the skip link and landmarks, or let the consumer implement optional shortcuts.
9. `Audit` is a final-document test behind `feature = "audit"`, not a runtime sanitizer.
10. If an API name is uncertain, inspect `spec/elements.toml`, `crates/html-auy/src/lib.rs`, and the matching `docs/usage/` page instead of inventing one.

## Decision tree

### Full HTML page

Use `ShellData` + `View` + `Page`.

```rust
use html_auy::prelude::*;

#[derive(Clone, Copy, Msg)]
enum M {
    #[msg("Requests")]
    Requests,
    #[msg("Open requests")]
    OpenRequests,
    #[msg("No requests yet.")]
    NoRequests,
    #[msg("Read")]
    Read,
    #[msg("Name")]
    Name,
    #[msg("Send")]
    Send,
    #[msg("As shown on your document")]
    NameHelp,
    #[msg("Episode transcript")]
    Transcript,
    #[msg("5 November 2026")]
    DateLabel,
    #[msg("5 November 2026, 9 a.m.")]
    DateTimeLabel,
    #[msg("Skip to content")]
    Skip,
}

let lang = Lang::parse("en").expect("constant language");
let data = ShellData::new(
    Text::msg(M::Requests),
    Text::msg(M::Requests),
    lang,
)
.skip(Text::msg(M::Skip))
.dir(Dir::Ltr);

let view = View::new(html! {
    section {
        h2 { M::OpenRequests }
        p { M::NoRequests }
    }
});

let page = Page::new(&data, view);
let bytes = page.markup().as_str();
```

- `ShellData::new(title, heading, lang)` requires all three.
- `Page` emits the document, one `<main id="main" tabindex="-1">`, one `<h1>`, and the persistent status region.
- Do not add another `h1` in the view in the normal path.

### Fragment or component

Use `html!` and return `Markup`.

```rust
use html_auy::prelude::*;

fn card(title: &str, href: LUrl<'_>) -> Markup {
    html! {
        article class="card" {
            h2 { Text::new(title) }
            a href=(Href::internal(href)) { M::Read }
        }
    }
}
```

- `html!` validates the local block at compile time.
- Attribute names use `_` in Rust and `-` in HTML.
- Void elements end with `;`.
- Source whitespace does not affect output.

### Text

Use one of three forms:

```rust
html! {
    h2 { M::Title }                 // static translated Msg
    p { Text::new(&value) }         // dynamic external text
    section { (component_markup) }  // already-rendered Markup
}
```

For the static side, define the catalog once:

```rust
#[derive(Clone, Copy, Msg)]
enum M {
    #[msg("Open requests")]
    Title,
    #[msg("No requests yet.")]
    Empty,
}
```

- A string literal inside `html!` is an error.
- `Msg` is the consumer's translated-message enum. `#[derive(Msg)]` with `#[msg("…")]` generates the static table; manual impls remain valid.
- `Text` is escaped text; `AttrVal` is escaped attribute text. Use `Text::msg(M)` where a primitive requires `Text`.
- Never insert `Text` inside `Text`; type errors catch double escaping.

### URLs

```rust
use html_auy::prelude::*;
use html_auy::Fragment;

let internal = Href::internal(
    LUrl::with_segments("/requests", &[&id]).expect("dynamic route"),
);
// Constant internal routes are shorter and still validated:
// `a href="/requests" { … }`, `href="#fragment"`, `src="/asset"`.
let external = Href::external(ExternalUrl::parse("https://example.org").expect("https"));
let fragment = Href::fragment(Fragment::new("#main").expect("fragment"));
```

Use `LUrl` for a dynamically built same-origin path, `ExternalUrl` for absolute `http(s)`, and
`Href` to choose a permitted destination. A constant internal path should be the literal
`href="/…"`/`src="/…"`; external URLs stay typed. Do not use `Href::new(&str)`; it does not exist.
Never pass `javascript:` or `data:`.

### Attributes and classes

```rust
let id: Option<Id<'_>> = Some(Id::new("report"));

html! {
    section id?=(id) class="card card--wide" { }
    button type="button" disabled=(true) { M::Close }
    div data_kind="news" aria_label=(Label::text(M::News)) { }
}
```

- `attr=(value)` requires an `AttrValue` implementation.
- `attr?=(Option<T>)` emits only for `Some`.
- Boolean attributes use presence: `disabled=(true)` emits `disabled`; `false` omits it.
- `class`/`Class`/`ClassList` are sorted and deduplicated.
- `aria_label` requires `Label`, never `&str`.
- `data-*` values are scalar and validated.

### Forms

`<form>` is not in `html!`:

```rust
use html_auy::prelude::*;
use html_auy::{Action, AttrVal, CsrfToken, Field, FieldName, FieldState, Form, Method};

let action = Action::new(LUrl::new("/requests").expect("constant route"));
let token = CsrfToken::new(csrf).expect("non-empty token");

let form = Form::open(
    &action,
    Method::Post(token),
    html! {
        (Field::text(
            FieldName::new("name"),
            M::Name,
            &FieldState::new(AttrVal::new(&state.name))
                .required(true)
                .help(Text::msg(M::NameHelp)),
        ))
        button type="submit" { M::Send }
    },
);
```

- `Method::Post` requires `CsrfToken`; `Put`/`Patch`/`Delete` emit `_method`.
- `Form::open` accepts an internal `Action`, not an arbitrary URL.
- `Field::text` is `Field::input(..., InputType::Text, ...)`.
- `Field::select` returns `Option<Markup>` and is `None` when no option is selectable.
- `Field::file` never emits `value`.
- `FieldState` derives help/error ids and `aria-*`; do not hand-write them.
- `Autocomplete::parse("username webauthn")` is the passkey path.

### Images and media

```rust
use html_auy::prelude::*;
use html_auy::{Img, Src};

let image = Img::new(
    Src::parse("/media/cover.avif").expect("source"),
    Alt::new("Bus at the terminal"),
    WidthPx::new(720),
    HeightPx::new(480),
);
```

- `Img` requires source, alt, width, and height.
- `Alt::decorative()` is the only empty alt.
- Responsive images need both `SrcSet` and `Sizes`.
- `Video::new` requires captions; `Track::descriptions` is audio description.
- `Audio::transcript` and `Video::transcript` wrap the player in `<figure>` with a transcript link.
- `Transcript` is a typed link + label; it is not a `<track>`.

```rust
// Transcript::new takes Href, so a constant path still goes through LUrl here.
let transcript = Transcript::new(
    Href::internal(LUrl::new("/media/episode-transcript").expect("route")),
    Text::msg(M::Transcript),
);
let audio = Audio::new(Src::parse("/media/episode.mp3").expect("source"))
    .transcript(transcript);
```

### Shell composition

- `View::new(main)` carries `<main>` content.
- `.modal(Markup)` appends modals after the footer.
- `.asset(Asset)` adds a required asset.
- `.merge(other)` combines modals/assets; `self`'s main wins.
- `ShellData::regions(Regions)` carries header/before_main/after_main/footer; `Regions::new().header(…).footer(…)` is the builder form.
- `ShellData::help(Markup)` fixes the help slot between header and `<main>`.
- `ShellData::skip(Text)` emits the first-focusable skip link.
- `ShellData::dir(Dir)` sets `<html dir>`; use `Dir::Ltr` explicitly and `Dir::Rtl` for RTL locales.
- `Assets` preserve order; duplicates do not move position. `collect::<Assets>()` accepts `Option<Asset>` and skips `None`; `Assets::extend_options([...])` is the incremental form.
- Theme/CSS is consumer-owned; the crate does not emit CSS or resolve tokens.

### Dates

```rust
let date = Date::new(2026, 11, 5).expect("fixed date");
let instant = Timestamp::parse_rfc3339("2026-11-05T09:00:00-03:00").expect("RFC 3339");

html! {
    time datetime=(date) { M::DateLabel }
    time datetime=(instant) { M::DateTimeLabel }
}
```

- `Date` is date-only and does not invent time or offset.
- `Timestamp` preserves the RFC 3339 offset; fractional seconds are rejected.
- The visible date label belongs to the message catalog; the crate emits the machine value.

### JSON-LD

```rust
let ld = JsonLd::graph(&article); // article implements JsonGraph
let data = ShellData::new(title, heading, lang).schemas(vec![ld]);
```

- `JsonLd` serializes through the consumer's `JsonGraph`.
- It escapes `<`, `>`, `&`, `'` and Unicode line separators.
- Do not use a generic `<script>` or string-concatenated JSON.

## Correctness rules for LLMs

### Never invent these

| Wrong API | Correct path |
|---|---|
| `Markup::from(String)` | `Text`, `Markup::from_render`, or an existing renderer |
| `Raw`, `PreEscaped`, `|safe` | no equivalent; use typed values |
| `Href::new(&str)` | `Href::internal/external/fragment/mailto/tel` |
| `html! { form { ... } }` | `Form::open` |
| `html! { main { ... } }` | `Page` |
| `html! { "literal" }` | `Msg` or `Text::new` |
| `class=(String)` / `href=(String)` | `Class`/`ClassList` / `Href` |
| `disabled="false"` | `disabled=(false)` (presence semantics) |
| `style` as normal styling | `Class`/token; `style=` only as audited escape hatch |
| `accesskey="1"` / `Alt+1` | skip link + landmarks; consumer-owned shortcut |
| `audit` as sanitizer | `audit::check` as a test under `feature = "audit"` |
| `Video::new` without captions | `Video::new(src, Track::captions(...))` |
| `Track::captions` as an audio transcript | `Audio::transcript(Transcript)` |

### Always prefer

- Types over strings: `Href`, `LUrl`, `Src`, `Alt`, `Text`, `AttrVal`, `Label`, `Class`, `Date`, `Timestamp`, `Autocomplete`.
- Primitives over hand-written markup: `Field`, `Img`, `Picture`, `Video`, `Audio`, `Form`, `Page`.
- `Page` for documents; `html!` for fragments.
- `Msg` for UI text; `Text` for data; `(markup)` for composed markup; `Markup::concat([a, b])` for sibling fragments.
- `audit::check` in tests for the assembled page.
- `cargo test -p html-auy --features audit` when changing document-wide behavior.

### Preserve canonical output

- Do not depend on `HashMap` order, locale, source indentation, or attribute declaration order.
- Do not sort or rewrite canonical class/asset output yourself.
- Add tests for exact bytes when changing rendering behavior.
- If a change affects a public primitive, update its golden/trybuild/audit coverage.

### Accessibility is not optional

- `Img` must have `Alt`; decorative images use `Alt::decorative()`.
- `Video` must have captions.
- Use `Track::descriptions` for audio description and `Transcript` for textual alternatives; do not claim one substitutes for the other.
- Use `ShellData::skip` and `ShellData::help` for consistent page-level affordances.
- Use `Field`/`FieldState` instead of hand-writing labels, ids, `aria-describedby`, and `aria-invalid`.
- `aria-current` is unique per nearest `<nav>`; `false` is not a current item.
- Do not add `accesskey`; do not implement `Alt+1` as a crate feature.
- Computed contrast, target size, focus visibility, and tab order are consumer gates, not claims of this crate.

### Security boundaries

- Never write trusted HTML from foreign data through `Render`/`AttrValue` without escaping.
- `style=` is attribute-escaped but not CSS-sanitized; it needs explicit CSP handling.
- `JsonLd` is the only JSON/script-data path; it escapes the dangerous characters.
- `LUrl`/`ExternalUrl`/`Src` constrain URLs; do not bypass them with raw attributes.
- `Render` implementations write bytes; they are trust boundaries, not sanitizers.

## Recommended implementation sequence

1. Model the data and the view function; return `Markup`.
2. Choose the typed values and primitives before writing `html!`.
3. Use `Page`/`ShellData` only at the document boundary.
4. Use `Form::open`/`Field` for forms and `Img`/`Video`/`Audio` for media.
5. Add `audit` tests for full pages.
6. Run `cargo fmt`, `cargo clippy`, `cargo test`, and the audit-feature tests.
7. If changing the vocabulary or invariant behavior, regenerate/inspect the spec tables and run `tools/validate_spec.py`.

## Minimal review checklist

- [ ] Does it compile with the real public API and imports?
- [ ] Is text using `Msg`/`Text`, never a literal in `html!`?
- [ ] Are URLs/attrs/classes typed rather than strings?
- [ ] Are document elements and `<form>` outside `html!`?
- [ ] Does each image have `Alt`, dimensions, and a deliberate decorative choice if empty?
- [ ] Does each video have captions; is audio description/transcript handled correctly?
- [ ] Are labels/ids/help/errors derived by `Field` instead of hand-written?
- [ ] Is the skip link/help/status region owned by the shell where appropriate?
- [ ] Are `on*`, inline `<script>`, `<style>`, `srcdoc`, and `accesskey` absent?
- [ ] Is every `style=` justified and audited?
- [ ] Does the final document pass `audit` with `feature = "audit"`?
- [ ] Were golden/trybuild/audit tests updated for behavior changes?

## Verification commands

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test -p html-auy --features audit
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
python3 tools/gen_spec.py --check
python3 tools/validate_spec.py
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

For MSRV, the CI target is:

```bash
cargo +1.92 test --workspace --lib --locked
```

## References

- `README.md`
- `llms.txt`
- `llms-full.txt`
- `docs/design/acessibility.md`
- `docs/design/04-validacao.md`
- `docs/design/05-acessibilidade.md`
- `docs/usage/03-macro-html.md`
- `docs/usage/06-formularios.md`
- `docs/usage/07-midia.md`
- `docs/usage/08-shell-assets-e-tema.md`
- `spec/elements.toml`
