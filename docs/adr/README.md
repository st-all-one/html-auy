# ADRs — decisões técnicas

Um **ADR** (*Architecture Decision Record*) registra uma decisão que muda um contrato do projeto:
API pública, vocabulário, formato dos bytes, política de segurança, acessibilidade, versionamento ou
processo de engenharia. Uma decisão por arquivo; o arquivo nasce aceito ou proposto e não é reescrito
depois.

O registro histórico das decisões anteriores à adoção de ADRs continua em
[`../design/13-decisoes.md`](../design/13-decisoes.md). Ele é registro e contexto; uma decisão nova
não reescreve uma decisão antiga — ou a mantém, ou a supera com status explícito.

## Quando criar

Crie um ADR quando a mudança:

- altera a API pública, a semântica de um tipo ou o formato canônico de saída;
- muda a spec, uma invariante ou o escopo de uma regra (`block`/`type`/`output`);
- troca uma dependência, o toolchain, a MSRV ou o gate de CI;
- adiciona um escape hatch, uma exceção de CSP ou um caminho de confiança;
- muda a organização da documentação de forma que quebre links ou o processo de contribuição.

Não crie ADR para correção de bug, refactor interno sem mudança de contrato, formatação ou ajuste de
texto. Esses casos pertencem ao teste, ao commit e à documentação de uso.

## Formato mínimo

```text
# ADR NNNN — Título curto

- Status: proposto | aceito | rejeitado | superado por ADR-XXXX
- Data: AAAA-MM-DD
- Contexto: o problema e a força que obriga a decidir
- Decisão: o que fica valendo
- Consequências: o que melhora, o que custa e o que fica proibido
- Alternativas: o que foi considerado e por que não venceu
```

O arquivo `NNNN-slug.md` é imutável depois de aceito. Uma mudança de entendimento vira outro ADR com
`Status: superado por ADR-XXXX` no antigo.

## Exemplo mínimo da diferença

Exemplo didático (D36 e ADR 0002 não existem): o registro antigo é uma seção narrativa no ledger; o
ADR separa decisão, estado, consequências e alternativas em um arquivo próprio.

```diff
- ## D36 — Cache de página
- **Contexto.** A página pública é recalculada a cada request.
- **Decisão.** O adapter calcula `ETag` sobre os bytes finais.
- **Consequência.** `304` é possível; cache privado não reutiliza o público.
+ # ADR 0002 — Cache de página
+
+ - Status: proposto
+ - Data: 2026-11-09
+
+ ## Contexto
+ A página pública é recalculada a cada request.
+
+ ## Decisão
+ O adapter calcula `ETag` sobre os bytes finais.
+
+ ## Consequências
+ `304` é possível; cache privado não reutiliza o público.
+
+ ## Alternativas
+ `ETag` do modelo (rejeitado: diverge do HTML) e cache sem validação (rejeitado: serve conteúdo velho).
```

O ADR real e completo que inaugura esta pasta é o
[`0001-separar-design-de-adr.md`](0001-separar-design-de-adr.md).

## ADRs

| ADR | Decisão | Status |
|---|---|---|
| [0001](0001-separar-design-de-adr.md) | Separar `design/`, `usage/` e `adr/`; novos contratos viram ADR | aceito |

## Registro histórico (D0–D35)

Estas decisões foram tomadas antes da pasta `adr/` e permanecem no ledger original, com o texto
completo em [`../design/13-decisoes.md`](../design/13-decisoes.md).

| ID | Decisão | Estado |
|---|---|---|
| D0 | Não publicar como crate | superada por D35 |
| D1 | Contexto de escape é o tipo | aceita |
| D2 | Não existe `Raw`/`PreEscaped` público | aceita |
| D3 | `style=` é escape hatch; `on*` não existe | aceita |
| D4 | Sem escape hatch para atributo | aceita |
| D5 | A spec é dado versionado | aceita; emenda D27 |
| D6 | Um parser, três passes | aceita; emenda D28 |
| D7 | O macro é a API da página; as funções são o contrato | aceita |
| D8 | Validação local no macro, global no lint | aceita |
| D9 | Zero `unsafe`, dependências mínimas, sem `syn` | aceita |
| D10 | Acessibilidade: tipo primeiro, advisory com ratchet | aceita; emenda D28 |
| D11 | `Page` é o único construtor de resposta HTML | aceita |
| D12 | Tema é dado; CSS é derivado e servido como arquivo | aceita |
| D13 | JavaScript só como asset declarado | aceita |
| D14 | Byte é contrato: determinismo obrigatório | aceita |
| D15 | Congelamento por minor; quebra é major | aceita; emenda D28 |
| D16 | Esquema 2 da spec: `required` só de atributo, elemento multi-categoria | aceita |
| D17 | O corpus `_RUST/*` é referência, não dogma | aceita |
| D18 | `hx-*` não entra no vocabulário (o conceito entra) | aceita |
| D19 | Atributo booleano é presença, e o tipo é `bool` | aceita |
| D20 | Sem custom elements | aceita |
| D21 | Toda invariante declara escopo: `block`, `type` ou `output` | aceita |
| D22 | Elementos de documento só existem no `Page`; `html!` os recusa | aceita |
| D23 | `<form>` só existe via `Form::open`, que exige o token CSRF | aceita |
| D24 | Contribuições para o shell são valor devolvido (`View`), nunca pilha ambiente | aceita |
| D25 | `Field` + `FieldState` derivam todo `aria-*` de campo | aceita |
| D26 | `Class` é combinável; attribute bag não existe; `data-*` fecha por gate | aceita |
| D27 | A tabela de elementos: TOML normativo, gerador uma vez, paridade testada | aceita |
| D28 | Não existe binário de tarefas | aceita |
| D29 | Tema e shell de consumidor ficam fora do crate | aceita |
| D30 | `N4` é módulo atrás de feature, não crate nem binário | aceita |
| D31 | `Msg` é trait do consumidor; o crate não conhece locale | aceita |
| D32 | Toolchain: pin `1.99`, MSRV `1.92` | aceita |
| D33 | Flexibilidade de HTML válido não é escape hatch | aceita |
| D34 | Assets, root e `style` controláveis sem quebrar o contrato | aceita |
| D35 | Publicar `0.1.0` como crate público | aceita |

## Onde a decisão aparece

Uma decisão aceita precisa deixar rastro:

- no código, via tipo, invariante, teste ou `trybuild`;
- em `design/`, na página que explica o contrato;
- em `usage/`, quando muda o uso do consumidor;
- no `CHANGELOG.md`, quando muda o comportamento público.

O ADR registra a decisão e as alternativas; ele não substitui o teste nem a documentação de uso.
