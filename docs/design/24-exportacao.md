# 24 — Exportar para outros ecossistemas

> **Status: pós-MVP.** Nada aqui bloqueia o crate ([`21`](21-implementacao.md) §0). Está escrito agora
> porque a decisão de **como** exportar muda o que se guarda hoje (o corpus, a spec como dado) — e essa
> parte já está paga: `spec/elements.toml`, as invariantes e o corpus de conformidade existem. A
> execução (F7 + `html-vocab`) só entra depois de F6.

> **Pergunta:** isto pode virar PHP, Node, Java? **Resposta curta:** o *crate* não; o **contrato** sim.
> O que viaja é a spec + o modelo de valores + o corpus de conformidade. O mecanismo (macro, plugin,
> checker, linter) é detalhe de cada linguagem — e cada uma tem um **teto diferente**.

## 1. O que é exportável, em camadas

| Camada | Conteúdo | Portabilidade | Observação |
|---|---|---|---|
| **L0 — Spec** | `elements.toml`: 111 elementos, categorias, `children`, `required`/`allowed`, globais, enums de `aria`, `role`, invariantes + escopo, `only` | **100%** | é dado neutro; vira JSON Schema sem perda |
| **L1 — Corpus de conformidade** | casos `aceita` / `recusa(invariante)` | **100%** | é o que impede 5 implementações virarem 5 dialetos (§3) |
| **L2 — Modelo de valores** | *o escape é escolhido na construção, não na inserção* (`Text`, `AttrVal`, `Href`, `SrcSet`, `Alt`, `WidthPx`, `Class`, `FieldName`) | **~90%** | o princípio é neutro; a força depende da linguagem (tipo × verificação em tempo de build × gate) |
| **L3 — Sintaxe/DSL** | `html!` | **~30%** | precisa de hospedeiro: macro (Rust/Swift), JSX + plugin (Node), checker (Java/C#), linter (PHP/Go/Python) |
| **L4 — Aplicação** | `Page`, `View`, `Form`, `Field` | **~80%** | conceito neutro; implementação por linguagem |
| **L5 — Processo** | gates, determinismo, `#[expect]`, golden, supply chain | **~95%** | é disciplina de engenharia, não código — [`22`](22-rigor.md) |

A consequência estratégica: **a unidade de exportação é L0 + L1**, não o macro. Quem exporta o macro
exporta uma linguagem; quem exporta spec+corpus exporta a garantia.

## 2. Matriz por ecossistema (teto real, sem otimismo)

| Ecossistema | Mecanismo possível | Fidelidade | Custo | O que se perde |
|---|---|---|---|---|
| **Node/TS** | **JSX como sintaxe** + plugin Babel/SWC/Vite que roda as regras de L0; `.d.ts` carrega L2 | **build-time** (recusa antes de rodar, com span do fonte) | **médio** | TS é apagado em runtime: a checagem é do plugin, não do type checker — a mensagem é de plugin (boa, mas não nativa) |
| **Java** | **Error Prone** (checker no javac) ou Checker Framework; builder tipado gerado de L0 | **compile-time** | **alto** | exige integração com o build (javac plugin); sem `html!`, o autor escreve builder |
| **C#** | **Roslyn analyzer** + source generator (builder a partir de L0) | **compile-time** | **alto** | mesma natureza do Java; tooling de build por projeto |
| **Swift** | macros nativas (5.9+) — análogo quase 1:1 do proc-macro | **compile-time** | **médio-alto** | só Apple; não serve portal Linux |
| **Zig** | `comptime` validando a árvore | **compile-time** | **alto** | ecossistema pequeno |
| **PHP** | **sem compilação**: `ext-ast` é runtime. Teto = linter de Blade + PHPStan rules + value objects | **gate** (não compilação) | **baixo** | não impede escrever: impede **passar no CI**. O erro chega no build, não no editor |
| **Go** | `text/template` + linter; generics não expressam "filho permitido" | **gate** | baixo | idem PHP |
| **Python / Ruby** | linter sobre a árvore do template | **gate** | baixo | idem |

Leitura: **Node/TS é o segundo alvo natural** (JSX já existe, o plugin faz o papel do macro e o custo é o
mais baixo dos que chegam a build-time). **PHP é o alvo de maior impacto e menor fidelidade** — e é
justamente o ecossistema do projeto de origem.

## 3. O contrato que impede a deriva: corpus de conformidade

Cada implementação roda **os mesmos casos**; divergência = teste vermelho.

```json
// spec/conformance/children.json — um caso por linha lógica
{ "id": "p.rejects.div", "input": "<p><div>x</div></p>",
  "expect": "reject", "invariant": "no_flow_inside_phrasing", "scope": "block" }

{ "id": "img.requires.dimensions", "input": "<img src=x alt=x>",
  "expect": "reject", "invariant": "intrinsic_dimensions_required", "scope": "block" }

{ "id": "img.accepts.decorative", "input": "<img src=x alt=\"\" width=8 height=8>",
  "expect": "accept", "scope": "block" }

{ "id": "page.rejects.two.h1", "input": "<main><h1>a</h1><h1>b</h1></main>",
  "expect": "reject", "invariant": "single_h1", "scope": "output" }
```

Regras do corpus:

1. **`invariant` é obrigatório em `reject`** — um caso não prova "recusa" se não diz o que recusa.
2. **`scope` declarado** — um `output` só é verificado por quem monta página (`N4`), não pelo validador de bloco.
3. **O corpus é gerado a partir dos `trybuild`** deste crate: cada mensagem de erro é um caso. Assim o
   corpus nasce completo, não aspiracional.
4. **Nenhuma implementação pode adicionar caso** sem antes entrar na spec — senão o dialeto começa no corpus.

## 4. Dois modelos de exportação (e não são exclusivos)

### A. Reimplementação guiada (uma biblioteca por ecossistema)

```
spec/elements.toml ──► gerador ──► tabela na linguagem (Java, TS…)
spec/conformance/*.json ──► suíte de teste da implementação
```

Prós: melhor ergonomia possível em cada linguagem (JSX, builder, macro); erro no fonte do autor.
Contras: N implementações para manter; cada uma envelhece no seu ritmo.

### B. Um binário, N consumidores (`html-vocab check`)

```bash
# qualquer ecossistema, no CI, sem reimplementar nada
html-vocab check --spec spec/elements.toml --input page.html --format jsonl
# → {"id":"img.requires.alt","selector":"img","status":"fail","invariant":"intrinsic_dimensions_required"}
```

Prós: **uma** implementação, correção única, custo de adoção quase zero; é como `axe`/`pa11y`/`htmlhint`
se espalham — e é o mesmo desenho do `sniff-css` (CLI + JSONL que qualquer linguagem consome).
Contras: valida **saída**, não fonte — não há span do autor, então o DX é pior e a garantia é de **gate**,
não de "estado ilegal inexpressável".

**Recomendação:** B **primeiro** (uma semana, reaproveita o `N4`), A **só** onde há demanda real — e
começando por Node/JSX, onde o custo é o menor.

## 5. A regra de ouro da portabilidade

> **Garantia = spec + corpus. Mecanismo = detalhe de implementação.**

Três consequências práticas:

1. **Nada de regra no código sem linha na spec.** Uma regra que só existe no macro não é exportável — e
   já é dívida no próprio Rust (o `validate_spec.py` existe para isso).
2. **Escopo é parte do contrato.** Se um port não tem tipo `Page`, ele **não pode** alegar `type`; roda
   `output` no CI. A tabela de escopos ([`04`](04-validacao.md) §4.0) é o mapa de fidelidade de cada
   implementação.
3. **Determinismo é algoritmo, não linguagem.** Ordem canônica, `BTreeMap` em vez de hash, escape na
   escrita: tudo reimplementável em PHP/Node/Java com o mesmo resultado byte-a-byte — e testável com o
   mesmo golden.

## 6. O que fazer, e quando

| Quando | Ação | Custo |
|---|---|---|
| **agora** | nada de código; manter L0/L1 como fonte única (já é o caso) | 0 |
| **ao fechar F3** | **gerar** `spec/conformance/*.json` a partir dos `trybuild`; é o artefato que torna a exportação possível | ~1 dia |
| **depois de F6** | `html-vocab check` (L4 do `N4` + CLI + JSONL) — exporta para *qualquer* ecossistema | ~3 dias |
| **quando houver demanda** | plugin JSX/Vite em Node (L3 build-time) | ~2 semanas |
| **só com demanda forte** | Java (Error Prone) / C# (Roslyn) | ~1 mês cada |

## 7. Risco: cinco dialetos

| Sintoma | Antídoto |
|---|---|
| implementação X aceita o que a Y recusa | corpus de conformidade obrigatório no CI de cada port |
| port "melhora" uma regra | regra nova entra na spec primeiro, com escopo declarado |
| port declara `type` sem ter tipo | a matriz de escopos (§2, coluna de fidelidade) é normativa |
| port ignora determinismo | o mesmo golden byte-a-byte, em cada linguagem |

Uma regra de encerramento: **um port que não passa 100% do corpus não é um port — é uma opinião.**
