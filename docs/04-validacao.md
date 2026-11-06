# 04 — Validação: quatro níveis (N3 removido) e seus limites

## 1. Os níveis

| Nível | Mecanismo | Quando falha | Custo para o dev |
|---|---|---|---|
| **N1 — tipo** | ausência de API; parâmetro obrigatório | `cargo check` | zero (é o caminho normal) |
| **N2 — erro de macro** | validação no passe da proc-macro | `cargo check` | ler a mensagem |
| **N3 — lint de fonte** | *(removido — D28/A4)*: aviso sem consequência é documentação de pior qualidade; o que importa subiu para N2 ou virou teste | — | — |
| **N4 — lint de saída** | `html5ever` sobre a página renderizada, atrás de `feature = "audit"` (D30) | CI **do consumidor** | corrigir o componente |
| **N5 — gate de CI** | `axe`, `sniffCSS`, teclado, e2e | CI | bloqueia merge |

## 2. Limite técnico que define o desenho

**Proc-macro em Rust stable só consegue emitir erro, não aviso.** `proc_macro::Diagnostic` é instável.
Portanto:

- tudo que é **inegociável** vive em N1/N2 (erro);
- tudo que é **recomendação** vive em N4 — e N4 é **teste que falha** no CI do consumidor (D30), não
  um aviso com ratchet: a camada advisory (N3) foi cortada em D28/A4 justamente por não bloquear nada.

Não existe "warning do compilador de HTML" neste projeto. Isso é uma decisão, não uma limitação
contornada.

## 3. O que cada nível consegue ver

| Regra | N1 | N2 | N4 |
|---|---|---|---|
| `img` sem `alt` | ✅ | | |
| atributo desconhecido / typo | | ✅ | |
| `<div>` dentro de `<p>` | | ✅ | |
| `id` duplicado no bloco | | ✅ | |
| `label for` órfão no bloco | | ✅ | |
| `aria-label` em `<div>` sem papel | | | ✅ |
| `div`/`span` sem classe nem papel | | | ✅ | ✅ |
| `<h1>` único e ordem de níveis | | | | ✅ |
| `id` duplicado na página | | | | ✅ |
| `aria-labelledby` que não resolve | | | | ✅ |
| JSON-LD que não é JSON válido | ✅ (tipo) | | | ✅ |
| link com texto ambíguo (`Msg::ReadMore`) | | | ✅ | |
| contraste real | | | | ❌ (axe) |
| alvo de toque medido | | | | ❌ (sniffCSS) |
| ordem de foco / tabulação | | | | ❌ (e2e) |

**Fronteira explícita:** macro valida o **local** (um bloco); o lint valida a **página**. Uma
proc-macro não vê outra, então nada de "página inteira" no compilador.

## 4. N2 — o que a macro valida

1. **Sintaxe e aninhamento**: elemento conhecido, `void` sem filhos, `picture`/`table` com estrutura
   mínima.
2. **Atributos**: allowlist por elemento, obrigatórios presentes, `aria-*` conforme papel, `data-*`
   declarado, valores com tipo de contexto adequado. Atributo da lista
   `[meta] boolean_attributes` só existe no tipo como `bool` e é emitido como **presença**
   (`<button disabled>`): `disabled="false"` é `true` em HTML e o gerador nunca o aceita.
3. **Conteúdo-modelo**: categorias pai→filho, exceções (`<a>` em `<a>`).
4. **Coerência dentro do bloco**: `id` único; todo `for` tem `id` correspondente; todo
   `aria-describedby`/`aria-labelledby` resolve no bloco.
5. **Proibições duras**: `style`, `on*`, `srcdoc`, `Raw`.

Mensagem sempre no formato: `o que` → `por quê (regra)` → `como corrigir`. Toda mensagem tem teste
`trybuild` (ver [`12-testes.md`](12-testes.md)).

### 4.0 Três escopos de invariante (quem cobra cada regra)

Toda invariante declara dono em `spec/elements.toml` → `[invariants]`. O escopo não é documentação:
é a resposta para "quem falha, e quando".

| Escopo | Quem cobra | Promessa | Quantas |
|---|---|---|---|
| `block` | a macro, no `html!` | **erro de compilação**, com span no token | **62** |
| `type` | o construtor do crate (`Page`, `Form`) | **não compila** sem o dado: um `main`, um `h1`, um `title`, um banner no root, um CSRF | **6** |
| `output` | `N4` sobre o HTML final | falha o **teste**; nenhum dos dois vê | **5** |

As **5 de saída** — as únicas que ficam em cima de teste:

| Invariante | Por que nenhum tipo responde |
|---|---|
| `landmark_names_distinct` | depende de nomes de landmarks espalhados por fragmentos que nunca se veem |
| `no_content_derived_identifiers` | é uma propriedade do **conjunto** dos ids da página, não de um bloco |
| `ids_unique` | dois componentes independentes podem escolher o mesmo `id` (DOM clobbering, `label` quebrado) |
| `heading_levels_no_skips` | a sequência `h2 → h4` só existe na página montada (WCAG 1.3.1) |
| `popover_target_resolves` | o alvo do `popovertarget` pode viver em outro componente dinâmico |

As **6 de tipo**: cinco subiram de "teste" para "tipo" quando o shell virou `Page` — `main_unique`,
`single_h1`, `shell_owns_h1`, `document_title_required` e `banner_unique_at_root` não são mais regras a
lembrar. `Page::new(&ShellData { title, heading, … }, main)` recebe **exatamente um** miolo, o `<h1>` e o
`<title>` vêm de campos **não opcionais**, e o `<h1>` não compila em `html!` (`only = "shell"`). Um
fragmento não pode violar o que não consegue expressar. A sexta é `form_requires_csrf`: `Form::open`
exige o token, então o `<form>` sem `_token` não compila — o mesmo movimento, aplicado ao formulário.

O que resta em `output` é o que **nenhuma** ferramenta de compilação pode decidir sem possuir a página
inteira — e possuir a página inteira é exatamente o que mataria a composição (cada view teria que
conhecer o documento das outras). A fronteira é estrutural, não preguiça.

### 4.1 Registro normativo de invariantes

Cada nome abaixo é declarado em `invariants = [...]` do `spec/elements.toml` e **precisa** existir
aqui: `tools/validate_spec.py` compara as duas listas e falha se uma invariante compilada não estiver
documentada (ou o contrário). O nome é a chave da mensagem de erro e do teste.

| Invariante | Elementos | Regra |
|---|---|---|
| `href_is_typed` | `a` | `href` só existe via `Href` (https/relativo/mailto/tel); `javascript:`/`data:` não constroem |
| `blank_target_implies_noopener` | `a` | `target="_blank"` acrescenta `rel="noopener noreferrer"` automaticamente |
| `no_aria_label_over_visible_text` | `a`, `button` | com texto visível, nome acessível é o texto; `aria-label` divergente é erro |
| `a_rel_enum` | `a` | `rel` é enum fechado (`noopener`, `noreferrer`, `external`, `nofollow`, `license`, …); `opener` é proibido |
| `referrer_policy_enum` | `a`, `link`, `img` | `referrerpolicy` só nos valores padronizados; `unsafe-url` proibido |
| `no_content_derived_identifiers` | `a` e todo `id`/`name` | `id`/`name` nunca derivam do dado do usuário: são gerados por primitivo. Um `id` vindo de slug de conteúdo abre DOM clobbering (`window.x`) e colisão entre páginas |
| `banner_unique_at_root` | `header` | no máximo um `header` **filho direto do `body`** (banner); dentro de `article`/`section`/`nav`/`main`/`aside` não é banner e pode repetir |
| `legend_not_empty` | `fieldset` | `legend` com nome acessível não vazio (é o rótulo do grupo) |
| `select_has_selectable_option` | `select` | ao menos uma `option` selecionável; placeholder é `value=""` + `disabled`, nunca `selected` |
| `svg_informative_has_name` | `svg` | ícone decorativo é `aria-hidden`; SVG que carrega informação tem `svg_title`/`aria-labelledby` |
| `shell_only_elements` | `html`, `head`, `body`, `title`, `meta`, `link`, `script`, `noscript`, `h1` | elementos de documento **não compilam** em `html!`: só o `Document` do shell os emite. É a garantia estrutural que hoje existia apenas como convenção |
| `form_requires_csrf` | `form` | o `<form>` só existe via `Form::open(action, method, &CsrfToken)`: sem token não compila, e `PUT`/`PATCH`/`DELETE` derivam o `_method` oculto — esquecer CSRF deixa de ser possível |
| `control_has_accessible_name` | `a`, `button`, `input` | controle sem nome acessível (só ícone, só placeholder) não compila — WCAG 1.1.1/2.4.4/4.1.2 |
| `no_aria_hidden_on_focusable` | `a`, `button`, `input` | `aria-hidden="true"` num elemento focável é contradição (o foco some da árvore de acessibilidade) |
| `head_charset_first` | `head` | `meta charset` é o primeiro filho do `head` e cai nas primeiras 1024 posições do documento |
| `captions_kind_required` | `video` | o `track` exigido é `kind="captions"` (não basta `subtitles`; WCAG 1.2.2) |
| `named_is_landmark` | `form` | `form` só é landmark (`role="form"`) quando tem nome acessível; sem nome, não recebe papel |
| `single_h1` | `h1` | exatamente um `h1` por documento (o shell o emite; o módulo não) |
| `shell_owns_h1` | `h1` | `h1` só no shell/`Page`; view de módulo com `h1` próprio falha o lint |
| `main_unique`, `main_tabindex_minus_one` | `main` | um `main` por página; `tabindex="-1"` fixo para o skip link |
| `landmark_names_distinct` | `nav` | landmarks do mesmo tipo precisam de nomes distintos |
| `section_has_heading_or_name` | `section` | `section` sem heading e sem nome acessível não é seção — use `div` |
| `alt_empty_means_decorative` | `img` | `alt` só é vazio via `Alt::decorative()` (imagem decorativa explícita) |
| `intrinsic_dimensions_required` | `img` | `width` + `height` obrigatórios: sem dimensão intrínseca o espaço reservado é zero e o `CLS` estoura |
| `script_src_requires_defer` | `script` | na forma `src` externo, `defer` é obrigatório e `async` é proibido (ordem determinística e sem bloqueio de render) |
| `srcset_requires_sizes_and_dims` | `img` | `srcset` exige `sizes` + `width`/`height` (CLS 0) |
| `fold_controls_loading` | `img` | `loading="lazy"`/`fetchpriority` são derivados da posição, não digitados |
| `label_or_labelledby_in_block` | `input`, `select`, `textarea` | controle com `id` e `label for`/`aria-labelledby` resolvível no mesmo bloco |
| `label_exempt_types` | `input` | `hidden`/`submit`/`reset`/`button`/`image` dispensam rótulo |
| `attribute_by_input_type` | `input` | atributo só existe se o `type` o admite (`min`/`max`/`step` são de número/data) |
| `autocomplete_enum` | `input` | `autocomplete` é enum com tokens de identidade (WCAG 3.3.7 — nível A) |
| `pattern_flags_forbidden` | `input` | `pattern` é regex comum, sem flags inline; mensagem de erro é do `Field`, não do navegador |
| `for_matches_id_in_block` | `label` | `for` aponta para controle existente no bloco; envolver o controle dispensa `for` |
| `type_enum` | `button` | `type` é obrigatório e enumerado (nunca o `submit` implícito) |
| `method_enum`, `action_required_unless_dialog`, `novalidate_forbidden` | `form` | método enumerado; `action` obrigatório exceto `method="dialog"`; validação nativa sempre ligada |
| `scope_enum`, `scope_matches_section` | `th` | `scope` obrigatório e coerente com a seção (`rowgroup`/`colgroup`) |
| `caption_required`, `caption_not_empty` | `table` | `caption` é o primeiro filho e não é vazio (nome acessível da tabela) |
| `no_text_children` | `table` | texto solto em `table` é erro (vai para `caption`/`th`/`td`) |
| `tbody_explicit` | `table` | `tbody` sempre explícito (`thead`/`tfoot` quando houver) |
| `li_only_in_list` | `li` | `li` só em `ul`/`ol` |
| `details_name_accordion` | `details` | `name` só para acordeão exclusivo; sem `name` os `details` são independentes |
| `closedby_enum`, `no_aria_modal` | `dialog` | `closedby` enumerado (`any`/`closerequest`/`none`); `aria-modal` é derivado de `showModal()`, nunca digitado |
| `dialog_modal_semantics` | `dialog` | modal exige `aria-labelledby` apontando para o título do diálogo |
| `popover_target_resolves` | `button` | `popovertarget` resolve para elemento existente com `popover` |
| `media_requires_captions` | `video` | `track kind="captions"` presente (WCAG 1.2.2) |
| `srclang_required_for_subtitles` | `track` | `srclang` obrigatório quando `kind` é `subtitles` |
| `media_requires_transcript` | `audio` | áudio exige transcrição adjacente (o lint cobra o link, não o texto) |
| `source_profile_picture_only` | `source` | dentro de `picture` o atributo é `srcset` (nunca `src`) |
| `source_profile_media_only` | `media_source` | dentro de `video`/`audio` o atributo é `src` + `type` (nunca `srcset`) |
| `media_source_xor_src` | `video`, `audio` | `src` no elemento **ou** filhos `media_source`, nunca os dois |
| `meta_form_exclusive` | `meta` | três formas: `charset` · `name`+`content` · `property`+`content`; misturar é erro |
| `script_form_exclusive` | `script` | três formas: `src` externo · JSON-LD (`type`+texto) · `importmap`; nenhum executável inline |
| `summary_single_heading` | `summary` | no máximo um `heading` como conteúdo do `summary` |
| `noscript_context` | `noscript` | `metadata` quando em `<head>`, `flow`/`phrasing` quando em `<body>` |
| `cross_origin_requires_sandbox` | `iframe` | origem externa exige `sandbox` + `referrerpolicy` (e passa pela allowlist de terceiros) |
| `preload_requires_as_and_crossorigin` | `link` | `rel="preload"` exige `as` e `crossorigin` quando o recurso é fonte |
| `link_rel_enum` | `link` | `rel` é enum fechado (`stylesheet`, `icon`, `preload`, `manifest`, `alternate`, …) |
| `meta_name_enum`, `no_http_equiv` | `meta` | `name`/`property` de lista fechada; `http-equiv` é proibido (cabeçalho é do servidor) |
| `script_is_jsonld_only` | `script` | único `script` permitido no markup é `application/ld+json` |
| `document_title_required` | `title` | `Page` exige `title`; o shell compõe o sufixo |
| `datetime_profile` | `time` | `datetime` obrigatório quando o texto não é ISO-8601 |
| `preserve_whitespace` | `pre` | `pre` não é indentado pelo gerador (o espaço é conteúdo) |
| `svg_icon_default_decorative` | `svg` | ícone sem nome é `aria-hidden="true"`; com nome, `aria-labelledby` + `svg_title` |
| `no_flow_inside_phrasing` | `p` | `p` aceita só `phrasing` (pega `<div>` dentro de `<p>`) |

## 5. N3 — lint de fonte (removido)

O nível existia para dar **aviso educativo** sobre o AST. Foi cortado (D28/A4): aviso que não bloqueia
nada é documentação de pior qualidade, e a maior parte das regras nunca precisou de aviso — precisava de
tipo ou de default do gerador. O destino de cada uma:

| Regra que era N3 | Destino agora |
|---|---|
| `role` fora do enum do elemento | **N2** — o spec já restringe `role` por elemento (D16) |
| `a` sem `href` | **N2** — `required = ["href"]` no spec |
| `class` com token fora do vocabulário | **N2** — `Class`/`Tone` são enum |
| texto visível literal (não-`Msg`) | **N1/N2** — `Text` visível só aceita `M: Msg` (D31) |
| `button` sem `type` | **default do gerador** (`type="button"`, A3) |
| ícone decorativo sem `aria-hidden` | **default do gerador** (`svg_icon_default_decorative`) |
| `div`/`span` com filho único e sem papel | **descartada** — não é decidível sem heurística |
| `div` com `aria-label` sem papel | **N4** (página inteira) |
| link com `Msg` de texto ambíguo | **N4** (página inteira, 2.4.4) |

O relatório por página (percentual de elementos crus por view) continua existindo como **saída
informativa do teste de N4**; o que sai é o gate (`a11y-budget.toml` e o ratchet).

## 6. N4 — lint de saída (página inteira)

Implementação: `html5ever` para parse + regras próprias. Roda sobre:

1. fixtures de conformidade (golden);
2. as páginas capturadas pelo e2e (reuso: o snapshot do Playwright é a entrada).

Regras: `<main>` único · `<h1>` único e sem pular nível · landmarks nomeados distintos ·
`id` único global · `aria-*` que referenciam id existente · `lang`/`dir` corretos · `alt` diferente do
nome do arquivo · ausência de `<script>` sem `src` (exceto `application/ld+json`) · ausência de
`style=`/`<style>` · `href` externo dentro da allowlist do consumidor · ausência de
`<template shadowroot>`, `<slot>`, `part=`/`exportparts=` e de qualquer tag com hífen (D20).

### 6.1 Gate `árvore == saída`

O golden compara bytes; o gate N4 compara **estrutura**: o HTML emitido é reprocessado por
`html5ever`, e a árvore resultante precisa ser igual à árvore declarada (tag, atributos e texto, na
mesma ordem). É o que pega o que byte-diff esconde — elemento autocorrigido pelo parser (aninhamento
inválido), `void` que ganhou filho, atributo reordenado pelo navegador, texto que virou entidade.

## 7. Política de fail-safe

- Valor inválido vindo de **dado** (tema, token, cor): **ignorado**, a camada anterior vence. Nunca
  quebra a página (mesma política do shell como dado).
- Valor inválido vindo de **código**: erro de compilação. Não há meio-termo.

## 8. Custo declarado

| Item | Medida |
|---|---|
| `cargo check` por `html!` | aumenta com o tamanho do bloco; blocos de view típicos (< 200 linhas) ficam imperceptíveis |
| `html::audit` por página | ~5–15 ms (parse `html5ever`); o consumidor roda no CI dele (D30) |
| falso-positivo esperado em N4 | alvo: **zero** nos arquivos de referência (`M3`, [`16`](16-roadmap.md) §2) |

## 9. O que este esquema **não** promete

- Não promete acessibilidade correta: promete **estrutura e nome** corretos. "Este alt descreve a
  imagem?" e "este heading faz sentido?" continuam humanos.
- Não substitui `axe`/`sniffCSS`/teclado: esses medem o que só existe no navegador.
- Não vê marcação gerada por `format!`/construtor fora do vocabulário — e não deveria: se aparecer, é
  bug de arquitetura, e o próprio `Markup` privado impede.
