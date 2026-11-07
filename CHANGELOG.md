# Changelog

All notable changes to `html-auy` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow the policy in
[`docs/11-estabilidade.md`](docs/11-estabilidade.md).

## [0.1.1] - 2026-11-07

Lint and supply-chain hardening only. No public API or emitted-byte change.

### Changed

- Workspace lints now enforce `disallowed_names`, `fn_params_excessive_bools`,
  `struct_excessive_bools`, `exit`, `absolute_paths`, `let_underscore_untyped`,
  `await_holding_*`, and enum/array/error size checks.
- `clippy.toml` caps absolute paths at three segments and documents the active test exceptions.
- `deny.toml` bans `tokio`, `reqwest`, `hyper`, and `axum` so the `no_std` crate cannot acquire an
  HTTP runtime by accident.

## [0.1.0] - 2026-11-07

First public release. This release incorporates all work completed through the `0.1.0` preparation:
the typed vocabulary, shell, forms, media, accessibility, audit, documentation, agent context, and
dual licensing.

### Added

- `html!` procedural macro with compile-time element, attribute, content-model, accessibility, and
  form validation.
- Typed escaping values: `Text`, `AttrVal`, `Href`, `Src`, `SrcSet`, `JsonLd`, `Alt`, `Class`, and
  related media/URL types. `Autocomplete` includes `username webauthn` and `webauthn`.
- `Markup`/`Render` with deterministic, canonical bytes.
- Shell types: `Page`, `ShellData`, `View`, `Regions`, `Assets`, `RootAttributes`, hints, and
  Open Graph/alternate metadata.
- `ShellData::help` for a consistent help block between header and `<main>`.
- Shell assets preserve declaration order, deduplicate without moving, and support classic/module
  scripts plus explicit external/CDN URLs.
- Components: `Form`/`CsrfToken`/`Method`, `Field`/`FieldState`, `Img`/`Picture`/`Video`/`Audio`/
  `Track`, and response helpers.
- Media accessibility: `Transcript`, `Audio::transcript`, `Video::transcript`, and
  `Track::descriptions` for textual alternatives and audio description.
- `Class` for static design-system tokens, `ClassList` for opt-in dynamic tokens, and `Tone`/
  `ClassToken` integration.
- Optional `audit` feature backed by `html5ever` for final-document checks, including
  `aria_current_unique_per_nav`.
- Specific `accesskey` diagnostic pointing to the skip link instead of `Alt+1`.
- `popover="hint"` in the global vocabulary.
- Public packages `html-auy`, `html-auy-macros`, and `html-auy-macros-core`.
- Tag-driven release workflow that publishes to crates.io in dependency order and creates a GitHub
  Release without binary artifacts.
- Documentation and agent context: usage wiki, `docs/acessibility.md`, `llms.txt`,
  `llms-full.txt`, `SKILL.md`, expanded `AGENTS.md`, and contributor guidance for `0.1.0`.

### Changed

- The project is dual-licensed under `MIT OR Apache-2.0`; both `LICENSE-MIT` and `LICENSE-APACHE`
  are shipped at the workspace root and in each published crate.
- Repository attributes normalize text to LF and classify documentation, generated tables, Cargo
  metadata, and binary assets consistently.
