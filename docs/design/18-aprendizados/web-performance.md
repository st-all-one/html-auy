# web-performance — 2.0.0

Conjunto de 20 arquivos (3.616 linhas) que sintetiza os guias de Web Performance da MDN em referência
"AI-optimized" de padrões modernos (2024–2025): Core Web Vitals, caminho crítico de renderização,
estratégias de carregamento, cache/entrega, APIs de medição e anti-padrões. Aqui ele é lido com um
único filtro: **o que o HTML gerado por `html-auy` pode garantir por construção** — bytes no `<head>`,
atributos de imagem e fonte, regimes de cache, `ETag`/`304` — e o que, sendo métrica de campo, não
pertence a gate nenhum. O conjunto é forte em *ordem* e *presença de atributo*; é fraco exatamente onde
o nosso vocabulário fechado já é melhor, porque cada receita dele pressupõe `style=`, `on*` ou script
inline.

> Citações do corpus trazem `arquivo.md:linha` (verificadas nesta leitura). Afirmações sobre o repositório
> `html-auy` são citadas por arquivo + seção/chave, com `[sem linha]` quando a linha não é estável —
> `spec/elements.toml` e `docs/design/06`, `07`, `09` estavam em edição concorrente durante esta leitura.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-index.md` | 58 | mapa dos documentos, grafo de dependências, 10 princípios ("Measure first", "Native browser features > JS libraries") | fixa "medir antes de otimizar", que separa CI de RUM; nada normativo além disso |
| `01-core-concepts.md` | 75 | RAIL, quatro momentos da página, "First 14KB Rule", acima-da-dobra ≠ moderno | origem do orçamento de 14 KB do prefixo (A1–A3) |
| `02-how-browsers-work.md` | 117 | DNS→TCP→TLS→HTTP→parse, render-blocking vs parser-blocking, tabela de `<script>`, preload scanner | define o que bloqueia e as regras de `defer`/`async`/`crossorigin` (A11, A14–A16) |
| `03-critical-rendering-path.md` | 146 | DOM/CSSOM/render tree/layout/paint, checklist de CRP, inlining de CSS crítico, layout thrashing | as duas receitas de CSS (`<style>` inline, swap por `onload`) são refutadas por nós (A4–A6); proibição de `@import` é útil (A7) |
| `04-loading-strategies.md` | 168 | espectro eager→lazy, tabela de resource hints, `fetchpriority`, Speculation Rules, template completo de `<head>` | **documento mais acionável**: ordem do `<head>`, teto de `preconnect`, "preload só do que é descoberto tarde" (A9–A13, A17, A34) |
| `05-rendering-and-animation.md` | 143 | 16,7 ms/frame, propriedades compositor-only, CSS vs JS animation, `will-change` | quase todo irrelevante: não emitimos animação; sustenta apenas "não medimos FPS" (§5d) |
| `06-core-web-vitals.md` | 184 | limiares LCP/INP/CLS, causas, checklists, `font-display`, `font-size-adjust` | separa "atributo que causa" de "métrica"; base de A20–A26 |
| `07-measurement-and-monitoring.md` | 175 | Navigation/Resource/Long Tasks/LoAF/Event Timing, tabela RUM vs sintético, formato de budget Lighthouse CI | dá o formato do gate e o corte CI↔campo (A31–A33) |
| `08-modern-patterns.md` | 86 | checklist consolidado por área, incluindo "Resource Hints as HTTP headers" | tabela de conferência; a linha de headers é fronteira do consumidor (A35) |
| `09-anti-patterns.md` | 97 | tabelas ❌→substituto por área, incluindo mobile; "Golden Rules" | lista de proibições que viram invariantes (A18, A20, A28) |
| `10-startup-performance.md` | 195 | Web Workers, code splitting, decoders nativos, Emscripten, compressão, percepção | útil só em "compressão sempre ligada" (A30); o resto é JS, fora do escopo |
| `11-network-deep-dive.md` | 128 | TCP slow start/CWND, componentes de latência, presets de throttling, preload scanner | **fonte primária de A1–A3 e A10**; dá os números de throttling para medição sintética |
| `12-glossary.md` | 248 | 80+ termos e tabela final de limiares | referência de definição; a tabela só serve para RUM (§5d) |
| `13-performance-apis.md` | 185 | `performance.now`, bfcache/`notRestoredReasons`, Font Loading, Beacon, Navigation Preload, `requestIdleCallback` | rende um achado real: `no-store` bloqueia bfcache (A29) |
| `14-adaptive-content.md` | 241 | `srcset`/`sizes`/`<picture>`, Network Information, `deviceMemory`, Client Hints | fonte de A19 e da refutação de Client Hints (A36) |
| `15-delivery-and-caching.md` | 206 | Brotli vs gzip, `Cache-Control` por tipo de recurso, ETag/304, Service Worker, CDN | fonte do regime duplo de cache (A28–A30) |
| `16-profiling-and-tooling.md` | 166 | DevTools, flame graph, Lighthouse, WebPageTest, leitura de waterfall, mediana de 3 execuções | define o protocolo mínimo de qualquer medição manual (A33) |
| `EXAMPLE.md` | 911 | implementação de referência ponta a ponta (HTML, CSS crítico, JS, SW, Nginx, lhci, RUM, CI) | mina de números concretos; também a maior fonte de receitas incompatíveis com CSP |
| `SKILL.md` | 86 | "Core Rules", "Core Patterns", mapa de arquivos, ordem de leitura | resumo executivo; única menção a `subset` de fonte e ao par `immutable` + hash |
| `VERSION` | 1 | `2.0.0` | fixa a versão do título |

## 2. Achados

### A1. O primeiro 14 KB é o orçamento real do prefixo
- Citação: `11-network-deep-dive.md:19` — "Initial CWND = 10 MSS ≈ 14KB"
- Semântica: o servidor envia ~1 janela de congestionamento sem esperar ACK; o que não cabe custa RTT.
- Consequência: o `<head>` inteiro (charset, viewport, meta, hints, preload, stylesheet, scripts) precisa
  caber nesse orçamento — o que só é possível se a ordem e o conteúdo do `<head>` forem fixos.
- Veredito: confirma P6/D14 e o gate de `docs/design/09 §4`; **refina** (falta o número absoluto).

### A2. Cada byte além do 14 KB custa um round trip
- Citação: `11-network-deep-dive.md:125` — "**Ignoring the first 14KB budget** — every byte beyond it costs an extra RTT"
- Semântica: o custo é por pacote, não por byte; o excesso antecipa a segunda janela.
- Consequência: o gate precisa medir o **prefixo** (até o último elemento do `<head>`), não a página; a
  página tem orçamento separado, porque depois do primeiro paint o custo é banda, não latência.
- Veredito: refina o gate atual, que só compara "HTML de página de lista" (`docs/design/09 §4`).

### A3. A unidade do 14 KB é ambígua no corpus
- Citação: `EXAMPLE.md:189` — "/* This file is inlined in <head> — keep under 14KB (compressed) */"
- Semântica: aqui "14 KB" é comprimido; em `11-network-deep-dive.md:25` ("the first **14KB** of a
  response") é bytes no fio — que, com `Content-Encoding: br`, também são comprimidos.
- Consequência: o número só é verificável se a crate declarar a medida. Medir HTML descomprimido reprova
  páginas que cabem no primeiro pacote e produz um gate impossível de satisfazer.
- Veredito: **lacuna do corpus**; [INFERENCE] a leitura correta é "bytes no fio após Brotli".

### A4. "Inline critical CSS" é a receita central — e nós a proibimos
- Citação: `03-critical-rendering-path.md:72` — "└─ Inline critical CSS (< 14KB)"
- Semântica: o corpus trata `<style>` no `<head>` como o mecanismo padrão de acelerar o first paint.
- Consequência: proibido por D3/CSP (`<style>` não existe, `docs/design/07 §5`). O ganho passa a vir de outra
  alavanca: **um** stylesheet pequeno + ordem fixa + orçamento — nunca de inlining.
- Veredito: **refuta** para o nosso desenho; registra o custo aceito (perdemos o ótimo do corpus).

### A5. A troca `preload`→`stylesheet` por `onload` é impossível aqui
- Citação: `04-loading-strategies.md:129` — `<link rel="preload" href="/styles/styles.css" as="style" onload="this.onload=null;this.rel='stylesheet'" />`
- Semântica: a receita de CSS não-crítico do corpus depende de JavaScript inline no atributo.
- Consequência: `on*` não existe (`spec/elements.toml [element.link].allowed`, `[sem linha]`), e o
  `<noscript>` de fallback perde função sem o swap. Ou o CSS é crítico, ou não é carregado.
- Veredito: refuta; a alternativa real é `Link: rel=preload` no header (A35), fora do markup.

### A6. `<style>` no template canônico de `<head>`
- Citação: `04-loading-strategies.md:126` — `<style>/* ... */</style>`
- Semântica: o template de referência põe CSS inline entre os preloads e os scripts.
- Consequência: nosso `<head>` tem um slot a menos e um `Link` a mais; qualquer número de "recursos no
  head" copiado do corpus já conta um recurso que não emitimos.
- Veredito: refuta como receita; irrelevante como código.

### A7. `@import` serializa — e o CSS de tema é nosso, então dá para garantir
- Citação: `03-critical-rendering-path.md:134` — "❌ **`@import` in CSS** — serializes loading instead of parallel `<link>` tags"
- Semântica: `@import` cria dependência em série e atrasa o CSSOM inteiro.
- Consequência: `/theme.css` é gerado pela crate a partir de tokens (`docs/design/06 §4`); o schema já recusa
  `url()`/`@import` (`docs/design/08 §4`). Falta a asserção sobre os **bytes** do CSS emitido.
- Veredito: confirma D12; **refina** (falta teste de saída).

### A8. `media` como escape para CSS não-bloqueante
- Citação: `02-how-browsers-work.md:70` — "Critical CSS inline, media queries for non-critical, `preload`"
- Semântica: `<link rel=stylesheet media="print">` não bloqueia a renderização até a impressão.
- Consequência: `media` está na allowlist de `link`, então é emitível — mas usá-lo para esconder CSS
  não-crítico cria segunda folha e condição de mídia que precisa casar com o breakpoint real.
- Veredito: refina; vira restrição de vocabulário (`media="print"` apenas), não feature nova.

### A9. Existe uma ordem de `<head>` canônica no corpus
- Citação: `04-loading-strategies.md:114-115` — `<meta charset="utf-8" />` … `<meta name="viewport" content="width=device-width,initial-scale=1" />`
- Semântica: charset e viewport primeiro; depois hints, preloads, CSS, scripts.
- Consequência: o shell é dono único do `<head>` (`docs/design/06 §1`), logo essa ordem pode ser fixa e testada
  por bytes — a maior vitória barata deste conjunto.
- Veredito: confirma D11; **refina** (a ordem não está escrita em nenhum doc).

### A10. O preload scanner só vê o que está no HTML
- Citação: `11-network-deep-dive.md:127` — "**JavaScript-discovered resources** — the preload scanner cannot see them"
- Semântica: recurso criado por JS não é descoberto na varredura antecipada e perde um RTT.
- Consequência: validade retroativa de `ctx.require(Asset)` — todo asset existe como tag no HTML
  (`docs/design/07 §2`). Proíbe também "carregar CSS/JS condicionalmente por script".
- Veredito: confirma D13 por via independente; nenhum código novo.

### A11. Injeção condicional de recurso quebra o scanner
- Citação: `02-how-browsers-work.md:103` — "don't interfere by having server-side conditional resource injection that the speculative parser can't see"
- Semântica: a varredura antecipada depende de o HTML estar completo no primeiro pacote.
- Consequência: `ctx.require` precisa rodar antes do shell — o que a assinatura já força (`shell()` consome
  o `Ctx`, `docs/design/06 §9`) — e nenhum `<link>` pode aparecer depois do fecho do `<head>`.
- Veredito: confirma a assinatura atual; irrelevante como mudança.

### A12. `preload` só para o que é descoberto tarde
- Citação: `04-loading-strategies.md:154` — "❌ **`<link rel="preload">` for resources that are already eagerly discovered** — wastes preload scanner capacity"
- Semântica: preload de stylesheet/script que já está no `<head>` é trabalho duplicado.
- Consequência: no nosso `<head>` o único `preload` legítimo é o da **fonte** (descoberta só quando o CSSOM
  é parseado) e, opcionalmente, o da imagem herói (que está no `<body>`, depois do CSS).
- Veredito: refina a prática comum; vira invariante "preload só de fonte e imagem herói".

### A13. Teto de `preconnect`: 2–3 críticos, 3–6 no limite
- Citação: `04-loading-strategies.md:155` — "**Preconnecting to too many origins** — uses socket and memory resources; limit to 3-6"
- Semântica: cada `preconnect` abre socket e TLS de verdade, mesmo que nunca seja usado.
- Consequência: com origem única e sem CDN (`docs/design/07 §8`), o número normal é **zero**; o teto vira regra
  do caso de terceiro (VLibras) e precisa ser contado no gate.
- Veredito: confirma D9 (dependências mínimas); refina com número.

### A14. `preconnect`/`preload` de fonte **exigem** `crossorigin`
- Citação: `02-how-browsers-work.md:111` — "**Forgetting `crossorigin` on preconnected font origins** — fonts are loaded in anonymous mode; without `crossorigin`, only DNS lookup happens"
- Semântica: sem `crossorigin` a conexão não é reutilizada pela requisição de fonte (modo anônimo ≠ credentials).
- Consequência: o atributo não pode ser opcional no caminho de fonte; hoje ele **não está na allowlist de
  `link`**, embora o invariante `preload_requires_as_and_crossorigin` o exija — contradição executável (§4).
- Veredito: **refuta** o estado atual da spec; correção obrigatória.

### A15. `defer` vs `async` é escolha de ordenação, não de velocidade
- Citação: `02-how-browsers-work.md:97-99` — "| `<script defer>` | No | After parse, before `DOMContentLoaded` | Most scripts |"
- Semântica: `defer` preserva ordem entre scripts; `async` executa quando chega; módulo é deferido por padrão.
- Consequência: o shell emite assets em ordem canônica (`BTreeSet`); com `async` essa ordem deixa de
  significar algo. Regra: `defer` sempre, `async` só para asset sem dependência (`Analytics`).
- Veredito: confirma D13; refina a emissão (hoje `defer` e `async` são opcionais e combináveis).

### A16. Script síncrono no `<head>` bloqueia parser e render
- Citação: `02-how-browsers-work.md:108` — "❌ **`<script>` tags without `defer`/`async` in `<head>`** — blocks rendering"
- Semântica: é o anti-padrão mais citado do conjunto — e a spec atual o permite, porque `<script src>` sem
  `defer` nem `async` é HTML válido segundo `allowed`.
- Consequência: precisa ser impossível por tipo (enum `ScriptLoading::{Defer, Async}`), não por aviso.
- Veredito: **refuta** a spec atual; correção em `spec/elements.toml` + gerador.

### A17. `fetchpriority="high"` pertence a um elemento, não a uma página
- Citação: `04-loading-strategies.md:71` — `<img src="hero.webp" fetchpriority="high" />  <!-- LCP element -->`
- Semântica: a dica é relativa dentro do mesmo tipo de recurso; marcar tudo como `high` não muda nada.
- Consequência: a API precisa de prioridade explícita (`Fold`/`Priority`) no primitivo, e o lint de saída
  precisa falhar com **mais de um** `fetchpriority="high"` por página.
- Veredito: refina; nenhum tipo novo de contexto, só um enum fechado.

### A18. `loading="lazy"` na imagem acima da dobra atrasa o LCP
- Citação: `06-core-web-vitals.md:172` — "❌ **`loading="lazy"` on LCP image** — delays LCP; use `fetchpriority="high"` instead"
- Semântica: o herói é o candidato natural a LCP; adiá-lo é o pior caso possível.
- Consequência: `Fold::{Above, Below}` no primitivo. A spec já declara `fold_controls_loading`
  (`spec/elements.toml [element.img].invariants`, `[sem linha]`), mas o invariante não está definido em
  lugar nenhum (A37).
- Veredito: confirma a intenção registrada; refina (falta definição normativa e teste).

### A19. `srcset` sem `sizes` assume `100vw`
- Citação: `14-adaptive-content.md:236` — "**Ignoring `sizes` attribute** — without it, the browser assumes `100vw`, often leading to oversized image downloads"
- Semântica: sem `sizes` o navegador escolhe a variante para a largura inteira do viewport.
- Consequência: `SrcSet` e `Sizes` são **um par de tipos**; a spec já tem
  `srcset_requires_sizes_and_dims`, que também cobre dimensões e portanto resolve A20 no caso responsivo.
- Veredito: confirma o invariante já escrito; pendente de definição e teste (§5c).

### A20. Dimensões ausentes são a maior causa de CLS
- Citação: `06-core-web-vitals.md:173` — "❌ **Images without dimensions** — single biggest cause of CLS"
- Semântica: sem proporção intrínseca o navegador reserva 0×0 e desloca o conteúdo ao decodificar.
- Consequência: `width`/`height` são `Option` na spec e o gerador os declara opcionais
  (`docs/design/03 §7`). Toda imagem sem dimensão é CLS em potencial; o único caso legítimo de ausência é pixel
  de tracking, que não usamos.
- Veredito: **refuta** o "opcional"; correção em §4 (major, `docs/design/11 §2`).

### A21. `aspect-ratio` é o substituto moderno, mas mora no CSS
- Citação: `06-core-web-vitals.md:108` — `.hero { aspect-ratio: 16 / 9; }`
- Semântica: reservar a proporção por CSS evita shift mesmo com `height:auto`.
- Consequência: a crate emite só inteiros (`WidthPx`/`HeightPx`); `aspect-ratio` é vocabulário de CSS do
  consumidor. Nada a emitir — os atributos são o contrato mínimo.
- Veredito: irrelevante para o markup; registrar como nota.

### A22. `decoding="async"` em toda imagem
- Citação: `EXAMPLE.md:118-119` — `fetchpriority="high"` … `decoding="async"`
- Semântica: decodificação fora do caminho síncrono do parser; não bloqueia a primeira pintura.
- Consequência: é atributo já na allowlist de `img` e sempre correto; logo deve ser emitido pelo primitivo,
  sem opção, custando zero bytes de decisão.
- Veredito: refina; vira invariante de emissão (§5a), não regra de lint.

### A23. `content-visibility` sem `contain-intrinsic-size` troca pintura por shift
- Citação: `EXAMPLE.md:126` — `<section style="content-visibility: auto; contain-intrinsic-size: 400px">`
- Semântica: a seção fora da tela não renderiza; sem altura intrínseca declarada a barra de rolagem pula.
- Consequência: é CSS, não atributo; o caminho legítimo é `SectionId` + regra fixa no `/theme.css`.
  Emitir por elemento exigiria `style=`, que não existe (D3). O `docs/design/09 §3.1` já registra o par.
- Veredito: confirma D12 e o doc atual; **armadilha** se tentarmos modelar altura no markup (§6).

### A24. `display:none` para conteúdo fora da tela não economiza nada
- Citação: `09-anti-patterns.md:32` — "**`display: none` to hide off-screen content** | DOM still exists; doesn't save layout/paint work"
- Semântica: o nó permanece no DOM e o custo de estilo permanece; só `content-visibility` pula o trabalho.
- Consequência: nenhum atributo nosso produz esse efeito; a regra alcança apenas o CSS do consumidor.
- Veredito: irrelevante para o markup.

### A25. `font-display: swap` é a única escolha aceitável — e é testável em bytes
- Citação: `09-anti-patterns.md:40` — "**`font-display: block` (or default)** | FOIT — invisible text for up to 3s"
- Semântica: com `block` (o default), o texto fica invisível enquanto a fonte carrega.
- Consequência: o `@font-face` é gerado pela crate no CSS de tema; "swap sempre" é gate sobre os bytes do
  `/theme.css`, um dos poucos lugares onde medimos CSS diretamente.
- Veredito: confirma `docs/design/09 §3`; refina (o doc afirma, não testa).

### A26. `font-size-adjust: from-font` reduz — não zera — o shift do swap
- Citação: `06-core-web-vitals.md:129-133` — "**CSS font-size-adjust** (modern) to reduce layout shift when fallback fonts swap" … `font-size-adjust: from-font;`
- Semântica: casa a altura-x da fallback com a da fonte real, diminuindo o reflow do swap.
- Consequência: `from-font` só funciona se a fallback declarar métricas; portanto é regra de uma linha no
  CSS base, nunca por componente.
- Veredito: refina o "CLS 0" registrado antes; vira nota, não garantia.

### A27. Fonte: uma família, poucos pesos, `woff2`, subset
- Citação: `SKILL.md:29` — "Fonts: `font-display: swap`, preload critical fonts, subset."
- Semântica: cada peso é um arquivo; preload de vários compete com o herói pelo primeiro pacote.
- Consequência: o `docs/design/09 §3` fala em "`preload` do único arquivo de texto", frase que descreve artefato
  inexistente (texto não tem arquivo). Precisa virar: 1 família, `woff2`, `font-display: swap`,
  `preload as="font" type="font/woff2" crossorigin`, ≤ 2 pesos.
- Veredito: **corrige** a redação atual; sem impacto de arquitetura.

### A28. Dois regimes de cache, nunca um só
- Citação: `15-delivery-and-caching.md:87` — "| Versioned JS/CSS | `max-age=31536000, immutable` | `app.abc123.js` |"
- Semântica: asset com hash no nome nunca muda → `immutable` por um ano; HTML muda → revalidação.
- Consequência: o regime de asset já está em `docs/design/07 §8`; o lado HTML só tem `ETag` em `docs/design/09 §2`.
  Como o nome carrega `sha256` do conteúdo, `immutable` é seguro **por construção**.
- Veredito: confirma D14; refina (o lado HTML precisa da regra explícita em `docs/design/09`).

### A29. `304` custa zero bytes; `no-store` custa o bfcache
- Citação: `15-delivery-and-caching.md:80` — "# Response: 304 Not Modified (zero bytes transferred)"
- Semântica: revalidar é mais barato que reenviar; `no-store` impede qualquer cache, inclusive o de histórico.
- Consequência: `13-performance-apis.md:67` lista `Cache-Control: no-store` como bloqueio de bfcache. Logo
  HTML público **nunca** leva `no-store`; o `docs/design/09 §2` precisa dizer isso e reservar `no-store` para
  preview/área autenticada (`docs/design/06 §6`).
- Veredito: confirma `ETag`/304; **refina** a política de `Cache-Control`.

### A30. Brotli pré-comprimido no build — e o serving tem duas linhas obrigatórias
- Citação: `15-delivery-and-caching.md:5` — "Compression reduces transfer size by 70-85%"
- Semântica: o ganho é de ordem de magnitude em texto; comprimir por request paga CPU sem ganho.
- Consequência: `docs/design/09 §3` já manda gerar `.br`/`.gz` no build com hash — coerente. Falta a consequência
  de serving: `Content-Encoding: br` + `Vary: Accept-Encoding`, senão cache intermediário entrega binário
  a cliente que não aceita.
- Veredito: confirma `docs/design/09 §3`; refina (headers).

### A31. Orçamentos são numéricos e por categoria de recurso
- Citação: `07-measurement-and-monitoring.md:72-75` — `"resource-summary": { "script": { "maxSize": "300KB" }, "image": { "maxSize": "500KB" }, "total": { "maxSize": "1MB" } }`
- Semântica: budget é limite quantitativo por categoria, verificado a cada PR.
- Consequência: dá o formato; os números do exemplo servem a uma SPA e precisam ser reescalados para um
  portal sem JS de aplicação — ver §5b.
- Veredito: confirma o gate de `docs/design/09 §4`; refina (números).

### A32. RUM vs sintético: papéis que não se substituem
- Citação: `07-measurement-and-monitoring.md:53` — "| **Repeatability** | ✅ Yes (deterministic) | ❌ No (varies per user) |"
- Semântica: laboratório é determinístico e serve a regressão; campo tem variância e serve a tendência.
- Consequência: o único teste honesto em CI é sobre **atributos e bytes**; qualquer número de tempo em CI
  é ruído com nome bonito. Valida o desenho "orçamento de bytes com ratchet".
- Veredito: confirma `docs/design/12 §7` e D10 (tipo primeiro, advisory com ratchet).

### A33. Métrica de campo não é gate
- Citação: `07-measurement-and-monitoring.md:163` — "❌ **Relying solely on Lighthouse scores** — they're lab data, not real user experience"
- Semântica: score sintético ≠ experiência real; P75 de RUM exige milhares de amostras (`09-anti-patterns.md:59`).
- Consequência: gate de CWV numérico no nosso CI seria caro e não confiável; o protocolo mínimo de qualquer
  medição manual é mediana de 3 execuções (`16-profiling-and-tooling.md:162`).
- Veredito: confirma a decisão de não medir campo no CI; base do §5d.

### A34. Speculation Rules são `<script>` inline — fora por construção
- Citação: `04-loading-strategies.md:44` — "<script type=\"speculationrules\">"
- Semântica: prefetch/prerender declarados como JSON inline no documento.
- Consequência: `script` exige `src` e só aceita `JsonLd` (`spec/elements.toml [element.script]`); a única
  forma seria arquivo externo via header `Speculation-Rules` — decisão de servidor, não de markup.
- Veredito: refuta como markup; possível apenas como header do consumidor.

### A35. `103 Early Hints` é a alternativa legítima ao preload no HTML
- Citação: `08-modern-patterns.md:81` — "| **Resource Hints as HTTP headers** | `Link: <...>; rel=preload` via `103 Early Hints` ou response headers | High |"
- Semântica: o header chega antes do corpo; o download começa antes do primeiro byte de HTML.
- Consequência: é fronteira do consumidor (`docs/design/08 §7`: cabeçalhos HTTP são do consumidor). A crate pode
  documentar a receita e **não** deve ter API de hints.
- Veredito: confirma o não-objetivo; vira nota em `docs/design/09`, não código.

### A36. Client Hints mudam a resposta por cliente e quebram o `ETag`
- Citação: `14-adaptive-content.md:166` — `res.setHeader('Vary', 'DPR, Width, ECT');`
- Semântica: servir imagem por `DPR`/`ECT` exige `Vary` e multiplica variantes de cache.
- Consequência: `meta http-equiv="Accept-CH"` (`EXAMPLE.md:44`) é proibido pela spec (`no_http_equiv`) —
  decisão certa: negociação por cliente fragmenta cache e destrói o `ETag` canônico de D14.
- Veredito: confirma a proibição; refuta a receita do corpus para o nosso caso.

### A37. Invariantes nomeados sem catálogo normativo
- Citação: `spec/elements.toml [element.img].invariants` — `["srcset_requires_sizes_and_dims", "alt_empty_means_decorative", "fold_controls_loading"]`
- Semântica: a spec cita nomes de invariantes (`preload_requires_as_and_crossorigin`,
  `fold_controls_loading`, `srcset_requires_sizes_and_dims`) que não são definidos em nenhum arquivo do
  repositório — só o corpus sustenta o significado, implicitamente.
- Consequência: contrato citado sem definição não é verificável nem testável; cada nome precisa de uma
  linha normativa em `docs/design/03 §3` e de um teste (§5c).
- Veredito: **conflito de processo**: a spec referencia contrato inexistente.

## 3. Confirmações do desenho

1. **Ordem determinística de assets** (`BTreeSet`, `docs/design/07 §2`) é pré-condição do orçamento de 14 KB: sem ordem estável, o prefixo varia por request (`11:19`).
2. **`ctx.require` declarativo** confirma-se por dois lados: o scanner só vê tags (`11:127`) e injeção condicional o atrapalha (`02:103`).
3. **Origem única sem CDN** (`docs/design/07 §8`) elimina a maior parte do capítulo de hints: `preconnect`/`dns-prefetch` para a própria origem são inúteis (`04:156`).
4. **`ETag` = `sha256` dos bytes finais** (`docs/design/09 §2`) é o que torna `immutable` seguro em asset com hash (`15:87`) e `304` sem corpo (`15:80`).
5. **`SrcSet` ordenado por largura e agrupado por MIME** (`docs/design/02 §7`) é o único jeito de o `srcset` ser estável — e ele exige `sizes` casado (`14:236`).
6. **`WidthPx`/`HeightPx` (> 0, sem unidade)** (`docs/design/02 §7`) é exatamente o formato com que o navegador reserva espaço (`06:36`).
7. **CSP sem inline** ganha justificativa de performance além da de segurança: as duas receitas centrais do corpus (`03:72`, `04:129`) exigem inline; sobra um head menor e mais previsível.
8. **`no_http_equiv`** (`spec/elements.toml [element.meta]`) barra `Accept-CH` e `Refresh`, ambos com efeito em cache (`14:166`).
9. **Orçamento por ratchet** (`docs/design/12 §7`) é a forma correta de gate, porque só bytes são determinísticos em CI (`07:53`).
10. **Dimensões e `Alt` no tipo** mantêm a decisão no call site — o corpus mostra que "é decorativa?" e "que tamanho?" nunca têm default seguro (`09:39`).

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata + arquivo) |
|---|---|---|
| `spec/elements.toml [element.link].allowed` — `["as", "type", "media", "integrity", "hreflang", "sizes", "fetchpriority", "disabled"]` (sem `crossorigin`) | `02:111` — sem `crossorigin` "only DNS lookup happens" | Em `spec/elements.toml`, inserir `"crossorigin"` na allowlist de `link`, logo após `"integrity"`. O invariante `preload_requires_as_and_crossorigin` já o exige. |
| `script` aceita `defer` e `async` ambos opcionais (`[element.script].allowed`) | `02:108` — script sem `defer`/`async` bloqueia render | Em `spec/elements.toml`, `[element.script]`: `required = ["src", "defer"]` e `async` sai da allowlist, passando a tipo `ScriptLoading::{Defer, Async}` no gerador. |
| `img` com `width`/`height` opcionais (`[element.img].required = ["src", "alt"]`; `docs/design/03 §7` mostra `width?: Option<WidthPx>`) | `06:173` — "single biggest cause of CLS" | Em `docs/design/03 §3`, linha do `img`: "`img` — `src` + `alt` + `width` + `height` obrigatórios no primitivo `Img`; ausência de dimensão é erro de compilação". |
| `docs/design/09 §3` — "Fontes: `woff2` com `font-display: swap`, `preload` do único arquivo de texto." | `02:89` + `SKILL:29` — preload é de **fonte**, com `crossorigin`, `type` e subset | Substituir a frase por: "Fontes: uma família, no máximo dois pesos, `woff2` com subset, `font-display: swap`, e `preload as=\"font\" type=\"font/woff2\" crossorigin` do peso do corpo de texto." |
| Nenhuma ordem de `<head>` declarada | `04:114-133` — charset, viewport, hints, preload, CSS, scripts | Inserir em `docs/design/06 §1`, após a lista de campos do `ShellCtx`: "**Ordem fixa do `<head>`**: `charset` → `viewport` → `title` → `meta` de SEO → `preconnect`/`dns-prefetch` → `preload` → `link rel=stylesheet` → `link` do `/theme.css` → `script defer` → `JsonLd`. Outra ordem é erro do shell, não configuração." |
| `docs/design/09 §2` — `Cache-Control: public, no-cache` para HTML com formulário, sem tratar `no-store`/bfcache | `15:87-92`, `13:67` | Completar: "HTML público: `public, no-cache` + `ETag`; área autenticada: `private, no-cache`; `no-store` só em preview e rascunho — `no-store` em página navegável desliga o bfcache." |
| `docs/design/09 §4` — só ratchet percentual ("não pode crescer além de N%") | `11:25` — o prefixo tem limite absoluto | Inserir em `docs/design/09 §4`: "Além do ratchet: prefixo do `<head>` (até o último elemento do head) ≤ 14 KB comprimidos; HTML de página de lista ≤ 50 KB comprimidos." |
| `docs/design/09 §3` — `.br`/`.gz` no build, sem regra de serving | `15:5`, `15:13` | Acrescentar: "O `.br` é servido com `Content-Encoding: br` e `Vary: Accept-Encoding`; o `.gz` é fallback, nunca o padrão." |
| `link` sem enum de `rel` (`link_rel_enum` citado, não definido) | `04:22-29` lista os hints válidos | Em `docs/design/03 §3`, linha do `link`: "`rel` ∈ {`stylesheet`, `preload`, `preconnect`, `dns-prefetch`, `canonical`, `alternate`, `icon`}; `preload` exige `as`, exige `crossorigin` quando `as=\"font\"`, e é proibido para recurso já presente no `<head>`." |
| `docs/design/07 §8` — "Sem CDN" sem consequência de TTFB | `06:42` — "Improve TTFB (CDN, early hints, server optimization)" | Acrescentar em `docs/design/09 §3`: "Sem CDN, o TTFB é do servidor do consumidor: o orçamento de 800 ms é medido no origin e não é compensável pelo markup." |
| Nenhum limite no número de hints | `04:155` — "limit to 3-6" | Inserir em `docs/design/06 §8`: "No máximo dois `preconnect` por página, e nenhum para a origem do próprio site." |
| `docs/design/06 §8` não menciona `preload` | `04:154` — preload de recurso já descoberto desperdiça o scanner | Acrescentar ao item 6 da lista: "`preload` só para fonte e imagem herói; nunca para stylesheet ou script que já estão no `<head>`." |

## 5. Adições prontas para colar

### (a) Regras de emissão por primitivo

| Primitivo / elemento | Emite sempre | Nunca emite | Camada que garante |
|---|---|---|---|
| `Img` herói | `width`, `height`, `decoding="async"`, `fetchpriority="high"` | `loading="lazy"` | N1 (`Fold::Above`) |
| `Img` abaixo da dobra | `width`, `height`, `decoding="async"`, `loading="lazy"` | `fetchpriority="high"` | N1 |
| `Img` responsivo | `srcset` + `sizes` + `width` + `height` | `srcset` sem `sizes` | N1/N2 (`srcset_requires_sizes_and_dims`) |
| `Picture` | `source` (avif → webp, nessa ordem) + `img` com dimensões | `<picture>` sem `img` final | N2 |
| `Iframe` | `title`; `loading="lazy"` quando abaixo da dobra | embed terceiro sem `loading` | N2 |
| `Video` | `preload="none"` (ou `metadata`), `poster` com dimensões, `track` | `preload="auto"` | N1 |
| Fonte (`/theme.css`) | `@font-face{font-display:swap}` + `preload as="font" type="font/woff2" crossorigin` | segundo `preload` de fonte | N4 (bytes do CSS + head) |
| Estilos | exatamente um `<link rel="stylesheet">`; `/theme.css` só com override | `@import`, `<style>`, `media` condicional | N4 |
| Scripts | `defer` (ou `async` só em `Asset::Analytics`) | `<script>` sem `src` e sem `defer` | N1/N2 |
| `Head` | ordem fixa de §4 | hint fora de `preconnect`/`dns-prefetch`/`preload` | N4 |
| `JsonLd` | último elemento do `<head>` | qualquer script executável | N1 |

### (b) Orçamentos (números concretos)

| Item | Limite | Como medir | Fonte do número |
|---|---|---|---|
| Prefixo do `<head>` (até o último elemento do head) | ≤ 14 KB comprimidos | tamanho do `.br` do trecho, no build | `11:19`, `11:25` |
| HTML de página de lista | ≤ 50 KB comprimidos | `.br` do corpo de `Page` | `docs/design/09 §4` + formato de `07:72-75` |
| CSS base | ≤ 30 KB comprimidos | `.br` do asset | `15:5` |
| `/theme.css` | ≤ 8 KB comprimidos | `.br` do CSS canônico | `docs/design/06 §4` (determinismo) |
| JS inicial por página | ≤ 150 KB comprimidos | soma dos assets de `Ctx` | `EXAMPLE.md:671` |
| JS total (todos os bundles) | ≤ 400 KB comprimidos | registro de assets | `EXAMPLE.md:672`, ajustado às 2.804 LOC |
| Fonte | ≤ 50 KB por arquivo, ≤ 2 arquivos | tamanho do `woff2` | `EXAMPLE.md:673` |
| Imagem herói | ≤ 200 KB | asset | `07:74` |
| Imagens da página | ≤ 500 KB no total | soma dos assets | `07:74` |
| Requisições no `<head>` | ≤ 4 (CSS + theme + fonte + script) | contagem na saída | `04:110-133` |
| `preconnect` | ≤ 2, nunca a própria origem | contagem na saída | `04:155`, `04:156` |
| `fetchpriority="high"` | exatamente 1 por página | contagem na saída | `04:71`, `06:172` |
| Medição manual | mediana de 3 execuções, Regular 3G (750/250 kbps, 100 ms) | protocolo | `16:162`, `11:54`, `11:60` |

### (c) Invariantes de teste

| Invariante | Onde falha | Fonte |
|---|---|---|
| prefixo do `<head>` ≤ 14 KB | N4 (bytes) | `11:25` |
| toda `img` tem `width` + `height` | N1 (tipo) + N4 (rede) | `06:173` |
| `srcset` ⇒ `sizes` + dimensões | N1/N2 | `14:236` |
| `loading="lazy"` nunca na imagem herói | N1 (`Fold`) | `06:172` |
| no máximo um `fetchpriority="high"` | N4 | `04:71` |
| `preload` só de fonte e imagem herói | N4 | `04:154` |
| todo `preload as="font"` tem `crossorigin` e `type="font/woff2"` | N2 | `02:89`, `02:111` |
| `preconnect`/`dns-prefetch` nunca para a origem do site | N4 | `04:156` |
| exatamente um `link rel=stylesheet` por página | N4 | `03:134` |
| `/theme.css` tem `font-display:swap` e não tem `@import` | N4 (bytes do CSS) | `09:40`, `02:109` |
| nenhum `<script>` executável sem `defer`/`async` | N2/N4 | `02:108` |
| ordem do `<head>` igual à canônica (sequência de tags) | N4 (golden) | `04:114-133` |
| nenhum `meta http-equiv` | N2 | `EXAMPLE.md:44`, `14:166` |
| dois renders → mesmos bytes | property | `docs/design/12 §3` |

### (d) O que **não** se testa, e por quê

| Não testar | Por quê (evidência) |
|---|---|
| LCP, INP, CLS numéricos | são de campo; em CI variam por VM e rede (`07:53`, `07:163`) |
| score do Lighthouse, Speed Index, TBT | "lab data, not real user experience" (`07:163`); score agrega pesos arbitrários |
| TTFB medido | depende do servidor e do caminho de rede; sem CDN é ruído local (`docs/design/07 §8`) |
| P75 de qualquer métrica | exige milhares de amostras (`09:59`); PR não tem amostra |
| taxa de compressão real | depende do conteúdo; o gate é o tamanho do `.br` (`15:5`) |
| FPS, long tasks, jank | não emitimos JS nem animação (`05:16-22`) |
| bytes transferidos por rede | já coberto por "tamanho do asset", sem variância de rede |
| bfcache hit rate | depende da navegação do usuário (`13:54-69`); só a política de `Cache-Control` é nossa |

## 6. Armadilhas de complexidade

1. **Critical CSS automático (Critters/Penthouse)** — exigiria browser headless no build para descobrir o
   CSS acima da dobra; entra dependência grande, morre o determinismo e o `ETag`. O corpus trata isso como
   padrão (`03:72`); aqui é proibido. Custo: build não reprodutível.
2. **Inlining de CSS no `<head>`** — exigiria `<style>`, reabrindo D3. Custo: CSP, `ETag` e cache por página.
3. **Detecção de LCP no servidor** — "qual imagem é o herói" só existe em runtime; heurística vira
   configuração por view. Custo: superfície de config sem garantia.
4. **`imagesrcset`/`imagesizes` no `preload`** — resolver preload de imagem responsiva exigiria dois
   atributos novos em `link` sincronizados com o `srcset` do `<img>`; errar gera download duplo.
   [INFERENCE] O corpus não trata esse par (`04:123` usa `as="image"` com `href` fixo).
5. **Client Hints / `Accept-CH`** — negociação por cliente fragmenta cache e mata o `ETag` (`14:166`).
   Custo: N variantes por página.
6. **Speculation Rules / prerender** — é `<script>` inline (`04:44`); exigiria exceção de CSP. Custo: D3.
7. **Service Worker / navigation preload** (`15:135-160`) — não é markup e cria um segundo regime de cache
   fora do nosso controle. Custo: modo de falha invisível ao `cq check`.
8. **RUM embutido na crate** — exigiria script próprio e endpoint; a crate passaria a ter opinião sobre
   analytics. Custo: acoplamento e `connect-src`.
9. **`content-visibility` por elemento** — exigiria `style=` ou classe por seção; modelar
   `contain-intrinsic-size` no markup é a armadilha (A23). Caminho barato: `SectionId` + CSS fixo.
10. **Subset de fonte no build** — exige ferramenta externa e conhecimento dos glifos dos 4 locales; fica
    no pipeline do consumidor, não no crate.

## 7. Lacunas do corpus

1. **Unidade do 14 KB** — comprimido ou não; o corpus usa as duas leituras (A3). A crate precisa decidir.
2. **`preload` de imagem responsiva** — nenhum tratamento de `imagesrcset`/`imagesizes`; sem eles, o preload
   do herói com `srcset` pode baixar a variante errada.
3. **Alternativa ao CSS crítico** — o corpus não oferece saída para quem não pode usar `<style>`; a nossa
   ("uma folha, pequena, ordenada") é inferência.
4. **`Cache-Control` de página com formulário** — o corpus cobre HTML genérico (`15:89`) e bfcache (`13:67`),
   mas não o caso "página pública com formulário em área própria".
5. **Métricas de fallback de fonte** — `font-size-adjust` é citado, sem dizer que sem
   `ascent-override`/`size-adjust` o efeito é parcial.
6. **`<video preload>`** — nenhuma recomendação; a escolha `none`/`metadata` fica sem lastro.
7. **`Vary` × `ETag`** — só aparece de raspão (Client Hints); não há regra para conteúdo negociado.
8. **Orçamento de CSS por seção** — o corpus fala de CSS total; nosso modelo é por `SectionId`, sem número.
9. **INP sem JavaScript** — o corpus mede INP sempre em página com script; não define "bom" para página
   sem script, que é o nosso caso base.
10. **Compressão × CWND** — o corpus não relaciona Brotli (reduz o corpo) com o CWND (limita bytes no fio),
    nem diz se o head é comprimido antes do flush.
