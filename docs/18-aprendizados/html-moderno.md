# `html-moderno` — HTML Living Standard (snapshot 2025)

Corpus de 23 arquivos (4.253 linhas) que compila o HTML Living Standard do WHATWG em forma consultável:
catálogo de tags e categorias de conteúdo, padrões modernos (dialog/popover/details/search),
acessibilidade, segurança, performance, `meta`/`http-equiv`/`script type`, atributos globais, constraint
validation, formatos de data/hora, modos de renderização, comentários e image maps. Serve como
**contraprova externa** do vocabulário de `html-auy`: cada linha de `spec/elements.toml` pode ser
confrontada com o que o padrão (e o corpus que o resume) afirma sobre aquele elemento, atributo e
modelo de conteúdo.

`spec/elements.toml` **foi reescrito durante esta auditoria** — de `schema = 1` (65 blocos, 532 linhas)
para `schema = 2` / `spec_version = "2.0.0"`. Avanço medido: 90 blocos / 720 linhas às 14:58 →
**91 blocos / 739 linhas às 15:05** (`sha256 33c500733e9f…`), com correções sendo aplicadas por outro
agente enquanto este texto era escrito. Duas consequências para quem lê:

- os achados valem para o **vocabulário** (o que falta), não para um snapshot congelado; a coluna
  "status" de §2.13 diz, achado por achado, o que já está corrigido na revisão `33c5007` e o que segue aberto;
- as referências `spec:NN-MM` foram tomadas na revisão das 14:58 e podem deslocar. **Localize pelo bloco**
  (`grep -A6 '^\[element\.meta\]'`), nunca pela linha.

O corpus também erra (um erro grave em `05_SECURITY.md:215`): ele é evidência, não autoridade.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00_INDEX.md` | 62 | Mapa dos 20 arquivos + "padrão mínimo do documento" | Referência da ordem do `<head>`; origem de A1 |
| `01_ALL_TAGS.md` | 208 | Catálogo por função com "Modelo de Conteúdo"/"Void" (~110 tags) e os 22 `input` types | Base da tabela §2.14 e da enum de `type` (A26) |
| `02_SEMANTIC_STRUCTURE.md` | 222 | 7 categorias, outline, landmarks, tabela de aninhamento, headings | Fonte das categorias e da regra "só filhos nomeados" (A7) |
| `03_MODERN_PATTERNS.md` | 343 | Web Components, dialog (`closedby`, Invoker Commands), popover, `details name`, `<search>`, forms, `srcset`, microdata | Fonte dos atributos que dispensam JS (A30, A38) |
| `04_ACCESSIBILITY.md` | 255 | ARIA, landmarks, labels, tabindex, `inert`, live regions, tabelas, checklist | Sustenta `accessible_name` e o rótulo implícito (A9) |
| `05_SECURITY.md` | 236 | CSP (inclusive por `meta`), SRI, `crossorigin`, `rel`, `sandbox`, clickjacking | Base de SRI/`sandbox`; contém erro em `frame-ancestors` (A19) |
| `06_PERFORMANCE.md` | 183 | `async`/`defer`, lazy, `fetchpriority`, resource hints, CLS | Ordem do head (A2) e invariante de preload (A22) |
| `07_INTERDEPENDENCIES.md` | 257 | Interfaces DOM, content vs IDL, eventos, papéis ARIA implícitos | Confirma "não emitir o que depende de JS" (§3) |
| `08_DEPRECATED.md` | 125 | 30 elementos + 30 atributos obsoletos + `rel` obsoletos | `type` em `ol` é obsoleto (A37) |
| `09_BOILERPLATE.md` | 357 | 4 templates completos (geral, mínimo, e-mail, PWA) | Contraprova prática: tudo ali deveria compilar |
| `10_DATE_TIME_FORMATS.md` | 72 | ISO 8601 na variação HTML (7 perfis), Y2K38, Y10K | Refuta `Timestamp` único para `datetime` (A34, A35) |
| `11_CONSTRAINT_VALIDATION.md` | 123 | Atributos por tipo, `pattern` sem flags, `validity`, pseudo-classes | Origem de A31/A32 e de `pattern_flags_forbidden` (§3) |
| `12_QUIRKS_STANDARDS_MODE.md` | 61 | Três modos; `<!DOCTYPE html>` é o único gatilho de no-quirks | A1: a crate não menciona doctype em lugar algum |
| `13_HTML_COMMENTS.md` | 65 | Sintaxe hostil (`-->`, `--!>`, aninhamento), contextos raw-text | Justifica não existir comentário no vocabulário (§3) |
| `14_IMAGE_MAPS.md` | 66 | `<map>`/`<area>`, `shape`/`coords`, `alt` por área, recomenda evitar | A13: `usemap` ausente; proibição a documentar |
| `15_DEFINE_TERMS.md` | 118 | `<dfn>`, `<abbr title>`, `<dl>` como glossário | A40 (`title` inacessível) e ausência de `dfn` |
| `16_SPECIFIC_ATTRIBUTES.md` | 146 | `capture`, `dirname`, `form`, `readonly`, `size`, `step`, `elementtiming` | Lista de atributos faltantes (§2.14) |
| `17_LINK_RELATIONS_EXTRA.md` | 68 | `manifest`, `me`, `alternate stylesheet`, `compression-dictionary` | A21 (`link` sem `title`) |
| `18_GLOBAL_ATTRIBUTES.md` | 241 | `accesskey`, `autocapitalize`, `dir`, `hidden`/`until-found`, `inert`, `inputmode`, `translate`, `spellcheck` | Fonte de `[globals]` e lacunas de A39 |
| `19_META_SCRIPT_TYPES.md` | 163 | `meta name` (color-scheme, referrer, robots, theme-color), `http-equiv`, `importmap`, `speculationrules` | A18/A19 (`meta`) e A20 (`script`) |
| `modern_html_example.md` | 784 | Página única com 60+ padrões + checklist | Melhor contraprova: cada linha é um caso de teste |
| `SKILL.md` | 95 | Resumo operacional (doctype, um `<h1>`, `alt` sempre) | Confirma o núcleo; nada novo |
| `VERSION` | 3 | `HTML Living Standard (WHATWG)` / `Snapshot: 2025` | Fixa a versão citada no cabeçalho |

## 2. Achados

#### A1. `<!DOCTYPE html>` não existe em nenhum lugar da crate
- Citação: `12_QUIRKS_STANDARDS_MODE.md:15` — "O `<!DOCTYPE html>` no início do documento ativa **no-quirks mode**."
- Semântica: o doctype não declara versão; sua única função é escolher o modo. `12:30`: "DOCTYPE incompleto ou ausente → **quirks mode**"; `12:36-37` mostra o teste (`BackCompat` = quirks).
- Consequência: `grep -rn "DOCTYPE" docs/ spec/ README.md` devolve **zero**. Sem doctype, `box-sizing`/`rem` saem errados e nenhum gate acusa — `html5ever` parseia documento sem doctype em silêncio.
- Veredito: **refina** — invariante de shell + regra N4 (§5.1).
#### A2. `head.first_child = "title"` conflita com charset-primeiro
- Citação: `09_BOILERPLATE.md:7-9` — "`<meta charset="UTF-8" />` … `<meta name="viewport" …>` … `<title>Título da Página | Site</title>`" (idem `modern_html_example.md:6-9`, `06_PERFORMANCE.md:139-141`).
- Semântica: o corpus põe `charset` e `viewport` antes de `title`; `06:135` chama o bloco de "Meta tags essenciais no topo do `<head>`".
- Consequência: `spec/elements.toml:74-78` exige `title` primeiro, e um campo `first_child` não expressa **sequência**; charset depois de ~1024 bytes força re-parse de encoding.
- Veredito: **refina** — `head_order` (§5.2).
#### A3. O corpus se contradiz sobre `header`/`footer`; a crate escolheu o lado certo
- Citação: `01_ALL_TAGS.md:37` — "`<footer>` … Sem heading/sectioning descendentes"; `01_ALL_TAGS.md:38` — "`<header>` … Sem footer/header descendentes".
- Semântica: se `01:37` valesse, o boilerplate do próprio corpus seria inválido — `09_BOILERPLATE.md:337` tem "`<h1>App Name</h1>`" dentro de `<header>`, e `09:222` tem "`<nav aria-label="Rodapé">`" dentro de `<footer>`.
- Consequência: `spec/elements.toml:127-142` proíbe só `header`/`footer` aninhados, que é a leitura correta do escopo `banner`/`contentinfo` de `02:129-130`. Não propagar a regra da linha 37.
- Veredito: **confirma** a crate; **refuta** `01_ALL_TAGS.md:37`.
#### A4. `body` classificado como `metadata`
- Citação: `01_ALL_TAGS.md:31` — "| `<body>` | Corpo do documento | Flow content | Não | Apenas um por documento |".
- Semântica: `02:56` lista `<body>` entre os *sectioning roots* ("`<body>`, `<blockquote>`, `<details>`, `<dialog>`, `<fieldset>`, `<figure>`, `<td>`"); não é metadata content.
- Consequência: `spec/elements.toml:80-84` declara `category = "metadata"`. Hoje inofensivo, mas qualquer regra "metadata só em head" passa a mentir.
- Veredito: **refina** — `category = ["sectioning"]`.
#### A5. `summary` não aceita heading, mas o corpus põe `<h3>` dentro
- Citação: `modern_html_example.md:268` — "`<summary><h3>Dia 1 — <time datetime="2025-09-15">15/09</time></h3></summary>`".
- Semântica: `summary` aceita phrasing content **ou um único heading**; é o disclosure cujo título é título de seção.
- Consequência: `spec/elements.toml:689-695` tem `children = ["phrasing"]`; o exemplo do corpus é erro de compilação e, em FAQ/agenda, o dev cai para `<span>`, perdendo os títulos no outline.
- Veredito: **refina** — `children = ["phrasing","heading"]` + `summary_single_heading` (§5.3).
#### A6. `noscript` não pode conter `link` (o uso clássico no head)
- Citação: `01_ALL_TAGS.md:132` — "| `<noscript>` | Fallback quando JS desabilitado | Transparent | Não | Metadata se em `<head>`, flow se em `<body>` |".
- Semântica: em `<head>` só aceita `link`/`meta`/`style`; em `<body>`, flow.
- Consequência: `spec/elements.toml:119-126` declara `children = ["flow","phrasing"]`; `<noscript><link rel="stylesheet"></noscript>` (`09:97`, `06:145`) é erro — `link` é `metadata`.
- Veredito: **refina** — `children = ["metadata","flow","phrasing"]` + `noscript_context`.
#### A7. Controles de formulário não são `phrasing` → não cabem em `p`/`li`/`td`
- Citação: `01_ALL_TAGS.md:172` lista `<input>` como "Vazio" na família Forms; `02_SEMANTIC_STRUCTURE.md:145` dá `<li>` como "Flow content".
- Semântica: no HTML, `label`, `input`, `select`, `textarea`, `button`, `output`, `progress`, `meter` são **phrasing content** (e portanto flow); `fieldset` é flow.
- Consequência: `spec/elements.toml:537-598` dá a eles só `["form"]`/`["form","interactive"]`; como `td`/`li` têm `children = ["flow"]`, `<td><input></td>` e `<li><label>…</label></li>` são **erros**. O cabeçalho da spec (l. 12-15) manda usar multi-categoria — a regra não foi aplicada aqui.
- Veredito: **refuta** — corrigir categorias (§5.4). Maior impacto depois de A10.
#### A8. `table` aceita `tr` direto e exige `tbody`: as duas regras se anulam
- Citação: `02_SEMANTIC_STRUCTURE.md:153` — "| `<table>` | `<caption>`, `<colgroup>`, `<thead>`, `<tbody>`, `<tfoot>`, `<tr>` | Texto direto |".
- Semântica: `tr` direto é válido, mas o parser cria `<tbody>` implícito — bytes escritos ≠ bytes parseados.
- Consequência: `spec/elements.toml:626-630` inclui `"tr"` em `children` e, no mesmo bloco, `invariants = ["tbody_explicit"]`: contradição interna.
- Veredito: **refina** — remover `"tr"` de `children` (§5.5).
#### A9. `label` com `for` obrigatório — decisão defensável, precisa ser declarada
- Citação: `04_ACCESSIBILITY.md:110-111` — "<!-- IMPLÍCITA -->" / "`<label>Nome completo: <input type="text" name="nome" /></label>`".
- Semântica: a associação implícita é válida; a explícita é a preferida pela própria doc (`04:107`).
- Consequência: `spec/elements.toml:537-545` exige `for` e, com A7, o rótulo envolvente é duplamente impossível. Força o padrão bom, mas o corpus não pode ser citado como contraprova sem a nota.
- Veredito: **confirma por decisão**; registrar em `docs/03 §3` ("rótulo implícito não existe").
#### A10. `video` exige `src`, mas o corpus usa `<source>` filhos
- Citação: `modern_html_example.md:513-521` — "`<video controls width="800" preload="metadata" poster="…" playsinline>`" com dois "`<source src="…webm" type="video/webm" />`" e dois `<track>`.
- Semântica: `src` e `<source>` são alternativas mutuamente exclusivas; `01:108` dá o modelo como "`<source>` + `<track>` + transparent".
- Consequência: `spec/elements.toml:443-451` exige `required = ["src"]` **e** permite `children = ["source","track"]`. O vídeo multi-formato (webm+mp4) não compila; com `src`, a escolha de fonte fica duplicada em dois lugares.
- Veredito: **refuta** — `src` opcional + "`src` XOR `<source>`" (§5.6).
#### A11. `source` proíbe `src` na allowlist, mas o comentário admite o uso
- Citação: `spec/elements.toml:433-442` — "`required = ["srcset"]` … # no contexto `<video>`/`<audio>` o atributo é `src` (profile abaixo, ver invariants)".
- Semântica: o atributo válido depende do pai — `srcset`+`sizes` em `picture`, `src`+`type` em `video`/`audio` (`modern:516`).
- Consequência: um invariante **não** amplia allowlist: `source(src: …)` é erro de compilação, o vídeo multi-fonte continua inemitível e o comentário contradiz a allowlist do mesmo bloco.
- Veredito: **refuta** — separar `source` (picture) e `media_source` (video/audio) (§5.6).
#### A12. `audio`/`video` exigem `track` sem exigir `kind` de legenda
- Citação: `01_ALL_TAGS.md:107` — "| `<track>` | … | `srclang`, `kind`, `label` |"; `spec/elements.toml:448-450` — `required_children = ["track"]` / `media_requires_captions`.
- Semântica: WCAG 1.2.2 exige legendas em **vídeo**; áudio pede transcrição (1.2.1), que não é `<track>`. E `track kind="metadata"` não cumpre nada.
- Consequência: `audio` obriga um WebVTT que pode não existir (o dev emite vazio para compilar — pior que não ter) e `video` aceita `kind` que não é legenda.
- Veredito: **refina** — `captions_kind`; `audio` sem `track` obrigatório (§5.6).
#### A13. `img` sem `usemap`; `map`/`area` fora do vocabulário
- Citação: `14_IMAGE_MAPS.md:11-13` — "`<img src="mapa.png" alt="" usemap="#meu-mapa" />`" / "`<map name="meu-mapa">`"; `14:8-9` — "Prefira links de texto (estilizados com CSS) a image maps".
- Semântica: `usemap` só é válido com `<map name>` correspondente; `area` exige `alt` e `shape`/`coords`.
- Consequência: `spec/elements.toml:416-424` não tem `usemap` e `map`/`area` não existem. O caso real (mapa de escola no portal) fica sem representação; como o corpus recomenda não usar, a resposta correta é **proibir explicitamente** em `docs/03 §5`.
- Veredito: **confirma a ausência**; **refina** — registrar a proibição.
#### A14. `img` sem `crossorigin`/`referrerpolicy`/`elementtiming`
- Citação: `16_SPECIFIC_ATTRIBUTES.md:131` — "`<img src="hero.jpg" alt="Hero" elementtiming="Imagem principal" />`"; `05_SECURITY.md:73-79` lista `crossorigin` em "`<audio>`, `<img>`, `<link>`, `<script>`, `<video>`".
- Semântica: `elementtiming` marca o elemento para o LCP via `PerformanceObserver`; `crossorigin` condiciona canvas e mensagens de erro.
- Consequência: nenhum dos três está em `spec/elements.toml:418-420`; `elementtiming` é o único modo de medir LCP por elemento sem JS próprio — o que o gate de performance do consumidor precisa.
- Veredito: **refina** — `elementtiming` como id estável + `crossorigin` enum (§5.7).
#### A15. `iframe` sem `allow`/`allowfullscreen`
- Citação: `01_ALL_TAGS.md:118` — "| `<iframe>` | … | `sandbox`, `loading`, `allow`, `srcdoc` |".
- Semântica: `allow` é Permissions Policy por frame; sem ele só se embute conteúdo passivo.
- Consequência: `spec/elements.toml:470-477` tem `sandbox` (bom) e não tem `allow`/`allowfullscreen`; embed de mapa/vídeo de terceiro não funciona e o dev pede escape hatch — o que D4 existe para impedir.
- Veredito: **refina** — `allow` como lista fechada de tokens, nunca string (§5.7).
#### A16. `svg` declara `viewbox` em minúsculas
- Citação: `modern_html_example.md:532` — "`<svg viewBox="0 0 400 200" width="400" height="200"`".
- Semântica: atributos SVG são case-sensitive; `viewbox` é ignorado pelo parser e o gráfico perde a escala, **sem erro de validação**.
- Consequência: `spec/elements.toml:478-484` lista `"viewbox"`; como `allowed` também define a ordem de emissão (D14), corrigir o nome é pré-requisito do primitivo `Icon` (`docs/05 §3`).
- Veredito: **refuta** — renomear (§5.8).
#### A17. O conteúdo interno de `<svg>` não existe no vocabulário
- Citação: `modern_html_example.md:533-540` — "`<rect x="40" y="100" width="80" height="80" fill="…" rx="4" />` … `<text x="80" y="196" …>2022</text>`".
- Semântica: SVG tem elementos próprios (`path`, `circle`, `g`, `rect`, `text`); ícone real é `path`.
- Consequência: `spec/elements.toml:478-484` declara `children = ["svg_title","desc","phrasing"]`; sem `path`/`rect`, `<svg>` só contém HTML — o elemento existe e **nenhum ícone pode ser desenhado**.
- Veredito: **refuta** — recorte fechado (`title`, `desc`, `path`, `circle`, `rect`, `g`) ou remover `svg` da v1 (§5.8, §6.1).
#### A18. `meta` exige `content`, e `charset` não tem `content`
- Citação: `01_ALL_TAGS.md:20` — "| `<meta>` | Metadados (charset, viewport, etc.) | Vazio | SIM | charset="UTF-8" obrigatório |".
- Semântica: `meta` tem três formas exclusivas — `charset`; `http-equiv`+`content`; `name`/`property`+`content`.
- Consequência: `spec/elements.toml:93-101` declara `required = ["content"]` e permite `charset`. Logo `<meta charset="utf-8">`, a primeira tag do head em **todos** os templates (`09:7`, `modern:6`), não compila.
- Veredito: **refuta** — perfis (`Meta::charset`/`::name`/`::property`) + `meta_form_exclusive` (§5.9).
#### A19. `no_http_equiv` está certo; o corpus erra ao propor CSP por `meta`
- Citação: `05_SECURITY.md:213-215` — "`<meta http-equiv="Content-Security-Policy"` / `content="frame-ancestors 'none'" />`"; `modern_html_example.md:719` — "`[x] frame-ancestors 'none' (proteção clickjacking)`".
- Semântica: `frame-ancestors`, `sandbox` e `report-uri` são **ignorados** quando entregues por `<meta>`; só o header os aplica, então o exemplo do corpus não protege contra clickjacking.
- Consequência: `spec/elements.toml:100` proíbe `http-equiv` (`no_http_equiv`) e `docs/08 §6` define CSP como header — a crate está certa e é mais estrita que o corpus.
- Veredito: **confirma** D3; **refuta** `05_SECURITY.md:215` e `modern:719`.
#### A20. `script` com `src` obrigatório não emite JSON-LD
- Citação: `spec/elements.toml:111-118` — "`required = ["src"]` … `children = ["json"]  # SOMENTE JsonLd`"; `19_META_SCRIPT_TYPES.md:125` — "`src`, `async`, `defer` … NÃO são permitidos" (para `importmap`).
- Semântica: dos tipos de script, só o executável tem `src`; JSON-LD e `importmap` são inline por definição.
- Consequência: `docs/07 §6` mostra `html! { head { JsonLd::graph(&g) } }` como cidadão de primeira classe — e ele não compila; `children = ["json"]` sem `src` opcional é contradição no mesmo bloco.
- Veredito: **refuta** — duas formas: `script(src, …)` e `script(JsonLd)` (§5.9).
#### A21. `link` sem `title` torna `alternate stylesheet` inemitível
- Citação: `17_LINK_RELATIONS_EXTRA.md:40-41` — "`<link href="fancy.css" rel="alternate stylesheet" title="Fancy" />`"; `17:47-50` diz que `title` **presente** é o que define *preferred* e *alternate*.
- Semântica: sem `title`, `alternate stylesheet` é inválido e a escolha do usuário desaparece.
- Consequência: `spec/elements.toml:102-110` não lista `title`, mas o enum `link_rel_enum` inclui valores que dependem dele; como o corpus recomenda `prefers-color-scheme` (`17:53`), a saída limpa é tirar o valor do enum.
- Veredito: **refina** — remover `alternate stylesheet` do enum de `link` (§5.9).
#### A22. `preload_requires_as_and_crossorigin` exige demais
- Citação: `06_PERFORMANCE.md:52` — "`<link rel="preload" href="font.woff2" as="font" crossorigin />`"; `06:59` — "**Sempre especificar `as`**: `image`, `font`, `style`, `script`…".
- Semântica: `crossorigin` é obrigatório para **fonte** (e `fetch`/módulo), não para `image`/`style`/`script` da mesma origem (`modern:86` preload de imagem sem `crossorigin`).
- Consequência: `spec/elements.toml:110` junta as duas condições; aplicado literalmente, todo preload de imagem sai com `crossorigin="anonymous"` e erra o modo CORS de CDN pública.
- Veredito: **refina** — dividir em dois invariantes (§5.9).
#### A23. `option` com `value` obrigatório impede o placeholder de `select`
- Citação: `09_BOILERPLATE.md:191` — "`<option value="">Selecione...</option>`" (idem `modern:338`).
- Semântica: o item vazio é o mecanismo nativo de "nada selecionado" com `required` no `select`.
- Consequência: `spec/elements.toml:582-588` exige `value` e `docs/02 §4` afirma que "`""` é sempre um bug de intenção"; duas regras boas colidindo — o placeholder é o único caso legítimo de valor vazio.
- Veredito: **refina** — `OptionValue::Placeholder` explícito (§5.4).
#### A24. `progress`/`meter` não aceitam `id`, mas exigem nome acessível
- Citação: `modern_html_example.md:470-471` — "`<label for="vagas">Vagas preenchidas:</label>`" / "`<progress id="vagas" value="3400" max="5000">68%</progress>`".
- Semântica: `label`+`for` é o caminho mais curto para nomear medidores; a alternativa é `aria-label`.
- Consequência: `spec/elements.toml:606-619` declara `accessible_name = "required"` com allowlist `["value","max","class"]` (e `[…,"min","max","low","high","optimum","class"]`); sem `id`, `for` não tem alvo e o primitivo é forçado a `aria-label`, contra a preferência por rótulo visível (3.3.2).
- Veredito: **refuta** — incluir `id` (§5.4).
#### A25. `input` sem `autofocus`, `capture`, `dirname`, `form`
- Citação: `16_SPECIFIC_ATTRIBUTES.md:3` — "Cobre: `capture`, `dirname`, `form`, `readonly`, `size`, `step`, `elementtiming`"; `04_ACCESSIBILITY.md:160` — "`<input type="text" autofocus />`" com a regra "Apenas UM por página" (`04:158`).
- Semântica: `capture` decide câmera em mobile; `dirname` envia direcionalidade; `form` associa a controle fora do `<form>`; `autofocus` é o foco inicial.
- Consequência: `spec/elements.toml:546-557` tem `size`/`list`/`inputmode`/`enterkeyhint`/`spellcheck` e não tem os quatro; `autofocus` falta **também** em `button`, embora `modern:378` use "`<button value="close" autofocus>Fechar</button>`" — quebra o primitivo `Dialog`.
- Veredito: **refina** — os quatro + `autofocus_unique` (§5.4).
#### A26. Falta a tabela `type de input → atributos`, e a enum precisa dos 22 valores
- Citação: `01_ALL_TAGS.md:180` — "`button`, `checkbox`, `color`, `date`, `datetime-local`, `email`, `file`, `hidden`, `image`, `month`, `number`, `password`, `radio`, `range`, `reset`, `search`, `submit`, `tel`, `text` (default), `time`, `url`, `week`".
- Semântica: `min`/`max`/`step` valem em 6 tipos e `minlength` em 7 (`11:23-28`); `attribute_by_input_type` só funciona com essa tabela como dado.
- Consequência: a schema 1 listava 13 tipos num comentário; a schema 2 removeu a lista e ficou só com o invariante — sem a tabela, o gerador não sabe quais combinações rejeitar.
- Veredito: **refina** — `[input_type.<nome>]` gerado (§5.4).
#### A27. `datalist` está `deprecated`, mas `input.list` continua permitido
- Citação: `spec/elements.toml:599-605` — "`deprecated = "sem nome acessível garantido e sem estilo controlável…"`"; `spec/elements.toml:552` mantém `"list"` na allowlist de `input`.
- Semântica: `list` sem `datalist` não tem destino.
- Consequência: um atributo aponta para um elemento proibido; duas decisões incompatíveis no mesmo arquivo, descobertas no primeiro `cargo check`.
- Veredito: **refuta** — remover `"list"` enquanto `datalist` for `deprecated` (§5.4).
#### A28. `form` sem `rel`, `target`, `autocomplete`
- Citação: `01_ALL_TAGS.md:165` — "| `<form>` | … | `action`, `method`, `novalidate`, `rel` |".
- Semântica: `rel` é a lista de relações da navegação de submissão; `target` é a janela; `autocomplete` no form vira default dos campos.
- Consequência: `spec/elements.toml:514-522` aceita `method`/`action`/`enctype` e proíbe `novalidate`. `target="_blank"` em form (impressão de relatório) é inemitível — o dev cai para `<a>` e perde o POST.
- Veredito: **refina** — `target` enum + `rel` reaproveitado de `a` (§5.4).
#### A29. `fieldset` sem `form`
- Citação: `16_SPECIFIC_ATTRIBUTES.md:45` — "Associa um controle de formulário a um `<form>` **não ancestral** … Aplicável a: `<button>`, `<fieldset>`, `<input>`, `<object>`, `<output>`, `<select>`, `<textarea>`."
- Semântica: grupo de controles fora do form que os submete (layout em duas colunas).
- Consequência: `spec/elements.toml:523-530` tem `name` e não tem `form`, enquanto `button` tem `form` (l. 592): inconsistência interna que só aparece em formulário dividido.
- Veredito: **refina** — `form` nos cinco elementos (§5.4).
#### A30. `button` sem `command`/`commandfor` e sem `autofocus`
- Citação: `03_MODERN_PATTERNS.md:66` — "`<button command="show-modal" commandfor="modal">Abrir Modal</button>`"; `03:80` marca Invoker Commands como "(experimental)".
- Semântica: `command`/`commandfor` é o único caminho **sem JS** para abrir `<dialog>` modal; `popovertarget` cobre apenas popover.
- Consequência: `spec/elements.toml:589-596` aceita `popovertarget`/`popovertargetaction` e não tem `command`/`commandfor` nem `autofocus`; modal fica dependente de asset JS — coerente com D13, mas a assimetria com popover não está justificada.
- Veredito: **refina** — `autofocus` entra; `command`/`commandfor` com nota "experimental" (§5.4).
#### A31. `form novalidate` proibido está certo — e o corpus é o contraexemplo
- Citação: `09_BOILERPLATE.md:172` — "`<form action="/api/contato" method="post" novalidate>`"; `modern_html_example.md:743` — "`[x] novalidate + custom validation via JS`".
- Semântica: `novalidate` desliga a validação interativa do browser e pressupõe JS próprio (`11:119`).
- Consequência: `spec/elements.toml:521` (`novalidate_forbidden`) está certo: sem JS inline, `novalidate` remove a única validação client-side existente. O corpus está preso ao modelo que D3/D13 rejeitam.
- Veredito: **confirma** a crate; **refuta** `09:172`.
#### A32. `readonly` + `required` é combinação inútil
- Citação: `16_SPECIFIC_ATTRIBUTES.md:80-81` — "`required` não tem efeito em campos `readonly`"; `11_CONSTRAINT_VALIDATION.md:121` — "`readonly` fields não participam de constraint validation".
- Semântica: dois atributos presentes, um sem efeito, e a UI mente (campo parece obrigatório e é imutável).
- Consequência: `spec/elements.toml:546-557` permite ambos sem invariante; uma linha de validação N2 evita um bug real de CMS (campo "protocolo" readonly+required).
- Veredito: **refina** — `readonly_excludes_required` (§5.4).
#### A33. `step` sem `min` rejeita entrada válida
- Citação: `16_SPECIFIC_ATTRIBUTES.md:106-120` — tabela de defaults ("`number`, `range` | 1 … `time` | 60 segundos"), "`step="any"` permite qualquer valor", "Step base = `min` (se definido), senão `value`, senão `0`".
- Semântica: com base 0, `step="2"` rejeita 1, 3, 5 — erro invisível em `:valid`/`:invalid`.
- Consequência: `spec/elements.toml:549` aceita `step` como atributo solto; precisa de `Step { Any, Fixed(u32) }` **e** do invariante que exige `min`.
- Veredito: **refina** — `step_requires_min` (§5.4).
#### A34. `time` com perfil único RFC 3339 não cobre o corpus
- Citação: `10_DATE_TIME_FORMATS.md:55-67` — "Date | `YYYY-MM-DD`…", "Time | `HH:MM` ou `HH:MM:SS`…", "Week | `YYYY-Www`", "Month | `YYYY-MM`", "datetime-local | `YYYY-MM-DDTHH:MM`", "Global datetime | …`Z`".
- Semântica: `datetime` aceita sete perfis; o corpus usa dois fora do RFC 3339 estrito: `modern:270` "`<time datetime="09:00">`" e `modern:207` "`<time datetime="2025-09-15">`".
- Consequência: `spec/elements.toml:333-339` exige `datetime` + `datetime_profile`; se `Timestamp` (docs/02 §2) só emite RFC 3339 em UTC, **nem o exemplo do corpus compila**.
- Veredito: **refuta** `Timestamp` como tipo único; enum `DateTimeValue` de 6 formas (§5.10).
#### A35. `Timestamp` em UTC impede `datetime` com offset
- Citação: `10_DATE_TIME_FORMATS.md:49-53` — "`YYYY-MM-DDTHH:MM:SS±HH:MM` — offset explícito" / "`1789-08-22T12:30:00.1-04:00` — EDT".
- Semântica: evento com hora local explícita (`-03:00` para Brasília) não depende do fuso do servidor.
- Consequência: `modern:229` usa "`<time datetime="2025-09-15T10:00-03:00">`"; `docs/02 §7` fixa "RFC 3339 em UTC". UTC puro perde o fuso do evento e faz o `ETag` variar com o TZ do processo — viola D14.
- Veredito: **refuta** — offset obrigatório (§5.10).
#### A36. `del`/`ins` têm `datetime` sem tipo, ao contrário de `time`
- Citação: `10_DATE_TIME_FORMATS.md:3` — "…e os atributos `datetime` de `<ins>` e `<del>`."
- Semântica: mesmo atributo, mesma gramática, em três elementos.
- Consequência: `spec/elements.toml:309-320` põe `datetime` na allowlist de `del`/`ins` sem invariante, enquanto `time` tem `datetime_profile`: duas portas para o mesmo dado, uma validada e outra não.
- Veredito: **refina** — mesmo invariante nos três (§5.3).
#### A37. `ol` aceita `type`, que o corpus marca obsoleto
- Citação: `08_DEPRECATED.md:62` — "| `type` | `<li>`, `<ol>`, `<ul>` | CSS `list-style-type` |".
- Semântica: `type` em lista é presentacional e foi substituído por CSS.
- Consequência: `spec/elements.toml:364-369` mantém `"type"` em `ol.allowed` (herdado da schema 1): é a única sobrevivência clara de atributo obsoleto no vocabulário — o tipo de coisa que D4 deveria impedir.
- Veredito: **refuta** — remover (§5.5).
#### A38. `popover` é global no padrão e não existia na crate
- Citação: `03_MODERN_PATTERNS.md:88` — "## 3. Popover API (Atributo Global `popover`)"; `03:95-97` — "`popover="auto"` … `"hint"` … `"manual"`".
- Semântica: atributo global, aplicável a qualquer elemento; o alvo tem `popover`, o gatilho tem `popovertarget`.
- Consequência: `[globals]` (`spec` l. 45-62) não tinha `popover`, mas `button.popovertarget` estava permitido (l. 592): o gatilho existia e o alvo não podia ser declarado. O `Tooltip` de `docs/05 §3` (mapeado em `popover="hint"`, `modern:384-387`) perdia o veículo.
- Veredito: **corrigido** — `popover` em `[globals]`, incluindo `hint` (§5.11).
#### A39. Globais: lacunas pontuais e duplicação de fonte
- Citação: `18_GLOBAL_ATTRIBUTES.md:154` — "`until-found` | Oculta até ser encontrado por find-in-page"; `18:48-61` (`autocapitalize`); `04_ACCESSIBILITY.md:183` — "`<div aria-live="polite" aria-atomic="true">`".
- Semântica: `hidden="until-found"`, `inert` e `translate` são as três capacidades que permitem UI sem JS; `aria-atomic` é o que faz a região live anunciar a mensagem inteira.
- Consequência: `spec/elements.toml:45-62` já tem `hidden`/`inert`/`translate`/`spellcheck`/`dir` (vitória da schema 2), mas não tem `autocapitalize`, `aria-atomic`, `aria-details` nem `tabindex` tipado; e 30+ blocos repetem `class`/`id`/`data-*`/`aria-*`, criando duas fontes para a ordem de emissão de D14 (`strong`/`small`/`abbr`/`kbd`/`mark`/`hr`/`caption` listam só `class`).
- Veredito: **refina** — globais aplicados incondicionalmente e nunca repetidos (§5.11).
#### A40. `abbr` obriga `title`, que não é acessível em toque
- Citação: `15_DEFINE_TERMS.md:47` — "O atributo `title` em `<abbr>` não substitui uma definição inline. O conteúdo de `title` só aparece em hover (mouse), sendo inacessível para touch e screen readers."
- Semântica: `title` é expansão suplementar; a expansão legível precisa estar no fluxo.
- Consequência: `spec/elements.toml:278-284` faz `required = ["title"]`, o que garante a expansão para mouse e **sugere** que a acessível está resolvida — contra `docs/05 §4` ("`title` como único nome acessível" é anti-padrão).
- Veredito: **refina** — manter `title` + advisory "`abbr` repetido sem expansão textual" (§5.3).

## 2.13 Status de cada achado na revisão `33c5007`

`corrigido` = o bloco já não tem o defeito; `aberto` = segue como descrito; `n/a` = era confirmação/refutação, não correção. 9 corrigidos · 28 abertos · 3 n/a.

| achado | status | evidência na revisão atual |
|---|---|---|
| A1 doctype | aberto | nenhum `DOCTYPE` em `docs/`/`spec/`/`README` |
| A2 head order | aberto | `[element.head] first_child = "title"` |
| A3 header/footer | n/a | `forbidden_children = ["header","footer"]` (correto) |
| A4 body categoria | aberto | `[element.body] category = "metadata"` |
| A5 summary | corrigido | `children = ["phrasing","heading"]` + `summary_single_heading` |
| A6 noscript | corrigido | `children = ["metadata","flow","phrasing"]` + `noscript_context` |
| A7 categorias form | corrigido | `input`/`label`/`textarea`/`select`/`fieldset` incluem `phrasing` |
| A8 table `tr` | aberto | `children` de `table` ainda lista `"tr"` |
| A9 label implícito | aberto | `[element.label] required = ["for"]` (decisão a documentar) |
| A10 video `src` | aberto | `children = ["source","track"]` sem `media_source`; `required_children = ["track"]` |
| A11 source | corrigido | existe `[element.media_source]`; `source_profile_picture_only` |
| A12 track/captions | aberto | `media_requires_captions` sem `captions_kind` |
| A13 `usemap` | aberto | `[element.img] allowed` sem `usemap` |
| A14 img extra | aberto | ganhou `width`/`height` obrigatórios; sem `crossorigin`/`elementtiming`/`referrerpolicy` |
| A15 iframe `allow` | aberto | `[element.iframe] allowed` sem `allow` |
| A16 `viewBox` | corrigido | `allowed = ["viewBox", …]` |
| A17 SVG interno | aberto | `[element.svg] children` sem `path`/`circle`/`rect` |
| A18 meta `content` | corrigido | `required = []` + `meta_form_exclusive` |
| A19 CSP por meta | n/a | `no_http_equiv` mantido |
| A20 script `src` | corrigido | `required = []` + `script_form_exclusive` + `script_src_requires_defer` |
| A21 link `title` | aberto | `[element.link] allowed` sem `title` |
| A22 preload | aberto | `preload_requires_as_and_crossorigin` ainda combinado |
| A23 option placeholder | aberto | `required = ["value"]`, sem `option_value_typed` |
| A24 progress/meter `id` | corrigido | `"id"` em `allowed` nos dois |
| A25 input extra | aberto | sem `autofocus`/`capture`/`dirname`/`form`/`autocapitalize` |
| A26 `[input_type.*]` | aberto | sub-tabela não existe |
| A27 datalist/`list` | aberto | `datalist` `deprecated` e `"list"` ainda em `input.allowed` |
| A28 form extra | aberto | sem `rel`/`target`/`autocomplete` |
| A29 fieldset `form` | aberto | `allowed = ["class","id","name","disabled","data-*"]` |
| A30 button extra | aberto | sem `command`/`commandfor`/`autofocus` |
| A31 novalidate | n/a | `novalidate_forbidden` mantido |
| A32 readonly+required | aberto | sem `readonly_excludes_required` |
| A33 step/min | aberto | sem `step_requires_min` |
| A34 `time` perfil | aberto | `datetime_profile` existe; `DateTimeValue` não |
| A35 offset | aberto | `docs/02 §7` ainda fixa "RFC 3339 em UTC" |
| A36 del/ins | aberto | `datetime` sem `datetime_profile` |
| A37 `ol type` | corrigido | `allowed = ["class","data-*","start","reversed"]` |
| A38 `popover` global | corrigido | `popover` em `[globals]`, inclusive `hint` |
| A39 globais | aberto | sem `autocapitalize`/`aria-atomic`/`aria-details`/`tabindex` |
| A40 abbr | aberto | `required = ["title"]`, sem `abbr_expansion_nearby` |

## 2.14 Tabela elemento → veredito (91 blocos; veredito da revisão das 14:58, status atual em §2.13)

`ok` = compila e o modelo está correto · `falta atributo` = allowlist incompleta · `conteúdo-modelo errado` = `children`/`required`/categoria incorretos · `deve sair` = remover ou marcar `deprecated`.

| elemento | veredito | nota |
|---|---|---|
| html | ok | doctype fica no shell (A1) |
| head | conteúdo-modelo errado | `first_child` vs charset (A2) |
| body | ok | categoria `metadata` (A4) |
| title | ok | `document_title_required` |
| meta | ok (era errado) | `meta_form_exclusive` corrigiu A18 |
| link | falta atributo | `title`; invariante de preload largo (A21/A22) |
| script | ok (era errado) | `script_form_exclusive` corrigiu A20 |
| noscript | ok (era errado) | aceita `metadata` no head (A6) |
| header | ok | escolha correta contra `01:37` (A3) |
| footer | ok | idem |
| main | falta atributo | `tabindex` desnecessário |
| nav | ok | `accessible_name = required` |
| aside | ok | — |
| section | ok | invariante deveria ser N2 |
| article | ok | — |
| search | ok | landmark (`02:141`) |
| address | ok | candidato "só em sectioning" (`01:33`) |
| div | ok | — |
| span | ok | `lang` local (3.1.2) |
| p | ok | `no_flow_inside_phrasing` |
| h1 | ok | `shell_owns_h1` |
| h2 | ok | — |
| h3 | ok | — |
| h4 | ok | ausente na schema 1 |
| h5 | ok | idem |
| h6 | ok | idem |
| strong | ok | allowlist só `class` (A39) |
| em | ok | ausente na schema 1 |
| small | ok | allowlist só `class` (A39) |
| code | ok | — |
| pre | ok | `tabindex` sem tipo (A39) |
| abbr | falta atributo | `title` sem alternativa acessível (A40) |
| cite | ok | — |
| q | ok | `cite` tipado como URL |
| blockquote | ok | idem |
| s | ok | — |
| del | falta atributo | `datetime` sem perfil (A36) |
| ins | falta atributo | idem |
| kbd | ok | — |
| mark | ok | — |
| time | falta atributo | perfil único (A34/A35) |
| br | ok | — |
| hr | ok | — |
| ul | ok | `children = ["li"]` |
| ol | ok (era `type`) | `08:62` aplicado (A37) |
| menu | deve sair | redundante com `ul` |
| li | falta atributo | `value` (numeração de `ol`) |
| dl | ok | `required_children = ["dt"]` |
| dt | ok | — |
| dd | ok | — |
| a | falta atributo | `download`, `type` |
| img | falta atributo | `usemap`, `crossorigin`, `elementtiming` (A13/A14) |
| picture | ok | correção da schema 2 |
| source | conteúdo-modelo errado | `src` proibido em contexto de mídia (A11) |
| media_source | ok | perfil novo que absorveu `src` de vídeo/áudio (A11) |
| video | conteúdo-modelo errado | `src` obrigatório + `<source>` (A10) |
| audio | conteúdo-modelo errado | `track` obrigatório sem `kind` (A12) |
| track | ok | falta enum de `kind` (A12) |
| iframe | falta atributo | `allow`, `allowfullscreen` (A15) |
| svg | conteúdo-modelo errado | `viewBox` corrigido; `children` ainda sem `path/rect` (A17) |
| svg_title | ok | pseudo-elemento SVG |
| desc | ok | idem |
| figure | ok | — |
| figcaption | ok | — |
| form | falta atributo | `rel`, `target`, `autocomplete` (A28) |
| fieldset | falta atributo | `form` (A29) |
| legend | ok | — |
| label | conteúdo-modelo errado | `for` obrigatório + categoria (A7/A9) |
| input | falta atributo | 4 atributos + categorias (A7/A25/A26) |
| textarea | falta atributo | `enterkeyhint`; categoria (A7) |
| select | falta atributo | `form`; categoria (A7/A29) |
| optgroup | ok | falta `forbidden_children` |
| option | conteúdo-modelo errado | `value` vs placeholder (A23) |
| button | falta atributo | `autofocus`, `command`/`commandfor` (A30) |
| datalist | deve sair | `deprecated` já; `input.list` contradiz (A27) |
| output | ok | `for` obrigatório correto |
| progress | ok (era falta `id`) | A24 corrigido |
| meter | ok (era falta `id`) | A24 corrigido |
| table | ok | remover `"tr"` de `children` (A8) |
| caption | ok | `caption_not_empty` |
| colgroup | ok | — |
| col | ok | — |
| thead | ok | — |
| tbody | ok | — |
| tfoot | ok | ausente na schema 1 |
| tr | ok | — |
| th | ok | `scope` + `scope_matches_section` |
| td | ok | advisory "1ª célula é `td`" |
| details | ok | `name` + accordion |
| summary | ok (era errado) | aceita `heading` (A5) |
| dialog | ok | modelo a imitar (`closedby`) |

## 3. Confirmações do desenho

1. **Escape por contexto** — `05_SECURITY.md:180-190` ("Output encoding: `<` → `&lt;`…") e `05:196-198` ("`textContent`… nunca `innerHTML` com dados não sanitizados") descrevem a lacuna que `Text`/`AttrVal`/`Href`/`JsonLd` fecham. Confirma D1.
2. **Sem inline** — o corpus usa `on*` em três padrões centrais: `06:144-146` (`onload="this.rel='stylesheet'"`), `03:203` (`oninvalid=…`), `09:97`. A crate recusa por D3 e substitui por `preload`/`Link:`. Confirma D3 e `docs/09 §2`.
3. **Um `<h1>`, um `<main>`, landmarks nomeados** — `04:20`, `02:133` ("`<main>` … 1 por página"), `02:141-144`. Confirma D8 e `single_h1`/`main_unique`/`landmark_names_distinct`.
4. **`alt` por tipo** — `04:96` ("**Toda `<img>` precisa de `alt`** — mesmo que vazio para decorativas") e `04:90`. Confirma `Alt::decorative()`.
5. **`<caption>` + `th scope`** — `04:218-226`. Confirma `table.required_children` e `th.required = ["scope"]`.
6. **`lang` obrigatório e válido** — `04:246` + `02:8`. Confirma `html.required = ["lang"]` com BCP-47.
7. **`pattern` sem flags** — `11:21` ("flags global/ignoreCase/multiline desligadas"). Confirma `pattern_flags_forbidden`.
8. **Comentário HTML proibido** — `13:26-27` (não pode conter `-->`, `--!>`, nem terminar com `<!-`) e `13:55` ("visíveis no source"). Confirma a ausência de `Comment` em `docs/02 §3`.
9. **`<search>` substitui `role="search"`** — `03:139-141`. Confirma `search.accessible_name = required`; `09:125` ainda usa `role="search"` (erro do corpus).
10. **`template`/`slot`/`canvas`/`object` fora** — `01:98-101`, `01:200`, `03:35-45`: todos exigem conteúdo ativo ou JS de componente. Confirma as exclusões de `docs/03 §8`.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata, onde) |
|---|---|---|
| nenhum `DOCTYPE` é mencionado | `12:15` doctype é o único gatilho de no-quirks | `docs/06 §8`, item 9: "`<html>` é precedido por `<!DOCTYPE html>`; o shell não emite comentários." + regra N4 (`docs/04 §6`) |
| `meta.required = ["content"]` | `01:20` `charset` obrigatório, sem `content` | `spec`: `required = []` + `invariants = ["meta_form_exclusive"]`; `docs/03 §3`: "`meta` tem três formas exclusivas: `charset`, `http-equiv`+`content`, `name`/`property`+`content`." |
| `script.required = ["src"]` com `children = ["json"]` | `07 §6` JSON-LD sem `src`; `19:125` importmap sem `src` | `spec`: `required = []` + `invariants = ["script_form_exclusive"]`; comentário: "# `src` OU `json`, nunca os dois." |
| `video.required = ["src"]` com `<source>` filhos | `modern:513-521` `<video>` só com `<source>` | `spec`: `required = ["controls"]` + `media_source_xor_src`; `docs/03 §3`: "`video`/`audio` usam `src` (uma fonte) **ou** `<source>` filhos, nunca ambos." |
| `source.required = ["srcset"]`, comentário admite `src` | `modern:516` `<source src type>` | `spec`: criar `[element.media_source]` (`required = ["src","type"]`); `source` fica só em `picture` |
| `ol.allowed` inclui `type` | `08:62` obsoleto | remover `"type"`; `docs/03 §3`: "`type` em lista é obsoleto; numeração por `list-style-type`." |
| `summary.children = ["phrasing"]` | `modern:268` `<summary><h3>` | `children = ["phrasing","heading"]` + `summary_single_heading` |
| `noscript.children = ["flow","phrasing"]` | `01:132` metadata no head | `children = ["metadata","flow","phrasing"]` + `noscript_context` |
| `label/input/select/textarea/fieldset.category = ["form"]` | controles são phrasing (`01:172`, `02:145`) | `category = ["form","interactive","phrasing"]` (e `label` `["form","phrasing"]`) |
| `svg.allowed` tem `"viewbox"` | `modern:532` `viewBox` | `"viewBox"` + comentário "# atributo SVG é case-sensitive" |
| `progress`/`meter` sem `id`, nome obrigatório | `modern:470-471` `<label for>` + `id` | adicionar `"id"` em `allowed` |
| `input.allowed` inclui `list` | `spec:599` `datalist` `deprecated` | remover `"list"`; `docs/03 §3`: "`datalist` sai do v1; busca incremental é `Field`." |
| `link.allowed` sem `title` | `17:40-50` alternate exige `title` | remover `alternate stylesheet` de `link_rel_enum` |
| `table.children` inclui `"tr"` | `req` `tbody_explicit` no mesmo bloco | remover `"tr"` de `children` |
| `[globals]` sem `popover` (na revisão) | `03:88-97` popover é global | `popover = ["", "auto", "hint", "manual"]` — aplicado |
| `docs/03 §1` fala "63 elementos" | `spec` tem 91 blocos (89 HTML + `svg_title`/`desc`) | `docs/03 §1`: "91 blocos em `spec_version = "2.0.0"`, contra 63 no escopo medido." |

## 5. Adições prontas para colar

`spec/elements.toml`, schema 2. Blocos completos para elementos ausentes; linhas de substituição (dentro do bloco existente, marcado em comentário) para os demais. A ordem dos atributos continua sendo a ordem de emissão (D14).

```toml
# 5.1 docs/06 §8, item novo (não é TOML):
#   9. `<html>` é precedido por `<!DOCTYPE html>`; o shell não emite comentários.

# 5.2 [element.head] — substitui `first_child = "title"` e `invariants`
head_order = ["charset", "viewport", "title"]      # 06_PERFORMANCE.md:139
invariants = ["head_order", "one_title"]

# 5.3 elementos ausentes
[element.hgroup]                   # 02_SEMANTIC_STRUCTURE.md:164
category = ["heading"]
allowed = ["class"]
accessible_name = "none"
children = ["h1", "h2", "h3", "h4", "h5", "h6", "p"]
required_children = ["h1"]
first_child = "h1"
invariants = ["hgroup_max_two"]

[element.data]                     # 01_ALL_TAGS.md:77, modern_html_example.md:438
category = ["phrasing"]
required = ["value"]
allowed = ["class"]
accessible_name = "none"
children = ["phrasing"]
invariants = ["value_typed"]       # Count | Money | Timestamp, nunca string

[element.dfn]                      # 15_DEFINE_TERMS.md:10
category = ["phrasing"]
allowed = ["class", "id"]
accessible_name = "none"
children = ["phrasing"]
invariants = ["dfn_first_occurrence"]

[element.media_source]             # <source src type> em video/audio (modern:516)
category = ["media"]
void = true
required = ["src", "type"]
allowed = ["media"]
accessible_name = "none"
children = ["none"]

[element.path]                     # SVG; `d` é PathData, nunca AttrVal
category = ["media"]
void = true
allowed = ["d", "class", "fill", "stroke", "stroke-width"]
invariants = ["path_data_grammar"]

# 5.4 substituições em blocos existentes
# [element.summary]
children = ["phrasing", "heading"]
invariants = ["summary_single_heading"]
# [element.noscript]
children = ["metadata", "flow", "phrasing"]
invariants = ["noscript_context"]
# [element.del]   (idem [element.ins])
invariants = ["datetime_profile"]
# [element.abbr]
invariants = ["abbr_expansion_nearby"]
# [element.input]
category = ["form", "interactive", "phrasing"]
required = ["type"]                # `name` só quando submittable
allowed = ["name", "value", "placeholder", "required", "disabled", "readonly", "autocomplete",
           "inputmode", "enterkeyhint", "autocapitalize", "spellcheck", "pattern", "min", "max",
           "step", "minlength", "maxlength", "size", "checked", "multiple", "accept", "form",
           "dirname", "capture", "autofocus", "aria-*", "data-*"]
invariants = ["label_or_labelledby_in_block", "label_exempt_types", "attribute_by_input_type",
              "autocomplete_enum", "pattern_flags_forbidden", "step_requires_min",
              "readonly_excludes_required", "autofocus_unique"]
# [element.label] / [element.textarea] / [element.select]
category = ["form", "phrasing"]                    # textarea/select: ["form","interactive","phrasing"]
# [element.option]
invariants = ["option_value_typed"]                # OptionValue::Placeholder é o único vazio
# [element.progress]
allowed = ["id", "value", "max", "class"]
# [element.meter]
allowed = ["id", "value", "min", "max", "low", "high", "optimum", "class"]
# [element.form]
allowed = ["action", "class", "id", "data-*", "enctype", "rel", "target", "autocomplete", "aria-*"]
# [element.button]
allowed = ["class", "id", "name", "value", "disabled", "form", "popovertarget",
           "popovertargetaction", "command", "commandfor", "autofocus", "aria-*", "data-*"]

# 5.5 listas e tabela
# [element.ol]
allowed = ["class", "data-*", "start", "reversed"]     # `type` removido (08_DEPRECATED.md:62)
# [element.li]
allowed = ["class", "data-*", "value", "aria-*"]
# [element.table]
children = ["caption", "colgroup", "thead", "tbody", "tfoot"]   # sem `tr` (A8)

# 5.6 mídia
# [element.video]
required = ["controls"]
allowed = ["src", "preload", "poster", "muted", "loop", "playsinline", "width", "height",
           "class", "crossorigin"]
children = ["media_source", "track"]
invariants = ["media_source_xor_src", "captions_kind"]
# [element.audio]                  (sem poster/muted/playsinline/crossorigin)
required = ["controls"]
children = ["media_source", "track"]
# [element.track]
invariants = ["kind_enum", "srclang_required_for_subtitles", "captions_kind"]
# [element.source]
invariants = ["srcset_requires_sizes_in_picture"]   # `src` deixa de existir aqui (A11)

# 5.7 imagem e iframe
# [element.img]
allowed = ["width", "height", "loading", "decoding", "fetchpriority", "sizes", "srcset",
           "crossorigin", "referrerpolicy", "elementtiming", "class", "data-*"]
# [element.iframe]
allowed = ["width", "height", "loading", "referrerpolicy", "sandbox", "allow", "class"]
invariants = ["cross_origin_requires_sandbox", "allow_tokens_closed"]

# 5.8 SVG
# [element.svg]
allowed = ["viewBox", "class", "width", "height", "aria-*", "focusable"]
children = ["svg_title", "desc", "path", "circle", "rect", "g"]
invariants = ["svg_icon_default_decorative", "svg_attribute_case"]

# 5.9 metadados
# [element.meta]
required = []
allowed = ["name", "property", "charset", "content", "media"]
invariants = ["meta_form_exclusive", "meta_name_enum", "no_http_equiv"]
# [element.link]
allowed = ["as", "type", "media", "title", "integrity", "crossorigin", "hreflang", "sizes",
           "fetchpriority", "disabled"]
invariants = ["link_rel_enum", "preload_requires_as",
              "preload_crossorigin_for_font_and_fetch", "stylesheet_unique"]
# [element.script]
required = []
allowed = ["src", "type", "defer", "async", "integrity", "crossorigin"]
invariants = ["script_form_exclusive", "script_is_jsonld_only"]   # `src` OU `json`

# 5.10 docs/02 §7 — linha da tabela de tipos
#  | `datetime` | `DateTimeValue` | enum: `Date` (2025-09-15) · `Month` (2019-01) · `Week`
#    (2001-W37) · `Time` (13:44) · `Local` (1986-01-28T11:38) · `Global` (RFC 3339 **com offset
#    obrigatório**) — nunca string livre, nunca UTC implícito (10_DATE_TIME_FORMATS.md:55-67) |

# 5.11 [globals] completo
[globals]
hidden = ["", "until-found"]
dir = ["ltr", "rtl", "auto"]
translate = ["yes", "no"]
spellcheck = ["true", "false"]
autocapitalize = ["off", "none", "on", "sentences", "words", "characters"]
inert = "presence"
popover = ["auto", "hint", "manual"]
tabindex = ["0", "-1"]             # > 0 é sempre erro (SC 2.4.3)
lang = "bcp47"
class = "tokens"
id = "id"
data = "declared"
aria = ["aria-atomic", "aria-busy", "aria-controls", "aria-current", "aria-describedby",
        "aria-details", "aria-expanded", "aria-haspopup", "aria-hidden", "aria-invalid",
        "aria-label", "aria-labelledby", "aria-live", "aria-modal", "aria-pressed",
        "aria-required", "aria-selected"]
# globais aplicados incondicionalmente; a allowlist do elemento NÃO repete
# class/id/lang/data-*/aria-* (uma só fonte para a ordem de emissão, D14)
```

## 6. Armadilhas de complexidade

1. **SVG completo** — ~60 elementos próprios e `d` é uma linguagem; todo ícone em markup vira fonte editável contra um `<img src>` de asset com hash (D13). Fica só o recorte de §5.8.
2. **`http-equiv`** — cada valor é uma gramática (`content="3;url=…"`); `refresh` redireciona sem interação (`19:92`) e `set-cookie`/`x-ua-compatible` são deprimidos (`19:77-78`). CSP é header (`docs/08 §6`).
3. **Invoker Commands** — resolve modal sem JS, mas é experimental (`03:80`) e move estado de UI para o HTML: entra como atributo, nunca como requisito.
4. **`popover="manual"` com estado no servidor** — sem light-dismiss, abrir/fechar exige script e o estado não é representável no `Markup`: entra o atributo, não o estado.
5. **`contenteditable`** (+`plaintext-only`, `virtualkeyboardpolicy`, `writingsuggestions`) — editor no browser: sem JS não salva, com JS devolve HTML de usuário. Substituído por `Asset::Editor` (`docs/07 §2`).
6. **`template`/`slot`/`shadowrootmode`/`is`/`exportparts`** — exigem script de componente e criam conteúdo inerte que o lint não alcança; já proibidos (spec l. 33-36).
7. **`canvas`/`object`/`embed`/`math`** — conteúdo ativo ou vocabulário paralelo (`object` pediria `param`).
8. **Microdata (`itemscope`/`itemprop`)** — `03:294-320` espalha Schema.org por 10 elementos; `JsonLd` tipado (`docs/07 §6`) dá um grafo, um bloco, determinístico.
9. **`speculationrules`/`importmap` inline** — exigem `<script>` sem `src` com JSON livre, reabrindo a porta que `script_is_jsonld_only` fecha; `rel=modulepreload` cobre.
10. **`accesskey`/`autocorrect`/`anchor`/`draggable`** — atalhos invisíveis e hints sem cobertura de teste (`18:30-43`, `18:66-70`). Só `autocapitalize` entra.

## 7. Lacunas do corpus

- **Locale por trecho e `dir`** — `04:246` só trata `lang` no `<html>`; nada sobre `dir="auto"` em texto de usuário nem precedência entre `lang` do elemento e do documento. A crate decide (`Text::lang(Locale)`).
- **`alternate` + `hreflang`** — `link.hreflang` está na allowlist, mas o corpus não trata o par como canônico de portal multilíngue.
- **Precedência de `referrerpolicy`** — três níveis aparecem (`05:96` `rel="noreferrer"`, `19:26` `meta name="referrer"`, header) e nenhum arquivo diz quem vence. A crate fixa "header > meta > elemento".
- **`fetchpriority` + `loading="lazy"`** — `06:107-119` mostra os dois juntos sem notar que prioridade alta e carregamento tardio se contradizem.
- **Tamanho de `alt`/`caption`** — exigidos, nunca limitados. A crate pode impor advisory sem evidência no corpus.
- **`hidden="until-found"` + `beforematch`** — `18:154-158` cita o valor, não o evento; sem JS não há reação ao "encontrado". Fica o valor, sem o evento.
- **Re-render pós-POST** — `04:190-200` mostra `aria-describedby`/`role="alert"`, não o contrato de quando `aria-invalid` aparece. Decisão da crate (só depois do POST).
- **`step` em `time` com segundos não zero** — `10:37-45` dá os formatos e `16:106-120` os defaults, mas não a interação entre `step="900"` e `value` com segundos.
