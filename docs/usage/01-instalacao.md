[← Filosofia](00-filosofia.md) · [Índice](README.md) · [Próximo →](02-primeira-pagina.md)

# 01 — Instalação e configuração

**Em uma frase.** Adicione `html-auy` à sua aplicação; o crate público traz o macro e os tipos, e a
feature `audit` adiciona o passe opcional sobre o HTML final.

## Requisitos

| Requisito | Valor |
|---|---|
| Rust | **1.92** ou mais novo |
| Edição | 2024 |
| `no_std` | sim, com `alloc` |
| Dependências normais | nenhuma no default |
| Feature opcional | `audit` (`html5ever` + `markup5ever_rcdom`) |
| Binário | não existe |

O crate é uma biblioteca. Ele não instala servidor, CLI, `build.rs` ou gerador de arquivos.

## Adicionar

```toml
[dependencies]
html-auy = "0.1"
```

Com o passe de documento inteiro:

```toml
[dependencies]
html-auy = { version = "0.1", features = ["audit"] }
```

O nome do pacote usa hífen; o nome do crate Rust usa sublinhado:

```rust
use html_auy::{html, Lang, Page, ShellData, Text, View};
```

Não adicione `html-auy-macros` nem `html-auy-macros-core` como dependências de aplicação. Eles são
publicados porque a proc-macro precisa deles; a superfície que você importa é `html-auy`.

## O que a feature `audit` faz

Sem `audit`, o crate não tem dependências normais e não abre parser de HTML:

```rust
// feature = "audit"
let findings = html_auy::audit::check(page.markup().as_str());
for finding in &findings {
    eprintln!("{}: {}", finding.rule, finding.message);
}
```

Com `audit`, entre `html5ever` e `markup5ever_rcdom`. Use a feature no CI e nos testes de página; se
você só quer gerar HTML em produção, pode deixá-la desligada. O custo e as regras estão em
[Testes e diagnóstico](13-testes-e-diagnostico.md) e [Acessibilidade](11-acessibilidade.md).

## `no_std`

O crate é `#![no_std]` e usa `alloc`. Não há `std::io`, `HashMap`, relógio, ambiente ou locale. Para
usar em um alvo sem `std`, mantenha um allocator disponível e compile com a feature `audit` desligada
(o parser de auditoria não é `no_std`).

## Versão e compatibilidade

`0.1.x` é a primeira série pública. A política é semver para a API pública de `html-auy`; em `0.x`,
uma quebra de código válido sobe o **minor**. A política completa, incluindo MSRV e congelamento da
spec, está em [`docs/design/11`](../design/11-estabilidade.md).

## Configuração mínima de uma aplicação

```rust
use html_auy::{html, Lang, Page, ShellData, Text, View};

fn render_page() -> Page {
    let lang = Lang::parse("pt-BR").expect("idioma constante");
    let data = ShellData::new(
        Text::new("Início"),
        Text::new("Início"),
        lang,
    );
    let view = View::new(html! {
        p { Text::new("Conteúdo") }
    });
    Page::new(&data, view)
}
```

Isso compila com o crate padrão. Para uma aplicação real, você vai querer um `enum` de mensagens
(veja [Texto e i18n](05-texto-e-i18n.md)), assets ([Shell e assets](08-shell-assets-e-tema.md)) e um
adaptador HTTP ([HTTP e integrações](12-http-e-integracoes.md)). O esqueleto completo está em
[Projeto mínimo com Axum](16-projeto-minimo-axum.md).

## Comandos de verificação

```bash
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Se você habilitou `audit`, rode também os testes que chamam `audit::check` sobre páginas reais.

## Erros de instalação comuns

| Sintoma | Causa | Correção |
|---|---|---|
| `use html_auy` não resolve | dependência escrita como `html_auy` no `Cargo.toml` | o pacote é `html-auy`; o crate é `html_auy` |
| `html!` não encontrado | importou só tipos | `use html_auy::html;` |
| `audit::check` não existe | feature desligada | adicione `features = ["audit"]` |
| erro de edição/MSRV | toolchain antiga | use Rust 1.92+ e edição 2024 |
| dependência de parser no build | feature `audit` ligada sem necessidade | desligue no default; ligue só no teste/CI |

---

[← Filosofia](00-filosofia.md) · [Índice](README.md) · [Próximo →](02-primeira-pagina.md)
