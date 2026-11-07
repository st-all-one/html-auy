# Accessibility in `html-auy`

This is the English companion to [`05-acessibilidade.md`](05-acessibilidade.md). It describes the accessibility model implemented by the crate, the boundary between compile-time guarantees and consumer-owned browser gates, and the concrete APIs an application should use.

The target is **WCAG 2.2 AA** for the project context, with eMAG concerns covered through the same mechanisms. Accessibility is not a post-render checklist: it is emitted by the vocabulary and by primitives, then checked again over the final document where the local compiler cannot see the whole page.

## 1. Strategy

The project uses four layers, in decreasing order of strength:

1. **Absence of API / type-level requirements.** The wrong thing cannot be constructed: `img` without `Alt`, `Video::new` without captions, a state-changing `Form` without `CsrfToken`.
2. **Primitive defaults and macro validation.** `Field`, `Img`, `Page`, `ShellData`, and `Form` emit the plumbing; `html!` rejects invalid nesting, missing required attributes, unresolved `label for`, and forbidden attributes in the local block.
3. **Final-document audit (`N4`).** `html_auy::audit::check` runs over the assembled HTML behind `feature = "audit"`. It catches document-wide problems that no fragment can decide: duplicate ids, multiple `h1`s, skipped heading levels, landmark name collisions, unresolved ARIA references, and repeated `aria-current`.
4. **Consumer browser gates (`N5`).** `axe`, `sniffCSS`, keyboard tests, and end-to-end tests cover computed style, contrast, target size, focus order, occlusion, and post-swap behavior.

There is deliberately no advisory lint layer. A rule either makes valid code fail to compile, or it is a test that fails in CI. A warning that fires twenty times per page is not treated as an accessibility guarantee.

## 2. What the crate guarantees

### 2.1 Images and non-text content

- `Img::new` requires `Src`, `Alt`, `WidthPx`, and `HeightPx`.
- `Alt::new("")` means explicit decoration; a missing or accidental empty alt does not exist as a path.
- Decorative SVG has a typed path; informative SVG requires a name.
- Intrinsic dimensions are required to prevent layout shift and empty reserved space.

```rust
let image = Img::new(
    Src::parse("/media/cover.avif").expect("valid source"),
    Alt::new("Bus at the terminal"),
    WidthPx::new(720),
    HeightPx::new(480),
);
```

### 2.2 Forms and accessible names

`Field` derives the pieces that are normally forgotten:

- `<label for="...">` and the input `id`;
- `aria-describedby` for help and error spans;
- `aria-invalid="true"` when an error exists;
- `required` when required;
- the required `*` outside the `<label>`, so it does not enter the accessible name;
- a group `<fieldset>` + `<legend>` for checkbox/radio groups.

```rust
let name = FieldState::new(AttrVal::new(&state.name))
    .required(true)
    .help(Text::new("As shown on your document"))
    .error(Text::new("Enter the full name"));

let field = Field::text(FieldName::new("name"), M::Name, &name);
```

Rules that follow from the local block:

- a control without an accessible name does not compile;
- `aria-hidden="true"` on a focusable control is rejected;
- `label for` must resolve to an `id` in the same block;
- `th` requires `scope`;
- `button` requires `type`;
- `autocomplete` only comes from `Autocomplete::parse`; it is a closed vocabulary, not a free string.

Passkey/authentication support is typed:

```rust
let state = FieldState::new(AttrVal::new(""))
    .autocomplete(Autocomplete::parse("username webauthn").expect("valid token"));

let password = Field::input(
    FieldName::new("user"),
    M::User,
    InputType::Password,
    &state,
);
```

The crate emits the accessible form contract; the consumer owns the login flow, CAPTCHA policy, paste behavior, and passkey ceremony.

### 2.3 Page structure, skip link, and help

`Page` is the only document constructor. It emits:

- `<!DOCTYPE html>`, `<html lang>`, `<head>`, `<body>`;
- `<title>` from required `ShellData`;
- one `<main id="main" tabindex="-1">`;
- one persistent `<div role="status" aria-atomic="true"></div>`;
- one `<h1>` from `ShellData` in the normal path;
- the skip link when `ShellData::skip` is provided.

The skip link is the first focusable element:

```rust
let data = ShellData::new(
    Text::new("Requests"),
    Text::new("Requests"),
    Lang::parse("en").expect("language"),
)
.skip(Text::new("Skip to content"));
```

Consistent help is a fixed shell slot:

```rust
let help = html! {
    p { Text::new("Need help? Contact the service desk.") }
};

let data = ShellData::new(title, heading, lang).help(help);
```

`help` is emitted after the header and before `before_main`/`<main>`, so it occupies the same structural location on every page that provides it. The consumer owns the content and whether the slot exists.

### 2.4 Landmarks, headings, and `aria-current`

- `Regions` owns header, before-main, after-main, and footer contributions.
- With two or more `nav`/`aside` landmarks, each needs a distinct accessible name.
- `audit` enforces `landmark_names_distinct` and `landmark_names_required`.
- `Page` emits one `h1`; views normally use `h2`–`h6`; `audit` enforces exactly one `h1` and no skipped heading levels.
- `aria-current` is unique within each nearest `<nav>`; separate navigations can each have their own current item. The audit rule ignores `aria-current="false"`.

```rust
let mut regions = Regions::new();
regions.header = Some(html! {
    header {
        nav aria_label=(Label::text(M::MainNav)) {
            a href=(home) aria_current="page" { M::Home }
        }
    }
});
```

### 2.5 Live status

The shell always emits a persistent empty status region:

```html
<div role="status" aria-atomic="true"></div>
```

The consumer fills it through swaps or scripted updates. A non-persistent region inserted after the update may not be announced reliably. Do not add a second `aria-live` to a `role="status"`/`role="alert"`; the role already implies the live behavior.

### 2.6 Media alternatives

The media vocabulary encodes the distinction between captions, audio description, and a full textual alternative:

- `Video::new(src, Track::captions(...))` requires captions for prerecorded video (WCAG 1.2.2).
- `Track::descriptions(src, label)` is the typed audio-description track (WCAG 1.2.3/1.2.5).
- `Video::transcript(Transcript)` is a complete textual alternative for video when no description track exists; it is not a substitute for the descriptive track required by 1.2.5.
- `Audio::transcript(Transcript)` is the textual alternative for prerecorded audio (WCAG 1.2.1). A `Track` is not a transcript for audio because audio has no visual caption area.

```rust
use html_auy::{Audio, Href, LUrl, Src, Text, Transcript};

let transcript = Transcript::new(
    Href::internal(LUrl::new("/media/episode-transcript").expect("route")),
    Text::new("Episode transcript"),
);

let audio = Audio::new(Src::parse("/media/episode.mp3").expect("source"))
    .transcript(transcript);
```

`Audio::transcript` and `Video::transcript` wrap the media in `<figure>` and add a `<figcaption>` containing the transcript link. The figure is the semantic relationship between the media and its alternative.

### 2.7 ARIA and attributes

- `aria_label` accepts `Label` (`Label::text`/`Label::sr_only`), not `&str`.
- `role` is an enum per element, not a free string.
- `aria-checked`, `aria-expanded`, and `aria-current` are allowed only where the role permits them.
- `aria-label` over visible text is rejected when the name diverges.
- `data-*` is scalar and validated; do not serialize JSON into an attribute.
- `accesskey` is forbidden: keyboard activation differs across browsers and screen readers and can collide with native shortcuts.
- `popover="hint"` is a declared global value for tooltip/hint surfaces; `popovertarget` resolution is checked by the audit.
- `draggable` is forbidden; drag interactions need explicit button/keyboard alternatives.

### 2.8 `Alt+1` and custom shortcuts

The crate does not implement `Alt+1`. It provides the skip link and landmark navigation. `accesskey` is intentionally forbidden. If a product requires `Alt+1`, that is a consumer-level keyboard listener or platform decision, not an HTML vocabulary feature. The crate cannot guarantee activation across browser/screen-reader combinations.

## 3. Document-wide audit

Enable `audit` in tests or CI:

```toml
[dev-dependencies]
html-auy = { version = "0.1", features = ["audit"] }
```

```rust
let findings = html_auy::audit::check(page.markup().as_str());
assert!(findings.is_empty(), "{findings:#?}");
```

`Finding` exposes the rule, message, and node information; output is deterministic and follows document order with rule-order tie-breaking.

The 13 rules are:

| Rule | What it catches |
|---|---|
| `main_unique` | zero or multiple `<main>` elements |
| `single_h1` | zero or multiple `<h1>` elements |
| `heading_levels_no_skips` | `h2` → `h4` and similar skips |
| `ids_unique` | duplicate `id` |
| `landmark_names_distinct` | same-type landmarks with the same name |
| `landmark_names_required` | repeated `nav`/`aside` without an accessible name |
| `aria_current_unique_per_nav` | more than one current `aria-current` in the same `<nav>`; `false` is ignored |
| `aria_refs_resolve` | `aria-labelledby`/`describedby`/`controls` pointing to a missing id |
| `popover_target_resolves` | `popovertarget` without a matching `popover` |
| `no_content_derived_identifiers` | `id`/`name` outside the stable generated alphabet |
| `no_inline_script` | `<script>` without `src` and not `application/ld+json` |
| `no_style_attribute` | any `style=` |
| `no_event_handler` | any `on*` |

`audit` is not a sanitizer. It is a final-document test. It does not prove that alt text is useful, that a link name is descriptive in context, or that the rendered layout is usable.

## 4. What remains a consumer gate

The crate cannot measure or guarantee:

- contrast against composited backgrounds;
- target size in CSS pixels;
- visible focus, focus not obscured, or focus order inside dialogs/popovers;
- text resize/reflow and text spacing;
- orientation restrictions;
- animation, `prefers-reduced-motion`, `forced-colors`, `prefers-contrast`, or reduced transparency;
- occlusion, sticky headers covering focused elements, or transform/overflow edge cases;
- post-swap htmx behavior;
- the quality, not merely presence, of alt text and link text.

Use `axe`, `sniffCSS`, browser keyboard tests, and content review for those. The project provides mechanisms and a document audit, not a browser.

## 5. Testing accessibility

The repository tests the mechanisms at several levels:

- `trybuild` UI tests for rejected markup and actionable diagnostics;
- golden/snapshot tests for canonical HTML output;
- unit and property tests for value invariants;
- parser fuzzing outside the main workspace;
- `audit` tests over complete pages with `feature = "audit"`;
- spec validation and docs traceability in `tools/validate_spec.py`;
- consumer-side `axe`/`sniffCSS`/keyboard/e2e gates.

A code change that alters an accessibility primitive should update the corresponding compile-time test, golden output, and audit behavior. A new rule that only the final document can decide belongs in `audit`, not in `html!`.

## 6. Design rules

1. **Correct by default.** The primitive emits the plumbing; the developer chooses the component, not the ARIA.
2. **Visible escape hatches.** `Alt::decorative()`, sr-only labels, and explicit transcript links make the exception explicit.
3. **No silent advisory.** A rule either fails compilation or fails a test.
4. **One source of document structure.** `Page` owns the document; views own the `<main>` content; regions own landmarks.
5. **Noisy rule = primitive bug.** If a rule fires repeatedly on one page, fix the primitive instead of asking every view to work around it.
6. **The boundary is stated.** If the crate cannot observe it, the consumer must gate it.

## 7. Related docs

- [`05-acessibilidade.md`](05-acessibilidade.md) — full Portuguese WCAG 2.2 AA/eMAG mapping.
- [`04-validacao.md`](04-validacao.md) — validation levels and invariant scopes.
- [`23-acessibilidade-e-css.md`](23-acessibilidade-e-css.md) — what is decidable in the crate versus browser CSS.
- [`usage/11-acessibilidade.md`](usage/11-acessibilidade.md) — operational accessibility guide.
- [`12-testes.md`](12-testes.md) — test layers and CI gates.
