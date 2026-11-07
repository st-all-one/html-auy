# web-accessibility — WCAG 2.2 AA + WAI-ARIA 1.2 (`_RUST/web-accessibility/`, 29 arquivos)

Corpus de consulta sobre acessibilidade: fundamentos (HTML semântico, ARIA, teclado, contraste),
formulários, live regions, mídia, movimento, roles/atributos avançados, tabelas/grids, SVG/canvas,
`inert`/`forced-colors`, preferências de SO, padrões APG, ferramentas de teste e uma página-exemplo.
Serve aqui como **fonte de vocabulário e de matriz de mecanismo** para auditar `docs/05-acessibilidade.md`
e `spec/elements.toml` SC por SC. Norma em divergência: W3C/MDN (D17 — o corpus é referência, não dogma).
Snapshot auditado: `docs/05` com 141 linhas e `spec/elements.toml` em `schema = 2`/`spec_version = "2.0.0"`
com 811 linhas. A spec mudou **três vezes durante esta auditoria** (D16 → revisões 15:06/15:08/15:09);
por isso toda citação da spec traz a linha **e** o texto verbatim — para localizar por `grep` se o arquivo
tiver andado. Quando um achado descreve algo já corrigido, ele diz isso explicitamente.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-index.md` | 65 | POUR, populações-alvo, mapa do guia | só orientação; nada normativo novo |
| `01-semantic-html.md` | 164 | landmarks, headings, botão vs link, tabela, checklist | base do `accessible_name`; sustenta 1.3.1/2.4.6 |
| `02-aria.md` | 200 | 5 regras ARIA, roles, atributos, nome acessível, armadilhas | fonte do enum de `aria-*` e da proibição de `alert` estático |
| `03-keyboard-accessibility.md` | 161 | `tabindex`, roving, foco visível, focus trap, skip link | confirma `tabindex` só `-1|0`; `:focus-visible` |
| `04-color-and-contrast.md` | 106 | 4.5:1 / 3:1, uso da cor, luminância | `save-time` de paleta (1.4.3/1.4.11) |
| `05-live-regions.md` | 125 | `aria-live/atomic/relevant/busy`, roles implícitas | política de região persistente; proibição de duplicar `aria-live` |
| `06-forms-and-labels.md` | 190 | label, `fieldset/legend`, erros, placeholder, `autocomplete` | fonte dos invariantes de `Field`, `label`, `fieldset` |
| `07-multimedia.md` | 112 | tipos de `alt`, `video/audio`, GIF animado | confirma `alt` tipado e ausência de `autoplay` |
| `08-mobile-accessibility.md` | 116 | alvos 44/48, gestos, orientação, `inputmode` | alimenta 2.5.2/2.5.8/1.3.4 e o alvo interno de 44 px |
| `09-seizures-motion.md` | 187 | 2.3.1, `prefers-reduced-motion`, transparência, listras | regras de tema (`@layer a11y`) |
| `10-cognitive-accessibility.md` | 216 | tempo, previsibilidade, erro, autenticação, linguagem simples | 2.2.1, 3.2.4, 3.3.4 e 3.3.8 (CAPTCHA/paste) |
| `11-testing-tools.md` | 135 | axe/Lighthouse/Pa11y, teste manual, limitações (~30-50%) | justifica o gate de CI e seu teto |
| `12-wcag-quick-reference.md` | 133 | tabela SC A/AA/AAA | **erro de numeração em 3.3.7-3.3.9** (A39) |
| `13-aria-advanced-roles.md` | 240 | `application`, `feed`, `term/definition`, `note`, `math` | `application`/`feed` ficam fora do vocabulário |
| `14-structural-roles.md` | 166 | mapa HTML → role estrutural, inclui `associationlist` | conflita com `25` sobre `<dl>` (A10) |
| `15-aria-advanced-attributes.md` | 274 | `aria-keyshortcuts`, `roledescription`, `flowto`, `details`, `dropeffect` | quase tudo fora: sem atalhos nem `roledescription` |
| `16-table-grid-attributes.md` | 233 | `colcount/colindex`, spans, grids esparsos, table vs grid | regra "`display:grid` mata `<table>`" (A14) |
| `17-web-components-shadow-dom.md` | 262 | shadow DOM, `delegatesFocus`, slots | **irrelevante**: a crate não emite custom elements |
| `18-svg-canvas.md` | 242 | SVG acessível (`title/desc/role`), canvas, fallback | valida `svg_title`/`desc`; canvas vira decisão (A15, A37) |
| `19-inert-forced-colors.md` | 351 | `inert` vs `aria-hidden`, `prefers-contrast`, `forced-colors` | base de A28 e A36 |
| `20-os-browser-settings.md` | 262 | configurações Win/macOS/iOS/Android, `@custom-media` | media queries obrigatórias no tema (A36) |
| `21-keyboard-patterns.md` | 490 | teclado por widget, dialog, tooltip, feed | contrato de JS para `Dialog` (A29) |
| `22-design-patterns.md` | 425 | APG: accordion, carousel, combobox, menu, breadcrumb, pagination | `aria-expanded`+`aria-controls`, `aria-current`, acordeão (A8, A25, A26) |
| `23-advanced-testing.md` | 550 | axe em unit/e2e, Lighthouse CI, Pa11y, Storybook | modelo de pipeline; a crate fica antes do gate |
| `24-wcag-supplement.md` | 225 | SC faltantes (2.5.5, 2.5.6, 3.2.5), tabelas por versão | corrige `12` em parte; ainda erra 3.3.7 (A39) |
| `25-tag-role-reference.md` | 128 | elemento → role implícita + nuances | **tabela de checagem** da spec (§5b) |
| `EXEMPLO-MODERNO.md` | 1120 | página completa (HTML/CSS/JS/testes) | padrões de referência; também exibe 3 erros (A7, A9, A27) |
| `SKILL.md` | 100 | regras absolutas resumidas | cabeçalho de política; coincide com `docs/05` |
| `VERSION` | 3 | WCAG 2.2 (2023) AA, ARIA 1.2, HTML Living Standard 2025 | fixa o alvo normativo do corpus |

## 2. Achados

### A1. Landmark só existe nomeado — e o nome deveria vir do heading
- Citação: `25-tag-role-reference.md:23` — "| `<section>` | `region` | **somente** se tiver nome acessível (label/labelledby/title) |"
- Semântica: `<section>`/`<form>` só entram na navegação por landmark quando nomeados; `nav/main/aside/header/footer` sempre entram.
- Consequência: `section_has_heading_or_name` (`spec/elements.toml:198` — "invariants = [\"section_has_heading_or_name\"]") e `named_is_landmark` em `form` (`:618`) cobrem o "se"; falta a preferência do corpus (`01-semantic-html.md:129`: `<section aria-labelledby="section-title">`), então `aria-label` em `section` ainda passa sem advisory.
- Veredito: confirma D10; adicionar `name_from_inner_heading`.

### A2. `banner`/`contentinfo` só na raiz
- Citação: `25-tag-role-reference.md:17` — "| `<header>` | `banner` | **somente** fora de `article`/`aside`/`main`/`nav`/`section` |"
- Semântica: `header` de artigo não é landmark; só o do `body` é.
- Consequência: `banner_unique_at_root` (`spec/elements.toml:164`) resolve o caso exato; `footer` (`:166`) não tem invariante gêmeo, mas `contentinfo` é único por definição.
- Veredito: confirma D10/D16; nada a mudar.

### A3. Nomes de landmark distintos em todo o documento
- Citação: `22-design-patterns.md:322` — "- Use `<nav>` com `aria-label=\"Breadcrumb\"`"
- Semântica: cada landmark é nomeado pelo propósito; dois homônimos deixam o AT anunciando "navegação Principal" duas vezes.
- Consequência: `landmark_names_distinct` (`spec/elements.toml:185`) está só em `nav`; `aside` (`:187`) e `search` (`:206`) exigem nome mas não entram na checagem de unicidade — o invariante é de documento, não de elemento.
- Veredito: confirma D10; mover para o passe global.

### A4. `<main>` único e focável
- Citação: `25-tag-role-reference.md:20` — "| `<main>` | `main` | sempre (1 por documento) |"
- Semântica: um `main` por documento, e o alvo do skip link precisa receber foco.
- Consequência: `main_unique` + `main_tabindex_minus_one` (`spec/elements.toml:178`) são o par exigido por `03-keyboard-accessibility.md:135`.
- Veredito: confirma D10; nada a mudar.

### A5. `<search>` como único vocabulário de busca
- Citação: `01-semantic-html.md:26` — "| `<search>` | `search` | Função de busca |"
- Semântica: `<search>` é o landmark de busca moderno; `role="search"` em `div` é o legado.
- Consequência: `element.search` (`spec/elements.toml:206`) exige nome; `div` só aceita `roles = ["status", "alert", "tooltip"]` (`:221`), então o legado não compila.
- Veredito: confirma D4/D10.

### A6. Falta o invariante de sequência de headings
- Citação: `01-semantic-html.md:83` — "Nunca pule níveis (`<h1>` → `<h3>`). Use heading para títulos, não para estilização."
- Semântica: a sequência de níveis não pode pular; heading não é ferramenta de estilo.
- Consequência: `h1` tem `single_h1` + `shell_owns_h1` (`spec/elements.toml:246`), mas h2–h6 (`:248-276`) não têm nenhuma regra de ordem — 2.4.6/1.3.1 dependem do revisor humano.
- Veredito: refina D10 — `heading_level_no_skip` no passe global (docs/04 §4).

### A7. O nível do heading precisa ser parâmetro do primitivo
- Citação: `EXEMPLO-MODERNO.md:525` — "createElement('h3', {}, [product.name]),"
- Semântica: cards fixam `h3` independentemente do contexto de montagem.
- Consequência: dentro de uma seção que já é `h3`, o card gera salto (A6); o primitivo deve receber `HeadingLevel` e o shell decide o nível do módulo (`h1` é só do shell, `:246`).
- Veredito: refina D7 — `Card::title(level)`.

### A8. Acordeão: `details`/`summary` vs button + `aria-expanded`
- Citação: `22-design-patterns.md:15` — "aria-expanded=\"false\"" com `:16` "aria-controls=\"accordion-1-panel\">"
- Semântica: o APG recomenda `button` + `aria-expanded` + `aria-controls` + `hidden`; `<details>` entrega o mesmo sem `aria-*`.
- Consequência: a spec escolheu `details` (`spec/elements.toml:788`) com `summary_single_heading` (`:802`), o que dispensa `aria-controls` (A26) — decisão certa e mais simples; registrar em docs/13 para não reintroduzir o padrão APG.
- Veredito: refuta a técnica do corpus; confirma a escolha da spec.

### A9. `role="list"` não é expressável — então o CSS não pode matar listas
- Citação: `EXEMPLO-MODERNO.md:1046` — "- [ ] `role=\"list\"` + `role=\"listitem\"` quando `display` quebra semântica"
- Semântica: `list-style:none`/`display:flex` em `<ul>` descarta a semântica de lista; a correção usual é `role="list"`.
- Consequência: `ul` (`spec/elements.toml:388`) não tem `roles` e `role` não é global (`:87`) — a saída é **proibir** esse CSS em `ul`/`ol`, regra de `sniffCSS`/`@layer a11y` em docs/06.
- Veredito: refina D4 — a obrigação migra para o CSS.

### A10. `dl` — o corpus se contradiz sobre a role
- Citação: `14-structural-roles.md:15` — "| `<dl>` | `associationlist` | Lista de associações |"
- Semântica: `14` dá `associationlist`; `25-tag-role-reference.md:34` dá "| `<dl>` | `list` | `<dt>` | `term` |"; `associationlist` é proposta ARIA 1.3, não 1.2.
- Consequência: `dl`/`dt`/`dd` (`spec/elements.toml:408-425`) ficam nativos e sem `role` — correto; o validador não deve mapear `dl` para role nenhuma.
- Veredito: refuta `14`; `25` é a fonte da tabela de checagem.

### A11. `dl` sem agrupador `div`
- Citação: `10-cognitive-accessibility.md:159` — "<dl>" [bloco "Revisão Antes de Enviar"]
- Semântica: o HTML permite `<div>` como agrupador de pares; o corpus usa pares diretos.
- Consequência: `dl` com `children = ["dt", "dd"]` e `required_children = ["dt"]` (`spec/elements.toml:408`) fecha o agrupamento — simples, mas impede agrupar termos relacionados; documentar em docs/03.
- Veredito: confirma D5; documentar a limitação.

### A12. Tabela: `caption` não vazio + `scope` coerente
- Citação: `25-tag-role-reference.md:108` — "5. **`th`** → `rowheader` quando `scope=\"row\"`; `columnheader` caso contrário."
- Semântica: todo `th` precisa de `scope` (decide a role implícita); `<caption>` é o nome acessível da tabela.
- Consequência: `table` exige `caption` (`spec/elements.toml:717/724`), `caption_not_empty` (`:731`) e `scope_enum`/`scope_matches_section` (`:778`) cobrem 1.3.1; `01-semantic-html.md:157` pede o mesmo.
- Veredito: confirma D16; nada a mudar.

### A13. `headers` sem resolução de `id`
- Citação: `16-table-grid-attributes.md:47` — "## Mesclagem (colspan / rowspan)"
- Semântica: em cabeçalho composto, a associação célula→cabeçalho passa a ser `headers="id id"`.
- Consequência: `td` aceita `headers` (`spec/elements.toml:780`) e `th` aceita `id` (`:771`), mas nada garante que cada token resolva para um `th id` do bloco — é `aria-valid-attr-value` aplicado a `headers`.
- Veredito: refina; adicionar `headers_resolve_to_th_id`.

### A14. `display:grid` em `<table>` destrói a semântica — sem `role` para consertar
- Citação: `16-table-grid-attributes.md:205` — "**Problema conhecido**: Aplicar `display: grid` ou `display: block` a `<table>` remove sua semântica implícita. A solução é adicionar `role=\"table\"` explicitamente."
- Semântica: a correção usual exige `role="table"`, que a crate proíbe.
- Consequência: o tema não pode aplicar `display` ≠ `table` em `<table>`; tabela responsiva = wrapper com `overflow-x` (1.4.10). Regra em docs/06 + facet no `sniffCSS`.
- Veredito: refuta a técnica do corpus; vira proibição de CSS.

### A15. `grid`/`treegrid`/`canvas` fora do vocabulário — declarar
- Citação: `16-table-grid-attributes.md:231` — "- [ ] `grid` usado apenas quando há interatividade por célula"
- Semântica: `grid` só se justifica com célula interativa; canvas só com fallback textual equivalente (`18-svg-canvas.md:238`).
- Consequência: sem `role`, sem `tabindex` em `td` e sem `aria-activedescendant`, nenhum grid interativo é expressável; um gráfico do portal cabe em SVG + `<table>`. Registrar não-objetivo em docs/14.
- Veredito: confirma D4; declarar fora de escopo.

### A16. `label` sempre por `for`
- Citação: `06-forms-and-labels.md:8` — "<!-- 1. for/id (RECOMENDADO) -->"
- Semântica: `for`/`id` é o método recomendado; aninhamento (`06-forms-and-labels.md:12`) é alternativa válida.
- Consequência: `label` exige `for` (`spec/elements.toml:634`) com `for_matches_id_in_block` (`:641`), eliminando o aninhamento — decisão boa, mas obriga `Checkbox`/`Radio` a **gerar** `id` sempre.
- Veredito: refina D5; documentar "label sempre por `for`".

### A17. Enum de `type` de `input` fora do TOML
- Citação: `25-tag-role-reference.md:89` — "| `radio` | `radio` | `number` | `spinbutton` |"
- Semântica: cada `type` tem role implícita própria (`number`→`spinbutton`, `search`→`searchbox`) e alguns não têm nome próprio.
- Consequência: `accessible_name = "label"` + `label_exempt_types` (`spec/elements.toml:652`) é o modelo certo, mas o enum de `type` e `attribute_by_input_type` vivem só no código — a spec deixa de ser "fonte da verdade" para o atributo mais usado do vocabulário.
- Veredito: refina D5; publicar `input_types` no TOML.

### A18. `legend` não vazio e `select` com opções
- Citação: `06-forms-and-labels.md:39` — "<fieldset>" [bloco "fieldset + legend" com radios]
- Semântica: grupo de radios/checkbox precisa de `legend` com texto; lista suspensa sem `option` não é lista.
- Consequência: `fieldset` exige `legend` (`spec/elements.toml:625`) mas não proíbe `legend` vazio (ao contrário de `caption_not_empty`, `:731`); e `select` (`:664`) não tem `required_children`, então `<select>` sem opção compila.
- Veredito: refina; adicionar `legend_not_empty` e `select_requires_option`.

### A19. `roles` existe, mas falta `alertdialog`
- Citação: `02-aria.md:80` — "| `alertdialog` | — | Como dialog + alerta crítico, pelo menos 1 foco |"
- Semântica: diálogo destrutivo é `alertdialog`; informativo é `dialog`.
- Consequência: `div` ganhou `roles = ["status", "alert", "tooltip"]` (`spec/elements.toml:221`), mas `dialog` (`:804`) não declara `roles`, então confirmação de exclusão não tem como se anunciar como alertdialog; `no_aria_modal` (`:809`) já está certo.
- Veredito: refina D4 — `dialog.roles = ["dialog", "alertdialog"]`.

### A20. `role="alert"` estático não é anunciado
- Citação: `02-aria.md:146` — "| `role=\"alert\"` no HTML estático | Role alert é para conteúdo DINÂMICO; pré-existente não é anunciado |"
- Semântica: alert só anuncia conteúdo **inserido depois** que a região existe.
- Consequência: `docs/05:53` (4.1.3) já está correto (região persistente + `aria-atomic` + "nunca estático"), mas `docs/05:68` continua prometendo "erro com `role=\"alert\"`" no `Field` — em SSR o erro já vem no HTML e não é anunciado.
- Veredito: refuta `docs/05:68` (a linha 53 já tem a formulação certa).

### A21. `aria-errormessage` fora da allowlist
- Citação: `02-aria.md:128` — "| `aria-errormessage` | Referencia elemento de erro |"
- Semântica: é o atributo específico para mensagem de erro, mais preciso que `aria-describedby`.
- Consequência: `[globals.aria]` (`spec/elements.toml:70`) não o inclui, então `Field` só pode usar `aria-describedby`. Decisão defensável (suporte desigual), mas precisa estar escrita como decisão, não como omissão.
- Veredito: refina; registrar "erro entra por `aria-describedby` + `aria-invalid`".

### A22. `aria-label = "text"` reabre o escape hatch de i18n
- Citação: `spec/elements.toml:80` — "aria-label = \"text\""
- Semântica: o resto da allowlist é enum fechado; `aria-label` aceita string livre, exatamente o que `docs/05:128` nega ("`aria-label` nunca fica vazio por tradução faltando") e o que §4 penaliza ("texto visível literal (não-`Msg`)").
- Consequência: o nome acessível de `nav`/`aside`/`search`/`button` pode ser texto hardcoded em português e nenhum gate percebe; a paridade dos 4 locales deixa de ser garantida.
- Veredito: refuta — `aria-label = "msg"` (catálogo `Msg`), como em §5a.

### A23. `autocomplete` resolve 1.3.5; 3.3.7 é outro mecanismo
- Citação: `06-forms-and-labels.md:168` — "9. **Use `autocomplete`** para campos comuns (name, email, tel, address)"
- Semântica: `autocomplete` identifica o **propósito** do campo (1.3.5), não evita redigitação (3.3.7 Redundant Entry).
- Consequência: `autocomplete_enum` (`spec/elements.toml:653`) fecha 1.3.5; 3.3.7 exige que o servidor repreencha o valor já informado — contrato de aplicação, não de markup (`docs/05:50` diz "Field reutiliza valor": só se o app passar o valor).
- Veredito: refina — 3.3.7 vira "tipo + contrato de fluxo".

### A24. Três primitivos citados não existem na spec
- Citação: `docs/05-acessibilidade.md:42` — "**tipo**: `Sortable` exige alternativa por botão" (idem `:32` `Carousel`, `:51` `PasswordField`)
- Semântica: a tabela SC promete mecanismos que dependem de primitivos; a spec só descreve elementos.
- Consequência: `Sortable`, `Carousel` e `PasswordField` não aparecem em `spec/elements.toml` nem em §3 (que lista outros doze). Ou viram primitivos com decisão registrada, ou a linha SC aponta o mecanismo real (A34/A35).
- Veredito: refuta o estado atual de `docs/05:32,42,51`.

### A25. `aria-current`: um por conjunto, só no item atual
- Citação: `02-aria.md:95` — "| `aria-current` | `page`/`step`/`location`/`date`/`time`/`true` | Elemento atual em um conjunto |"
- Semântica: `aria-current` marca **um** elemento atual por conjunto; repetido, o AT anuncia vários "atual".
- Consequência: `Pagination`/`Breadcrumb` prometem "um único `aria-current`" (`docs/05:76-77`), mas `[globals.aria]` (`spec/elements.toml:70`) valida só o valor — a unicidade é uma propriedade do documento.
- Veredito: **corrigido** — `aria_current_unique_per_nav` (`output`, por `<nav>`).

### A26. `aria-expanded` sem alvo resolvível
- Citação: `22-design-patterns.md:15` — "aria-expanded=\"false\"" (+ `:16` "aria-controls=\"accordion-1-panel\">")
- Semântica: disclosure = `aria-expanded` no gatilho + `aria-controls` no painel + `hidden` fechado.
- Consequência: `button` tem `popover_target_resolves` (`spec/elements.toml:694`), mas nenhum invariante garante que `aria-controls` resolva; sem `id` válido o AT não liga gatilho e painel. Com A8 (`details`), o caso desaparece.
- Veredito: refina — `aria_controls_resolves` enquanto o padrão APG existir.

### A27. Toggle: `aria-pressed` não é switch
- Citação: `02-aria.md:200` — "| Alternador | `<input type=\"checkbox\" role=\"switch\">` | `role=\"switch\"` + `aria-checked` |"
- Semântica: alternador usa `aria-checked` (checkbox); `aria-pressed` é de botão toggle.
- Consequência: `docs/05:74` deixa "estado (`aria-pressed`/`aria-expanded`) quando aplicável" vago e `div` não pode ter `role="switch"` (`spec/elements.toml:221`); switch = `input type="checkbox"` puro.
- Veredito: refina — fixar qual estado cada primitivo emite.

### A28. `inert` é global e substitui `aria-hidden`
- Citação: `19-inert-forced-colors.md:75` — "**`inert` resolve o problema de `aria-hidden`**: elementos com `aria-hidden=\"true\"` ainda podem receber foco via Tab, criando um \"buraco negro\" de foco. `inert` previne isso."
- Semântica: `inert` remove da árvore **e** do foco; `aria-hidden` só da árvore.
- Consequência: `inert = "presence"` (`spec/elements.toml:62`) cobre drawer; com `aside` sem `roles`, a política é "overlay só via `<dialog>` + `inert` no resto" — falta escrevê-la em docs/06.
- Veredito: confirma D16; documentar a política de overlay.

### A29. O `<dialog>` nativo entrega trap, Escape e retorno de foco
- Citação: `21-keyboard-patterns.md:191` — "3. Ao fechar: foco retorna ao elemento que acionou a abertura"
- Semântica: `showModal()` cria trap, `Escape` fecha e o navegador devolve o foco ao invocador.
- Consequência: `dialog_modal_semantics` + `no_aria_modal` (`spec/elements.toml:809`) acertam; abrir/fechar e foco inicial são asset declarado — precisam estar no contrato de docs/07.
- Veredito: confirma D13; documentar o contrato de foco.

### A30. Foco após swap de conteúdo
- Citação: `12-wcag-quick-reference.md:58` — "| **2.4.11** Focus Not Obscured | Foco não fica oculto (mín. parcialmente visível) | AA |"
- Semântica: o foco precisa permanecer visível e previsível inclusive quando conteúdo é substituído.
- Consequência: em navegação de página inteira o browser reinicia o foco; em swap (htmx) o foco fica órfão. O corpus não define a técnica — a crate precisa fixar: após swap, foco no `<h1>`/`main` com `tabindex="-1"` (agora global: `spec/elements.toml:67`, `tabindex = ["-1", "0"]`).
- Veredito: lacuna do corpus (§7); decisão própria.

### A31. 2.4.11 precisa do offset da barra fixa como token
- Citação: `docs/05-acessibilidade.md:40` — "**CSS**: `scroll-padding-top` = altura da barra fixa"
- Semântica: elemento focado não pode ficar sob header `sticky`/`fixed`; `scroll-padding` resolve.
- Consequência: o valor não pode ser hardcoded — precisa de token (`--sticky-h`) no tema, e `sniffCSS` deve medir `getBoundingClientRect` do focado contra a barra.
- Veredito: confirma; falta o token.

### A32. Região viva: persistente, atômica, uma assertiva
- Citação: `05-live-regions.md:95` — "1. **Adicione `aria-live` ANTES das mudanças** — seja no HTML inicial ou via JS antes de inserir conteúdo"
- Semântica: a região existe vazia antes do texto (`:96`), anuncia o todo em contadores (`:70`: sem `aria-atomic`, AT anuncia só "34") e não há múltiplos `alert` (`:110`).
- Consequência: `docs/05:53` e `aria-atomic` (`spec/elements.toml:71`) já refletem isso; falta o identificador canônico (`data-a11y-live`), o invariante `at_most_one_assertive` e restringir `aria-live` a `div` com role.
- Veredito: confirma (parcialmente corrigido); completar spec+shell.

### A33. 2.5.8 (24 px) vs regra interna de 44 px
- Citação: `24-wcag-supplement.md:17` — "| **Nota** | Em WCAG 2.2, este critério foi sucedido por **2.5.8 Target Size (AA)** com requisito similar mas em nível AA. |"
- Semântica: 2.5.5 (44 px) é AAA; 2.5.8 (24 px) é o piso AA; links em linha de texto são isentos.
- Consequência: `docs/05:43` adota 24 px como requisito e 44 px como regra interna — coerente com `08-mobile-accessibility.md:10`. Falta que a isenção de prosa seja automática no `sniffCSS` (por contexto inline), não nota de rodapé.
- Veredito: confirma; automatizar a isenção.

### A34. 2.5.7: a alternativa tem de existir sem JS
- Citação: `12-wcag-quick-reference.md:65` — "| **2.5.7** Dragging Movements | Alternativa para arrastar (drag) | AA |"
- Semântica: arrastar precisa de alternativa por ponteiro único/teclado — não precisa **ser** arrastar.
- Consequência: sem `draggable`/`on*`, a única reordenação expressável é formulário com botões "subir/descer" (`type=submit`). Definir `Reorder` = lista + botões; 2.5.7 satisfeita por construção e `Sortable` (A24) sai do vocabulário.
- Veredito: refina; trocar `Sortable` por `Reorder`.

### A35. Gestos e orientação
- Citação: `08-mobile-accessibility.md:41` — "> **Conteúdo não deve ser restrito a uma única orientação** (retrato ou paisagem), a menos que essencial."
- Semântica: 1.3.4 proíbe conteúdo restrito a uma orientação; 2.5.2 exige ação em up-event (`08-mobile-accessibility.md:26`) e 2.5.4 alternativa para movimento.
- Consequência: sem `on*` (D3) não há `mousedown`/`touchstart`/`devicemotion`, então 2.5.2/2.5.4 são satisfeitas estruturalmente; para 1.3.4, `docs/05:23` cita "`@media (orientation)`" como mecanismo, o que pode virar a própria violação.
- Veredito: confirma D3 e refina a redação de `docs/05:23`.

### A36. Preferências de mídia ausentes do crate
- Citação: `19-inert-forced-colors.md:153` — "@media (forced-colors: active) {"
- Semântica: em cores forçadas o autor perde fundos e cores (foco usa `Highlight`, texto `CanvasText`); `prefers-contrast: more` (`20-os-browser-settings.md:205`), `prefers-reduced-transparency` (`09-seizures-motion.md:151`) e `prefers-reduced-motion` (`09-seizures-motion.md:159`) completam o conjunto.
- Consequência: `docs/05` não menciona `forced-colors` em lugar nenhum; o tema (docs/06) precisa de camada que mapeie tokens → `Canvas/CanvasText/ButtonFace/ButtonText/LinkText/Highlight/GrayText` e nunca `forced-color-adjust: none` fora de ícone. 2.3.3 é AAA e sai da meta.
- Veredito: lacuna real — nova linha SC (1.4.3/1.4.11) + regra de tema.

### A37. SVG informativo pode compilar sem nome
- Citação: `18-svg-canvas.md:18` — "| `<title>` | Nome curto do SVG (como `alt`) | Sim, para SVGs informativos |"
- Semântica: SVG informativo precisa de `<title>`/`aria-label`; decorativo, `aria-hidden="true"` + `focusable="false"`.
- Consequência: `svg` tem `accessible_name = "optional"` (`spec/elements.toml:513`) e apenas `svg_icon_default_decorative` (`:518`); não há invariante "nome **ou** `aria-hidden`", então o SVG informativo sem nome compila (1.1.1).
- Veredito: refina — `svg_name_or_hidden`.

### A38. `popovertarget` sem `popover`
- Citação: `spec/elements.toml:689` — "allowed = [\"class\", \"id\", \"name\", \"value\", \"disabled\", \"form\", \"popovertarget\","
- Semântica: `popovertarget` só funciona apontando para um elemento com `popover`; sem ele, o atributo é inerte.
- Consequência: nenhum elemento declara `popover`, então `popover_target_resolves` (`:694`) nunca pode ser satisfeito — API morta que compila. Ou se remove o atributo de `button`, ou se adiciona `popover` a `div` com invariante de par.
- Veredito: refuta — atributo sem contraparte é escape hatch silencioso.

### A39. Numeração 3.3.7/3.3.8/3.3.9 do corpus está errada
- Citação: `24-wcag-supplement.md:152` — "| 3.3.7 | Accessible Authentication | AA | Novo |" (e `:153` "| 3.3.8 | Accessible Authentication (No Exception) | AAA | Novo |")
- Semântica: pela numeração publicada pelo W3C, 3.3.7 é **Redundant Entry (A)**, 3.3.8 é **Accessible Authentication — Minimum (AA)** e 3.3.9 é **(Enhanced, AAA)**; `12-wcag-quick-reference.md:89-90` repete o erro.
- Consequência: `docs/05:50-51` já segue o W3C e registra o defeito (`docs/05:55-58`) — a crate está certa; auditorias futuras devem usar `docs/`, não o corpus.
- Veredito: refuta o corpus (D17 aplicado); manter a nota.

### A40. Automatizado cobre ~30-50% — o gate é necessário, não suficiente
- Citação: `11-testing-tools.md:112` — "Ferramentas automatizadas encontram ~30-50% dos problemas de acessibilidade."
- Semântica: axe/Lighthouse não sabem se `alt` é semanticamente correto, se a ordem de tabulação faz sentido ou se o texto é compreensível (`11-testing-tools.md:114-121`); a estimativa mais otimista é 40-50% (`23-advanced-testing.md:6`).
- Consequência: o gate de CI de `docs/05:10` deve declarar cobertura parcial e apontar §8 (`docs/05:130`) como o restante; senão "axe verde" vira falsa garantia.
- Veredito: confirma D10; explicitar o teto em docs/05 §8.

## 3. Confirmações do desenho

- **`alt` tipado** — `07-multimedia.md:10-12` valida `required = ["src", "alt", "width", "height"]` + `alt_empty_means_decorative` (`spec/elements.toml:442-450`).
- **Vídeo sem legenda não compila** — `media_requires_captions` + `captions_kind_required` (`spec/elements.toml:485`) correspondem a 1.2.2.
- **Ausência de `autoplay`** — `07-multimedia.md:62` marca autoplay como errado; o vocabulário não tem o atributo (1.4.2 por construção).
- **`tabindex` só `-1|0`** — `03-keyboard-accessibility.md:13`; agora global (`spec/elements.toml:67`), o que fecha 2.4.3 e viabiliza alvo de foco (A30).
- **`aria-*` com enum de valor** — `02-aria.md:95,116`; `[globals.aria]` (`spec/elements.toml:70`) resolve o escape hatch, inclusive `aria-atomic` (A32).
- **`banner`/`main` únicos** — `25-tag-role-reference.md:17,20` valida `banner_unique_at_root` (`:164`) e `main_unique`+`main_tabindex_minus_one` (`:178`).
- **`form` nomeado é landmark** — `25-tag-role-reference.md:22` valida `named_is_landmark` (`spec/elements.toml:618`).
- **`inert` em vez de `aria-hidden`** — `19-inert-forced-colors.md:72,75` valida `inert = "presence"` (`spec/elements.toml:62`).
- **`role="alert"` nunca estático** — `02-aria.md:146` valida `docs/05:53`.
- **Nada de `aria-live` junto de `role="alert"`** — `05-live-regions.md:98` já refletido em `docs/05:75`.
- **`Tooltip` por `aria-describedby`, não `aria-expanded`** — `02-aria.md:37,196` já refletido em `docs/05:30` e `docs/05:79`.
- **`aria-modal` desnecessário com `<dialog>`** — `22-design-patterns.md` valida `no_aria_modal` (`spec/elements.toml:809`).
- **WCAG 2.2 AA como meta** — `VERSION` e `SKILL.md:36` fixam o mesmo alvo de `docs/05`.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata, onde inserir) |
|---|---|---|
| `docs/05:68` "erro com `role="alert"`" | `02-aria.md:146` — alert pré-existente não é anunciado | Em `docs/05` §3: "erro entra por `aria-describedby` + `aria-invalid`; o anúncio vem da **região viva persistente do shell** ou do foco no resumo de erros (`tabindex=\"-1\"`)." |
| `spec/elements.toml:80` `aria-label = "text"` | `02-aria.md:101` — `aria-label` é o nome acessível | Trocar por `aria-label = "msg"  # catálogo Msg; nunca string livre (docs/05 §7)`. |
| `docs/05:23` `@media (orientation)` como mecanismo de 1.3.4 | `08-mobile-accessibility.md:41` — não restringir orientação | "**gate de CSS**: nenhuma regra condiciona conteúdo a `orientation`; `@media (orientation)` só para ajustes cosméticos." |
| `docs/05:32/42/51` citam `Carousel`, `Sortable`, `PasswordField` | a spec não tem nenhum dos três | "`Reorder` (lista + botões submit)" para 2.5.7; "`Carousel` removido — sem autoplay no vocabulário, 2.2.2 não se aplica"; "3.3.8 satisfeita por construção (sem `on*`, sem CAPTCHA), `autocomplete=current-password`". |
| `docs/05` sem `forced-colors` | `19-inert-forced-colors.md:153`, `20-os-browser-settings.md:205` | Nova linha SC: "1.4.3/1.4.11 — `forced-colors: active`: tema mapeia tokens para `Canvas/CanvasText/ButtonFace/ButtonText/LinkText/Highlight/GrayText`; `sniffCSS` roda com a emulação ativa." |
| `spec/elements.toml:804` `dialog` sem `roles` | `02-aria.md:80` — `alertdialog` para confirmação crítica | Em `[element.dialog]`: `roles = ["dialog", "alertdialog"]`. |
| `spec/elements.toml:664` `select` sem `required_children` | `06-forms-and-labels.md:46` — select sempre com opções | `required_children = ["option"]` (via `optgroup` conta o neto). |
| `spec/elements.toml:625` `fieldset` sem checagem de `legend` vazio | `06-forms-and-labels.md:39` | `invariants = ["legend_required", "legend_not_empty"]`. |
| `spec/elements.toml:780` `td` com `headers` sem resolução | `16-table-grid-attributes.md:47` | `invariants = ["headers_resolve_to_th_id"]`. |
| `spec/elements.toml:513` `svg` com nome opcional | `18-svg-canvas.md:18,31` | `invariants = ["svg_icon_default_decorative", "svg_name_or_hidden"]`. |
| `spec/elements.toml:686` `button` com `popovertarget` | `22-design-patterns.md:194` ensina `aria-haspopup`+`aria-expanded`+`aria-controls` | Remover `popovertarget`/`popovertargetaction` **ou** declarar `popover` em `div` com invariante de par (A38). |
| `docs/05:94` advisory "`tabindex` positivo" | `spec/elements.toml:67` já torna positivo impossível | Remover a regra (nunca dispara) — regra morta viola §6 regra 6. |

## 5. Adições prontas para colar

### (a) Linhas TOML

```toml
[globals.aria]
aria-label = "msg"                  # catálogo Msg; nunca string livre (A22)

[element.dialog]
roles = ["dialog", "alertdialog"]   # alertdialog só para ação destrutiva (A19)

[element.div]
roles = ["status", "alert", "tooltip"]
invariants = ["live_region_role_only", "one_assertive_per_document"]

[element.select]
required_children = ["option"]      # direto ou via optgroup (A18)

[element.fieldset]
invariants = ["legend_required", "legend_not_empty"]

[element.td]
invariants = ["headers_resolve_to_th_id"]

[element.th]
invariants = ["scope_enum", "scope_matches_section", "id_unique_in_table"]

[element.svg]
invariants = ["svg_icon_default_decorative", "svg_name_or_hidden"]

[element.nav]
# invariante é de documento, não de elemento: mover para o passe global
invariants = []

[element.caption]
invariants = ["caption_not_empty", "caption_not_aria_hidden"]

# novas invariantes de documento (passe global, docs/04 §4):
#   heading_level_no_skip            (A6)
#   landmark_names_distinct           (A3, escopo: nav|aside|search|form|section nomeados)
#   aria_current_unique_per_nav       (A25, output por <nav>)
#   at_most_one_assertive_region      (A32)
#   live_region_persistent_in_shell   (A32)
#   aria_controls_resolves            (A26)
#   name_from_inner_heading           (A1)
```

### (b) Primitivo → o que deve emitir

| Primitivo | Atributos/estados emitidos | Fonte normativa |
|---|---|---|
| `Field` | `id` gerado, `<label for>`, `aria-describedby` (hint), `aria-invalid`, `required`+`aria-required`, `autocomplete` (enum `Msg`) | `06-forms-and-labels.md:8,97,168`; 1.3.1/3.3.1/3.3.2/1.3.5 |
| `ErrorSummary` | container com `id`, `tabindex="-1"`, links `#campo`, foco programático | `06-forms-and-labels.md:103`; 3.3.1/3.3.3 |
| `Checkbox`/`RadioGroup` | `<fieldset>`+`<legend>` não vazio, `name`, `id`+`for` por opção | `06-forms-and-labels.md:39`; 1.3.1 |
| `Select` | `id`+`<label for>`, `<optgroup label>`, ≥ 1 `option`, `autocomplete` quando identidade | `06-forms-and-labels.md:46,50` |
| `Dialog` | `<dialog>`, `aria-labelledby` (id gerado), botão fechar nomeado, `alertdialog` para destrutivo | `21-keyboard-patterns.md:189-205`; 4.1.2 |
| `Disclosure` | `<details>`+`<summary><h2>` (sem `aria-expanded`/`aria-controls`) | `22-design-patterns.md:15-16`; 1.3.1 |
| `Table` | `<caption>` não vazio, `thead`/`tbody`/`tfoot`, `th scope`, `headers` só com `id` resolvido | `01-semantic-html.md:157`; `25-tag-role-reference.md:108` |
| `Pagination` | `<nav aria-label>`, `<ol>`, um `aria-current="page"`, `rel="prev"/"next"`, alvo ≥ 44 px | `22-design-patterns.md:332-341`; 2.5.8 |
| `Breadcrumb` | `<nav aria-label>`, `<ol>`, separador `aria-hidden`, um `aria-current="page"` | `22-design-patterns.md:313-322` |
| `SkipLinks` | primeiro focável; `href="#main"`; `main` com `id`+`tabindex="-1"` | `03-keyboard-accessibility.md:135`; 2.4.1 |
| `Icon` | `<svg aria-hidden="true" focusable="false">`; nomeável só por `Icon::labelled` (`<title>`) | `18-svg-canvas.md:31,60`; 1.1.1 |
| `Video` | `<video controls preload>` + `<track kind="captions" srclang>`; sem `autoplay` | `07-multimedia.md:62`; 1.2.2/1.4.2 |
| `Audio` | `<audio controls>` + **link para transcrição** (não `<track>`) | `07-multimedia.md:79`; 1.2.1 |
| `LiveRegion` | região fixa no shell (vazia), `role="status"` + `aria-atomic="true"`; uma `assertive` por página | `05-live-regions.md:70,95,110`; 4.1.3 |
| `Tooltip` | `role="tooltip"` + `aria-describedby` no gatilho + `Escape` sem mover foco | `02-aria.md:37,196`; 1.4.13 |
| `Reorder` | lista + botões `type=submit` "subir/descer" (sem arrastar) | `12-wcag-quick-reference.md:65`; 2.5.7 |

**SCs A/AA ausentes de `docs/05` §2** (a tabela cobre 33 das 51 exigíveis): 1.2.1, 1.2.3, 1.2.4, 1.2.5,
1.3.3, 1.4.2, 1.4.5, 1.4.12, 2.1.4, 2.2.1, 2.4.5, 2.5.1, 2.5.2, 2.5.4, 3.2.3, 3.2.4, 3.2.6, 3.3.4.
Mecanismos: transcrição/`alt` (1.2.1, 1.2.3, 1.2.5), transcrição ao vivo fora de escopo (1.2.4),
advisory "instrução não depende de forma/cor" (1.3.3), ausência de `autoplay` (1.4.2), `Text` nunca
aceita imagem-de-texto (1.4.5), `text-spacing` no tema (1.4.12), ausência de atalho de tecla única
(2.1.4), sessão com aviso (2.2.1), sitemap+busca no shell (2.4.5), ausência de `on*`
(2.5.1/2.5.2/2.5.4), nav/id de módulo fixos (3.2.3/3.2.4), bloco de ajuda no shell (3.2.6) e revisão
antes do submit (3.3.4).

### (c) O que **nunca** deve ser responsabilidade da crate

- **Contraste medido** (composição de ancestrais, imagem de fundo): `axe`/`sniffCSS` — `04-color-and-contrast.md:98-100`.
- **Semântica do `alt`** ("descreve bem?"): julgamento humano — `11-testing-tools.md:114-121`.
- **Ordem de tabulação real** e focus trap: e2e de teclado — `21-keyboard-patterns.md:467-484`.
- **Anúncio efetivo em leitor de tela** (NVDA/VoiceOver/TalkBack): teste manual — `23-advanced-testing.md`.
- **Persistência de valor entre etapas** (3.3.7 Redundant Entry): contrato da aplicação.
- **Bloqueio de colagem/CAPTCHA** (3.3.8): política de JS; a crate só garante que não há API para isso.
- **Alvo de toque medido** e **foco obscurecido medido**: `sniffCSS` com viewport real — `08-mobile-accessibility.md:107`, `12-wcag-quick-reference.md:58`.
- **Timers/timeouts** (2.2.1/2.2.6): comportamento de sessão do servidor.
- **Conteúdo piscante/PEAT** (2.3.1): análise de mídia — `09-seizures-motion.md`.
- **Grid/treegrid/canvas interativo, shadow DOM**: fora do vocabulário (A15; inventário `17`).

## 6. Armadilhas de complexidade

- **`role` livre "só para este componente"** — custo: reabre D4, o ratchet deixa de medir e morre a auditoria de role redundante; usar `roles` por elemento (já existe desde a revisão de 15:06).
- **`aria-*` como string** — custo: `aria-current="pg"` compila, axe falha em runtime e o type system perde a vantagem sobre templates; a allowlist já é enum, manter assim (A22 é o último furo).
- **Suportar `grid`/`treegrid`** — custo: roving tabindex, `aria-activedescendant`, `colindex/rowindex`, teclado 2D e `inert` de célula; o maior item de JS do APG para um CMS que só exibe dados (A15).
- **`application`** (`13-aria-advanced-roles.md:8`) — custo: desliga a navegação do AT e exige ARIA completa manual; um editor rich-text não vale o risco num portal.
- **`aria-roledescription`/`aria-keyshortcuts`/`aria-flowto`** (`15-aria-advanced-attributes.md:3,45,109`) — custo: suporte parcial, valor não localizável, ganho não mensurável.
- **Manter `Sortable`/`Carousel`/`PasswordField`** (A24) — custo: cada um arrasta JS para um crate que promete "sem JavaScript" fora de assets declarados.
- **Overlay não-`<dialog>`** (drawer/sheet) — custo: trap + `inert` + retorno de foco manuais em JS; `<dialog showModal>` entrega de graça (A29).
- **`display:grid` em `<table>`** (A14) — custo: perde semântica e não há `role` para repor; a "solução" vira exceção de CSS por tabela.
- **`aria-errormessage`** (A21) — custo: suporte desigual e dois caminhos de teste para o mesmo efeito.
- **`<canvas>`** (`18-svg-canvas.md:115-160`) — custo: fallback tabular espelhado + overlay ARIA; um gráfico do portal cabe em SVG + `<table>`.

## 7. Lacunas do corpus

- **Foco após swap/partial update** — nenhum arquivo diz para onde vai o foco quando um fragmento é substituído (A30).
- **SSR e live regions** — o corpus assume JS (`05-live-regions.md:52-58`); não trata erro já presente no HTML (A20).
- **`forced-colors`/`prefers-contrast` em SSR** — `19`/`20` dão CSS, não o mapeamento token → cor de sistema (A36).
- **`aria-current="step"`** — `02-aria.md:95` lista o valor, nenhum arquivo mostra o padrão de wizard.
- **`output`/`progress`/`meter` como status** — `25-tag-role-reference.md:74` dá `output → status`, nenhum arquivo diz quando usá-lo em vez de live region.
- **`abbr` + 3.1.4** — `10-cognitive-accessibility.md` cita 3.1.4 (AAA); a crate exige `title` em `abbr` (`spec/elements.toml:309`) sem política de "primeira ocorrência".
- **Mapa `elemento → role` como dado de máquina** — `25` é markdown; o TOML equivalente falta para o advisory "role redundante".
- **`18-aprendizados/README.md`** — citado por D17 (`docs/13-decisoes.md:183`) e ainda inexistente; o índice do diretório precisa nascer junto deste arquivo.
