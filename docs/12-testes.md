# 12 — Testes

## 1. Pirâmide adotada

```
        e2e (Playwright + axe + sniff)      ← comportamento no navegador
      lint de saída (html5ever)             ← página inteira
   conformidade + golden (insta)            ← bytes do vocabulário
property / fuzz (escape, srcset, parser)    ← invariantes
trybuild (mensagens)                        ← ergonomia do erro
```

## 2. Golden (`insta`)

- **Por elemento**: uma fixture por elemento da spec, com todas as combinações obrigatórias,
  opcionais presentes e ausentes.
- **Por primitivo**: `Field`, `Dialog`, `Picture`, `Table`, `Button`, `Alert`, `Pagination`,
  `Breadcrumb`, `Tooltip`.
- **Por página**: as páginas do consumidor (2–3 fixtures de referência).
- O snapshot `.snap` é **o único HTML em arquivo** do projeto: é o artefato de revisão por quem é de
  HTML/CSS.
- Regra: todo snapshot alterado em PR precisa ser lido, não aceito em massa (`INSTA_FORCE_UPDATE` é
  proibido no CI).

## 3. Property tests (`proptest`)

| Alvo | Propriedade |
|---|---|
| escaper de texto | saída não contém `<`, `>`, `&`, `"`, `'` crus; entrada arbitrária (inclui unicode, controle, `\0`) |
| escaper de atributo | idem + `\r\n\t` viram entidades |
| `JsonLd` | `serde_json::from_str` do conteúdo é igual ao original; conteúdo não contém `</script` |
| `SrcSet` | idempotência: ordenar duas vezes dá o mesmo; toda variante aparece uma vez; largura crescente |
| `Class` | merge é associativo/commutativo e deduplicado |
| `Href` | nenhuma construção aceita esquema fora da allowlist (geração de esquemas hostis) |
| determinismo | `render(x) == render(x)` byte a byte |

## 4. Fuzz (`cargo-fuzz`)

Alvo: o **parser** da macro. Contrato:

- nunca `panic` sem mensagem controlada;
- todo `TokenStream` inválido produz `compile_error!` com span, nunca ICE do compilador;
- consumo linear (sem backtracking explosivo).

Entrada inicial: as fixtures de conformidade + corpos `html!` reais do consumidor.

## 5. `trybuild` (mensagens)

Uma fixture por mensagem normativa de [`10-ergonomia.md`](10-ergonomia.md) §2, mais:

- `Markup` não construível de fora;
- atributo desconhecido listando os válidos;
- conteúdo-modelo violado (3 casos);
- `Alt::new("")`;
- `href` com `javascript:`;
- elemento inexistente na spec;
- `style`/`on*` (não existem: erro de atributo desconhecido com dica).

- Caso que prova a **ausência** de API de template por string: qualquer tentativa de compilar template
  a partir de `String`/`&str` de runtime falha por tipo (não existe função pública para isso), e a falha
  é fixada em `trybuild`.

## 6. Conformidade

- `tools/validate_spec.py` valida o TOML (até F3; depois, o mesmo passe em Rust): campos obrigatórios,
  categorias existentes, `children` válido,
  `forbidden_children` consistente, nenhum `style`/`on*` na allowlist.
- Para cada elemento: renderizar fixture → parsear com `html5ever` → a árvore parseada deve
  corresponder à árvore esperada (pega erro de aninhamento e de void).
- Comparar `spec_version` com o histórico: nenhuma remoção dentro de minor.

## 7. Determinismo e orçamento

- Renderizar duas vezes e comparar bytes (`==`) em todas as fixtures.
- Orçamento de bytes por página com valores versionados, verificado no CI do consumidor.
- Teste de "nenhum atributo vazio": varre a saída e falha em `attr=""`.

## 8. `html-lint` (N4) testado

Cada regra de página tem um par de fixtures: uma que passa, uma que falha com a mensagem esperada.

| Regra | Fixture que falha |
|---|---|
| `<main>` único | duas `<main>` |
| `<h1>` único | zero e dois |
| ordem de heading | `h1` → `h3` |
| `id` único | repetido |
| `aria-labelledby` órfão | referência inexistente |
| `lang` inválido | `lang="pt_BR"` |
| `<script>` sem `src` | inline |
| `style=` | inline |
| link ambíguo | `Msg::ReadMore` |
| JSON-LD inválido | `{…` truncado |
| origem externa fora da allowlist | `cdn.example/x.js` |

## 9. Reuso dos gates do consumidor

- `axe` (0 violações A/AA) sobre as páginas reais.
- `sniffCSS` para alvo mínimo, uniformidade e animação.
- e2e de teclado (ordem de foco, `Escape`, `<dialog>`).
- Quando htmx estiver em uso: **`axe` depois do swap**, obrigatório.
- Gate `no-custom-elements`: `grep` na saída e no bundle por `customElements.define`,
  `<template shadowroot`, `<slot`, `part=`, `attachShadow` — falha o build (D20).
- Todos esses testes **não são reescritos**: o crate só reduz o número de violações que chegam até eles.

## 10. Cobertura

- Cobertura de linha ≥ 90% em `html-core` e `html-macro` (excluindo código gerado de `html-spec`).
- Mutação (`cargo-mutants`) nos módulos de escape, `SrcSet`, `Class` e validador da macro.
- A cobertura **não** substitui golden/conformidade: bytes são o contrato, não linhas executadas.

## 11. Como rodar (referência)

```
cargo test                      # unidade + golden
cargo test --features trybuild  # mensagens (compila fixtures)
cargo fuzz run parser           # parser
cargo test --features audit     # spec + validação + golden + trybuild + N4 (D30)
```

### 10.1 Gates de processo (herdados do `sniff-css`, ver [`22`](22-rigor.md))

| Gate | Comando | Cobre |
|---|---|---|
| formato | `cargo fmt --all -- --check` | só opções estáveis no `rustfmt.toml` |
| lints | `cargo clippy --workspace --all-targets -- -D warnings` | duas camadas: níveis no `Cargo.toml`, valores no `clippy.toml` |
| supply chain | `cargo deny check` · `cargo audit` · `cargo machete` · `typos` | licença/advisory/dep morta/typo |
| UB | `cargo miri test -p html --test miri_smoke` | núcleo puro (`forbid(unsafe_code)`) |
| pânico no parser | `cargo fuzz run parser` (smoke) | input adverso nunca vira pânico — sempre `compile_error!` |
| golden | `cargo test` + `UPDATE_GOLDEN=1` para regenerar | bytes determinísticos, comparação byte-for-byte |

### 11.0 Gates fora do crate

| Gate | O que cobra |
|---|---|
| `tools/validate_spec.py` | spec ↔ docs (escopos, contagens, rastreabilidade de invariante) |
| `tools/data-check.py` (D26) | `data-*` declarado ↔ usado ↔ lido pelo JS |
| `N4` (`dev-dependency`) | as 3 invariantes de `output` sobre o HTML final |

## 11.1 Inventário mínimo de `trybuild` (uma mensagem = um caso)

`elemento desconhecido` · `atributo desconhecido` · `atributo obrigatório ausente` · `aninhamento
ilegal` · `valor fora do enum` · `role` inválido · `aria-*` fora do enum · `aria-hidden` em focável ·
`label` sem `id` pareado · `srcset` sem `sizes`/dims · `elemento de documento em html!` · `<form>` à mão ·
`Form::open` sem `CsrfToken` · `Page::new` sem `title`/`heading` · `Field::text` sem `FieldState`.

Cada caso guarda o `.stderr` versionado: mudar a mensagem é mudar o contrato ([`10`](10-ergonomia.md) §2).
