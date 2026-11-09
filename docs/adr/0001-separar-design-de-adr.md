# ADR 0001 — Separar `design/`, `usage/` e `adr/`

- Status: aceito
- Data: 2026-11-09

## Contexto

A raiz de `docs/` acumulava princípios, arquitetura, segurança, acessibilidade, decisões, roadmap,
FAQ, migração e um plano histórico. A wiki de uso já estava em `docs/usage/`, mas quem abrisse `docs/`
encontrava 28 arquivos numerados sem uma separação explícita entre **porquê**, **como usar** e
**decisão registrada**.

O arquivo [`13-decisoes.md`](../design/13-decisoes.md) registra D0–D35 no formato “contexto → decisão →
consequência”. Ele é um ledger histórico útil, mas não impõe um arquivo por decisão, status de
superação, data nem alternativas; também mistura decisões ativas com decisões já emendadas.

## Decisão

1. Mover os documentos de projeto para `docs/design/`: princípios, arquitetura, vocabulário,
   validação, acessibilidade, segurança, desempenho, testes, decisões históricas, roadmap, FAQ,
   revisão, implementação, rigor, exportação, exemplo, consumo e lições.
2. Manter `docs/usage/` como a wiki operacional.
3. Criar `docs/adr/` para decisões novas: um arquivo por decisão, com status, data, contexto, decisão,
   consequências e alternativas.
4. Manter D0–D35 em `design/13-decisoes.md` como registro histórico; o
   [`adr/README.md`](README.md) é o índice e as decisões novas não reescrevem esse texto.
5. Atualizar links, testes, ferramentas e referências de prosa para os caminhos novos. Não manter
   cópias antigas na raiz.

## Consequências

- `docs/` passa a ter três entradas: `design/`, `usage/` e `adr/`, além deste índice em
  [`../README.md`](../README.md).
- Links antigos para `docs/04-validacao.md`, `docs/acessibility.md` e similares deixam de existir;
  links internos e externos precisam apontar para `docs/design/…`.
- O histórico de decisões continua legível, mas passa a ter duas camadas: ledger D0–D35 e ADRs novos.
- Uma decisão nova é revisável isoladamente; o custo é mais um arquivo e a disciplina de não editar o
  ADR aceito.
- O CI de links e os testes que leem documentação passam a validar os caminhos novos, não os antigos.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Manter tudo na raiz de `docs/` | não resolve a mistura entre contrato, uso e decisão |
| Mover só as decisões para `adr/` | deixa o restante sem separação entre design e uso |
| Criar um ADR para cada D0–D35 agora | reescreve 35 decisões históricas de uma vez e cria duas fontes de verdade |
| Duplicar os arquivos antigos como redirects | mantém duas árvores e links ambíguos; o projeto prefere corte limpo |
| Deixar `usage/` na raiz | a wiki já é uma unidade coesa; movê-la não melhora a descoberta |
