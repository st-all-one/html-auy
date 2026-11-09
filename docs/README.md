# Documentação

A documentação está separada por finalidade. A raiz de `docs/` contém apenas este índice; o conteúdo
fica em três subpastas.

| Pasta | Finalidade |
|---|---|
| [`design/`](design/README.md) | princípios, arquitetura, vocabulário, segurança, acessibilidade, testes e histórico de design |
| [`adr/`](adr/README.md) | decisões técnicas em ADRs numerados e imutáveis |
| [`usage/`](usage/README.md) | wiki operacional: como implementar uma aplicação com o crate |

## Por onde começar

- **Entender o projeto:** [`design/00-principios.md`](design/00-principios.md) e
  [`design/01-arquitetura.md`](design/01-arquitetura.md).
- **Implementar uma aplicação:** [`usage/README.md`](usage/README.md) e
  [`usage/17-guia-de-implementacao.md`](usage/17-guia-de-implementacao.md).
- **Consultar uma decisão:** [`adr/README.md`](adr/README.md) e o registro histórico em
  [`design/13-decisoes.md`](design/13-decisoes.md).

## Limites

`design/` explica o **porquê** e o contrato; `usage/` explica o **como**; `adr/` registra a decisão
que mudou o contrato. Uma decisão nova não reescreve o passado: ela é um ADR novo que pode
superar um anterior.
