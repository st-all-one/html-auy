# Design

Documentos de contrato e justificativa. Eles explicam **por que** o crate é assim; o **como usar**
fica em [`../usage/`](../usage/README.md) e as decisões novas em [`../adr/`](../adr/README.md).

| Documento | Assunto |
|---|---|
| [00 — Princípios](00-principios.md) | tese, não-objetivos e critérios |
| [01 — Arquitetura](01-arquitetura.md) | crates, fronteiras e fluxo de compilação |
| [02 — Tipos e contextos](02-tipos-e-contextos.md) | escape, texto, URL e valores |
| [03 — Vocabulário](03-vocabulario-de-elementos.md) | elementos, atributos e conteúdo-modelo |
| [04 — Validação](04-validacao.md) | macro, escopos de invariantes, `audit` |
| [05 — Acessibilidade](05-acessibilidade.md) | garantias, componentes e lacunas |
| [06 — Shell e tema](06-shell-e-tema.md) | documento, regiões, assets e tema |
| [07 — JS e assets](07-js-e-assets.md) | script externo, JSON-LD e resposta parcial |
| [08 — Segurança](08-seguranca.md) | injeção, CSP e escape hatch |
| [09 — Desempenho e cache](09-desempenho-e-cache.md) | bytes canônicos, `ETag` e orçamento |
| [10 — Ergonomia](10-ergonomia.md) | macro, mensagens e checklist |
| [11 — Estabilidade](11-estabilidade.md) | semver, MSRV e release |
| [12 — Testes](12-testes.md) | unidade, golden, `trybuild` e gates |
| [13 — Decisões](13-decisoes.md) | ledger histórico D0–D35 |
| [14 — Alternativas](14-alternativas.md) | matriz de ecossistemas e tradeoffs |
| [15 — Migração de Blade](15-migracao-blade.md) | tradução de views |
| [16 — Roadmap](16-roadmap.md) | marcos e critérios de parada |
| [17 — FAQ](17-faq.md) | respostas curtas |
| [18 — Aprendizados](18-aprendizados/README.md) | análise dos corpus |
| [19 — Revisão arquitetural](19-revisao-arquitetural.md) | auditoria do desenho |
| [20 — Fronteira da aplicação](20-fronteira-de-aplicacao.md) | o que pertence ao consumidor |
| [21 — Implementação](21-implementacao.md) | plano histórico F0–F6 |
| [22 — Rigor](22-rigor.md) | lints, supply chain e CI |
| [23 — Acessibilidade e CSS](23-acessibilidade-e-css.md) | estilos e gates de browser |
| [24 — Exportação](24-exportacao.md) | portabilidade do vocabulário |
| [25 — Exemplo de uso](25-exemplo-de-uso.md) | contraste entre desenho e uso |
| [26 — D29 no consumidor](26-d29-consumidor.md) | tema e shell fora do crate |
| [27 — Lições do Eta](27-licoes-do-eta.md) | o que o corpus ensinou |
| [Acessibilidade](acessibility.md) | arquitetura de acessibilidade em português |

## Regras

- `design/` é a fonte do **contrato**; `usage/` é a fonte do **uso atual**.
- Uma decisão que muda contrato vira ADR em [`../adr/`](../adr/README.md); D0–D35 permanecem como
  registro histórico em `13-decisoes.md`.
- Links para fora de `design/` usam `../../` para a raiz do repositório.
