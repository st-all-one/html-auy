# Contribuindo

`html-auy` é um crate público desde `0.1.0`. Toda mudança precisa justificar o efeito na API, nos
bytes emitidos e no contrato de acessibilidade. Leia, nesta ordem:

1. [`docs/00-principios.md`](docs/00-principios.md) — princípios e não-objetivos.
2. [`docs/01-arquitetura.md`](docs/01-arquitetura.md) — os três crates e as fronteiras.
3. [`docs/11-estabilidade.md`](docs/11-estabilidade.md) — semver, MSRV e release.
4. [`docs/22-rigor.md`](docs/22-rigor.md) — lints, supply chain e CI.

## Regras

1. **Toda regra nova declara onde é cobrada.** Tipo, erro de macro, teste de saída (`N4`) ou gate de
   CI. Regra sem nível definido não entra.
2. **Regra que cabe no tipo não vira teste.** Se o defeito pode ser impedido por ausência de API ou
   parâmetro obrigatório, é ali que ele mora.
3. **Mudança em `spec/elements.toml` é mudança de contrato.** Exige justificativa em
   [`docs/13-decisoes.md`](docs/13-decisoes.md), regeneração de
   as duas cópias de `table.rs` com `tools/gen_spec.py` e teste de paridade.
4. **Nada de `unsafe`.** Dependência nova exige decisão registrada; a árvore normal de `html-auy` não
   tem dependências, e `html5ever` só entra com `audit`.
5. **Nada de template em runtime, `Raw`/`PreEscaped` público, `on*` ou `<style>`.** `style=` é escape
   hatch auditado (D3/D34), não caminho de apresentação.
6. **Toda mensagem de erro nova tem caso `trybuild`.** Mensagem não testada apodrece.
7. **Mudança pública incompatível sobe o minor em `0.x`.** Não publique uma tag sem atualizar a
   versão e o gate de release.

## Verificação local

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

O CI também roda `cargo deny`, `cargo audit`, `cargo machete`, miri e o smoke de fuzz. O workflow
`.github/workflows/release.yml` é o único que publica: tag `v*` → verificação → crates.io em ordem de
dependência → GitHub Release sem binários.

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
