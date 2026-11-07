[← HTTP e integrações](12-http-e-integracoes.md) · [Índice](README.md) · [Próximo →](14-receitas.md)

# 13 — Testes e diagnóstico

**Em uma frase.** Teste os **bytes** que o consumidor vê, não a forma do fonte; use `audit` para o
que só o documento inteiro revela e `trybuild` para o que não deve compilar.

## Quatro níveis

| Nível | O que prova | Ferramenta |
|---|---|---|
| tipo | `Markup`/CSRF/URL/alt não podem existir errados | compilador |
| bloco | aninhamento/atributos do fragmento | compilador + `trybuild` |
| página | `main`/`h1`/ids/ARIA/headings | `audit::check` |
| navegador | contraste/foco/layout pós-swap | `axe`, teclado, `sniffCSS`, e2e |

Os dois primeiros não precisam de teste de runtime: o código não compila. O terceiro é barato e
determinístico. O quarto é do consumidor.

## Snapshot do bytes

```rust
#[test]
fn card_tem_ordem_canonica() {
    let markup = html! {
        article class="card" data_kind="news" {
            h2 { M::Title }
            a href=(Href::internal(home())) { M::Read }
        }
    };
    insta::assert_snapshot!(markup.as_str());
}
```

O snapshot é o HTML legível; a indentação do Rust não entra. O macro ignora whitespace e a saída é
uma linha. Quando o snapshot muda, a pergunta é “a mudança de HTML foi intencional?”.

Não escreva testes que verificam apenas “não entrou em pânico”, “tem tamanho > 0” ou “contém
`<div>`”: eles não pegam regressão de contrato.

## Determinismo

```rust
#[test]
fn espacos_no_fonte_nao_mudam_bytes() {
    let compact = html! {div class="a"{span{M::X}}};
    let wide = html! {
        div class="a" {
            span { M::X }
        }
    };
    assert_eq!(compact.as_str(), wide.as_str());
}
```

Também vale para ordem de atributos, `class`, `data-*`, `srcset` e assets. O `ETag` depende disso.

## `audit` de página

```rust
#[cfg(feature = "audit")]
#[test]
fn paginas_publicas_passam() {
    for page in fixtures() {
        let findings = html_auy::audit::check(page.markup().as_str());
        assert!(findings.is_empty(), "{findings:#?}");
    }
}
```

`audit::check` devolve `Vec<Finding>` com `rule` e `message`. Ele parseia o HTML final, então vê a
composição de shell + views + modais. Rode no CI com a feature `audit`; sem a feature, o crate não
tem dependências normais.

O que o `audit` cobre está em [Acessibilidade](11-acessibilidade.md). Ele **não** cobre contraste,
alvo de toque ou layout.

## Testes de compilação

O repositório mantém `crates/html-auy/tests/ui/*.rs` e `.stderr` com `trybuild`:

```text
tests/ui/img_without_alt.rs
tests/ui/img_without_alt.stderr
tests/ui/classes_free_string.rs
tests/ui/shell_main.rs
```

Ao adicionar uma regra, adicione o caso que **deve falhar** e o texto da mensagem. No consumidor,
não é preciso reproduzir essa suíte; ela protege a API pública.

## Gates do repositório

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
cargo test -p html-auy --features audit --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
python3 tools/validate_spec.py
python3 tools/gen_spec.py --check
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

Para release, `cargo package` do núcleo e `cargo publish` em ordem; veja o workflow em
`.github/workflows/release.yml`.

## Diagnóstico de erros de compilação

| Mensagem | Onde olhar |
|---|---|
| `texto literal não é permitido` | [Texto e i18n](05-texto-e-i18n.md) |
| `atributo desconhecido` | `spec/elements.toml`; a mensagem lista os válidos |
| `img exige alt/width/height` | [Mídia](07-midia.md) |
| `aria-label literal` | `Label::text` |
| `label for sem id` | use `Field::*` |
| `main/form` shell/CSRF | [Macro](03-macro-html.md) e [Formulários](06-formularios.md) |
| `h1` duplicado no `audit` | remova `h1` da view; o `Page` tem um |
| `no_style_attribute` | [Segurança](10-seguranca.md) |

O erro aponta para o token original, não para o código expandido. Se a mensagem não bastar, isole o
fragmento em um `html!` mínimo e compile; o `trybuild` da biblioteca mostra o formato esperado.

## O que não testar

- a implementação interna de `Writer`/tabela (já coberta por `spec_parity`);
- a indentação do fonte;
- `Markup::as_str().len()` como prova de conteúdo;
- a mesma regra em três camadas;
- comportamento de browser em teste unitário.

## Consumer gates

Mesmo com todos os testes verdes, o consumidor ainda precisa de:

- `axe`/`sniffCSS` nas páginas reais;
- teste de teclado em diálogo/popover;
- `axe` depois de swap parcial;
- revisão de texto alternativo e tradução;
- medição de contraste e alvo de toque.

O crate reduz o custo de chegar a esses gates; não os substitui.

---

[← HTTP e integrações](12-http-e-integracoes.md) · [Índice](README.md) · [Próximo →](14-receitas.md)
