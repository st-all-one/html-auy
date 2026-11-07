# 07 — JavaScript e assets

## 1. Regra única

**JavaScript nunca mora no markup.** Vive em arquivo estático, compilado com hash de conteúdo, e é
**declarado por tipo** pelo Rust. O vocabulário do `html-auy` emite
`<script type="module" defer src=…>` (ou clássico, sem `type="module"`) e
`<link rel=stylesheet href=…>`. `<script>` inline e `on*` continuam inexistentes. `style=` inline
existe como **escape hatch** (ver §5): `audit` sinaliza e o CSP precisa de `style-src` compatível.

Consequência: CSP `script-src 'self'` sem `unsafe-inline`/`unsafe-eval`; com `style=` inline, a
política de estilo deixa de ser `'none'` e passa a exigir uma decisão explícita.

## 2. Tipos

```rust
// html_auy::shell — asset é (tipo, URL validada, ordem de declaração); hash é do consumidor
pub struct Asset { /* privado */ }
impl Asset {
    pub fn style(href: &'static str) -> Option<Asset>;             // `/assets/app.css`
    pub fn style_media(href: &'static str, media: Media<'static>) -> Option<Asset>; // `media="print"`
    pub fn script(href: &'static str) -> Option<Asset>;            // `/assets/app.js`, módulo
    pub fn classic_script(href: &'static str) -> Option<Asset>;    // `/assets/app.js`, clássico
    pub fn external_style(href: &'static str) -> Option<Asset>;    // `https://cdn…/app.css`
    pub fn external_script(href: &'static str) -> Option<Asset>;   // CDN, módulo
    pub fn external_classic_script(href: &'static str) -> Option<Asset>; // CDN, clássico
}
pub struct Assets { /* Vec na ordem declarada, com dedupe */ }

// consumidor: a ordem é a do código, não a do tipo nem a do alfabeto
fn base() -> Assets {
    [
        Asset::external_style("https://cdn.example/reset.css"),
        Asset::style("/assets/app.css"),
        Asset::external_classic_script("https://cdn.example/bootstrap.js"),
        Asset::script("/assets/app.js"),
    ]
    .into_iter()
    .flatten()
    .collect()
}
```

O shell emite os assets **na ordem em que foram definidos**, misturando `link` e `script` sem
inferência. Se a cascata exige reset → tema → componentes → utilities, essa é a ordem da lista (ou
um único CSS com `@layer`). Dedupe é por igualdade de `(tipo, URL, media, módulo)`: repetir não
move a posição.

```rust
let view = home_editor(&data)                // a view declara o que exige…
    .asset(Asset::script("/assets/home.js").expect("asset")); // …e o `View` deduplica
let shell = ShellData::new(title, heading, lang).assets(base()); // base do documento
Page::new(&shell, view)                      // shell primeiro, depois assets da view, na ordem
```

CDN é aceito por construtor próprio (`external_style`/`external_script`, `http`/`https` validados)
e é **desencorajado** em produção: self-host dá CSP estrita, SRI e cache sob seu controle.

Mídia de conteúdo (`<img>`, `<picture>`, `<video>`, `<audio>`) **não** é `Asset`: ela carrega dados do
documento e usa `Src`/`SrcSet`/`Alt` em [`media`](../crates/html-auy/src/media.rs), sem hash de build.

## 3. Onde o JS passa a ler o estado

Markup emite `data-*` **tipados**; o JS é genérico e burro sobre eles. O valor é sempre **escalar**
(string, número, enum) — nunca JSON serializado no atributo: atributo com payload é estado de
hidratação disfarçado, dobra o HTML e volta a exigir parser no cliente.

| Atributo | Valores | Consumidor |
|---|---|---|
| `data-a11y-contrast` | `high`\|`low` | `Shared` |
| `data-a11y-font-scale` | `1`..`1.5` | `Shared` |
| `data-mask` | `cpf`\|`cnpj`\|`phone`\|`money` | `Shared` |
| `data-select-search` | `""` | `Shared` |
| `data-field-help` | id do tooltip | `Shared` |
| `data-content-actions` | `""` | `Shared` |
| `data-batch-upload` | URL do endpoint | `GalleryBatch` |
| `data-editor` | `rich`\|`home` | `Editor`/`HomeEditor` |
| `data-analytics` | ID configurado | `Analytics` |

O JS não conhece domínio: o mesmo `Shared` serve notícias, licitações e o que vier.

## 4. Bundles de referência (dados medidos no escopo de origem)

| Bundle | Arquivos de origem (loc) | Carrega em |
|---|---|---|
| `shared` | a11y 142 · form-mask 217 · form-progress 177 · select-search 229 · search-input 53 · cms-field-help 117 · content-actions 25 (≈ 960) | portal + CMS |
| `cms` | shell do CMS 374 | shell do CMS |
| `editor` | editor 196 · toolbar 170 · dialogs 301 · schema 239 · preview 51 (≈ 957) | páginas com editor |
| `gallery-batch` | batch 428 | galeria |
| `home-editor` | home-editor 45 | composição da home |
| `analytics` | analytics 28 | portal, opt-in |

Total ≈ 2.804 LOC — o mesmo que já existe. O que muda é o **acoplamento**: o bundle é declarado na
montagem, não empurrado por `@push` em qualquer view.

## 5. Fronteira com CSP

| Permitido | Proibido | Motivo |
|---|---|---|
| `<script src defer>` de asset com hash | `<script>` inline | CSP, `ETag`, cache |
| `<link rel=stylesheet href>` de asset com hash | `<style>` | CSP, cache |
| `style=` inline confiável | `style=` com dado não confiável | escape hatch auditado; CSP muda |
| `<script type="application/ld+json">` | script com conteúdo executável | dado ≠ script |
| `addEventListener` em módulo | `onclick=`/`hx-on=`/`js:` filters | exige `eval`/inline |
| `fetch`/`form` nativos | `Function`/`eval` | CSP |

`script-src 'self'`; `style-src 'self'`; `object-src 'none'`; `base-uri 'none'`. Sem `nonce`, porque
o caminho padrão não tem inline — e uma página cacheável não pode carregar nonce. CDN externo exige
`script-src`/`style-src` com o host explícito; `style=` inline exige `style-src-attr`
(`'unsafe-inline'` ou `'unsafe-hashes'`).

- **`new Function`/compilação de template no cliente é proibido.** Template que vira código em
  runtime exige `'unsafe-eval'` e foge do hash de asset: o mesmo arquivo passa a ter comportamento
  diferente conforme o dado. Ver D2 e [`08`](08-seguranca.md) §6.

## 6. JSON-LD é dado, não script

```rust
let g: Graph = seo_graph(&meta);
ShellData::new(title, heading, lang).schemas(vec![JsonLd::graph(&g)]);
// o `Page` emite, no fim do `<head>`: <script type="application/ld+json">{"@context":…,"name":"A\u003cB"}</script>
```

- `JsonLd` escapa `<` `>` `&` `'` para `\u003c` `\u003e` `\u0026` `\u0027` **sempre**.
- `<script type="application/ld+json">` não é executável: fora do escopo de `script-src`, mas
  **dentro** do escopo do escape (um `</script>` em um título quebraria a página).
- Validar o JSON-LD (parse no `audit`) é pós-MVP: hoje o gate cobre o escape, e o consumidor valida o grafo.

## 7. Fragmento e swap (o que vale com ou sem htmx)

O vocabulário **não** inclui `hx-*`: nenhum `hx-*` compila no v1 (D18). O que o corpus `htmx-4`
provou é que o *conceito* (HTML-over-the-wire) já é o nosso modelo — o servidor devolve HTML
completo ou fragmento, nunca JSON para o cliente renderizar. As regras abaixo valem para qualquer
mecanismo de troca parcial que o consumidor declare como asset:

1. **Duas formas, uma rota.** Toda rota com resposta parcial responde ao pedido normal (shell +
   fragmento) e ao pedido de fragmento; o consumidor modela com `Page` ou `Partial`. A mesma
   rota, sem URL paralela, para que a navegação sem JS funcione.
2. **`Vary` casa com o que o handler lê.** Resposta que difere entre fragmento e completa:
   `Vary: HX-Request`. Se o handler **roteia por alvo** (lê `HX-Target`), é
   `Vary: HX-Request, HX-Target`. Resposta única para os dois clientes (`hx-select` no cliente)
   dispensa `Vary` — mas então não há economia de bytes, só de swap.
3. **`304` só depois de um `200`.** `ETag` é sempre o `sha256` dos bytes finais
   ([`09`](09-desempenho-e-cache.md) §2); `mtime` como `ETag` é proibido (não sobrevive a deploy) e
   fragmento nunca responde `304` na primeira carga — região vazia é pior que byte repetido.
4. **Foco e anúncio pós-swap são contrato, não bônus.** `aria-busy="true"` na região durante a
   requisição (removido pelo swap), foco movido para o título da região depois, `aria-live="polite"`
   só na região de status persistente do shell ([`05`](05-acessibilidade.md) §2, 4.1.3). O corpus
   tem **uma** menção de acessibilidade em 2.380 linhas: nada disso vem de graça.
5. **`id` duplicado é erro de render.** Se a resposta usa `HX-Retarget`, o fragmento **não** repete
   o `id` do alvo; dois elementos com o mesmo `id` no DOM falham o lint de acessibilidade.
6. **Matriz de status explícita.** `2xx` troca; `204`/`304` não trocam; `4xx`/`5xx` não trocam o
   conteúdo principal e renderizam erro **dentro** da região, com `role="alert"`. Nada de
   `hx-status:NNN` (DSL de string com 6 campos) no vocabulário.
7. **Nada de expressão avaliada.** `hx-on`, `hx-vals="js:"`, filtros `[expr]` e `js:` são proibidos
   (exigem `eval`, colidem com CSP sem nonce — [`08`](08-seguranca.md) §6). Enter vem de `<form>` +
   submit; confirmação vem de `<dialog>`; prompt vem de campo no próprio formulário.
8. **Gatilho de escrita só em `submit`.** `change`/`blur` que dispara troca de contexto viola WCAG
   3.2.2; quando existir, é com `swap: none` e feedback em região `role="status"`.
9. **Configuração sem inline.** Se o consumidor vendorizar o htmx, a configuração vai em
   `<meta name="htmx-config">` com JSON no `<head>`, antes do `<script src>`; `htmx.config.*` por
   script inline é proibido.
10. **`axe` depois do swap** é obrigatório no e2e — senão o gate fica verde com foco perdido.

### 7.1 Contrato de resposta parcial

O crate não emite `hx-*`, mas o consumidor precisa de um modelo de resposta que não seja string
livre. `html_auy::Partial` é esse modelo; ele **não** renderiza nem escolhe transporte — o adaptador do
consumidor serializa para OOB, `<hx-partial>` ou outro mecanismo.

- `Partial::new(Markup)` substitui o alvo principal; `Partial::keep_main()` preserva o alvo e só
  aplica atualizações. `MainSwap` é explícito, sem `swapEmpty`.
- `Partial::update(Id, Swap, Markup)` acrescenta uma região; a ordem de `updates()` é a ordem de
  aplicação. `Swap` cobre `Inner`, `Outer`, `BeforeBegin`, `AfterBegin`, `BeforeEnd`, `AfterEnd`,
  `Delete` e `None`; não existe string de swap.
- `select`, `reselect` e `select_oob` são `Id` tipados; permitem o caminho “corpo único para
  navegador e fragmento” sem `Vary` (`hx-select`/`HX-Reselect`/`hx-select-oob`).
- `focus` aponta para o título/controle que recebe foco depois do swap; nunca para o gatilho.
- `RequestClass::vary` fixa a matriz: `Full` sem `Vary`; `Partial` `HX-Request`; `Targeted`
  `HX-Request, HX-Target`; `HistoryRestore` inclui `HX-History-Restore-Request`; `Boosted` inclui
  `HX-Boosted`.
- `Sync` existe como enum fechado `Drop`, `Replace`, `Abort`. `Queue` não entra: fila pode aplicar
  resposta antiga depois da nova.

#### 422 é re-render do formulário

`4xx` não deve apagar o que o usuário digitou. O contrato do consumidor é:

- `422`: renderizar o **mesmo** `<form>`, com `FieldState` preenchido com o valor submetido,
  `erro` por campo, `aria-invalid`/`aria-describedby` e um resumo `role="alert"` com links para os
  `id`s `f-…`. O status continua `422`; o corpo é o formulário, não uma página de erro genérica.
- `400`: requisição malformada; `409`: conflito de estado/replay divergente; `303`: sucesso.
- `SubmitKey`/`Form::open_with_submit` emite `_submit` nos métodos não-GET. O servidor trata a chave
  como uso único: replay com o mesmo corpo devolve o resultado original (`303`); corpo diferente
  devolve `409`.

#### Fronteira do swap

O gatilho deve ficar **fora** da região que ele substitui; formulário não deve ficar dentro de região
substituída. `aria-busy="true"` fica na região durante a requisição e é removido pelo swap. Não usar
`hx-preserve`/morph para “resolver” foco/caret: o próprio htmx documenta que `input` perde foco e
caret (`hx-preserve.md`), e o DOM deve continuar projeção do último render.

#### Configuração segura do htmx

Se o consumidor vendorizar htmx, `ShellData::htmx_config(HtmxConfig::safe())` emite
`<meta name="htmx-config">` **antes** dos assets, com `allowEval:false`, `allowScriptTags:false`,
`includeIndicatorStyles:false`, `selfRequestsOnly:true`, `withCredentials:false`,
`historyCacheSize:0` e `timeout:10000`. Sem essa chamada, nenhum byte de htmx é emitido.

## 8. Pipeline de asset

```
css (tailwind + daisyUI)  ─┐
js  (esbuild, ESM)        ─┼─▶ hash sha256 do conteúdo ─▶ nome.a1b2c3.css/.js
fonts (woff2, subset)     ─┤                              ─▶ .br/.gz pré-comprimidos
icons (svg inline)        ─┘

/sw.js  ·  /manifest.webmanifest   ──▶ URL FIXA, sem hash, com ETag
```

- O shell aceita CDN por construtor explícito, mas o pipeline recomendado é vendorizar: sem `latest`, tudo com hash no repositório do consumidor.
- **Dois assets não levam hash no nome**, e o motivo é técnico: `/sw.js` é atualizado por
  *diferença de bytes na mesma URL* (o navegador só troca o worker se o byte a byte do script mudou) e
  o manifest tem `start_url`/`scope` resolvidos contra a URL dele — hashear mudaria a base relativa.
  Os dois são servidos com `ETag` do corpo e `Cache-Control: no-cache`.
- Nome com hash é para o que é **imutável**: `Cache-Control: public, max-age=31536000, immutable`.
- `integrity` (SRI) obrigatório para asset **enviado pelo usuário** (ex.: painel de suporte).
- Asset novo que não esteja no registro falha o **teste de assets** do consumidor (evita 404 silencioso).

### 8.1 PWA — a fronteira

Entra na crate: `link rel="manifest"` via `ShellData::manifest(LUrl)` (já emitido, com URL fixa e
`ETag` no consumidor). `meta name="theme-color"`, `meta name="apple-mobile-web-app-capable`, os ícones
(192/512/maskable) e a referência ao `/sw.js` continuam no roadmap tipado, alimentados pelo tema (D12) —
`theme_color`/`background_color` são o terceiro e o quarto consumidores do token, nunca configuração
paralela.

Não entra: o service worker em si (estratégia de cache, `skipWaiting`, precache). Ele é arquivo do
consumidor, servido na URL fixa, e a crate só garante que ele **não** é hasheado e **não** é precacheado
pelo shell. `skipWaiting()` no install é proibido no guia: worker novo + cache podado + aba antiga =
chunk hasheado que já não existe (`404`); a troca acontece em release, com o cache antigo preservado
até a última aba sair.

## 9. VLibras e terceiros

- `Asset::ExternalOnDemand(Origem::Vlibras)` — carregado por `Shared` **após ação explícita** do
  usuário (nunca no load).
- Analytics: só quando um ID está configurado; emissor próprio, same-origin, sem inline.
- Qualquer outra origem externa exige decisão registrada e allowlist (do consumidor). Reprovar
  `src`/`href` externo no `audit` é pós-MVP.

## 10. Resumo do contrato

1. JS nunca dentro do markup; sempre asset com hash.
2. Declaração por tipo (`ctx.require`), ordem determinística.
3. Estado vai por `data-*` tipado; o JS não conhece domínio.
4. Nada de inline, nada de `eval`; CSP sem nonce.
5. JSON-LD é tipo, com escape fixo.
6. Parcial com `Vary` coerente com o handler, `aria-busy`/foco pós-swap e `axe` depois da troca.
7. O asset **não** injeta markup: nada de `innerHTML`, Shadow DOM, `<slot>` ou troca de tag de
   elemento existente. Quando ele muda estado, muda só o que o servidor já renderizou (D20).
