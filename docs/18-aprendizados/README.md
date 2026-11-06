# 18 — Aprendizados extraídos do corpus `_RUST/`

Este documento registra a leitura sistemática de 14 conjuntos de documentação técnica (≈ 63 mil linhas)
e o que cada um muda, confirma ou proíbe na proposta.

Método: um leitor dedicado por conjunto; cada um devolveu achados com evidência (arquivo + trecho),
conflitos com a proposta, adições prontas e armadilhas de complexidade. Nada entra aqui sem
consequência técnica explícita — o critério é sempre **simplicidade**: se o aprendizado exige runtime,
JavaScript obrigatório, dependência grande ou API que aceite string crua, ele é registrado como
armadilha, não como requisito.

## Índice — um documento por corpus

Cada arquivo é a leitura completa de um corpus: inventário arquivo-a-arquivo, achados com citação
verbatim (`arquivo:linha`), conflitos com a proposta **e a frase exata de correção**. A coluna final
diz o que mudou no repositório por causa daquele corpus.

| Corpus | Documento | Achados | O que mudou aqui |
|---|---|---|---|
| askama 0.16 | [`askama.md`](askama.md) | 44 | `|safe` com custo na mesa ([`14`](../14-alternativas.md) §askama); `/` não é escapado ([`02`](../02-tipos-e-contextos.md) §3); `SIZE_HINT` ([`09`](../09-desempenho-e-cache.md) §1); nenhuma entrada aceita string de template (D2) |
| css-moderno | [`css-moderno.md`](css-moderno.md) | 42 | ordem canônica de `@layer` com `print` por último, zero regra fora de camada, `@property` obrigatório com `initial-value`, `oklch` canônico ([`06`](../06-shell-e-tema.md) §4.1) |
| eta-templates 4.6 | [`eta-templates.md`](eta-templates.md) | 38 | template por string é impossível por tipo (D2); knobs de whitespace proibidos ([`09`](../09-desempenho-e-cache.md) §2); slot vazio colapsa ([`06`](../06-shell-e-tema.md) §1) |
| html-moderno | [`html-moderno.md`](html-moderno.md) | 40 | forma exclusiva de `meta`/`script`, `media_source` separado de `source`, controles são `phrasing`, `viewBox` case-sensitive, `<h1>` em `<header>`, DOCTYPE ([`03`](../03-vocabulario-de-elementos.md)) |
| htmx 4 | [`htmx-4.md`](htmx-4.md) | 34 | **nenhum `hx-*` compila** (D18); contrato de fragmento: `Vary` coerente, `304` só após `200`, `aria-busy` + foco, matriz de status ([`07`](../07-js-e-assets.md) §7) |
| leptos 0.8 | [`leptos.md`](leptos.md) | 39 | sem WASM/hidratação; `data-*` escalar; nenhum id de build/estado no HTML ([`09`](../09-desempenho-e-cache.md) §2); `303` após `POST`; gate "árvore == saída" ([`04`](../04-validacao.md) §6.1) |
| lit 3 | [`lit.md`](lit.md) | 40 | **sem custom elements** (D20): `attachShadow`/`<slot>`/`part` fora, gate `no-custom-elements` ([`12`](../12-testes.md) §9) |
| mdn-web-api-docs | [`mdn-web-api-docs.md`](mdn-web-api-docs.md) | 29 | referência por tipo, nunca por slug de string (confirma D8); lacunas declaradas (`<dialog>`, popover, `inert`, `Intl`) |
| pwa | [`pwa.md`](pwa.md) | 40 | `/sw.js` e manifest **sem hash** e `no-cache`; `skipWaiting` proibido; `theme_color` vem do token de tema ([`07`](../07-js-e-assets.md) §8.1) |
| rust 1.99 / guide | [`rust.md`](rust.md) | 44 | MSRV **1.92 / edição 2024** fixada com job de CI, lints em `[workspace.lints.clippy]`, `publish = false`, lockfile versionado ([`11`](../11-estabilidade.md) §7.1) |
| topcoat | [`topcoat.md`](topcoat.md) | 52 | atributo booleano é presença e o tipo é `bool` (D19); atributos ordenados, nunca `HashMap`; escapes `*_unescaped` recusados |
| web-accessibility | [`web-accessibility.md`](web-accessibility.md) | 40 | níveis WCAG corrigidos (3.3.7 é **A**, 3.3.8 é **AA**), 2.5.8 = 24 px AA com 44 px como regra interna, 4.1.3 = região viva persistente ([`05`](../05-acessibilidade.md)) |
| web-performance | [`web-performance.md`](web-performance.md) | 37 | `img` exige `width`+`height`, `script src` exige `defer`, `link` aceita `crossorigin`; ordem fixa do `<head>` e orçamento de 14 KB do prefixo ([`06`](../06-shell-e-tema.md) §1, [`09`](../09-desempenho-e-cache.md) §2.1) |
| web-security-review | [`web-security-review.md`](web-security-review.md) | 39 | `a.rel`/`referrerpolicy` são enum, `id`/`name` nunca derivam de conteúdo (DOM clobbering), CSP com `*-src-attr`, Fetch Metadata `same-origin` ([`08`](../08-seguranca.md) §6) |

Decisões nascidas dessa leitura: **D16** (esquema 2 da spec), **D17** (o corpus é referência, não
dogma), **D18** (`hx-*` fora), **D19** (booleano é presença), **D20** (sem custom elements) — em
[`13-decisoes.md`](../13-decisoes.md).

## Versões lidas

| Conjunto | Versão / base |
|---|---|
| `askama` | 0.16.0 |
| `css-moderno` | referência 2026 (nesting, layers, subgrid, container queries, `@property`) |
| `eta-templates` | 4.6.0 |
| `html-moderno` | HTML Living Standard (snapshot 2025) |
| `htmx-4` | v4 |
| `leptos` | 0.8 |
| `lit` | 3 |
| `mdn-web-api-docs` | guia v2.0 (base MDN, 1228+ entradas, estado 2026-Q2) |
| `pwa` | guia v2.0 (manifest + service worker, CR do W3C) |
| `rust` | 1.99.1 |
| `topcoat` | 0.11.0 |
| `web-accessibility` | WCAG 2.2 (AA), WAI-ARIA 1.2, HTML Living Standard 2025 |
| `web-performance` | 2.0.0 |
| `web-security-review` | 2.0.0 (snapshot 2026-Q2) |

---

## `topcoat` 0.11.0 — a API mais parecida com a nossa, e a que mais ensina pelo erro

### Confirmações
- **Declaração de asset como constante com hash de conteúdo** (`const FERRIS: Asset = asset!("./ferris.png")`
  → `/_topcoat/assets/ferris-<hash>.png`) é exatamente o desenho de `ctx.require(Asset)`; o nome
  carrega o hash, então `immutable` é seguro.
- **`#[default]`/`#[into]` em props de componente** e "filhos depois dos nomeados" é a melhor ergonomia
  de chamada de componente do ecossistema; nosso builder deve mirar o mesmo conforto.
- **Escaping é HTML-genérico**, e a doc deles admite: "protege contra quebrar a sintaxe, não valida
  semântica" (tabela com URL/CSS/JS/`srcdoc`/nome de atributo como lacunas). Confirma D1: contexto
  tem que ser tipo, não posição.
- **Identidade determinística por call site** (arquivo/linha/coluna) gerando `id`/`for` — resolve
  `label for` sem id manual. Nossos primitivos precisam do mesmo.

### Conflitos (e o que fazemos)
| Prática do topcoat | Problema | Nossa decisão |
|---|---|---|
| `<my-widget>` e `<(tag) (attr)="v">` dinâmicos | nome avaliado em runtime, "nome inválido → panic (500)" | vocabulário fechado; nome nunca vem de dado (D4) |
| `(attrs)` spread como `HashMap` | "ordem não especificada", `class`/`CLASS` coexistem | se houver spread, `Vec<(AttrKey, AttrVal)>` ordenado |
| `suspense`/`live!`/`emit!` como markup | exigem substituição de DOM no browser | fora: HTML é o artefato |
| status/headers como nós de view | "sem isso a resposta é 200"; "renderizar para string descarta metadados" | `Page` carrega status/cabeçalhos (D11) |
| 4 escotilhas `*_unescaped*` | HTML cru por quatro caminhos | nenhuma (D2) |
| Tailwind baixado no `build.rs` sem verificação de checksum | dependência de rede, não determinística | CSS/tokens gerados localmente; sem download em build |

### Adições aceitas
1. **Atributo booleano é presença, nunca valor**: `attr=(bool)` emite vazio quando `true` e omite
   quando `false`; literal `"false"` é erro (a doc deles mostra que `disabled="false"` **desabilita** —
   armadilha clássica).
2. **Condicional dentro da lista de atributos**: `if cond { aria-current="page" class="active" }`,
   permitido só em posição de atributo, nunca de nó.
3. **`class` com condicional**: `"token-a" "token-b" if cond` — cada token resolvido contra a
   allowlist; lista vazia **omite** o atributo. A crate **não** promete resolução de conflito de
   classes (eles documentam que não há).
4. **`id` derivado do call site** (arquivo/linha/coluna + ordinal), com `key` obrigatória em laço —
   determinismo e `for`/`aria-describedby` sem id manual.
5. **Item de navegação como fonte única de estado** (`is_current`): `aria-current` e a classe ativa
   saem do mesmo cálculo.
6. **`asset!`-like só renderiza via `Src`/`Href`/`SrcSet`**; nunca por string.
7. **Fontes**: `@font-face` com `src` obrigatoriamente `Asset`, `crossorigin="anonymous"`, sem CDN
   padrão (a doc deles tem `crossorigin="true"`, que é valor inválido — bug a não repetir).
8. **E-mail com perfil de elementos próprio**: reusar `Markup` com vocabulário restrito (sem `link`,
   `script`), e `text:` explícito quando o cliente exigir alternativa — derivar texto do HTML é
   *best-effort*, nunca sanitização.
9. **Chave de tradução é tipo** (`Key::PageTitle`), validada em compilação; locale no contexto da
   request e `<html lang dir>` emitidos pelo shell. A doc deles admite que `t!` "não valida chaves em
   compile-time" e que locale em layout "não alcança a página" — dois motivos para fazer diferente.

### Armadilhas registradas
`$(...)`, `@event`, signals, `#[shard]`, `#[procedure]`, `live!`/`emit!`, execução assíncrona
"em burst" (primeiro byte espera todos; ordem indefinida), e qualquer hash/asset com leitura de
arquivo por request.

---

## `rust` 1.99.1 — disciplina de API

### Confirmações
- **Newtype com campo privado + construtor validante** é o padrão "tipo de validação" da linguagem
  (`pub struct Guess { value: i32 }` + `new` que valida): é literalmente a justificativa de `Markup`
  sem construtor público.
- **`From` é infalível** — logo `impl From<&str> for Markup` destruiria o escape por tipo em uma linha.
  Só `TryFrom`/`new() -> Result`.
- **Lints no workspace** (`[workspace.lints]`: `unsafe_code = "forbid"`, `clippy::unwrap_used/expect_used/panic = "deny"`)
  e `publish = false` + `rust-version` declarada.
- **`PhantomData` para parâmetro que só existe no type level** — o mesmo mecanismo serve para marcar
  fases do pipeline (validado/formatado) sem campo em runtime.

### Conflitos e correções
| Ponto | Correção |
|---|---|
| MSRV 1.85 vs guia 1.99.1 | fixar MSRV **1.92** (edição 2024, sem API de 1.98/1.99) e proibir APIs acima dela no CI |
| `#[should_panic]` como prática | não usar: nossos erros são de compilação (`trybuild`), não panic; `clippy::panic = deny` |
| `impl Into<Markup>` (APIT) | **proibido** em elemento/primitivo: reabre a porta por `From`. Parâmetro concreto. |
| `Cow<'a, str>` generalizado | não; começar `String` owned e medir. Lifetime no tipo contamina toda a API |
| `Box<dyn Node>` para filhos | não; enum fechado + `match` exaustivo |
| empurrar `tokio`/`axum`/`serde` para o core | core sem I/O, sem async, sem dependências web |

### Adições aceitas
1. `#![forbid(unsafe_code)]` nos três crates; lints do workspace com `unwrap_used`/`expect_used`/`panic` = deny.
2. `publish = false`, `Cargo.lock` versionado, zero features, CI em `--locked` na MSRV.
3. Enum público com `#[non_exhaustive]`; `match` interno sempre exaustivo, sem `_`.
4. Nenhum tipo de `proc_macro`/`proc-macro2` atravessa a fronteira de `html-core`.
5. Erros por domínio (`SpecError`, `EscapeError`), `Send + Sync`; `anyhow` só no binário `cq`.
6. `tests/ui/pass` além de `tests/ui/fail` no `trybuild`: mensagem é contrato, e o caso que **deve**
   compilar também é testado.

### Armadilhas registradas
Lifetime no tipo `Markup`; `From` infalível; `Box<dyn Node>`; repository/`async_trait`/`sqlx` no core;
depender de `repr(transparent)` para newtype (semântica de layout não é contrato de API).

---

## `web-performance` 2.0.0 — o que o HTML pode garantir por construção

### Confirmações
- **Primeiro 14 KB** (CWND ≈ 10 MSS) é orçamento real do prefixo crítico — justifica a ordem fixa do
  `<head>` e um teste de bytes.
- **`preload` de fonte exige `crossorigin`**; sem ele só acontece DNS — emissor não pode deixar
  opcional.
- **`loading="lazy"` em imagem acima da dobra atrasa LCP** → o primitivo precisa de `Fold::{Above,Below}`,
  não de booleano livre.
- **`SrcSet` sem `Sizes` assume `100vw`** → os dois são **um par de tipos**, não dois campos.
- **Preload scanner não descobre recurso carregado por JS** → todo asset tem de existir como tag no
  HTML (confirma `ctx.require(Asset)`).
- **Dois regimes de cache**: HTML `no-cache`/`must-revalidate`; JS/CSS versionado `immutable`.
- **`width`/`height` ausentes = maior causa de CLS**; `content-visibility: auto` exige
  `contain-intrinsic-size`.

### Conflitos e correções
| Prescrição do corpus | Nossa correção |
|---|---|
| CSS crítico **inline** em `<head>` | proibido (CSP sem inline). CSS único externo + `Link: rel=preload` / `103 Early Hints` |
| `<link ... onload="this.rel='stylesheet'">` | proibido (`on*` não existe) |
| `<script type="speculationrules">` (script inline) | fora |
| "CLS 0" genérico | reescrever como **"sem shift de mídia e de layout estrutural"**; swap de fonte é mitigado por `font-size-adjust`, não zerado |
| critical-CSS automático (Critters/Penthouse) | fora: dependência de build grande; CSS único + orçamento |

### Adições aceitas
1. **Emissão obrigatória por primitivo**: imagem com `width`+`height`+`decoding="async"`, `loading` por
   `Fold`, `fetchpriority` só no herói; fonte com `font-display: swap` e `preload crossorigin` (≤ 1
   família, ≤ 2 pesos); exatamente **um** stylesheet render-blocking, `@import` proibido; JS só
   `type="module"`/`defer`.
2. **Ordem fixa do `<head>`**: charset → viewport → preconnect (≤ 3) → preload → stylesheet → restante.
3. **Orçamentos testáveis**: prefixo ≤ 14 KB; HTML ≤ 50 KB; CSS ≤ 50 KB br; fonte ≤ 50 KB; payload
   inicial ≤ 500 KB; JS ≤ 150 KB. Ratchet por página, como o de acessibilidade.
4. **Invariantes de emissão**: todo `<img>` tem dimensões; `SrcSet ⇒ Sizes`; zero inline; `preconnect`
   nunca para a própria origem; preload só de recurso não descoberto pelo scanner.
5. **Nada de métrica de campo no CI** (LCP/INP/CLS numéricos, Lighthouse score, Speed Index): testam-se
   os **atributos que as causam**, não o valor.

---

## `eta-templates` 4.6.0 — o modelo runtime, e por que ele perde aqui

### O que o runtime realmente entrega (e como substituímos)
| Facilidade do Eta | Equivalente na crate | O que se perde |
|---|---|---|
| `layout()` antes do corpo, layouts encadeáveis | um **único** shell; `Page::new(ctx, main)` | trocar layout por nome em runtime (proibido) |
| `block('x')` por string, fallback silencioso | `enum Slot` fechado, preenchido uma vez; duplicata = erro | nenhum valor real |
| partial com contexto **mesclado** (`it` + extras) | primitivo com props explícitas | mesclagem implícita (desejável perder) |
| `capture()` de fragmento | `let` antes do `html!` (reuso de `Markup`) | nada |
| edição de `.eta` sem rebuild | `cargo check` incremental + revisão por `.snap` | hot-reload de template |

### Confirmações fortes
- A doc do Eta é explícita: escape é **HTML, não contextual** e "assumir que `<%=` escapa URLs/atributos
  JS é um erro" — exatamente a lacuna que D1 fecha com tipos.
- Template compila para `new Function`: exige `'unsafe-eval'` em CSP restritiva. Custo decisivo e
  mensurável contra "página cacheável + `script-src 'self'`".
- Tipagem é decorativa: `interface HomeData` + `satisfies` não impede `it.titel` — erro só na request.
- Determinismo não existe: `autoTrim`, `rmWhitespace`, `plugins` e `filterFunction` alteram a saída.
  Um `ETag` canônico exige **um** formato, sem knobs.

### Adições aceitas
1. **Um shell, slots declarativos e exaustivos** (`enum Slot`), nunca preenchimento imperativo com
   ordem implícita nem duplicata silenciosa.
2. **`it.body` nunca existe**: o shell recebe `Markup`, jamais `String` de template.
3. **`cq fmt` sem configuração**: um único modo de emissão; nenhum `autoTrim`/`rmWhitespace`.
4. **Nenhum ponto de entrada aceita template por string** (`renderString`, `loadTemplate`) — isso é o
   furo `Raw` por outra porta.
5. **Views síncronas**: nenhuma variante `*Async`; `await` no corpo é impossível por construção.
6. **Custom tag (handler que devolve HTML não escapado) não existe**; i18n só por `Msg`.
7. **Calcular uma vez** (`let` antes de `html!`), nunca recomputar dentro de `for`.

### Armadilhas registradas
`plugins`/`processAST`/`customTags` (hooks de transformação no parser), `*Async`, marcadores de
whitespace (`-`, `_`, `slurp`), `useWith`/`varName` (escopo implícito), `output()`/`<%~` (concatenação
crua). Todas exigem gramática ou runtime extra para um ganho que a crate não quer.

---

## `mdn-web-api-docs` — correção sobre o próprio corpus

**O conjunto não contém documentação de Web APIs.** O `SKILL.md` o define como guia de *redação de
páginas MDN* (front matter, sidebars, page-types); grep por `dialog`, `popover`, `inputmode`,
`autocomplete`, `reportValidity`, `inert` só devolve falsos positivos. O conteúdo útil para a crate
está em `html-moderno`, e foi lido lá (abaixo). Fica registrado para não se repetir a expectativa.

Único dado aproveitável direto: exemplos MDN dependem de iframe + JS (`document.querySelector`) — os
nossos exemplos de primitivo **não** podem seguir esse modelo (CSP sem inline, funciona sem JS).

---

## `html-moderno` — UI sem JavaScript (o achado de maior impacto desta rodada)

### Tabela: necessidade → recurso nativo → o que o primitivo emite

| Necessidade | Recurso | Emissão | Ainda exige JS |
|---|---|---|---|
| modal | `<dialog>` + `closedby` | `id`, `aria-labelledby`, `closedby="closerequest"`, `<form method="dialog">` + `<button value>` | **abrir**: `commandfor`/`command` (experimental) ou asset declarado |
| fechar por `Esc` / clique fora | `closedby` | `closerequest` / `any` | não |
| foco preso + resto inerte | semântica de `showModal()` | — (implícito) | não, se aberto modalmente |
| tooltip / menu / dropdown | popover | `popover="hint"`/`"auto"`, `popovertarget`, `popovertargetaction` | não |
| acordeão / FAQ | `<details name>` | `name`, `open`, `<summary>` primeiro filho | não |
| busca do site | `<search>` + `form method=get` | `label` no `<search>`, `type="search"`, `enterkeyhint="search"` | não |
| erro de campo (cliente) | constraint validation | `required`/`type`/`pattern`/`minlength`/`min`/`step` | não (mensagem do browser) |
| erro de campo (pós-POST) | — | `aria-invalid="true"` + `role="alert"` + `:user-invalid` no CSS | não |
| preenchimento automático | `autocomplete` | enum de tokens (+ `webauthn`) | não |
| teclado virtual | `inputmode` + `enterkeyhint` | enum de 8 e 7 valores | não |
| gaveta / banner oculto | `inert` | `inert` + `aria-expanded` no gatilho | alternar exige JS ou rota |
| data legível | `<time datetime>` | RFC 3339 | não (formatar em Rust) |

### Conflitos concretos encontrados na nossa spec (todos corrigidos no `spec/elements.toml`)

| Lacuna | Consequência | Correção |
|---|---|---|
| `dialog` sem `closedby` | `Esc` e clique-fora impossíveis sem JS | `closedby` (enum `none\|closerequest\|any`) |
| `button` sem `popovertarget`/`commandfor`/`form` | nenhum popover/dialog acionável sem JS | atributos aceitos, com validação de `id` existente |
| `form.method` sem `dialog` | `<dialog>` não fecha devolvendo `returnValue` | enum inclui `dialog`; `novalidate` **proibido** |
| `inert`/`hidden` ausentes | não há como representar conteúdo inerte/oculto | atributos globais, `hidden` com `until-found` |
| `details` sem `name` | sem acordeão nativo | `name` + advisory de duplicata |
| `input` sem `list`/`enterkeyhint`/`spellcheck`/`size`; tipos faltando | teclado e dica de entrada limitados | enum completo + `url`/`range`/`time`/`month`/`week`/`datetime-local` |
| `textarea` sem `minlength`/`autocomplete` | validação nativa parcial | aceitos |
| `<datalist>`, `<output>`, `<meter>`, `<progress>` ausentes | — | `meter`/`progress` entram; **`datalist` fica fora** (sem nome acessível garantido e sem estilo controlável) |

### Adições de tipo
1. **`Autocomplete`** (enum fechado; `password` obriga `current-password`/`new-password`). `autocomplete="off"`
   não impede gerenciador de senha — é token, não string.
2. **`Pattern`** — string compilável, sem flags `g/i/m`: a macro **rejeita** `(?i)`/`(?m)` (mesma regra
   do browser).
3. **`Inputmode`** e **`Enterkeyhint`** como enums com os conjuntos fechados.

### Invariantes novos
- `Field` com `required`/`pattern`/`minlength` **sempre** emite `aria-describedby` para o texto de erro;
  `aria-invalid` só no re-render pós-POST (nunca no primeiro render).
- `popovertarget`/`commandfor`/`aria-controls`/`for` apontando para `id` inexistente no bloco = erro de
  compilação (mesma família do `label for`).
- Correção do parser do browser **nunca** é aceita como saída: `<tbody>` é emitido explicitamente;
  `<div>` em `<p>` é erro (ver também `leptos`, abaixo).
- Formatação de data/hora é feita em Rust (`Timestamp` → RFC 3339 + texto localizado); nada de `Intl`
  no cliente.

### Armadilhas registradas
`reportValidity()`/`setCustomValidity()`/`checkValidity()` (runtime JS próprio); `Intl` no cliente;
`autocomplete` como `String`; Invoker Commands como caminho **obrigatório** (é experimental);
`popover="manual"` com estado aberto modelado em Rust (sincronizar estado de UI com servidor exige JS);
`<datalist>` no v1.

---

## `leptos` 0.8 — composição sem reatividade

### O que portamos
1. **Componente é função que roda uma vez** (setup, não render) — já é o nosso desenho; confirmado como
   correto para HTML estático.
2. **Props são argumentos tipados**; struct nomeada (`FieldProps`) só acima de ~4 argumentos, nunca
   builder fluente.
3. **Slots nomeados e tipados** (`Table::new(header, body, caption)`, `Picture` com `caption`), nunca
   mapa de nomes.
4. **Children é valor de primeira classe** — mas passamos `Markup`/`impl Render` já renderizado, sem
   boxing e sem closure.
5. **Controle de fluxo é Rust** (`if`/`match` retornam valor), sem `Either`/`into_any`.
6. **`List<T: Render>`** emite o `<li>` ele mesmo (resolve o `ChildrenFragment` + rewrap deles).

### O que rejeitamos, com o motivo técnico citado
| Prática | Por quê |
|---|---|
| **spread de atributos** (`{..}`, `attr:`, `prop:`, `node_ref`) | "atributos do componente são aplicados a **todos** os elementos de topo": invalida allowlist e ARIA |
| `inner_html` | é o `Raw` que D2 elimina |
| `#[prop(into)]` | `.into()` automático recria superfície aceita-tudo (`From` infalível) |
| props genéricas de closure (`Fn() -> T`) | closure vista como prop é `Raw` disfarçado e impede determinismo |
| `provide_context`/`Store` | estado ambiente implícito, ordem-dependente, invisível no tipo; nosso `Ctx` é `&mut` explícito |
| signals/effects/`NodeRef` | arena global por request, leitura de signal descartado dá panic; effects não rodam no servidor |
| `Resource`/`Suspense`/OOO streaming | flush parcial torna `sha256` do corpo impossível — mata `ETag` |
| hidratação/WASM/ilhas | HTML ganha `<script>` de WASM, props em JSON (`data-props='{"…"}'`) e `data-component="Counter_12345"` (hash que muda por build) |
| `leptos_meta` | `<head>` coletado durante o render, ordem não determinística |

### Evidência mais útil de todas
A doc deles recomenda, para ergonomia de rota, `oninput="this.form.requestSubmit()"` — isto é, o
conforto do framework empurra para **`unsafe-inline`**. É a demonstração de que a ergonomia que a nossa
crate precisa vem do desenho (forms nativos + `Vary: HX-Request`), não de script inline.

### Adições aceitas
- `docs/02`: "nenhum primitivo aceita closure/callback como prop; `children` é consumido uma vez".
- `docs/04`: "correção do parser do browser nunca é aceita; `<tbody>` emitido explicitamente".
- `docs/07`: "não existe `nonce` — nenhum byte variável por request entra no HTML".
- `docs/09`: "nenhum identificador de build/componente no HTML; assets por hash de conteúdo do arquivo";
  "estado nunca serializado no HTML".
- Checklist de determinismo: rebuild sem mudança de fonte → `ETag` idêntico.
