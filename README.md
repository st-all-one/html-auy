# html-auy

**A typed HTML vocabulary for Rust: compile-time attribute validation, context-aware escaping, and
accessibility by construction.**

`html-auy` renders HTML as an artifact, not as a projection of a reactive tree. The public crate is
[`html-auy`](https://crates.io/crates/html-auy); its `html!` macro is implemented by
[`html-auy-macros`](https://crates.io/crates/html-auy-macros) and
[`html-auy-macros-core`](https://crates.io/crates/html-auy-macros-core).

```toml
[dependencies]
html-auy = "0.1"
```

```rust
use html_auy::{Href, LUrl, Text, html};

let link = html! {
    a href=(Href::internal(LUrl::new("/news").expect("constant route"))) {
        (Text::new("News"))
    }
};

assert_eq!(link.as_str(), r#"<a href="/news">News</a>"#);
```

The macro rejects unknown elements and attributes, missing `alt` on `img`, controls without an
accessible name, invalid nesting, and other page vocabulary errors at compile time. Values such as
`Href`, `Src`, `SrcSet`, `Text`, and `JsonLd` choose the escaping context; `Markup` has no public
constructor from `String`.

## Why

Every family of HTML generation in Rust leaves at least one of these as a runtime or review problem:

| Family | Representatives | What is left unchecked |
|---|---|---|
| macro → `Markup`/`String` | maud, horrorshow, markup.rs | attribute names and per-context escaping |
| compile-time file templates | askama, sailfish | escape hatches such as `\|safe`; rigid composition |
| macro → reactive tree/VDOM | leptos, yew, dioxus, sycamore, topcoat | the HTML stops being the artifact |
| runtime templates over `Value` | minijinja, tera, handlebars | no typology; failures reach the request |

`html-auy` occupies a different quadrant:

> **typed props + non-reactive `Markup` + attributes validated at compile time + accessibility by
> construction**

The design came from a municipal portal with 222 views, 6,765 tags, 111 declared elements, WCAG 2.2
AA, a CSP without inline script, ETag/304, and a no-JavaScript baseline. The measurements behind
each decision are in [`docs/15-migracao-blade.md`](docs/15-migracao-blade.md).

## Guarantees

1. **The HTML is the artifact.** Output is deterministic bytes, with no VDOM, hydration markers, or
   serialized state.
2. **Escape context is a type.** `Href`, `Src`, `Text`, `JsonLd`, and `SrcSet` carry the context;
   there is no public raw/`PreEscaped` escape hatch.
3. **Accessibility is absence of API.** `img` without `Alt` does not compile; naming a `div` does
   not create a landmark; components such as `Form` and `Field` derive their semantics.
4. **Errors are compile errors.** The macro reports actionable spans, and its messages are covered
   by `trybuild`.
5. **Zero `unsafe`, zero runtime, zero embedded JavaScript.** The core is `no_std` + `alloc` and
   does no I/O, clock, environment, or locale lookup.
6. **Bytes are canonical.** Attribute, class, `srcset`, theme-token, and asset order are stable, so
   `ETag` is trustworthy.

## Non-goals

- It is not a web framework: no HTTP, database, session, domain i18n, or routing.
- It does not accept runtime templates, HTML from a database, or `on*` attributes. `style=` exists
  only as an audited escape hatch; `<style>` and inline event handlers remain forbidden.
- It does not replace `axe`, `sniffCSS`, or keyboard tests; it reduces the cost of reaching them.

Details: [`docs/00-principios.md`](docs/00-principios.md) and
[`docs/14-alternativas.md`](docs/14-alternativas.md).

## Crates

| Package | Role |
|---|---|
| [`html-auy`](crates/html-auy) | Public library: values, `Markup`, shell, components, optional `audit`. |
| [`html-auy-macros`](crates/html-auy-macros) | The `html!` procedural macro. |
| [`html-auy-macros-core`](crates/html-auy-macros-core) | Parser, validator, and code generator; testable and fuzzable without a compiler plugin. |

Only `html-auy` is meant as an application dependency. The other two are published because the
macro needs them.

## Status and compatibility

`0.1.0` is the first public release. The project follows semantic versioning for the public
`html-auy` API: a change that makes previously valid code stop compiling is a major release (for
`0.x`, a minor bump), and the policy is documented in
[`docs/11-estabilidade.md`](docs/11-estabilidade.md).

- **MSRV:** Rust 1.92, edition 2024. The development toolchain is pinned separately in
  `rust-toolchain.toml`; CI compiles and tests the MSRV.
- **Features:** `audit` enables `html_auy::audit::check`, a final-document pass backed by
  `html5ever`. It is not part of the default dependency tree.
- **Licence:** MIT OR Apache-2.0.

## Verifying the repository

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test -p html-auy --features audit
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
python3 tools/validate_spec.py
python3 tools/gen_spec.py --check
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

Releases are created by pushing a `v*` tag; `.github/workflows/release.yml` verifies the tag against
`Cargo.toml`, publishes the three crates in dependency order, and creates the GitHub Release without
binary artifacts.

## Repository map

| Path | Contents |
|---|---|
| [`crates/`](crates) | The three public crates. |
| [`spec/elements.toml`](spec/elements.toml) | Normative element/attribute/invariant table. |
| [`spec/data.toml`](spec/data.toml) | Consumer `data-*` table used by `tools/data_check.py`. |
| [`tools/`](tools) | Table generator, spec validator, link checker, and their tests. |
| [`CHANGELOG.md`](CHANGELOG.md) | Public release history. |
| [`docs/`](docs) | Design, decisions, accessibility, security, testing, and migration notes. |
| [`demo/`](demo) | Side-by-side html-auy/askama/maud examples (not part of the release). |
| [`fuzz/`](fuzz) | Parser fuzz target (separate workspace, not published). |

## Documentation

| Document | Subject |
|---|---|
| [`docs/usage/`](docs/usage/README.md) | **usage wiki:** philosophy, install, pages, macro, values, forms, media, shell, security, accessibility, HTTP, recipes |
| [`docs/00-principios.md`](docs/00-principios.md) | problem, principles, non-goals, glossary |
| [`docs/01-arquitetura.md`](docs/01-arquitetura.md) | the three crates, boundaries, request flow |
| [`docs/02-tipos-e-contextos.md`](docs/02-tipos-e-contextos.md) | `Markup`, typed values, escaping |
| [`docs/03-vocabulario-de-elementos.md`](docs/03-vocabulario-de-elementos.md) | `spec/elements.toml`, per-element invariants |
| [`docs/04-validacao.md`](docs/04-validacao.md) | validation levels and their limits |
| [`docs/05-acessibilidade.md`](docs/05-acessibilidade.md) | WCAG 2.2 AA → mechanism |
| [`docs/acessibility.md`](docs/acessibility.md) | Portuguese accessibility architecture: guarantees, audit rules, and consumer gates |
| [`docs/06-shell-e-tema.md`](docs/06-shell-e-tema.md) | shell, theme tokens, `/theme.css` |
| [`docs/07-js-e-assets.md`](docs/07-js-e-assets.md) | JavaScript, bundles, CSP, JSON-LD |
| [`docs/08-seguranca.md`](docs/08-seguranca.md) | HTML threat model, URLs, supply chain |
| [`docs/09-desempenho-e-cache.md`](docs/09-desempenho-e-cache.md) | allocation, ETag, compression |
| [`docs/10-ergonomia.md`](docs/10-ergonomia.md) | diagnostics, formatting, anti-noise |
| [`docs/11-estabilidade.md`](docs/11-estabilidade.md) | semver, MSRV, spec freezing, release |
| [`docs/12-testes.md`](docs/12-testes.md) | golden, property, fuzz, conformance |
| [`docs/13-decisoes.md`](docs/13-decisoes.md) | decision records |
| [`docs/14-alternativas.md`](docs/14-alternativas.md) | comparison with nine alternatives |
| [`docs/15-migracao-blade.md`](docs/15-migracao-blade.md) | measured migration notes |
| [`docs/16-roadmap.md`](docs/16-roadmap.md) | milestones and kill criteria |
| [`docs/17-faq.md`](docs/17-faq.md) | difficult questions |
| [`docs/18-aprendizados/`](docs/18-aprendizados/README.md) | technical corpus reading notes |
| [`docs/19-revisao-arquitetural.md`](docs/19-revisao-arquitetural.md) | architectural review |
| [`docs/20-fronteira-de-aplicacao.md`](docs/20-fronteira-de-aplicacao.md) | application-boundary patterns |
| [`docs/21-implementacao.md`](docs/21-implementacao.md) | implementation plan and file map |
| [`docs/22-rigor.md`](docs/22-rigor.md) | lint, supply-chain, and CI policy |
| [`docs/23-acessibilidade-e-css.md`](docs/23-acessibilidade-e-css.md) | decidable CSS/accessibility rules |
| [`docs/24-exportacao.md`](docs/24-exportacao.md) | post-1.0 export notes |
| [`docs/25-exemplo-de-uso.md`](docs/25-exemplo-de-uso.md) | end-to-end usage example |
| [`docs/26-d29-consumidor.md`](docs/26-d29-consumidor.md) | consumer-owned theme, CSS, and ETag |
| [`docs/27-licoes-do-eta.md`](docs/27-licoes-do-eta.md) | lessons absorbed from the `eta` engine |

## License

Licensed under **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)** at your option.

Repository: <https://github.com/st-all-one/html-auy>.
