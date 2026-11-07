# Changelog

All notable changes to `html-auy` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow the policy in
[`docs/11-estabilidade.md`](docs/11-estabilidade.md).

## [Unreleased]

### Added

- `Transcript`, `Audio::transcript`, and `Video::transcript` for WCAG 1.2.1/1.2.3 alternatives;
  `Track::descriptions` for WCAG 1.2.3/1.2.5 audio description.
- `ShellData::help` for a consistent help block (WCAG 3.2.6).
- `Autocomplete` accepts `username webauthn` and `webauthn` (accessible authentication / passkeys).
- `popover="hint"` in the global vocabulary.
- `audit::check` rule `aria_current_unique_per_nav`.
- Specific `accesskey` diagnostic pointing to the skip link instead of `Alt+1`.

## [0.1.0] - 2026-11-07

First public release.

### Added

- `html!` procedural macro with compile-time element, attribute, content-model, accessibility, and
  form validation.
- Typed escaping values: `Text`, `AttrVal`, `Href`, `Src`, `SrcSet`, `JsonLd`, `Alt`, `Class`, and
  related media/URL types.
- `Markup`/`Render` with deterministic, canonical bytes.
- Shell types: `Page`, `ShellData`, `View`, `Regions`, `Assets`, `RootAttributes`, hints, and
  Open Graph/alternate metadata.
- Shell assets preserve declaration order, deduplicate without moving, and support classic/module
  scripts plus explicit external/CDN URLs.
- Components: `Form`/`CsrfToken`/`Method`, `Field`/`FieldState`, `Img`/`Picture`/`Video`/`Audio`/
  `Track`, and response helpers.
- `Class` for static design-system tokens, `ClassList` for opt-in dynamic tokens, and `Tone`/
  `ClassToken` integration.
- Optional `audit` feature backed by `html5ever` for final-document checks.
- Public packages `html-auy`, `html-auy-macros`, and `html-auy-macros-core`.
- Tag-driven release workflow that publishes to crates.io in dependency order and creates a GitHub
  Release without binary artifacts.
