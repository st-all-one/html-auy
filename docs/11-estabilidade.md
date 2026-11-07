# 11 — Estabilidade, versionamento e release

## 1. O que é estável

`html-auy` é um crate público a partir de `0.1.0`. Estável significa: código que compila e produz os
bytes documentados continua compilando na mesma linha `0.x` (salvo erro de segurança), e as mudanças
que quebram têm release explícita.

| Contrato | Garantia |
|---|---|
| **API pública de `html-auy`** | itens em `pub` e a sintaxe de `html!`; mudança incompatível é minor antes de 1.0 |
| **spec de elementos** | `spec_version` própria (`2.1.0`), versionada junto com o crate |
| **MSRV** | Rust 1.92, verificada no CI; subir a MSRV é mudança minor |
| **semântica de escape** | nunca muda para pior sem release quebrada; há property tests |
| **bytes de `Markup`** | ordem canônica de atributos, classes, `srcset`, assets e tema; snapshots versionados |

Não são contrato: mensagens de erro (texto), ordem interna de regras de auditoria, implementação do
parser e números de benchmark.

## 2. Política de versão

Enquanto a versão for `0.x`:

| Mudança | Bump |
|---|---|
| correção de bug que não muda API nem bytes documentados | patch |
| nova API compatível, novo elemento/atributo opcional, nova regra que não quebra código válido | minor |
| remoção/renomeação de item público, parâmetro obrigatório novo, erro novo em marcação antes válida, subida de MSRV | **minor** (`0.x` → `0.(x+1)`) |
| mudança de semântica de escape ou de bytes canônicos | **minor** com nota de migração |

A partir de `1.0.0`, incompatibilidade passa a exigir major. Em `0.x`, a regra prática é:
**se um `cargo check` que passava para de passar, o minor sobe**.

O `spec_version` pode subir dentro de um patch se a mudança for só de documentação; se altera
validação, acompanha o bump do crate.

## 3. MSRV e toolchain

- MSRV declarada em `[workspace.package] rust-version = "1.92"` e replicada em `clippy.toml`.
- `rust-toolchain.toml` fixa a toolchain de desenvolvimento (pin), que é diferente da MSRV.
- CI tem job `msrv` com `RUSTUP_TOOLCHAIN=1.92`, `cargo build --workspace --locked` e
  `cargo test --workspace --lib --locked`.
- `resolver = "3"` faz a resolução respeitar a MSRV.
- O núcleo do macro usa apenas `proc-macro2` e `quote`; não usa `syn` nem APIs instáveis.

Subir a MSRV é mudança minor em `0.x` e major em `1.x`, sempre com nota no release.

## 4. Congelamento da spec

- `spec/elements.toml` é a fonte normativa; `tools/gen_spec.py` gera duas cópias idênticas de
  `table.rs` (`crates/html-auy/src/spec/` e `crates/html-auy-macros-core/src/`), versionadas.
- `spec_parity` falha se a tabela divergir do TOML.
- Dentro de uma minor, a spec só ganha **adições compatíveis**: elemento novo, atributo opcional
  novo, invariante que não recusa marcação antes válida.
- Remoção, atributo que passa a obrigatório e regra nova que recusa código antes válido exigem novo
  minor (ou major a partir de 1.0), com entrada em [`13-decisoes.md`](13-decisoes.md).

## 5. Deprecação

```rust
#[deprecated = "use `Tone::Primary`; remoção na próxima minor"]
```

- Item deprecado continua funcionando por uma minor inteira.
- A remoção vem com nota de migração no release e, quando aplicável, em [`15`](15-migracao-blade.md).
- Não existe camada de aviso de acessibilidade: ou é tipo/erro de compilação, ou teste de `audit`.

## 6. Release

O processo é por tag:

1. Atualizar `version` em `Cargo.toml` e [`CHANGELOG.md`](../CHANGELOG.md).
2. Rodar o gate local da seção 7.
3. Commitar, criar tag anotada `v0.1.0` e enviar a tag.
4. O workflow [`release.yml`](../.github/workflows/release.yml) verifica que a tag coincide com a
   versão do workspace e publica, nesta ordem:
   - `html-auy-macros-core`
   - `html-auy-macros`
   - `html-auy`
5. O mesmo workflow cria o GitHub Release com notas geradas. **Não há binários**: o artefato é o
   código-fonte e os pacotes do crates.io.

O segredo `CARGO_REGISTRY_TOKEN` precisa existir no repositório (Settings → Secrets and variables →
Actions). O token deve ter escopo de publicação no crates.io.

## 7. Gate de release

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test -p html-auy --features audit
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
cargo package -p html-auy-macros-core --allow-dirty
python3 tools/validate_spec.py
python3 tools/gen_spec.py --check
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

`cargo package` da crate principal só verifica contra o crates.io depois que os dois crates internos
estão publicados; até lá, `cargo publish --dry-run -p html-auy` pode ser usado como ensaio depois de
publicar as dependências. O workflow de release é a autoridade final.

## 8. O que pode mudar sem aviso

- Texto de mensagens de erro, desde que preserve as três partes (o quê, onde, como corrigir).
- Ordem de regras no relatório de `audit`.
- Implementação interna do parser e do gerador.
- Números de benchmark e tamanho de snapshots de diagnóstico.
