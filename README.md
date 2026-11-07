<div align="center">

# html-auy

Vocabulário HTML tipado para Rust — atributos validados em compilação, escape por contexto e acessibilidade por construção.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2b3a42?style=for-the-badge)](./LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.92%2B-DEA584?style=for-the-badge&logo=rust&logoColor=000)](https://www.rust-lang.org/)
[![crates.io](https://img.shields.io/badge/crates.io-0.1.1-009739?style=for-the-badge)](https://crates.io/crates/html-auy)
[![Version](https://img.shields.io/badge/version-0.1.1-6E56CF?style=for-the-badge)](CHANGELOG.md)

</div>

O `html-auy` é uma biblioteca Rust que trata HTML como **artefato**, não como projeção de uma árvore reativa. O macro `html!` valida elementos, atributos, aninhamento e semântica em tempo de compilação; valores tipados escolhem o escape; a saída é determinística e acessível por construção. Não há VDOM, hidratação, template de runtime, JavaScript embutido ou dependência de framework HTTP.

```toml
[dependencies]
html-auy = "0.1"
```

```rust
use html_auy::{html, Lang, Page, ShellData, Text, View};

let data = ShellData::new(
    Text::new("Início"),
    Text::new("Início"),
    Lang::parse("pt-BR").expect("idioma"),
);
let view = View::new(html! {
    p { Text::new("Olá, mundo.") }
});
let page = Page::new(&data, view);
```

O crate publicado é [`html-auy`](https://crates.io/crates/html-auy). O macro é implementado por [`html-auy-macros`](https://crates.io/crates/html-auy-macros) e [`html-auy-macros-core`](https://crates.io/crates/html-auy-macros-core).

## Instalação

Requisitos: **Rust 1.92+**, edição 2024, `no_std` + `alloc`. O toolchain de desenvolvimento é fixado separadamente em `rust-toolchain.toml`; a MSRV é testada no CI.

```bash
cargo add html-auy
```

Para habilitar o passe de documento inteiro:

```toml
[dependencies]
html-auy = { version = "0.1", features = ["audit"] }
```

`audit` adiciona `html5ever` + `markup5ever_rcdom` e expõe `html_auy::audit::check`. Ele fica fora da árvore de dependências padrão.

## Uso

```rust
use html_auy::{Href, LUrl, Text, html};

let link = html! {
    a href=(Href::internal(LUrl::new("/news").expect("rota constante"))) {
        (Text::new("Notícias"))
    }
};

assert_eq!(link.as_str(), r#"<a href="/news">Notícias</a>"#);
```

- Texto estático traduzido usa `Msg`; texto dinâmico usa `Text::new`; markup já renderizado entra como `(markup)`.
- Atributos tipados usam `attr=(valor)`; opcionais usam `attr?=(Option<T>)`.
- Booleanos usam presença (`disabled=(true)`), nunca `disabled="false"`.
- `class` é canônica: ordenada e sem duplicatas.
- `Form::open` é a única porta para `<form>`; `Field` deriva `label`, `id`, `aria-describedby`, `aria-invalid` e `required`.
- `Page` é a única porta para `html`, `head`, `body`, `main` e `<title>`.

## Garantias

- **HTML é o artefato.** Bytes determinísticos, sem VDOM, hidratação ou estado serializado.
- **Escape é tipo.** `Text`, `AttrVal`, `Href`, `Src`, `SrcSet` e `JsonLd` carregam o contexto; `Markup` não nasce de `String`.
- **Acessibilidade por ausência de API.** `img` sem `Alt` não compila; `Video::new` sem legendas não compila; `Field` emite a semântica.
- **Erros são de compilação.** Diagnósticos com span e mensagens cobertas por `trybuild`.
- **Sem runtime.** `no_std` + `alloc`, zero `unsafe`, zero JavaScript embutido, zero I/O.
- **Bytes canônicos.** Ordem de atributos, classes, `srcset`, assets e datas é estável; `ETag` é confiável.

O crate **não é** framework web, roteador, ORM, template de runtime, sanitizador de HTML de banco ou substituto de `axe`, `sniffCSS` e testes de teclado. Ele reduz o custo de chegar a esses gates; não os substitui.

## Crates

| Pacote | Papel |
|---|---|
| [`html-auy`](crates/html-auy) | Biblioteca pública: valores, `Markup`, shell, componentes e `audit` opcional. |
| [`html-auy-macros`](crates/html-auy-macros) | Proc-macro `html!`. |
| [`html-auy-macros-core`](crates/html-auy-macros-core) | Parser, validação e geração de código; testável e fuzzável sem plugin de compilador. |

Aplicações dependem apenas de `html-auy`. Os outros dois são publicados porque a proc-macro precisa deles.

## Acessibilidade

Acessibilidade é parte do vocabulário, não uma revisão final:

- `Img` exige `Alt` e dimensões; decoração é explícita com `Alt::decorative()`.
- `Video::new` exige `Track::captions`; `Track::descriptions` cobre audiodescrição.
- `Transcript`, `Audio::transcript` e `Video::transcript` cobrem alternativas textuais.
- `Field` deriva label, ids, ajuda, erro e `aria-*`; `Autocomplete` aceita passkeys (`username webauthn`).
- `Page` emite skip link, `<main>`, região viva persistente e o slot `ShellData::help`.
- `audit` cobre 13 regras de documento, incluindo `aria_current_unique_per_nav`.
- Contraste, alvo de toque, foco visível e layout continuam gates do consumidor (`axe`, `sniffCSS`, e2e).

Detalhes: [`docs/acessibility.md`](docs/acessibility.md) e [`docs/05-acessibilidade.md`](docs/05-acessibilidade.md).

## Documentação

| Tema | Referência |
|---|---|
| Wiki de uso | [`docs/usage/README.md`](docs/usage/README.md) |
| Guia para agentes/LLM | [`SKILL.md`](SKILL.md) |
| Índice para LLM | [`llms.txt`](llms.txt) |
| Contexto completo para LLM | [`llms-full.txt`](llms-full.txt) |
| Acessibilidade | [`docs/acessibility.md`](docs/acessibility.md) · [`docs/05`](docs/05-acessibilidade.md) |
| Arquitetura | [`docs/01-arquitetura.md`](docs/01-arquitetura.md) |
| Validação | [`docs/04-validacao.md`](docs/04-validacao.md) |
| Segurança | [`docs/08-seguranca.md`](docs/08-seguranca.md) |
| Testes | [`docs/12-testes.md`](docs/12-testes.md) |
| Decisões | [`docs/13-decisoes.md`](docs/13-decisoes.md) |
| Histórico de mudanças | [`CHANGELOG.md`](CHANGELOG.md) |
| Contribuição | [`AGENTS.md`](AGENTS.md) |

## Verificação

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test -p html-auy --features audit
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
python3 tools/validate_spec.py
python3 tools/gen_spec.py --check
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

## Licença

Licenciado sob **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)** — uso livre, inclusive comercial, com atribuição.

Repository: <https://github.com/st-all-one/html-auy>.
