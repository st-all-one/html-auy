# Contribuindo

`html-auy` é um crate público desde **`0.1.0`**. O estado atual — API, `spec/elements.toml`, documentação, licença dual MIT OR Apache-2.0 e gates de CI — é o baseline da versão. Toda mudança posterior precisa justificar o efeito na API, nos bytes emitidos e no contrato de acessibilidade.

Leia, nesta ordem:

1. [`AGENTS.md`](AGENTS.md) — regras de evolução e fronteiras do projeto.
2. [`docs/design/00-principios.md`](docs/design/00-principios.md) — princípios e não-objetivos.
3. [`docs/design/01-arquitetura.md`](docs/design/01-arquitetura.md) — os três crates e as fronteiras.
4. [`docs/design/04-validacao.md`](docs/design/04-validacao.md) — níveis de validação e escopos das invariantes.
5. [`docs/design/11-estabilidade.md`](docs/design/11-estabilidade.md) — semver, MSRV e release.
6. [`docs/design/22-rigor.md`](docs/design/22-rigor.md) — lints, supply chain e CI.

## Regras

1. **Toda regra nova declara onde é cobrada.** Tipo, erro de macro, teste de saída (`N4`) ou gate de CI. Regra sem nível definido não entra.
2. **Regra que cabe no tipo não vira teste.** Se o defeito pode ser impedido por ausência de API ou parâmetro obrigatório, é ali que ele mora.
3. **Mudança em `spec/elements.toml` é mudança de contrato.** Exige ADR em
   [`docs/adr/`](docs/adr/README.md) (e referência ao ledger em
   [`docs/design/13-decisoes.md`](docs/design/13-decisoes.md)), regeneração das duas cópias de
   `table.rs` com `tools/gen_spec.py` e teste de paridade.
4. **Nada de `unsafe`.** Dependência nova exige decisão registrada; o crate público é `no_std` + `alloc`, e `html5ever` só entra com `audit`. Não introduza `build.rs`; a tabela é gerada por `tools/gen_spec.py` e versionada.
5. **Nada de template em runtime, `Raw`/`PreEscaped` público, `on*` ou `<style>`.** `style=` é escape hatch auditado (D3/D34), não caminho de apresentação.
6. **Toda mensagem de erro nova tem caso `trybuild`.** Mensagem não testada apodrece.
7. **Acessibilidade não é opcional.** Preserve `Img`/`Alt`, `Video`/`Track::captions`, `Field`/`FieldState`, `ShellData::skip`/`help`, `Transcript` e as regras de `audit`. Não transforme `Alt+1` em feature: `accesskey` é proibido.
8. **O crate não mede CSS.** Contraste, alvo, foco visível, oclusão, reflow e tabulação continuam gates do consumidor (`axe`, `sniffCSS`, e2e). Não prometa no README ou na doc o que o crate não observa.
9. **Não edite tabelas geradas.** Edite `spec/elements.toml`; rode `python3 tools/gen_spec.py`; valide com `python3 tools/gen_spec.py --check` e `python3 tools/validate_spec.py`.
10. **Mudança pública incompatível sobe o minor em `0.x`.** Não publique uma tag sem atualizar a versão, o `CHANGELOG.md` e o gate de release.
11. **Documentação tem idioma definido.** `README.md`, `docs/`, `docs/usage/` e `docs/design/acessibility.md` são em português; `SKILL.md`, `llms.txt` e `llms-full.txt` são em inglês.
12. **Licença é dual.** Mantenha `LICENSE-MIT` e `LICENSE-APACHE` na raiz e nos três crates; o campo do workspace é `MIT OR Apache-2.0`.

## Fluxo de mudança

1. Abra a proposta com o problema real, o nível de validação e a alternativa rejeitada.
2. Se a mudança toca o vocabulário, edite `spec/elements.toml` primeiro.
3. Regenere as tabelas e adicione o teste no nível correto: `trybuild` para diagnóstico, golden/`insta` para bytes, `audit` para o documento inteiro.
4. Atualize a documentação afetada e as contagens de invariantes/regras quando mudarem.
5. Rode o gate local; só então abra o commit ou a tag.

## Verificação local

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
cargo test -p html-auy --features audit --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
python3 tools/gen_spec.py --check
python3 tools/validate_spec.py
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

MSRV no alvo do CI:

```bash
cargo +1.92 test --workspace --lib --locked
```

Os `.stderr` de `trybuild` são artefatos do rustc 1.99 e não são comparados na MSRV. Não os regenere para “consertar” uma diferença de compilador sem intenção.

O CI também roda `cargo deny check`, `cargo audit`, `cargo machete`, Miri e o smoke de fuzz. O workflow `.github/workflows/release.yml` é o único que publica: tag `v*` → verificação → crates.io em ordem de dependência → GitHub Release sem binários.

## Formato de proposta

```text
## Proposta

### Problema
Qual defeito real de HTML, acessibilidade ou segurança isso evita? Cite a regra WCAG,
o atributo ou o cenário concreto.

### Nível de validação
tipo | erro de macro | teste de saída | gate de CI

### Custo
LOC estimadas, falso-positivos esperados, impacto em ergonomia e em tempo de build.

### Alternativa rejeitada
O que já existe (maud/askama/axe/…) e por que não resolve.
```

## O que não será aceito

- Sintaxe nova cujo único ganho seja brevidade.
- Escape hatch para atributo ou marcação arbitrária.
- Qualquer coisa que dependa de JavaScript para o HTML ficar correto.
- Regra advisory de qualquer tipo: ou é erro/tipo, ou é teste que falha.
- Release sem `cargo package`/dry-run e sem gate de CI verde.
- Documentação que prometa contraste, alvo, foco ou layout sem um gate de navegador correspondente.
