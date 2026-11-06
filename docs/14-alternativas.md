# 14 — Alternativas avaliadas

## 1. O quadrante que ninguém ocupa

| | props tipadas | HTML é o artefato (sem runtime) | atributo intrínseco validado |
|---|---|---|---|
| maud / horrorshow / markup.rs | ✅ (fn) | ✅ | ❌ |
| askama / sailfish | ✅ (struct) | ⚠️ (ver §2) | ❌ |
| topcoat | ✅ | ✅ (server) | ❌ |
| leptos / yew / dioxus / sycamore | ✅ | ❌ (projeção) | ❌ |
| minijinja / tera / handlebars | ❌ (`Value`) | ✅ | ❌ |
| **html-auy** | ✅ | ✅ | **✅** |

Nenhum framework Rust valida atributo intrínseco: `hreff`, `aria-lable`, `altt` compilam em todos.
Nenhum separa "props tipadas" de "reatividade": props tipadas vêm sempre junto com um runtime de
árvore ou com um template de arquivo.

## 2. Comparação detalhada

| Critério | maud | askama | sailfish | leptos | yew | dioxus | sycamore | topcoat | minijinja/tera | **html-auy** |
|---|---|---|---|---|---|---|---|---|---|---|
| HTML final = artefato | ✅ | ✅ | ✅ | ⚠️ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Erro de marcação em compilação | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Atributo intrínseco validado | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Escape por contexto | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ⚠️ (`tojson`) | ✅ |
| Bypass de escape possível | ✅ (`PreEscaped`) | ✅ (`\|safe`) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ (`\|safe`) | ❌ |
| Props tipadas | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Script/estado no HTML | ❌ | ❌ | ❌ | ⚠️ | ✅ | ✅ | ✅ | ⚠️ | ❌ | ❌ |
| Funciona sem JS | ✅ | ✅ | ✅ | ⚠️ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Formatação | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ (`topcoat fmt`) | ✅ | ✅ (rustfmt; o macro ignora whitespace) |
| Maturidade | alta | alta | alta | alta | alta | alta | média | **experimental** | alta | projeto |
| Dependência do crate | nenhuma | 1 | 1 | grande | grande | grande | média | média | grande | 2 (`proc-macro2`,`quote`) |
| A11y embutida | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Invariante de shell/página | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |

## 2.1 Matriz de defeitos: o que cada abordagem deixa passar

Quinze defeitos que aparecem de verdade numa prefeitura, com o resultado de cada abordagem. "passa" =
compila, publica e o defeito chega ao cidadão.

| Defeito | Askama | JSX/TSX | `html!` | Quem recusa em `html!` |
|---|---|---|---|---|
| tag não fechada | recusa | recusa | recusa | parser |
| tag inexistente (`<dev>`) | passa | recusa | recusa | macro (vocabulário) |
| atributo inexistente (`hreff`) | passa | recusa | recusa + lista válida | macro (vocabulário) |
| aninhamento ilegal (`div` em `p`, `li` fora de lista) | passa | passa | recusa | macro (`children`) |
| `img` sem `alt` | passa | passa (`alt?: string`) | recusa | macro (`required`) |
| `img` sem `width`/`height` | passa | passa | recusa | macro + tipo `WidthPx`/`HeightPx` |
| `href="javascript:alert(1)"` | passa | passa (`href?: string`) | não é expressável | tipo `Href` |
| `class` montada de dado (`"btn {{x}}"`) | passa | passa | não é expressável | tipo `Class` (tokens fechados) |
| `style=` / `onclick="…"` | passa | passa (`style={{}}`, index signature) | não existe | vocabulário (não há atributo) |
| `aria-*`/`role` inválido ou contraditório | passa | passa (`string`) | recusa | macro (enums + invariantes) |
| `label` sem `for`/`id` pareado | passa | passa | recusa | macro (`Field::text` gera os dois) |
| link ambíguo ("leia mais") | passa | passa | recusa | macro (por variante de `Msg`) |
| página sem `title`, sem `main`, dois `h1` | passa | passa | não é expressável | **tipo `Page`** |
| ordem de atributos/classe não canônica (ETag instável) | passa | passa | impossível | codegen (ordem da spec) |
| escape desligável (`\|safe`, `dangerouslySetInnerHTML`) | passa | passa | não existe | vocabulário (`Markup` só do macro) |

**Leitura da matriz.** Askama entrega *um* item (fechamento de tag) e o escape automático com desligamento
opcional. JSX entrega dois (nome de tag e de prop, porque `JSX.IntrinsicElements` é uma tabela de tipos) e
falha em tudo que é **semântica de HTML** — porque os tipos vêm do DOM, que precisa aceitar `data-*`,
custom elements e `style` para ser compatível. `html!` entrega os quinze porque a única coisa que ele tem
a mais que os dois é **a tabela de elementos**, que nenhum deles controla.

## 2.2 Por que Askama e JSX não chegam lá sem virar isto

- **Askama** trata a marcação como fluxo de tokens: sabe onde as tags abrem e fecham, não sabe o que é um
  `img`. Para saber, precisa da tabela de elementos; com a tabela, precisa validar aninhamento e
  atributos; com a validação, precisa controlar o codegen para a ordem canônica; controlando o codegen, o
  template deixa de ser template. Não é esforço — é o produto de outra pessoa.
- **JSX** tem a tabela (`JSX.IntrinsicElements`), mas ela é do **DOM**, não de um contrato nosso: `alt` é
  opcional, `href` é `string`, `style` é objeto, e todo `data-*` é livre por especificação. Fechar o
  vocabulário é incompatível com ser React.

A conclusão não é "os dois são ruins": é que os dois resolvem **outro** problema (interpolar texto +
compor componentes). O problema declarado aqui é **não conseguir emitir HTML inválido** — e isso exige ser
dono do vocabulário.

## 2.3 Onde uma invariante nova vai parar (regra de três perguntas)

1. Depende só do bloco `html!`? → **`block`**: a macro recusa, com span.
2. Depende do documento, mas o documento é um **tipo**? → **`shell`**: torne inexpressável
   (`Page` recebe um `main`, um `h1`, um `title`).
3. Depende de fragmentos que nunca se veem? → **`output`**: `N4` sobre a saída. **Nunca** anunciada como
   garantia de compilação — é o erro que a revisão de 2026-11-06 corrigiu em 5 das 8 regras.

## 3. Vereditos

### maud — o mais próximo, e é o fallback
Ganha em simplicidade (macro, `Markup`, sem `|safe`). Perde em: atributo não validado, escape só de
HTML, sem formatação, sem a11y. **Veredito:** se `M1` do projeto falhar, a combinação "maud + camada
de valores tipados (~600 LOC)" resolve ~70% do problema com 10% do risco. É o plano B formal.

### askama / sailfish — quando o time é de HTML
Templates em arquivo, herança, macros com `caller()`, formatação intocada. Perde em: `|safe` como
porta, slot nomeado exigindo captura de string + `|safe`, falta de validação de atributo.
**Veredito:** a escolha certa se o gargalo for "gente de HTML/CSS não edita Rust". Nesse caso, aceite
o `|safe` com um grep-gate — mas com o custo na mesa: o corpus do askama promove `|safe` em quatro
lugares do próprio FAQ, rotula `{{ data | json | safe }}` dentro de `<script>` como "Bom" e mostra `|safe`
como **no-op** quando `ext = "txt"` (herança silenciosa: um template `.txt` que vira HTML não é
escapado). O "HTML é o artefato" só vale enquanto não há `|safe`; por isso a linha da tabela é ⚠️ e não ✅.

### leptos — a arquitetura certa para outro produto
Suporta MPA com `<form>` real e tem feature `nonce` para CSP. Mas: `hydrate`/`islands` injetam script e
estado serializado; sem eles, sobra uma máquina reativa gerando string. **Veredito:** incompatível com
"página cacheável + CSP sem inline + funciona sem JS". Não é questão de gosto, é de contrato.

### yew / dioxus / sycamore — VDOM/hidratação
`html!`/`rsx!`/`view!` produzem árvore; o HTML é consequência. `Properties` do yew é o mais próximo de
JSX/TS em tipagem de props. **Veredito:** descartados pelo mesmo motivo do leptos, com menos vantagem.

### topcoat — o híbrido mais promissor e o mais arriscado
`view!` renderiza no servidor (HTML real) com `$(…)` compilado para JS e `#[shard]`. Mas está em
"early-stage and experimental, expect breaking changes", e o roadmap aberto inclui auth, jobs, imagem,
validação e rate limit. **Veredito:** é plataforma, não camada de HTML; e o runtime JS conflita com o
contrato de zero-JS.

### horrorshow / markup.rs — experimentos úteis
Confirmam que macro → `Markup` funciona. Não trazem validação. **Veredito:** referência histórica.

### minijinja / tera / handlebars — a família Eta/EJS
Template em runtime sobre `Value`: zero tipagem, erro em request. **Ganho único:** minijinja tem
`|tojson` com escape para `<script>` (resolve JSON-LD corretamente) e template editável sem rebuild.
**Veredito:** o "editar template sem rebuild" é justamente o que o consumidor proíbe (conteúdo é dado;
apresentação é código).

### Handlebars/mustache puros
Mesmo do anterior, com menos recursos. **Veredito:** fora.

## 4. O que se perde ao não usar cada um

| Alternativa | Perda concreta assumida |
|---|---|
| maud | maturidade e simplicidade de código de um crate já testado por terceiros |
| askama | ergonomia de arquivo/formatação gratuita |
| leptos | reatividade e hidratação (que não são requisito) |
| topcoat | router por árvore de arquivos, assets, Fluent, mail (que são de plataforma, não de HTML) |
| minijinja | edição de template sem rebuild |

O projeto assume essas perdas em troca de: atributo validado, escape por contexto, a11y por
construção e HTML determinístico como contrato.

## 5. Quando reavaliar

Reavaliar a decisão se, e apenas se:

1. `M1` não bater o critério de linhas (ver [`16-roadmap.md`](16-roadmap.md)) → adotar maud;
2. o time de layout passar a ser majoritariamente não-Rust → adotar askama;
3. algum framework passar a validar atributo intrínseco por spec + escape por contexto → reavaliar
   custo/benefício de manter código próprio.
