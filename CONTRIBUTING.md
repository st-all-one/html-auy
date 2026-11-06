# Contribuindo

Este repositório é **documentação-primeiro**. Antes de qualquer código, a mudança tem que estar
justificada contra os princípios e os critérios de [`docs/16-roadmap.md`](docs/16-roadmap.md).

> **Leia nesta ordem:** [`docs/19`](docs/19-revisao-arquitetural.md) (o que foi cortado) →
> [`docs/21`](docs/21-implementacao.md) §0 (o que falta para começar) → [`docs/22`](docs/22-rigor.md)
> (as regras do compilador). Os docs 01–18 são registro do desenho anterior onde esta revisão mudou algo.


## Regras de contribuição

1. **Toda regra nova é classificada em um nível de validação.**
   Tipo · erro de macro · teste de saída (`N4`) · gate de CI.
   Ver [`docs/04-validacao.md`](docs/04-validacao.md). Regra sem nível definido não entra — e **não há
   nível advisory**: aviso que não bloqueia foi cortado (D28/A4).
2. **Regra que já cabe no tipo não vira teste.** Se o defeito pode ser impedido por ausência de API ou
   por parâmetro obrigatório, é ali que ele mora; `N4` só recebe o que depende do HTML final. Regra de
   `N4` não pode disparar em PR pequeno: o alvo é zero ocorrência nos arquivos de referência
   (ver [`docs/10-ergonomia.md`](docs/10-ergonomia.md) §5).
3. **Mudança em `spec/elements.toml` é mudança de contrato.**
   Exige: justificativa em `docs/13-decisoes.md`, entrada nova no histórico da spec e teste de
   conformidade atualizado.
4. **Nada de `unsafe`. Nada de dependência nova sem decisão registrada.**
   O conjunto de dependências permitido é pequeno e explícito (ver
   [`docs/11-estabilidade.md`](docs/11-estabilidade.md)).
5. **Nada de template em runtime, `Raw`/`PreEscaped` público, `style=` ou `on*`.**
   São não-objetivos permanentes ([`docs/00-principios.md`](docs/00-principios.md)).
6. **Toda mensagem de erro nova tem teste `trybuild`.**
   Mensagem não testada apodrece.

## Verificação local (antes de abrir PR)

```bash
python3 tools/validate_spec.py   # spec × docs: campos, invariantes, allowlists
python3 tools/check_links.py     # todo link relativo entre documentos resolve
```

O primeiro é a barreira que impede a spec de apodrecer: ele cruza `spec/elements.toml` com
`docs/04-validacao.md` §4.1 (nenhuma invariante compilada sem registro normativo) e recusa elemento
com hífen, atributo booleano fora de `[meta] boolean_attributes`, `required_children` que não casa com
`children`, e `role` livre. Quando o passe de validação em Rust existir (F3), ele herda estas mesmas regras.

## Formato de proposta

```
## Proposta

### Problema
Qual defeito real de HTML/acessibilidade/segurança isso evita? Cite a regra WCAG,
o atributo, ou o cenário concreto.

### Nível de validação
tipo | erro de macro | lint de fonte | lint de saída | gate de CI

### Custo
LOC estimadas, falso-positivos esperados, impacto em ergonomia, impacto em tempo de build.

### Alternativa rejeitada
O que já existe (maud/askama/axe/…) e por que não resolve.
```

## O que não será aceito

- Sintaxe nova cujo único ganho seja brevidade.
- "Escape hatch" para atributo ou marcação arbitrária.
- Qualquer coisa que dependa de JavaScript para o HTML ficar correto.
- Regra advisory de qualquer tipo (D28/A4): ou é erro/tipo, ou é teste de `N4` que falha.
