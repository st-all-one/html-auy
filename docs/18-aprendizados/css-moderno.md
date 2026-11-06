# `css-moderno` — referência 2026 (nesting, layers, subgrid, container queries, `@property`)

Conjunto de 35 arquivos (`_RUST/css-moderno/`, 12.471 linhas) que descreve o CSS que um navegador
2024+ aceita: Grid/Subgrid, `@layer`, `@property`, container queries, `oklch`, `@starting-style`,
print. Não é um guia de arquitetura de CSS para servidor: é um **catálogo do que o navegador faz**.
Para a crate, o valor está em duas perguntas: o que o gerador **pode emitir** sem quebrar (tokens,
camadas, unidades) e o que ele **precisa proibir** para não reintroduzir `style=`, `@import`, JS
embutido ou especificidade incontrolável. Cada achado abaixo é verificável pela linha citada.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-introducao.md` | 68 | meta-guia; 7 princípios ("`@property` sobre var() simples", "`@layer` + `:where()` eliminam guerras") | fixa a filosofia; nada acionável além de §2 |
| `01-layout-grid.md` | 140 | grid explícito/implícito, `fr`, `grid-template-areas`, `auto-fill`+`minmax`, gap | fonte do layout do shell; `gap` > margin (01:133) |
| `02-subgrid.md` | 196 | subgrid em cards/formulários; comparação com `display:contents` | sustenta "sem `display:contents`" (02:131) |
| `03-performance.md` | 226 | pipeline style→layout→paint→composite; `content-visibility`, `contain`, `will-change`; loading | base das proibições de `@import`/inline/critical-CSS |
| `04-interoperabilidade.md` | 189 | resiliência do CSS, `@supports`, fallback 3 níveis, prefixos, tabela de suporte | decide o que é baseline 2026 e o que precisa de `@supports` |
| `05-variaveis-camadas.md` | 373 | custom properties, fallback, `@property`, `env()`, `attr()`, `@layer`, tema light/dark | **arquivo central**: ordem de camadas e falha silenciosa de `var()` |
| `06-responsivo.md` | 274 | grid intrínseco, `clamp`, container queries, unidades `cq*`, `env()`, tabela de decisão | define quando media query ainda é legítima |
| `07-padroes-componentes.md` | 362 | card/media/dialog/tabs/accordion/form; princípios de componente | contrato de componente: `--*-*` exposto, `contain` |
| `08-propriedades-registradas.md` | 230 | `@property` (`syntax`/`inherits`/`initial-value`), tabela de tipos, union, `env`, `attr` | **fonte da tabela do `ThemeSchema`** |
| `09-funcoes-modernas.md` | 236 | `min/max/clamp`, `round/mod/rem`, `oklch`, `color-mix`, relative color, `color-contrast` | define funções permitidas e as não portáveis |
| `10-seletores-avancados.md` | 231 | `:has`, `:is`, `:where`, `:not`, estados nativos, tabela de especificidade | regra de especificidade do CSS gerado |
| `11-tipografia-moderna.md` | 219 | `@font-face`, `font-display`, variable fonts, `text-wrap`, hifenização, stacks | emissão de fonte e de tipografia fluida |
| `12-stacking-scroll.md` | 235 | stacking context, `z-index` por camadas, scroll-snap, `overscroll-behavior`, sticky | origem dos tokens `--z-*` |
| `13-acessibilidade.md` | 218 | `prefers-*`, `forced-colors`, `color-scheme`, `:focus-visible`, text spacing, skip link | **fonte da camada `a11y`** |
| `14-animacoes-discretas.md` | 268 | `@starting-style`, `allow-discrete`, view transitions | como animar dialog/toast sem JS |
| `15-web-component-optimization.md` | 1416 | 7 web components completos (JS + Shadow DOM), tema global, SVG | irrelevante: exige JS e `style="--x"` inline |
| `16-flexbox.md` | 326 | flex container/items, `gap`, padrões, aviso de `order` | `order` proibido (16:178) |
| `17-seletores-basicos.md` | 333 | seletores, combinadores, pseudo-classes, tabela de especificidade, BEM | proibição de seletor por ID (17:46) |
| `18-box-model.md` | 328 | content/border-box, padding/margin, colapso, BFC, lógicas | `border-box` global no reset |
| `19-cascata-heranca.md` | 319 | cascata, origem, `!important`, keywords globais, `all`, shorthands | política de `!important` |
| `20-posicionamento-display.md` | 397 | `display`, `position`, float, BFC, `inset`, aviso de `contents` | `display:contents` proibido (20:266) |
| `21-backgrounds-borders.md` | 352 | backgrounds, bordas, `outline`, sombras, gradientes | focus ring = `outline`, não `border` (21:237) |
| `22-animacoes-keyframes.md` | 421 | `@keyframes`, propriedades, performance, a11y | animação sem JS; só composite |
| `23-transicoes.md` | 327 | transições, gatilhos, `allow-discrete`, customs animáveis | lista de propriedades transicionáveis |
| `24-cores-fundamentos.md` | 260 | hex/rgb/hsl/oklch, `currentColor`, `accent-color`, `forced-colors` | formato de cor aceito no schema |
| `25-transforms.md` | 411 | transform 2D/3D, `transform-origin`, props independentes | `translate/scale/rotate` como props |
| `26-sintaxe-at-rules.md` | 345 | estrutura, `@charset`, `@import`, `@supports`, `@scope`, ordem das regras | ordem canônica do arquivo CSS emitido |
| `27-print.md` | 390 | `@media print`, `@page`, `break-*`, `orphans/widows`, `print-color-adjust` | camada `print` gerada pelo crate |
| `28-contadores.md` | 400 | `counter-*`, `@counter-style`, `::marker`, `list-style` | numeração: Rust > counter (i18n) |
| `29-imagens.md` | 319 | `object-fit/position`, `aspect-ratio`, `image-set`, `<picture>`, `srcset` | `srcset`/`sizes` ficam no HTML |
| `30-filtros.md` | 400 | `filter`, `backdrop-filter`, blend modes; custo e a11y | `backdrop-filter` exige fallback sólido |
| `31-tipos-valores-unidades.md` | 292 | tipos de valor, unidades, keywords, lógicas, tabela unidade→relativo | gramática de valor do validador |
| `32-recommended-modern-implementation.md` | 1862 | dashboard completo: layers, `@property`, componentes, print, a11y, checklist | **síntese**: mostra o CSS-alvo e 2 bugs a não copiar |
| `SKILL.md` | 103 | front matter (categoria `languages`, versão 2026), regras centrais, file map | regras normativas em uma linha cada |
| `VERSION` | 1 | `CSS Moderno (referência 2026 — CSS Nesting, Layers, Subgrid, Container Queries, @property)` | justifica tratar tudo como baseline |

## 2. Achados

### A1. A ordem de `@layer` é uma string única, declarada antes de qualquer regra
- Citação: `05-variaveis-camadas.md:193` — "`@layer reset, base, components, utilities;`"
- Semântica: a ordem de precedência é a ordem de declaração (direita vence), e o checklist exige `@layer` antes de qualquer estilo (`05:371`; `26:345`).
- Consequência: o CSS emitido abre com **uma** linha `@layer …;`, idêntica em `app.css` e `theme.css`; nenhuma regra pode antecedê-la.
- Veredito: confirma P6/D14 (bytes canônicos) e refina D12 (o arquivo gerado tem forma fixa).

### A2. Estilo fora de `@layer` vence **todas** as camadas
- Citação: `05-variaveis-camadas.md:219` — "`/* Estilos fora de @layer têm MAIOR prioridade que qualquer @layer */`"
- Semântica: uma regra não-camada derrota `utilities`, `a11y` e qualquer override de tenant.
- Consequência: o gerador nunca emite declaração no topo do arquivo; o lint do CSS gerado conta "declarações fora de camada = 0". Sem isso, um override de tema anula a camada de acessibilidade.
- Veredito: refina (regra nova, inviável de expressar por tipo; vira gate).

### A3. "A11y por último" é decisão nossa; o corpus põe `print` por último
- Citação: `32-recommended-modern-implementation.md:47` — "`@layer reset, base, components, utilities, print;`"
- Semântica: em `32`, as regras de acessibilidade vivem dentro de `utilities` (32:1519) e `print` fecha a lista; `06-shell-e-tema.md:140` afirma "`@layer a11y`, sempre última".
- Consequência: ordem final `@layer tokens, reset, base, components, utilities, a11y, print;` — `a11y` é a última camada de tela, `print` a última camada (só dentro de `@media print`).
- Veredito: refina D12/§7 do `06` (correção textual em §4).

### A4. `var()` inválido cai no valor **inicial**, não no herdado — falha silenciosa
- Citação: `05-variaveis-camadas.md:40` — "Quando `var()` produz um valor inválido para a propriedade, o navegador usa o **valor inicial** da propriedade, não o valor herdado:"
- Semântica: `--bad-val: 16px` usado em `color` não "herda" nem avisa: vira `black`.
- Consequência: um token de tipo errado em `/theme.css` produz texto preto sem erro visível; é exatamente o que `@property` + validação no save eliminam (D12).
- Veredito: confirma D12 e P3 (prevenção > detecção).

### A5. `@property` é o contrato de tipo do token
- Citação: `08-propriedades-registradas.md:5` — "Sem `@property`, o navegador trata `--*` como strings genéricas — ele não sabe se o valor é uma cor, comprimento ou número."
- Semântica: `syntax` faz o navegador **descartar** valor fora do tipo, mantendo o `initial-value`.
- Consequência: cada token do `ThemeSchema` mapeia para uma `syntax`; token sem `@property` é token sem validação em runtime.
- Veredito: confirma D12; produz a tabela obrigatória de §5.

### A6. `inherits` e `initial-value` são decididos pela classe do token, não pelo autor
- Citação: `08-propriedades-registradas.md:226` — "- [ ] `inherits: false` para propriedades de escopo local (performance)"
- Semântica: tema herda (`inherits: true`); token local não herda; `initial-value` nunca pode faltar (`08:228`: "`initial-value` sempre definido (nunca depende de fallback externo)").
- Consequência: o schema **não** expõe `inherits`/`initial-value` ao operador; ambos são derivados do papel do token e emitidos fixos.
- Veredito: adição (reduz a superfície aceita, P3).

### A7. Union só onde a propriedade realmente aceita duas famílias
- Citação: `08-propriedades-registradas.md:55` — "### Múltiplos Tipos (Union)"
- Semântica: `syntax: "<length> | <percentage>"` é válido e usado; `initial-value` é um valor concreto (`16px`).
- Consequência: espaçamento fluido (`clamp()`) continua em `:root`; o `initial-value` registrado é um `rem` fixo. `clamp()` dentro de `initial-value` fica proibido (o valor precisa ser independente de contexto).
- Veredito: refina (`[INFERENCE]`: a exigência de "computationally independent" não está no corpus; a decisão é nossa).

### A8. Fallback de `var()` tem teto de 3 níveis — e o corpus manda usar JS acima disso
- Citação: `05-variaveis-camadas.md:58` — "**Regra**: Máximo de 3 níveis de fallback aninhado. Acima disso, usar JavaScript para pré-resolução."
- Semântica: cada nível custa resolução na cascata; o texto aceita JS como saída.
- Consequência: o gerador emite **zero** fallback aninhado (usa `initial-value` do `@property`); a saída "usar JavaScript" é refutada (P5). Lint rejeita `var(` dentro de `var(`.
- Veredito: confirma P5; refuta a recomendação de JS.

### A9. `env()` é a única forma de safe area, e o nome é case-sensitive
- Citação: `05-variaveis-camadas.md:137` — "`/* Case-sensitive: safe-area-inset-top ≠ Safe-Area-Inset-Top */`"
- Semântica: `env(safe-area-inset-*, 0px)` vem do user-agent; nome errado = valor ignorado sem aviso.
- Consequência: o shell emite as quatro safe areas com fallback `0px`, a partir de um enum de nomes `env`, nunca de string.
- Veredito: adição (estrutural: só o shell conhece `env()`).

### A10. `attr()` tipado existe, mas não é portável o bastante para carregar layout
- Citação: `06-responsivo.md:133` — "`aspect-ratio: attr(width) / attr(height);`"
- Semântica: o corpus usa `attr()` tipado em `aspect-ratio` (`06:133`, `29:79`) e em `content` (`17:205`, `32:497`, `32:1390`).
- Consequência: para `aspect-ratio`, o gerador calcula a razão em Rust a partir de `WidthPx`/`HeightPx` (determinístico, sem depender de `attr()` tipado). `attr()` sobrevive só em `content:` de print/`data-label`.
- Veredito: refuta o uso em layout; confirma P6.

### A11. `:where()` zera especificidade; `:is()` pega a maior da lista
- Citação: `10-seletores-avancados.md:74` — "`/* :is() — a especificidade é a MAIOR da lista */`"
- Semântica: `10:78` ("`:where()` — especificidade SEMPRE ZERO") e `10:212` (`:where(.gap-sm) { … } /* 0-0-0 */`).
- Consequência: reset e utilitários usam `:where()`; estado de componente usa `.classe:where(:hover)`; `:is()` é proibido no CSS gerado (especificidade imprevisível para o autor).
- Veredito: refina (regra de geração; a "Regra de Ouro" está em `10:190`).

### A12. Seletor por ID nunca entra no CSS emitido
- Citação: `17-seletores-basicos.md:46` — "⚠️ IDs têm **alta especificidade** — evite para estilos de componente. Prefira classes."
- Semântica: ID = 1-0-0 e vence qualquer composição de classes; `19:319` fecha com "Nenhum estilo autor ultrapassa especificidade 0-1-0 ou 1-0-0 (IDs evitados)".
- Consequência: identidade visual usa `data-section`/`data-tone`; `id` existe só para `for`/`aria-*`/âncora.
- Veredito: confirma D12 §5 ("identidade, não CSS") e D4.

### A13. `!important` inverte a origem — só a camada de acessibilidade/print pode usar
- Citação: `19-cascata-heranca.md:46` — "`!important` **inverte a prioridade das origens** — author !important > user !important > user-agent."
- Semântica: `19:48` recomenda `@layer` em vez de `!important`; `13:16` e `32:1431` usam `!important` só nos blocos de reduced-motion e de print.
- Consequência: allowlist de `!important` restrita a `@layer a11y` e `@layer print`, validada por lint no CSS gerado; nenhum token de tenant consegue emitir `!important` (o schema rejeita `!`).
- Veredito: confirma D12 e P3.

### A14. Componente se adapta por container query, não por viewport
- Citação: `SKILL.md:30` — "- Container queries (`container-type: inline-size`) over viewport media queries for components."
- Semântica: `06:51-56` mostra o par `container-type: inline-size` + `@container card (min-width: 400px)`; `32:1632` repete no checklist.
- Consequência: primitivo declara seu container; media query fica restrita a `print`, `prefers-*` e mudanças globais de shell (a tabela de decisão de `06` lista só exceções).
- Veredito: confirma P4 (o componente se adapta sozinho) e refina o desenho do shell.

### A15. Query num container **não** estiliza o próprio container (bug em `32`)
- Citação: `32-recommended-modern-implementation.md:403` — "`@container dashboard (min-width: 500px) {`"
- Semântica: em `32:353-354` o próprio `.dashboard-grid` é `container-type/name`, e `32:404` tenta mudar as colunas **dele** pela query que ele hospeda.
- Consequência: a regra é no-op (nenhum ancestral se chama `dashboard`); o gerador precisa recusar `@container N` aplicado ao elemento que declara `container-name: N` — checagem estática no gerador.
- Veredito: refuta o exemplo; gera regra de geração (e teste).

### A16. Unidade `cq*` exige ancestral com `container-type`, e o nome precisa de namespace
- Citação: `06-responsivo.md:92` — "`/* Todas as unidades: cqw, cqh, cqi, cqb, cqmin, cqmax */`"
- Semântica: `cqw/cqi/...` medem o container consultado; sem container declarado o valor degenera (viewport pequena) — `[INFERENCE]`, o corpus não trata o caso.
- Consequência: `container-name` vem de um enum (`Container::Card`, `Container::Layout`), prefixado para não colidir entre módulos; unidade `cq*` só é emitida junto do container correspondente.
- Veredito: adição (evita bug silencioso de layout).

### A17. `@scope` é experimental; nesting não é necessário para o gerador
- Citação: `26-sintaxe-at-rules.md:259` — "### @scope — Escopo de Estilos (Experimental)"
- Semântica: `@scope` aparece só como referência; `VERSION` anuncia "CSS Nesting" suportado, e `15`/`32` usam `&:hover`.
- Consequência: proibidos no CSS gerado. O gerador emite seletores planos (`.card`, `.card:hover`), o que mantém bytes previsíveis e o `sniffCSS` simples.
- Veredito: confirma P3/P6; refuta nesting como recurso de saída.

### A18. `:has()` é aprimoramento, nunca o único sinal de estado
- Citação: `10-seletores-avancados.md:3` — "## :has() — Seletor Parental" (uso em `32:470`: `.data-table-row:has(input[type="checkbox"]:checked)`).
- Semântica: `:has()` estiliza pai a partir do filho e é usado para estado visual de seleção/erro.
- Consequência: `:has()` só pode *reforçar* um estado que já existe em atributo (`aria-expanded`, `aria-invalid`, `checked`); nunca ser a única indicação (WCAG 1.4.1) e nunca carregar comportamento obrigatório.
- Veredito: refina P4; vira item de lint.

### A19. `color-scheme` e `accent-color` são globais e derivados do tema
- Citação: `32-recommended-modern-implementation.md:114` — "`accent-color: oklch(from var(--color-primary) 50% 0.25 h);`"
- Semântica: `color-scheme: light dark` no `:root` (32:112) e `accent-color` global estilizam controles nativos (13 §`Accent-color`).
- Consequência: o crate emite `color-scheme` e um `accent-color` **literal** (não relative color, ver A21); `meta name="color-scheme"` acompanha no `<head>`.
- Veredito: adição (dois itens obrigatórios no `/theme.css` base, não por tenant).

### A20. Cor de tema: `oklch()`; nome de cor e hex "solto" ficam fora
- Citação: `24-cores-fundamentos.md:242` — "`| Paleta de cores do tema | oklch() |`"
- Semântica: `24:88` desaconselha nomes ("**⚠️ Evitar** para valores que precisam de alteração programática"); a tabela de formatos dá `oklch` como P3+.
- Consequência: o `ThemeSchema` aceita apenas gramática `oklch(L C H[ / A])` (e `oklab()` se necessário), serializada em forma canônica; hex/nome/hsl são rejeitados — o que também dá o grafo de contraste calculável em Rust.
- Veredito: confirma D12; corrige `06:71-73` para aceitar **só** `oklch()`.

### A21. Relative color syntax é ótima para o nosso código, péssima para o tenant
- Citação: `09-funcoes-modernas.md:154` — "### relative color syntax"
- Semântica: `oklch(from var(--primary) l c calc(h + 180))` deriva cores de uma base; suporte Chrome 119+/Safari 16.2+ (`09:234`).
- Consequência: derivados (`hover`, `soft`, `border`) podem ser emitidos **pelo crate** como relative color; o valor vindo do banco nunca contém `from`/`calc` (o schema rejeita a função).
- Veredito: refina (fronteira: derivado = código; base = dado).

### A22. `prefers-contrast: more` reescreve tokens — logo precisa de camada superior
- Citação: `13-acessibilidade.md:48` — "`@media (prefers-contrast: more) {`"
- Semântica: o bloco redefine `--color-text`, `--color-bg`, `--border-width` (13:50-52) e às vezes remove `backdrop-filter`.
- Consequência: essas redefinições vivem em `@layer a11y` (última camada de tela), com `@property` das mesmas chaves para não haver mudança de tipo; testável por `sniffCSS` com a media feature forçada.
- Veredito: confirma §7 do `06`; adiciona o requisito de `@property` casado.

### A23. `forced-colors: active` obriga cores de sistema (`ButtonText`)
- Citação: `24-cores-fundamentos.md:206` — "`@media (forced-colors: active) {`"
- Semântica: no modo de alto contraste do SO o navegador sobrescreve `color`/`background`; só bordas/outlines com cores de sistema sobrevivem (`13:185-187`).
- Consequência: a camada `a11y` emite `border: 2px solid ButtonText` (e `LinkText`, `Canvas`, `Highlight`) para controles; nenhum token de tenant é usado nesse bloco.
- Veredito: adição; confirma P4 (sem isso, botão "desaparece" no modo de alto contraste).

### A24. `prefers-reduced-transparency` e `prefers-reduced-data` fazem parte do contrato
- Citação: `32-recommended-modern-implementation.md:1565` — "`@media (prefers-reduced-data: reduce) {`"
- Semântica: reduzem `backdrop-filter`/`opacity` e escondem mídia pesada (`13` §Reduced Transparency, `30:393`).
- Consequência: o gerador só pode emitir `backdrop-filter` se houver declaração de fallback sólido na mesma camada; caso contrário o lint barra.
- Veredito: adição (par obrigatório `backdrop-filter` + fallback).

### A25. Reduzir movimento: gate `no-preference` no componente, override `reduce` na a11y
- Citação: `13-acessibilidade.md:7` — "`@media (prefers-reduced-motion: no-preference) {`"
- Semântica: duas estratégias coexistem no corpus: envolver a animação em `no-preference`, ou zerar tudo em `reduce` com `!important` (`13` e `22:301`).
- Consequência: componentes animam dentro de `no-preference`; a camada `a11y` mantém o `reduce` global com `!important` como rede. Nunca as duas no mesmo seletor.
- Veredito: refina (evita `!important` disperso; concentra em A13).

### A26. `display: contents` é proibido no CSS emitido
- Citação: `20-posicionamento-display.md:266` — "`display: contents;  /* botão some do leitor de tela */`"
- Semântica: `13:119` ("remove o elemento da árvore de acessibilidade") e `02:131` (tabela: `subgrid` "Permanece na árvore" × `contents` "Removido").
- Consequência: `contain`/`subgrid` cobrem todos os casos; lint bloqueia `display: contents` e `sniffCSS` confirma que nenhum nó interativo sumiu da árvore.
- Veredito: confirma P4/D10 (regra inegociável, mas expressa em lint porque é CSS, não marcação).

### A27. Text spacing (1.4.12) proíbe dimensão fixa em texto; `rem`/`ch` são a unidade
- Citação: `13-acessibilidade.md:141` — "`line-height: 1.5;     /* mínimo WCAG */`"
- Semântica: `SKILL.md:27` — "- Never disable zoom; use `rem`/`ch` for type and spacing."; `31:285-286` (rem preferido, `max-width: 70ch`).
- Consequência: `font-size`, `line-height` (número), `letter-spacing`, `word-spacing` são validados no schema; `px` em tipografia é rejeitado; `overflow: visible` no container de texto.
- Veredito: confirma P4 e D12; adiciona três regras de schema.

### A28. `clamp()` é o mecanismo de tipografia fluida — com mínimo em `rem`
- Citação: `06-responsivo.md:29` — "`--text-h1: clamp(1.75rem, 1rem + 3vw, 3rem);`"
- Semântica: substitui séries de media queries; `32` usa `clamp` em toda a escala tipográfica e de espaço.
- Consequência: token fluido exige mínimo em `rem` (não `vw`), garantindo 1.4.4 (200% de zoom); `clamp()` só aparece em `:root`, nunca em `initial-value`.
- Veredito: refina; `[INFERENCE]` sobre o mínimo em `rem` (o corpus usa, não justifica).

### A29. O corpus cobre `size-adjust`, **não** `font-size-adjust`
- Citação: `11-tipografia-moderna.md:19` — "`size-adjust: 98%;           /* ajuste para reduzir layout shift */`"
- Semântica: `size-adjust`, `ascent-override`, `descent-override`, `line-gap-override` são descritores de `@font-face`; `font-size-adjust` (propriedade) não aparece em nenhum dos 35 arquivos.
- Consequência: a afirmação de `README.md:147` ("mitigado por `font-size-adjust`") não tem lastro neste corpus — é decisão própria; o que este corpus sustenta é `size-adjust` no fallback.
- Veredito: refuta parcialmente; vira correção textual em §4 e lacuna em §7.

### A30. Fonte: `font-display: swap` + `preload` com `crossorigin`
- Citação: `11-tipografia-moderna.md:214` — "4. **Preload** de fontes críticas:"
- Semântica: `font-display: swap` (11:11) para não bloquear; `size-adjust`/`ascent-override` reduzem CLS; o exemplo de preload inclui `crossorigin`.
- Consequência: `@font-face` é emitido pelo crate com `src` de `Asset` (nunca URL livre), `font-display: swap` fixo, e no máximo um `preload` de fonte — que é tag de `<head>`, portanto decisão do shell.
- Veredito: confirma D13 (asset declarado) e o desenho de `Src::asset`.

### A31. `text-wrap: balance` tem teto e não é para parágrafo
- Citação: `11-tipografia-moderna.md:83` — "`text-wrap: balance; /* máximo 6 linhas */`"
- Semântica: `balance` para títulos (limite de 6 linhas), `pretty` para corpo, `stable` para editável.
- Consequência: regra de `@layer base` emitida uma vez pelo crate (`h1..h3 { text-wrap: balance }`, `p { text-wrap: pretty }`); módulo não decide isso.
- Veredito: adição (cabe na camada base, custo zero).

### A32. `@import` é serial e bloqueante: proibido no CSS servido
- Citação: `26-sintaxe-at-rules.md:98` — "- `@import` **bloqueia** o download em série — cada import espera o anterior"
- Semântica: `26:122` — "usar `<link>` para CSS principal; `@import` apenas para organização de layers"; `03:147` repete ("bloqueia renderização em série").
- Consequência: `/theme.css` e `app.css` entram por `<link>`; o validador de tema rejeita `@import` e `url(...)` (já previsto em `08-seguranca.md:48`).
- Veredito: confirma D12/`09-desempenho`; refina para "no máximo dois links, ambos declarando a mesma string de `@layer`".

### A33. Print é uma camada gerada, com `@page`, `attr(href)` e `print-color-adjust`
- Citação: `32-recommended-modern-implementation.md:1421` — "`@layer print {`"
- Semântica: `27:19` (`@page`), `27:173` (`a[href]::after { content: " (" attr(href) ")"; }`), `27:254` (`print-color-adjust: exact`).
- Consequência: o crate emite o bloco de print uma vez (esconde nav/controles, mostra URL, `break-inside: avoid`); nenhum módulo escreve print. `@page` fica com `size`/`margin` fixos e documentados.
- Veredito: adição (elimina uma classe inteira de CSS por componente).

### A34. Animar só `transform`/`opacity`; `will-change` é proibido permanente
- Citação: `03-performance.md:56` — "`.tudo { will-change: transform; } /* consome memória GPU */`"
- Semântica: `03:20` ("Animar apenas `transform` e `opacity`"); `25` mostra `translate`/`scale`/`rotate` como propriedades independentes.
- Consequência: o vocabulário de `Class` só expõe animações de composite; lint do CSS gerado recusa `transition-property` com propriedade de layout e `will-change` fora de regra de estado.
- Veredito: confirma P4/D14 (determinismo visual) e vira regra de geração.

### A35. `content-visibility: auto` sem `contain-intrinsic-size` zera a altura
- Citação: `03-performance.md:28` — "`contain-intrinsic-size: 1px 500px; /* tamanho aproximado antes do paint */`"
- Semântica: isolar seção fora da viewport economiza render, mas o placeholder precisa de tamanho intrínseco.
- Consequência: par obrigatório (`content-visibility` ⇒ `contain-intrinsic-size`) verificado por lint; `contain: layout style` só onde não houver `position: sticky`/`fixed` descendente (`[INFERENCE]`: `contain` cria containing block; o corpus não alerta).
- Veredito: adição; `[INFERENCE]` marcada para o caso `contain` + sticky.

### A36. `border-box` global e propriedades lógicas são pré-requisito de layout
- Citação: `18-box-model.md:41` — "`box-sizing: border-box;`"
- Semântica: reset em `*, *::before, *::after` (18:40-43, `04` §Reset moderno); `31:290` exige `margin-inline`/`padding-block` no lugar de físicas; `01:133` prefere `gap` a `margin`.
- Consequência: `@layer reset` emite o trio (`box-sizing`, `:where(ul,ol)`, mídia `display:block`); tokens de espaço são lógicos; a view nunca escreve `margin-left`.
- Veredito: confirma o `lang`/`dir` do shell e P6.

### A37. `z-index` vem de camadas nomeadas; `order` é proibido
- Citação: `12-stacking-scroll.md:43` — "`/* Regra prática para z-index */`"
- Semântica: `12:46/51/56` fixa 100/200/300 (`overlay`/`modal`/`tooltip`); `16:178` — "`order` afeta apenas ordem visual, não a ordem de tabulação".
- Consequência: tokens `--z-overlay/modal/tooltip` (inteiros, `inherits: false`) e lint que recusa `order`/`flex-direction: *-reverse` no CSS gerado — sustenta "ordem do DOM = ordem visual" (`06` §8.5).
- Veredito: confirma D14 e a garantia 5 do shell.

### A38. `color-contrast()` e `round()` não são portáveis — contraste é calculado em Rust
- Citação: `09-funcoes-modernas.md:146` — "`/** Nota: suporte limitado (Safari 16+) */`"
- Semântica: `color-contrast()` só no Safari; `round()`/`mod()`/`rem()` ausentes no Safari (`09:230`); `color-mix()` é amplo (`09:232`).
- Consequência: contraste é verificado no **save** (`06` §7) com a matemática de `oklch` em Rust; CSS gerado não usa `color-contrast()` nem `round()`. `color-mix()` é liberado.
- Veredito: confirma D12; refuta "acessibilidade automática no CSS".

### A39. `@supports` e prefixos: quase nada justifica
- Citação: `04-interoperabilidade.md:80` — "**Nunca prefixar** propriedades padrão bem suportadas (Grid, Flexbox, Custom Properties)."
- Semântica: `04:61` usa `@supports (grid-template-rows: subgrid)` como fallback; a tabela de suporte dá subgrid desde 2023.
- Consequência: `@supports` proibido no CSS gerado (o alvo é baseline 2026 fixo) e prefixo de vendor proibido — exceto `-webkit-backdrop-filter` se o token de blur existir. Menos bytes, menos caminho não testado.
- Veredito: refuta o fallback de subgrid em `02`; simplifica (P9).

### A40. `@starting-style` + `allow-discrete` substitui JS na animação de entrada
- Citação: `14-animacoes-discretas.md:68` — "## transition-behavior: allow-discrete"
- Semântica: `display`/`overlay` passam a transicionar; `@starting-style` define o estado antes de entrar (14:21, 44, 61).
- Consequência: `Dialog`/`Toast` animam sem uma linha de JS — coerente com "tudo funciona sem JavaScript" (`07`) e com P5; o par `@starting-style` + `prefers-reduced-motion` é obrigatório.
- Veredito: confirma o desenho JS-free; adiciona o par ao primitivo.

### A41. `srcset`/`sizes` são HTML; `image-set()` fica de fora
- Citação: `29-imagens.md:149` — "`<!-- srcset + sizes: o navegador escolhe o melhor tamanho -->`"
- Semântica: a tabela "Imagens como Background vs. `<img>`" (29) separa conteúdo (HTML, com `alt`) de decorativo (CSS).
- Consequência: `SrcSet`/`Sizes` continuam par de tipos emitido no `<img>`; `image-set()`/`image()` no CSS gerado são proibidos (imagem de conteúdo nunca vira background).
- Veredito: confirma D1/`SrcSet` e a regra "imagem de conteúdo tem `alt`".

### A42. Contadores: só `list-style-type`, nunca numeração de conteúdo
- Citação: `28-contadores.md:398` — "- [ ] `::marker` para estilizar marcador de lista (não `::before`)"
- Semântica: `counter()`/`counters()` produzem texto a partir do CSS, sem locale e sem `Msg` (`[INFERENCE]`: nada no corpus trata i18n de número).
- Consequência: numeração visível (paginação, TOC, passos) é feita em Rust com `Count`/`Msg` — determinística e traduzível; CSS só usa `list-style-type`/`::marker` decorativo.
- Veredito: refina P6 (i18n vence a conveniência do counter).

## 3. Confirmações do desenho

1. **Tema é dado, CSS é derivado** (D12): `32` inteiro é um arquivo CSS canônico gerado a partir de tokens; nada ali é "CSS no banco".
2. **CSP sem inline** é viável para todo o CSS-alvo: nenhum efeito do corpus exige `style=` — os exemplos que usam (`15:436` `style="--split: …"`, `15:1030` spinner `style="--size: …"`, `32` §24 HTML de exemplo) são justamente os anti-padrões que a crate evita.
3. **Determinismo**: o corpus reforça ordem canônica (media queries aninhadas na ordem declarada, `@layer` explícito, `text-wrap` fixo) — compatível com `ETag` por `sha256` (P6/D14).
4. **Acessibilidade por tipo + CSS de camada**: `13` e `32` §21 dão o conjunto exato de regras que o crate deve emitir uma vez (`focus-visible`, `forced-colors`, `reduced-motion`, skip link, `.sr-only` 32:1587).
5. **`@property` como contrato**: `08` é a especificação de fato do `ThemeSchema` (tipo + herança + default).
6. **Sem JS**: `14`/`18-aprendizados`(html-moderno) fecham o ciclo — `@starting-style`/`allow-discrete`/`popover` cobrem o que antes exigia script.
7. **Componente se adapta ao contexto** (container query), não à viewport — sustenta primitivos reutilizáveis em módulos diferentes.
8. **Imagem com dimensão** (`aspect-ratio`/`object-fit`) previne CLS sem tocar em JS.
9. **Propriedades lógicas** são obrigatórias para `dir=rtl` do shell; o corpus é explícito (`31:290`, `04`).
10. **Camada de acessibilidade por último** é sustentável — só precisa incluir `print` depois dela (A3).

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata a inserir, e onde) |
|---|---|---|
| `06:140`: "A camada de acessibilidade (`@layer a11y`, sempre última)" | `32:47` ordena `…, utilities, print;` e põe print por último | Em `06-shell-e-tema.md` §7: "Ordem canônica: `@layer tokens, reset, base, components, utilities, a11y, print;` — `a11y` é a última camada de tela; `print` vem depois e só se aplica dentro de `@media print`." |
| `06` §4 mostra regras soltas (`.tone-primary{…}`) sem camada | `05:219`: estilo fora de camada vence qualquer camada | Em `06` §4: "Nenhuma regra é emitida fora de `@layer`; o lint do CSS gerado exige zero declarações não-camadas." |
| `06` §3 lista tokens sem `@property` | `08:5/228`: sem `@property` é string genérica; `initial-value` sempre presente | Em `06` §3: "Todo token que tem `syntax` possível é registrado com `@property`; `initial-value` é valor literal, nunca `var()`/`clamp()`." |
| `06` §3 aceita `radius = "0.5rem"` sem regra de unidade | `SKILL.md:27` + `31:285`: `rem`/`ch`; `13:141` text spacing | Em `06` §3: "`font-size` só em `rem` ou `clamp()` com mínimo em `rem`; `px` é aceito apenas em `border-width`/hairline e em `WidthPx`/`HeightPx`." |
| `06:71-73` aceita qualquer `oklch()` "validado" | `24:88/242`: `oklch()` para paleta, nomes/formatos ad hoc evitados | Em `06` §3: "Cor de tema é `oklch(L C H[ / A])` em forma canônica; hex, nome de cor, `hsl()` e relative color (`oklch(from …)`) são rejeitados no schema." |
| `README.md:147` cita `font-size-adjust` | `11:19` só cobre `size-adjust`/`ascent-override` (descritores `@font-face`) | Em `README.md` (linha do `web-performance`): "swap de fonte é mitigado por `size-adjust`/`ascent-override` no `@font-face`; `font-size-adjust` é decisão própria, fora deste corpus." |
| `README.md` (web-performance): "exatamente **um** stylesheet render-blocking" | `06` §4 exige `<link>` de `/theme.css` quando há override; `26:122` proíbe `@import` | Em `06` §4: "No máximo dois `<link rel=stylesheet>` — `app.css` e, só quando há override, `theme.css`; ambos abrem com a mesma string de `@layer`." |
| `06` §8 não proíbe padrões de CSS | `16:178` (`order` quebra tabulação), `20:266` (`display:contents`), `19:48` (`!important`) | Em `06` §8, itens 9-11: "9. CSS emitido não usa `order`, `flex-direction: *-reverse` nem `display: contents`. 10. `!important` só nas camadas `a11y` e `print`. 11. Nenhum `@import`, `@supports`, `@scope`, `@charset` ou prefixo de vendor no CSS servido." |
| `09-desempenho-e-cache.md` não trata isolamento de render | `03:28`: `content-visibility` sem `contain-intrinsic-size` zera altura | Em `09` §4 (orçamento), acrescentar: "`content-visibility: auto` só é emitido em par com `contain-intrinsic-size`; `will-change` só em regra de estado, nunca permanente." |

## 5. Adições prontas para colar

### 5.1 Cabeçalho canônico do CSS (emitido uma vez, em `app.css` e em `theme.css`)

```css
@layer tokens, reset, base, components, utilities, a11y, print;
```

Nenhuma regra antes dessa linha; nenhuma regra fora de camada (A1, A2).

### 5.2 Bloco `@property` (forma fixa; valores vêm do schema)

```css
@property --tenant-primary { syntax: "<color>"; inherits: true;  initial-value: oklch(0.55 0.14 250); }
@property --tenant-space-4 { syntax: "<length>"; inherits: false; initial-value: 1rem; }
@property --z-modal        { syntax: "<integer>"; inherits: false; initial-value: 200; }
@property --dur-fast       { syntax: "<time>";    inherits: false; initial-value: 150ms; }
```

### 5.3 `ThemeSchema` — token → sintaxe válida → validação

| família de token | exemplo válido | `syntax` | `inherits` | `initial-value` | schema **rejeita** |
|---|---|---|---|---|---|
| cor de papel (`--tenant-color-*`) | `oklch(0.55 0.14 250)` | `<color>` | `true` | a própria cor, literal | hex, nome, `hsl()`, `rgb()`, `oklch(from …)`, `color-mix()`, `currentColor`, `transparent`, `var()` |
| cor "on" (`--tenant-on-*`) | `oklch(0.99 0 0)` | `<color>` | `true` | a própria cor | igual acima **+** par (texto,fundo) < 4.5:1 na verificação de contraste |
| superfície/linha (`--tenant-surface`, `--tenant-border`) | `oklch(0.98 0.01 250)` | `<color>` | `true` | a própria cor | idem; alpha via `/ A` só com `A` numérico |
| espaço (`--tenant-space-*`) | `1rem`, `0.5rem` | `<length>` | `false` | `rem`, ≠ 0 quando usado como padding | `px`, `em`, `%`, `vw`, `clamp()`, `calc()`, `0` sem unidade |
| raio (`--tenant-radius-*`) | `0.5rem`, `9999px` | `<length>` | `false` | `rem`/`px` do sistema | `%`, `em`, negativo |
| borda (`--tenant-border-width`) | `1px`, `2px` | `<length>` | `false` | `1px` | `rem` < 1, negativo, `%` |
| tipografia (`--tenant-text-*`) | `1rem`, `clamp(1.75rem, 1rem + 3vw, 3rem)` | `<length>` | `true` | **apenas** um `rem` fixo (o `clamp` fica em `:root`) | mínimo de `clamp` em `vw`/`px`; `pt`; `px` |
| duração (`--tenant-duration-*`) | `150ms`, `0.2s` | `<time>` | `false` | `ms`/`s` | negativo, `0` sem unidade |
| ângulo (`--tenant-angle-*`) | `250deg` | `<angle>` | `true` | `deg` | `turn`/`rad`/`grad` (canonicalizar) |
| camada (`--z-*`) | `100`, `200`, `300` | `<integer>` | `false` | inteiro da tabela de camadas | qualquer inteiro fora da enum `ZLayer` |
| sombra (`--shadow-*`) | `0 1px 2px oklch(0 0 0 / 0.06)` | **não registrado** (`syntax: "*"` não dá tipo) | — | — | `inset`, mais de 2 camadas, cor que não seja `oklch` |
| família de fonte (`--font-*`) | `system-ui, sans-serif` | **não registrado** | — | — | qualquer família fora do catálogo; último item deve ser genérico (`11:219`) |

Regra transversal: nome de token obedece `--tenant-[a-z][a-z0-9-]*`; o valor não pode conter `;`,
`{`, `}`, `@`, `url(`, `\`, `!`, `<`, `>`; comprimento máximo por valor definido no schema
(junto com o limite de regras por seção de `09-desempenho-e-cache.md:44`).

### 5.4 Lint do CSS gerado (bloqueante, roda no `cq check`)

```text
1. arquivo abre com @layer tokens, reset, base, components, utilities, a11y, print;
2. zero declarações fora de @layer;
3. zero `!important` fora de @layer a11y/@layer print;
4. zero @import, @charset, @supports, @scope, @namespace, @starting-style fora de componente;
5. zero seletor com `#`; zero `:is()`; zero `:has()` sem atributo de estado no mesmo seletor;
6. zero `display: contents`, `order`, `flex-direction: *-reverse`;
7. zero `transition`/`animation` em propriedade de layout; zero `will-change` sem seletor de estado;
8. `content-visibility: auto` sempre com `contain-intrinsic-size` no mesmo bloco;
9. `backdrop-filter` sempre com `background` sólido no mesmo bloco;
10. `font-size` só `rem`/`clamp`; zero `px` em tipografia;
11. `@container N` nunca aplicado ao elemento que declara `container-name: N`;
12. todo `var(--tenant-*)` tem `@property` correspondente no mesmo arquivo.
```

### 5.5 Matriz de media features que a crate emite (e onde)

| media feature | camada | finalidade |
|---|---|---|
| `prefers-color-scheme` | `tokens` | tema claro/escuro |
| `prefers-contrast: more` | `a11y` | reescreve tokens de cor/borda |
| `forced-colors: active` | `a11y` | cores de sistema (`ButtonText`, `LinkText`) |
| `prefers-reduced-motion` | `a11y` | rede de segurança (`reduce`, com `!important`) |
| `prefers-reduced-transparency` | `a11y` | remove `backdrop-filter`/`opacity` |
| `prefers-reduced-data` | `a11y` | esconde mídia pesada |
| `print` | `print` | `@page`, URLs, `break-inside`, `print-color-adjust` |
| `min-width` (shell) | `components` | mudança global de layout (exceção da tabela de `06`) |
| `(hover: hover)` | `components` | só quando o efeito é puramente cosmético |

## 6. Armadilhas de complexidade

| Armadilha | Por que não entra | Custo se entrar |
|---|---|---|
| Relative color syntax com valor de tenant | validar `from`/`calc`/canais exige parser de função completo | parser novo + superfície de injeção dentro de `oklch()` |
| Geração condicional de CSS em runtime | quebra `ETag` estável e cache | bytes variáveis por request (P6/D14 morrem) |
| `@scope` para isolar módulos | experimental, e o vocabulário `Class` já dá escopo | camada extra de regras e navegadores sem suporte |
| `@supports` para tudo | caminho não testado, dobra o CSS | bytes e branches que nenhum teste cobre |
| `attr()` tipado como fonte de layout | suporte irregular; resultado é layout errado silencioso | bug de CLS impossível de reproduzir |
| `counter()` para numeração visível | texto não passa por `Msg`; sem locale | quebra i18n e o próprio link ambíguo por variante |
| `will-change` "preventivo" | memória de GPU por página | regressão de memória em listas longas |
| `contain: paint/strict` em todo componente | corta `outline` de foco e `sticky` | violação 2.4.7/2.4.11 descoberta só no `sniffCSS` |
| CSS crítico inline (citado em `03:141`) | CSP sem inline | elimina o modelo de segurança (D3) |
| `<link media="print" onload>` (`03:142`) | `on*` não existe | exige JS inline (D3) |

## 7. Lacunas do corpus

1. **`font-size-adjust`** não aparece (A29): a crate decide sozinha entre a propriedade (baseline novo) e `size-adjust` no fallback.
2. **`light-dark()`** não é citado: o corpus só faz tema por `prefers-color-scheme` + `[data-theme]`. Isso interage com `data-a11y-*` e cookies — decisão nossa.
3. **Gramática de `color()`/`display-p3`** não é dada: se o schema aceitar só `oklch()`, o gamut P3 vem de graça; se aceitar `color()`, precisa de validador novo.
4. **`@property` com `initial-value` de `clamp()`/`calc()`** não é discutido; a regra "valor independente de contexto" é `[INFERENCE]` (A7).
5. **Custo real de container queries em listas grandes** não é medido: só há afirmação qualitativa.
6. **`@view-transition` (cross-document)** não é tratado; `14` cobre só `::view-transition-*` na mesma página.
7. **`scrollbar-gutter`, `text-box-trim`, anchor positioning** ausentes — a crate não deve emitir nenhum dos três.
8. **Validação de `@font-face`** (nº de famílias/pesos, `unicode-range`) não é normatizada; vem de `web-performance`, não daqui.
9. **Interação `contain` × `position: sticky/fixed`** não é alertada (A35), e é a origem mais provável de bug visual no shell.
10. **Nenhum gate de CSS** é descrito: o corpus diz o que emitir, não como testar (fica com `sniffCSS`, já previsto em `docs/05-acessibilidade.md:10`).
