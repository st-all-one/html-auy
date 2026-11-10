# ADR 0002 — Sem exportação fora do Rust

- Status: aceito
- Data: 2026-11-10

## Contexto

[`24`](../design/24-exportacao.md) fixa a doutrina de exportação e recomenda, depois de F6, um
`html-vocab` (CLI + wasm) para checar HTML fora do `rustc`. Um plano detalhado e não versionado
(`HTML_VOCAB_CHECKER_PLAN.md`) chegou a desenhar a superfície: crates `html-auy-vocab` e
`html-auy-vocab-wasm`, ABI C manual, envelope JSON `html-auy-vocab/1` e glue `check.mjs` para
Node/browser.

A avaliação do consumo no lado TypeScript mostrou o teto antes do custo. O wasm valida uma **string**,
não a construção: não reproduz `html!`, não alcança o escopo `type` e, no modo `html`, não tem posição
(`RcDom` não guarda offset). O que sobra é um gate de documento — o que um teste do consumidor já faz
com o HTML emitido. Fora do `rustc` não existe a garantia que justifica o crate (estado ilegal
inexpressável); existe um segundo ABI, um alvo wasm e um crate a mais para manter em paridade.

## Decisão

O projeto permanece **Rust-only**. Não haverá:

1. crate `html-auy-vocab` nem binário `html-vocab`;
2. crate `html-auy-vocab-wasm`, alvo `wasm32` ou glue JS/TS;
3. F7 como entrega: os `trybuild` de F3 **são** o corpus; não existe `spec/conformance/` publicado;
4. porta, checker, plugin ou linter em PHP, Node/TS, Java ou qualquer outro ecossistema.

Continua valendo: `spec/elements.toml`, as invariantes e os `trybuild` são a fonte única e o gate do
crate; `audit` (feature opcional) verifica o documento **no** Rust. A regra de ouro de
[`24`](../design/24-exportacao.md) §5 permanece como critério de julgamento de uma porta eventual —
não como trabalho planejado.

## Consequências

- F7 sai do roadmap ([`16`](../design/16-roadmap.md), [`21`](../design/21-implementacao.md)) e a
  recomendação de §4B de [`24`](../design/24-exportacao.md) fica marcada como descartada; a análise
  permanece como registro.
- HTML que não nasce de `html!` não é verificado por este projeto. É consequência aceita: a garantia
  do crate é de compilação, e o que não passa pelo macro só pode ser gate de saída.
- Custo evitado: dois crates, uma ABI manual, um alvo wasm, um envelope versionado, um parser JSON
  adicional e a manutenção de paridade CLI × wasm.
- Verificação: `cargo tree -p html-auy -e normal` continua sem `html5ever`; nenhum gate do repositório
  muda, e nenhum consumidor existente é afetado.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| `html-vocab` só nativo (sem wasm) | ainda é um segundo artefato e um envelope a versionar; `audit` já é chamável no próprio Rust |
| wasm só para browser/editor | não devolve posição no modo `html` e não alcança o escopo `type`; o ganho não paga a ABI |
| plugin JSX/Vite build-time (doc 24 §6) | é uma segunda implementação das regras; a garantia passaria a ser a do port, não a do crate |
| adiar para "quando houver demanda" | a demanda avaliada era a do próprio autor; adiar mantém a decisão aberta e o documento ambíguo |
