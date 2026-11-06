# 03 — Vocabulário de elementos (`spec/elements.toml`)

## 1. Por que a tabela é dado

Os elementos usados pelo escopo de origem (6.765 ocorrências, medido) mais o que o vocabulário precisa
para emitir documento cabem em um arquivo. Manter a tabela como **dado versionado** dá três coisas de
graça:

1. o gerador (`html-spec`) não tem regra duplicada em código;
2. o validador da macro e `tools/validate_spec.py` leem a mesma fonte;
3. mudança de contrato é um diff revisável, com histórico próprio.

Regra: **atributo novo = linha nova no TOML**. Não existe escape para atributo desconhecido.

## 2. Esquema do arquivo

```toml
schema = 2

[meta]
# versão da spec; congelada por minor do projeto, ver 11-estabilidade.md
spec_version = "2.0.0"

[element.div]
category = "flow"                     # string ou array
required = []
allowed = ["class", "id", "data-*", "aria-*"]   # `required` já está implícito aqui
children = ["flow"]                   # categorias e/ou nomes de elemento
forbidden_children = ["none"]         # (vazio; existe só para exceções)

[element.img]
category = ["phrasing", "media"]      # multi-categoria: dentro de <p> E dentro de <figure>
void = true                           # não tem `children`
required = ["src", "alt"]             # implicitamente permitidos
allowed = ["width", "height", "loading", "decoding", "fetchpriority", "sizes", "srcset", "class", "data-*"]
accessible_name = "alt"               # nome acessível vem do alt
invariants = ["alt_empty_means_decorative", "srcset_requires_sizes_and_dims"]

[element.a]
category = ["phrasing", "interactive"]
required = ["href"]                   # validado por `Href`, não por regex
allowed = ["hreflang", "rel", "target", "download", "referrerpolicy", "class", "aria-*", "data-*"]
children = ["phrasing"]
forbidden_children = ["a", "button", "input", "select", "textarea", "details", "summary", "dialog"]
```

Campos (esquema 2):

| Campo | Semântica |
|---|---|
| `category` | string **ou array**; um elemento casa com `children` do pai se qualquer categoria dele está listada (multi-categoria é obrigatório: `<a>` é `phrasing`+`interactive`, `<img>` é `phrasing`+`media`) |
| `void` | renderiza `<br>`, nunca `<br></br>`; não aceita filhos |
| `required` | atributos cuja ausência é **erro de compilação**; são **implicitamente permitidos** (allowlist efetiva = `required ∪ allowed`) |
| `allowed` | allowlist adicional; qualquer outro nome é erro (com a lista de válidos na mensagem) |
| `accessible_name` | `required`/`optional`/`none` (ou o atributo que a fornece: `alt`, `title`, `label`, `legend`, `caption`, `summary`) — substitui o pseudo-atributo `label` |
| `children` | categorias **e/ou nomes de elemento**; nome tem precedência. `text` = só texto; `none` = apenas em `void` |
| `required_children` | filho(s) obrigatório(s), verificado no bloco (`table`→`caption`, `video`→`track`, `fieldset`→`legend`) |
| `first_child` | filho que precisa vir primeiro |
| `forbidden_children` | exceções sobre `children` |
| `invariants` | nomes das regras que o validador implementa (registro em [`04-validacao.md`](04-validacao.md) §4) |
| `deprecated` | quando presente, emite advisory e mantém compilação por uma minor |

> **Por que o esquema subiu para 2:** a auditoria mostrou que `required` estava sobrecarregado
> (misturava atributo com filho), `category` única tornava `<p>texto <a></a></p>` impossível, e
> `role`/`aria-*` como curinga reabriam o escape hatch que D4 fecha. Ver D16 em
> [`13-decisoes.md`](13-decisoes.md).

## 3. Invariantes implementados hoje (subconjunto normativo)

> **Vocabulário dimensionado:** a medição no projeto de origem encontrou **63 elementos** realmente
> usados; o `spec/elements.toml` 2.0.0 declara **97** (os 63 medidos + `h4`–`h6`, `special` de texto,
> `tfoot`/`colgroup`/`col`, `optgroup`, `output`, `meter`, `progress`, `desc`, `svg_title`, `br`, `em`).
> Cada adição nova entra com justificativa no PR — o vocabulário cresce por necessidade, não por completude.

| Elemento | Invariante |
|---|---|
| `img` | `src` + `alt` + `width` + `height` obrigatórios (dimensão intrínseca = `CLS` 0); `alt` nunca `""` literal (só `Alt::decorative()`) |
| `iframe` | `title` obrigatório (WCAG 4.1.2) |
| `html` | `lang` obrigatório e válido em BCP-47 |
| `time` | `datetime` obrigatório quando há conteúdo textual não numérico |
| `th` | `scope` obrigatório (`row`/`col`/`rowgroup`/`colgroup`) |
| `input` | `id` obrigatório **e** vinculado a um `label`/`aria-labelledby` no mesmo bloco |
| `label` | `for` obrigatório quando não envolve o controle |
| `button` | `type` obrigatório (`button`/`submit`/`reset`) |
| `select`, `textarea` | `id` vinculado a label, como `input` |
| `dialog` | nome acessível obrigatório (`aria-labelledby` gerado pelo primitivo) |
| `nav`, `aside` | nome acessível obrigatório (`Label`) |
| `figure` | `figcaption` obrigatório quando há conteúdo visual sem texto adjacente |
| `video`, `audio` | `track` de legendas obrigatório (WCAG 1.2.2); `src` **ou** `media_source`, nunca os dois |
| `picture` | `source` antes de `img`, `img` obrigatório como último filho |
| `option` | `value` obrigatório; `selected` só dentro de `select` sem `multiple` |
| `script` | forma exclusiva: `src` externo · JSON-LD · `importmap`; nenhum executável inline |
| `abbr` | `title` obrigatório (expansão) |
| `figure` | `figcaption` como primeiro ou último filho |
| `meta` | forma exclusiva: `charset` · `name`+`content` · `property`+`content`; `http-equiv` proibido |

### 3.1 Estado inicial de controle é renderizado pelo servidor

O formulário abre com o estado que o servidor mandou, sem JS — a forma de cada controle é parte do
tipo, não do script:

| Controle | Estado inicial |
|---|---|
| `select` | `selected` na `option` correspondente (nunca `value`-only no JS) |
| `select` sem `multiple` | `size="1"` (não é listbox) |
| `textarea` | conteúdo como **texto filho**, não `value` (atributo ignorado em `textarea`) |
| `input` checkbox/radio | `checked` na opção marcada; nenhum grupo termina sem marcação |
| `input` `date`/`time`/`number` | valor em ISO-8601; formatação é do `Intl` no cliente |
| `input type="hidden"` | sempre acompanhado do token de CSRF do formulário |
| `fieldset` | `disabled` desabilita o grupo inteiro (não os filhos um a um) |

## 4. Conteúdo-modelo

Categorias de conteúdo usadas:

`flow` · `phrasing` · `sectioning` (`section`, `article`, `aside`, `nav`) · `heading` (`h1..h6`) ·
`form` · `table` · `media` · `metadata` · `interactive` (`a`, `button`, `input`, `select`, `textarea`, `details`, `summary`, `dialog`) ·
`text` (elementos que só aceitam texto: `title`, `option`, `textarea`, `desc`, `svg_title`, `iframe`) ·
`svg_shape` (formas SVG: `path`, `circle`, `rect`, `line`, `polyline`, `polygon`, `g`, `use`) · `json` (só o `script` JSON-LD) · `none` (só em `void`)

O conteúdo textual é o **default** de qualquer `children`: um elemento sem `children` declarado aceita
só texto, nunca marcação arbitrária.

Regras verificadas no macro (dentro de um bloco):

- `<p>` não pode conter `flow` (só `phrasing`) — pega o clássico `<div>` dentro de `<p>`;
- `<ul>`/`<ol>` só aceitam `li`;
- `<table>` não aceita texto solto nem `tr` direto; `<tr>` só em `thead`/`tbody`/`tfoot`; `colgroup` antes
  do primeiro `tr`, `col` só em `colgroup`, `caption` como `first_child`;
- `<a>` não aninha `interactive`;
- `<button>` não aninha `interactive`;
- `<label>` não aninha outro `label`;
- `<picture>` exige `source*` seguido de `img`.

## 5. Atributos nunca permitidos (em nenhum elemento)

| Atributo | Motivo |
|---|---|
| `style` | CSP sem inline; estilo é token ([`06`](06-shell-e-tema.md)) |
| `on*` (`onclick`, `onload`, …) | CSP; comportamento é arquivo JS declarado ([`07`](07-js-e-assets.md)) |
| `srcdoc` | HTML aninhado como string — bypass de escape |
| `@*` (Alpine) | framework cliente não faz parte do vocabulário |
| `hx-on*`, `x-on:*` | exigem `eval`; quebram CSP `script-src 'self'` |
| `hx-*` (qualquer) | fora do vocabulário no v1 (D18); enhancement de fragmento tem contrato próprio em [`07`](07-js-e-assets.md) §7 |
| `formaction` | sobrepõe o destino do `<form>` no submit — destino passa a ser invisível na leitura |
| `ping` | requisições laterais silenciosas (perfilamento) |
| `slot`, `part`, `exportparts`, `shadowrootmode`, `is` | customizado/Web Components — fora do vocabulário |
| `xlink:href` | forma legada; em SVG o atributo é `href` |
| `accesskey` | conflito com atalhos do navegador/leitor de tela |
| `draggable` | drag-and-drop nativo é inacessível sem implementação extra |

## 6. `aria-*` e `data-*` não são texto livre

- **`aria-*`**: allowlist global (o conjunto de atributos ARIA realmente usados), com regras
  adicionais por elemento:
  - `aria-label`/`aria-labelledby` só em landmark, widget ou região nomeável;
  - `aria-hidden="true"` não pode estar em elemento focável;
  - `role` só existe como **enum por elemento** (nunca string livre); repetir o papel implícito é erro;
  - `aria-checked`, `aria-expanded`, `aria-current` só onde o papel permite.
- **`aria-label` é `Msg`, nunca `&str`**: é o último ponto onde texto visível entra no markup, e texto
  sem chave quebra a paridade dos 4 locales ([`10`](10-ergonomia.md) §8). `aria-label` sobre texto
  visível é erro (`no_aria_label_over_visible_text`).
- **`data-*`**: cada atributo é declarado no vocabulário do consumidor com valores enumerados
  (`data-a11y-contrast="high|low"`, `data-mask="cpf|cnpj|phone"`). O JS é genérico e lê o valor; o
  markup não inventa chaves.

## 7. Geração

`build.rs` de `html-spec` lê `spec/elements.toml` e gera:

```rust
pub fn img(src: Src, alt: Alt<'_>, width?: Option<WidthPx>, height?: Option<HeightPx>, …) -> Markup;
pub fn a(href: Href, children: impl Render) -> Markup;
```

- Nome da função = nome do elemento.
- Parâmetros obrigatórios primeiro, opcionais em seguida (todos `Option`, emitidos só quando `Some`).
- Ordem de emissão dos atributos é a ordem **declarada no TOML** (determinismo de bytes).
- Elementos `void` não recebem `children`.
- Qualquer elemento sem entrada na spec gera erro de compilação claro:

```
erro: elemento `blink` não existe no vocabulário html-auy
  = nota: veja spec/elements.toml ou proponha a inclusão (CONTRIBUTING.md §3)
```

## 8. Como adicionar um elemento (governança)

1. PR em `spec/elements.toml` com `category`, `required`, `allowed`, `children`.
2. Entrada em `docs/13-decisoes.md` com o motivo e o caso de uso real.
3. Teste de conformidade: fixture renderizada + golden.
4. `tools/gen_spec.py` regenera `crates/html/src/spec/table.rs` (commitado); o teste de paridade e
   `tools/validate_spec.py` validam (**D27**).

Não existe caminho alternativo. Se o elemento não está na spec, não dá para renderizá-lo — e é isso
que impede o vocabulário de virar "HTML inteiro com nomes bonitos".
