# 13 — Decisões (D0–D15)

Formato: **contexto → decisão → consequência**. Uma decisão por seção; mudar exige nova entrada.

---

## D0 — Não publicar como crate

**Contexto.** O projeto nasce de um consumidor único (portal municipal) com ADRs próprios; a
publicação criaria semver público e obrigação com consumidores desconhecidos.
**Decisão.** Repositório aberto, consumo por vendor/cópia, nenhuma publicação em registry.
**Consequência.** Estabilidade é medida pelo consumidor, não por estranhos; quebra de contrato é
barata de decidir, mas continua versionada (D15).

---

## D1 — Contexto de escape é o tipo

**Contexto.** Todo escape disponível no ecossistema é HTML-escape genérico. JSON-LD, URL e atributo
exigem escapes diferentes; o maud mantém a issue #181 aberta por isso.
**Decisão.** Cada contexto tem um tipo (`Text`, `AttrVal`, `Href`, `Src`, `JsonLd`, `SrcSet`), com um
construtor seguro e nenhum construtor cru.
**Consequência.** Esquecer de escapar deixa de existir como categoria de bug. Custo: mais tipos
(≈ 15) e disciplina de vocabulário.

---

## D2 — Não existe `Raw`/`PreEscaped` público

**Contexto.** Todo motor de template oferece uma porta de escape (`|safe`, `PreEscaped`,
`dangerouslySetInnerHTML`). A porta existe porque o tipo de contexto é fraco.
**Decisão.** Não há porta. `Markup` tem construtor privado ao crate.
**Consequência.** Uma necessidade legítima de "HTML pronto" (ex.: AST de texto rico) é resolvida por
renderização nó-a-nó no vocabulário, não por bypass.

---

## D3 — `style` e `on*` não existem

**Contexto.** CSP sem inline é requisito; estilo é dado (tokens), comportamento é arquivo.
**Decisão.** Atributos `style`, `on*`, `srcdoc`, `@*`, `hx-on*`, `x-on:*` não estão no vocabulário e
não há como adicioná-los em tempo de execução.
**Consequência.** CSP `'self'` sem `unsafe-inline`/`unsafe-eval`; `ETag` estável; estilo vira token.

---

## D4 — Sem escape hatch para atributo

**Contexto.** Validação que aceita "qualquer atributo" não valida: o typo passa.
**Decisão.** `attrs(...)` livre não existe. Atributo novo = linha em `spec/elements.toml` + decisão.
**Consequência.** Custo de mudança alto e explícito; em troca, `hreff`, `altt`, `aria-lable` são
impossíveis.

---

## D5 — A spec é dado versionado

**Contexto.** 63 elementos cobrem o escopo (medido: 6.765 tags). Regra duplicada em código apodrece.
**Decisão.** `spec/elements.toml` é a fonte; `build.rs` gera funções e tabelas; `cq spec` valida.
**Consequência.** Mudança de contrato é diff revisável, com histórico. Gerador precisa ser confiável
(coberto por conformidade).

---

## D6 — Um parser, três passes

**Contexto.** Validação, formatação e geração precisam do mesmo entendimento da marcação.
**Decisão.** Um AST, um parser; `fmt` reemite do AST.
**Consequência.** Formatação custa ~250 LOC em vez de uma gramática própria; toda regra nova vale para
macro e `fmt` automaticamente.

---

## D7 — O macro é a API da página; as funções são o contrato

**Contexto.** Escrever HTML em Rust puro, com construtores, é ilegível em listas.
**Decisão.** `html!` é a forma de escrever view; as funções geradas (`img(...)`, `a(...)`) são o alvo
da expansão e a API estável para primitivos.
**Consequência.** Primitivos podem ser escritos sem macro quando fizer sentido; a ergonomia da página
vem do macro.

---

## D8 — Validação local no macro, global no lint

**Contexto.** Proc-macro não vê outra proc-macro; página inteira é fora de alcance.
**Decisão.** Macro: estrutura, atributos, conteúdo-modelo, `id`/`for` dentro do bloco.
Lint de saída: `<h1>` único, ordem de heading, `id` global, `aria-*` resolvido, JSON-LD parseável.
**Consequência.** Fronteira explícita; nenhuma tentativa de "resolver tudo no compilador".

---

## D9 — Zero `unsafe`, dependências mínimas, sem `syn`

**Contexto.** Macro é posição privilegiada; superfície de dependência é risco.
**Decisão.** `#![forbid(unsafe_code)]`; produção usa apenas `proc-macro2` e `quote`; parser próprio.
**Consequência.** Parser de ~1,2k LOC auditável; CI na MSRV/stable/beta; `cargo deny` no pipeline.

---

## D10 — Acessibilidade: tipo primeiro, advisory com ratchet

**Contexto.** Proc-macro stable não emite warning (`proc_macro::Diagnostic` é instável); aviso que
dispara demais é ignorado.
**Decisão.** O que é inegociável vira tipo/erro; o resto vira lint com ratchet por página; nenhuma
regra advisory bloqueia PR pequeno.
**Consequência.** Duas ferramentas (compilador + `cq check`), cada uma com papel claro.

---

## D11 — `Page` é o único construtor de resposta HTML

**Contexto.** Em Blade, "toda página usa o shell" é convenção testada varrendo views.
**Decisão.** `Page::new(ctx, main, status)` é o único caminho; `Markup` não implementa `IntoResponse`.
**Consequência.** Impossível publicar página sem base estrutural; gate `layout-contract` só confirma.

---

## D12 — Tema é dado; CSS é derivado e servido como arquivo

**Contexto.** Cor no banco como CSS quebra CSP, cache e acessibilidade.
**Decisão.** Tokens validados por schema → `/theme.css` canônico com `ETag`; rascunho de preview em
endpoint privado (`no-store`); cor de elemento por `SectionId`/`Tone` (identidade), nunca `style`.
**Consequência.** Mudança de tema em runtime sem deploy, sem inline, sem cache envenenado; contraste
verificado no save.

---

## D13 — JavaScript só como asset declarado

**Contexto.** Parcial autocontido tende a "carregar o script junto".
**Decisão.** Componente declara `const ASSETS: &[Asset]`; quem emite é o shell, via
`Ctx::require` (dedup + ordem de declaração). Estado vai por `data-*` tipado.
**Consequência.** Coesão preservada sem inline; CSP e `ETag` intactos; JS genérico e sem domínio.

---

## D14 — Byte é contrato: determinismo obrigatório

**Contexto.** `ETag`/304 e comparação de snapshot dependem de bytes estáveis.
**Decisão.** Ordem canônica em atributos, `srcset`, classes, assets e regras de CSS; teste de
determinismo em todas as fixtures.
**Consequência.** `render(x) == render(x)` é invariante testado; minificação em runtime é proibida.

---

## D15 — Congelamento por minor; quebra é major

**Contexto.** O consumidor precisa de um ponto único de atualização, com diff legível.
**Decisão.** Dentro de uma minor só entram adições; qualquer mudança que faça código válido parar de
compilar é major, com nota de migração; deprecação por uma minor inteira.
**Consequência.** Atualização = `cargo check` + `cq fix` + revisão de snapshot.

Ampliação (revisão do corpus `eta-templates`): **nenhuma entrada do vocabulário aceita string de
template** — nem `include(path)`, nem `layout` por nome, nem delimitador customizado. Os quatro pontos
de entrada do Eta/EJS que recebem string (`include`, `layout`, `block`, `output`) existem aqui como
`enum` fechado; template vindo de dado não é "risco a documentar", é impossível de construir.

---

## D16 — Esquema 2 da spec: `required` só de atributo, elemento multi-categoria

**Contexto.** A revisão do `spec/elements.toml` encontrou defeitos objetivos: `required` misturava
atributo com filho (`table` exigia `caption`, `video` exigia `track`), a allowlist não continha os
próprios atributos obrigatórios (`img` sem `src`/`alt`), `category` única tornava `<p>texto <a></a></p>`
impossível, `<header>` proibia `<h1>`, e `role`/`aria-*` como curinga reabria o escape hatch que D4 fecha.
**Decisão.** Esquema 2, aplicado de uma vez e versionado em `[meta] spec_version = "2.0.0"`:
`required` só de atributo e **implicitamente permitido**; `required_children`/`first_child` para
estrutura; `accessible_name` substitui o pseudo-atributo `label`; `category` aceita array; a categoria
`text` nomeia os elementos que só aceitam texto; `role` livre é proibido (só enum por elemento).
**Consequência.** O validador de referência (`tools/validate_spec.py`) roda limpo e é a barreira até
`cq spec` existir. Toda mudança de esquema exige um ADR aqui + bump de `spec_version`.

---

## D17 — O corpus `_RUST/*` é referência, não dogma

**Contexto.** A auditoria cruzada dos 14 corpus contra a especificação viva encontrou erro factual
(níveis WCAG trocados: 3.3.7 é nível A, 3.3.8 é AA) e divergências de versionamento de ferramentas.
**Decisão.** O que este projeto entrega vem de `docs/`; o corpus serve para vocabulário e para a
matriz de "o que já existe no ecossistema". Quando divergirem, vale o `docs/` + a referência normativa
(W3C/MDN), e a divergência é registrada em [`14-alternativas.md`](14-alternativas.md) e
[`18-aprendizados/`](18-aprendizados/README.md).
**Consequência.** Copiar do corpus sem citar é proibido; cada afirmação normativa aponta a fonte.

---

## D18 — `hx-*` não entra no vocabulário (o conceito entra)

**Contexto.** O corpus `htmx-4` (2.380 linhas) foi lido inteiro. 34 achados, e o mais consequente não é
um atributo: metade do conjunto depende de DSL de string (`hx-status:NNN`, filtros `[expr]`, `js:`),
de `window.confirm`/`prompt`, ou de `hx-on` — todos incompatíveis com CSP sem nonce e com 3.2.2.
**Decisão.** Nenhum `hx-*` compila no v1. O conceito (HTML-over-the-wire, uma rota servindo shell e
fragmento) é o modelo do projeto e é escrito como contrato em [`07`](07-js-e-assets.md) §7: `Vary`
coerente com o handler, `304` só depois de `200`, `aria-busy` + foco pós-swap, matriz de status,
`id` nunca duplicado em retarget.
**Consequência.** Zero bytes e zero dependência no caminho padrão; o consumidor que quiser htmx o
vendoriza como asset declarado, e adotar `hx-*` no vocabulário exigiria um perfil `spec/htmx.toml`
com allowlist fechada — mudança de major, com decisão registrada.

---

## D19 — Atributo booleano é presença, e o tipo é `bool`

**Contexto.** Nos corpus `topcoat` e `lit`, o mesmo defeito aparece: valor de atributo booleano tratado
como string (`disabled="false"`, `switch="false"`), que em HTML é **verdadeiro**. `Attributes` como
`HashMap` de string agrava — qualquer string é aceita e a ordem se perde.
**Decisão.** `[meta] boolean_attributes` é lista fechada na spec; o tipo do campo é `bool`; o gerador
emite só o nome do atributo. `hidden` fica fora da lista porque aceita `until-found` (enum próprio).
**Consequência.** `disabled="false"` passa a ser erro de compilação, não bug silencioso; a allowlist de
atributos continua ordenada (D14) em vez de virar mapa.

---

## D20 — Sem custom elements (Lit/Web Components ficam fora)

**Contexto.** O corpus `lit` (5.796 linhas) é o caminho mais direto para reatividade e composição por
slot — e o único jeito de SSR que "funciona sem JS" emite Shadow DOM com `<style>` dentro de
`<template shadowroot>` (`11-ssr.md:77-79`), além de exigir `customElements.define` e um toolchain TS
cuja falha de configuração é silenciosa. Zero ocorrências de `:defined`/`whenDefined`/upgrade em 5.796
linhas: o corpus não tem defesa para o elemento indefinido piscando.
**Decisão.** Nenhum elemento personalizado, `attachShadow`, `<slot>`, `part`/`exportparts`. O que CE
ofereceria já existe por `Markup` + `enum Slot` (composição), `/theme.css` (tema por token) e `Msg`
(texto por chave). Gate `no-custom-elements` na saída e no bundle (D20 → [`12`](12-testes.md) §9).
**Consequência.** Sem WASM, sem hidratação, sem FOUC de tag indefinida; a composição continua sendo
enum fechado no servidor. Mudar isso exige área autenticada fora do requisito de cache/sem-JS **e** um
DSD que não emita `<style>` inline — decisão nova, não configuração.

---

## D21 — Toda invariante declara escopo: `block` (macro), `type` (construtor) ou `output` (teste)

**Contexto.** A spec declarava 67 invariantes por elemento, sem distinguir as que um bloco `html!`
consegue decidir das que só o HTML final responde. `single_h1`, `main_unique`, `landmark_names_distinct` e mais cinco eram anunciadas como garantia de compilação — mentira útil: um
fragmento não sabe quantos `<h1>` a página tem. O sintoma clássico é a regra que "existe" e nunca
dispara.
**Decisão.** `spec/elements.toml` → `[invariants]` mapeia nome → `block` | `shell` | `output`
(**60**, **5** e **3** hoje). `shell` é o escopo do tipo `Page`: não é o macro, é a ausência de
alternativa que recusa (`main` único, `h1` único, `title` obrigatório).
`tools/validate_spec.py` falha se uma invariante usada não declarar escopo, ou declarar valor inválido.
A macro só emite erro para `block`; `page` é cobrado pelo `N4` sobre a saída. A distinção está em
[`04-validacao.md`](04-validacao.md) §4.0.
**Consequência.** Nenhuma regra fica órfã: ou o compilador recusa, ou um teste falha. Prometer
compilação para regra de página passa a ser erro de processo detectável, não desatenção.

---

## D22 — Elementos de documento só existem no `Page`; `html!` os recusa

**Contexto.** [`06-shell-e-tema.md`](06-shell-e-tema.md) §1 afirmava que "um handler não consegue
devolver HTML sem passar pelo shell", mas nada impedia `html! { html { … } }`: a garantia era
convenção. Ao mesmo tempo, `Page` aparecia como se fosse do crate e implementava `IntoResponse` —
dependência de framework que o desenho proíbe.
**Decisão.** `html`, `head`, `body`, `title`, `meta`, `link`, `script` e `noscript` marcam
`only = "shell"` no spec e carregam a invariante `shell_only_elements` (escopo `block`): escrever
qualquer um deles em `html!` é erro de compilação. A base estrutural vive em `html-core::shell`
(`Page::new(&ShellData, main)`), sem `http` e sem `IntoResponse`; o `impl IntoResponse` são 6 linhas
do consumidor.
**Consequência.** A promessa estrutural passa a ser do compilador; o crate deixa de ter qualquer
dependência de framework, e o shell fica testável sem servidor.

---

## D23 — `<form>` só existe via `Form::open`, que exige o token CSRF

**Contexto.** No corpus de origem são **142 `@csrf` + 57 `@method`** espalhados por views: o token é um
campo oculto que cada formulário lembra de escrever. Esquecer não quebra layout — quebra segurança.
**Decisão.** `[element.form]` recebe `only = "component"`: `html!` recusa `<form>`. O único construtor é
`html-core::form::Form::open(action, method, children)`, com o token dentro do método
(`Method::Post(token)`), que emite `_token` e, para `Put`/`Patch`/`Delete`, o `_method` oculto. O crate **não gera** token (sessão é do consumidor); exige um. Invariante
`form_requires_csrf`, escopo `type` ([`04`](04-validacao.md) §4.1).
**Consequência.** Formulário sem CSRF deixa de ser possível — não por disciplina, por assinatura.

---

## D24 — Contribuições para o shell são valor devolvido (`View`), nunca pilha ambiente

**Contexto.** `@stack`/`@push` (5 regiões no corpus) permitem que um `<dialog>` de um formulário quatro
níveis abaixo apareça no rodapé do shell. Em Rust, a tradução literal seria `&mut` atravessando toda a
cadeia ou um `thread_local` — ordem de flush implícita e estado global.
**Decisão.** `View { main, modals, assets }`, combinável (`View::merge`); o fragmento devolve
`View::new(f) .modal(d)`; `Page::new(&ShellData, View)` consome uma vez, na ordem do shell. `assets` é
`Assets` (dedupe e ordem de declaração, sem inferência).
**Consequência.** A ordem de emissão é a declarada; `assert!(view.modals.len() == 1)` é
teste; `Page` continua função pura.

---

## D25 — `Field` + `FieldState` derivam todo `aria-*` de campo

**Contexto.** `old()` (188 usos), `$errors` (204) e `@error` (114) alimentam, à mão, `aria-describedby` e
`aria-invalid` em **dois componentes duplicados** (`form-group`, `select`), além de repetirem
`str_replace(['[', ']', '.', ' '], …)` para derivar id.
**Decisão.** `FieldState { valor: AttrVal, erro: Option<Msg>, ajuda: Option<Msg> }` e
`Field::text(FieldName, Msg, &FieldState)`: o construtor deriva `id`/`for`/`aria-describedby`/`aria-invalid`/
`required` e o `*` fora do `<label>`. `FieldName::new` valida a sintaxe de nome indexado. A fonte do estado
(`old()`/`$errors`) continua no consumidor, **em um lugar** (`FormState::from_request`).
**Consequência.** O par valor+erro deixa de ser dois caminhos que podem divergir; `aria-*` de campo não se
escreve à mão.

---

## D26 — `Class` é combinável; attribute bag não existe; `data-*` fecha por gate

**Contexto.** 478 usos de `$attributes` + 15 `->merge()`: o componente repassa atributo que não conhece.
É o único ponto em que o Blade é genuinamente mais flexível que um vocabulário fechado.
**Decisão.** (a) `Class::of(&[Tone])` + `Class::extend(Class)` cobrem o caso legítimo (classe do chamador
somada à do componente). (b) **Não** existe bag de atributos: passthrough é parâmetro tipado. (c) `data-*`
tem o nome validado pelo macro (sintaxe) e o **conjunto fechado** verificado por `tools/data-check.py`
(tabela ↔ uso ↔ leitor no JS) — o conjunto pertence ao JS do consumidor e não entra no crate.
**Consequência.** 478 → ~40 pontos de uso (classe), e um `data-*` inventado falha no gate, não em produção.
O preço é enumerar passthrough; o benefício é que o vocabulário continua fechado.


## D27 — A tabela de elementos: TOML normativo, gerador uma vez, paridade testada

**Contexto.** `build.rs` + esquema 2 + gerador em tempo de build custavam caro e falhavam em silêncio
(o próprio repositório anunciou "63 elementos" com 97 na tabela). Mas o `spec/elements.toml` é o único
artefato **neutro** — é o **L0** que torna a especificação portável ([`24`](24-exportacao.md) §1).
**Decisão.** (a) O TOML **continua normativo**: é a especificação, não insumo de build. (b) O gerador sai
do `build.rs` e vira `tools/gen_spec.py`, rodado **manualmente** quando a spec muda, emitindo
`crates/html/src/spec/table.rs` **versionado**. (c) Um teste de paridade falha se TOML e tabela
divergirem: a checagem deixa de ser disciplina e passa a ser teste. (d) `tools/validate_spec.py`
permanece o validador da spec até F3 cobri-lo ([`21`](21-implementacao.md) F1).
**Consequência.** Zero `build.rs`, zero esquema, zero código gerado em tempo de build — e o artefato
neutro sobrevive. O preço é um comando manual por mudança de spec, coberto pelo teste de paridade:
esquecer de rodar deixa o CI vermelho, não o produto errado.

## D28 — Não existe binário de tarefas

**Contexto.** O `cq` acumulava cinco papéis; quatro (`fmt`, `fix`, `audit`, `spec`) foram cortados na
revisão e o quinto (`check`) dependia da camada advisory, também cortada (A2–A5).
**Decisão.** Não há binário. O gate é `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings`
+ `cargo test`, duas vezes seguidas.
**Consequência.** −700 LOC e um artefato a menos. O consumidor já tem o próprio `make check`; nada aqui
compete com ele.

## D29 — Tema e shell de consumidor ficam fora do crate

**Contexto.** Emitir CSS de tema exige I/O (arquivo servido, `ETag`, cache) que o crate declarou não
fazer ([`13`](13-decisoes.md) D11/D12 e [`20`](20-fronteira-de-aplicacao.md)).
**Decisão.** O crate expõe `Page`, `ShellData`, `View`, `Form`, `Field`, `Class`; cores, CSS servido,
`ETag` e cache pertencem ao consumidor.
**Consequência.** Crate puro e vendorizável; o tema continua sendo **dado do consumidor**, não código
de biblioteca. A implementação de referência no consumidor e o desenho hipotético dentro do crate
estão em [`26`](26-d29-consumidor.md).

## D30 — `N4` é módulo atrás de feature, não crate nem binário

**Contexto.** O `M3` exige o lint rodando "no CI do consumidor sobre páginas reais"
([`16`](16-roadmap.md) §2); um `#[cfg(test)]` interno não é alcançável pelo consumidor, e um crate
separado é mais um artefato para manter.
**Decisão.** `html` ganha `[features] audit` com `html5ever` como dependência **opcional**;
`html::audit::check(&str) -> Vec<Finding>` é a superfície pública. Sem a feature, o crate não tem
nenhuma dependência.
**Consequência.** O consumidor roda o lint nas páginas dele sem binário e sem crate novo; a feature fica
fora do binário de produção por padrão.

## D31 — `Msg` é trait do consumidor; o crate não conhece locale

**Contexto.** Duas versões coexistiam: `docs/02` §6 com `enum Msg` gerado **no crate** e resolvendo
"pelo locale da request", e [`21`](21-implementacao.md) F0 com `trait Msg`. A primeira exige estado
ambiente de locale dentro do crate — proibido pelo rigor ([`22`](22-rigor.md) §3: nada de `env`, nada
de global).
**Decisão.** `pub trait Msg: Copy { fn as_str(self) -> &'static str; }`; o enum de mensagens é do
consumidor; `Text::msg(m)`, `Label::text(m)`/`sr_only(m)` recebem `M: Msg`. **Não** existe
`impl<M: Msg> From<M> for Text` (colidiria com `From<&str>` se o consumidor implementasse `Msg` para
`&str`).
**Consequência.** O crate permanece puro e testável com um `Msg` de teste; catálogo, locale e paridade
dos 4 locales são do consumidor.

## D32 — Toolchain: pin `1.99`, MSRV `1.92`

**Contexto.** Duas coisas distintas que estavam misturadas: o compilador com que se desenvolve (pin) e o
compilador mínimo que o crate promete (MSRV).
**Decisão.** `rust-toolchain.toml` fixa `1.99` + `clippy` + `rustfmt`; `rust-version` = MSRV **1.92** no
`Cargo.toml`, em `clippy.toml` (`msrv`) e num job de CI que compila **em 1.92**; `resolver = "3"` para a
resolução de dependências respeitar a MSRV.
**Consequência.** Build idêntico em toda máquina e MSRV verificada, não declarada. O pin sobe com a
stable; a MSRV só sobe em major (D15).

---

## Emendas da revisão de 2026-11-06

Decisões anteriores que **mudam de leitura** por causa de D27–D32 (o texto original fica como registro):

| Decisão | Emenda |
|---|---|
| **D5** — spec é dado versionado | `build.rs` não gera mais nada: o gerador vira `tools/gen_spec.py`, com saída versionada e teste de paridade (**D27**) |
| **D6** — um parser, três passes | o terceiro passe (`fmt`) sai: a macro ignora whitespace e a saída não depende do fonte (**D28**) |
| **D10** — tipo primeiro, advisory com ratchet | a camada advisory e o `cq check` saem; fica "ou é erro de compilação, ou é teste que falha" (**D28**) |
| **D15** — congelamento por minor | a atualização deixa de citar `cq fix`: é `cargo check` + revisão de snapshot (**D28**) |
| **D25** — `Field` deriva todo `aria-*` | `FieldState` carrega `Text` **resolvido** (`erro`/`ajuda`), não `Msg`: com D31, `Msg` é trait e o enum é do consumidor (**D31**) |
| **D16** — esquema 2 da spec | a barreira é `tools/validate_spec.py` até F3; não existe `cq spec` (**D27/D28**) |

## D33 — Flexibilidade de HTML válido não é escape hatch

**Contexto.** A primeira porta do `EXAMPLE.html` para `html!` custou 20 erros de compilação e
8,6 KB de HTML cru em quatro `impl Render`: `<div>` dentro de `<a>` (válido, mas `a.children =
["phrasing"]`), `title` em `button` (atributo global, mas ausente de `[globals]`), `h1` no hero
(o shell sempre emitia o seu) e `href="#"` (não havia tipo para fragmento). O `eta` mostrou o
contrário: superfície pequena, defaults explícitos e dados + loop no template.

**Decisão.** (a) `a` declara `transparent = true`: herda o content model do ancestral não
transparente e proíbe interativo em qualquer profundidade. (b) `title` entra em `[globals]` como
`Text`. (c) `Href::Fragment` valida `#`, `#id`. (d) `h1` deixa de ser `only = "shell"`; `Page`
continua emitindo por padrão, mas a view pode declarar o heading. (e) `single_h1` passa de `type`
para `output`; `shell_owns_h1` é removida. (f) O demo troca os quatro `Render` por `SERVICES`/
`CONTROLS` + `for` dentro de `html!`. (g) `class="a b"` estático é aceito e canonicalizado no
codegen; `Class::new("a b")` cobre o valor estático. Texto visível continua explícito em
`Text`/`Msg`; literal não é aceito.

**Consequência.** Menos falso-positivo, menos string crua e uma única forma de escrever cada
destino. O preço é explícito: o `h1` único deixa de ser garantia de tipo e vira auditoria de
documento (`audit`); um consumidor que usa `Page` mantém a garantia, um shell próprio assume a
contagem. A versão do vocabulário sobe para `2.1.0`; o crate entra em `0.1.0`.

## D34 — Assets, root e `style` controláveis sem quebrar o contrato

**Contexto.** O mundo real ordena CSS/JS manualmente, usa CDN, precisa de `class`/`data-*` no
`<html>`/`<body>` (dark mode, `data-bs-theme`) e, ocasionalmente, de `style=` inline. O crate tratava
tudo como se só existisse asset interno e CSS antes de JS.

**Decisão.** (a) `Assets` é uma lista na **ordem de declaração**, com dedupe; `Page` emite shell
primeiro e depois os assets da view, sem inferir espécie. (b) `Asset` ganha `external_style`,
`external_script`, `external_classic_script` e `classic_script`; CDN é aceito, mas documentado como
não recomendado. (c) `RootAttributes` permite `class` e `data-*` validados no `<html>` e no `<body>`.
(d) `style=` inline é permitido como escape hatch: o valor é escapado, `audit` sinaliza e o CSP deixa
de ser `style-src-attr 'none'`. `style` continua sem virar `Token`; a contra-recomendação é explícita.
(e) `ClassList::try_new` é a porta opt-in para classes dinâmicas; valida token, ordena, deduplica e
não substitui `Class`/`ClassToken`.

**Consequência.** O consumidor controla cascata, CDN, tema global e valores calculados sem pedir
mudança no crate. O preço é assumido: ordem manual pode errar, CDN exige CSP/SRI do consumidor,
`style=` pode injetar CSS se receber dado não confiável e `ClassList` não conhece o design system.
