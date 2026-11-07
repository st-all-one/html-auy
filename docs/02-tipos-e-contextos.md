# 02 — Tipos e contextos de escape

## 1. `Markup` — o único renderizável

```rust
pub struct Markup(String);          // construtor privado ao crate
```

Invariantes:

- Só é construído por (a) `html!`, (b) `Markup::from_render(impl Render)`, (c) `html-core` interno.
- **Não** implementa `From<String>`, `From<&str>`, `Display` público nem `Deref<Target = String>`.
- `Markup` é o único tipo que o shell aceita como conteúdo.

Consequência: um `String` que chega de fora (banco, usuário) não vira HTML sem passar por um valor
tipado de contexto. O bypass de escape não existe — não é proibido, é impossível.

### Streaming

```rust
pub trait Render {
    fn render(&self) -> Markup;                  // aloca uma vez
    fn render_to(&self, out: &mut String);       // sem alocação intermediária
}
```

Todo valor tipado implementa `Render`. `render_to` existe para o caso de páginas grandes (listas de
milhares de linhas) e para reuso de buffer no shell.

## 2. Valores tipados: contexto no tipo

O escape **não** é escolhido no ponto de inserção; é escolhido na construção do valor. Cada contexto
tem **um** tipo, com **um** construtor seguro e nenhum construtor cru.

| Tipo | Contexto | Construção | Regras |
|---|---|---|---|
| `Text<'a>` | texto de conteúdo | `Text::new(&str)` · `Text::msg(Msg)` | escapa `&` `<` `>` `"` `'` e NUL; **nunca** vira markup. `&str`/`String` também são `Render` escapado, e o literal `"…"` no `html!` vira esse caminho |
| `AttrVal<'a>` | valor de atributo comum | `AttrVal::new` | escapa os 5 + `\r` `\n` `\t` como entidades numéricas |
| `Href` | link | `Href::internal(LUrl)` · `Href::fragment(Fragment)` · `Href::external(https)` · `Href::mailto` · `Href::tel` | **recusa** `javascript:`, `data:`, `vbscript:`; normaliza esquema em minúsculas |
| `LUrl` | rota interna | `LUrl::new("/…")` · `LUrl::with_segments(base, &[seg])` | começa com `/`, sem `//`, sem `:`, sem `..` (nem `%2e%2e`); o locale/rota é helper do consumidor (C2) |
| `Src` | URL de recurso | `Src::media(&impl MediaPath)` · `Src::asset(Asset)` · `Src::parse(&str)` | interno validado (`LUrl`) ou `http(s)` absoluto; `javascript:`/`data:` não constroem |
| `SrcSet` | conjunto responsivo | `SrcSet::from_variants(&[Variant])` | `src` interno validado ou `http(s)` absoluto; ordena por largura, deduplica e emite `Nw`; **determinístico** |
| `Sizes<'a>` | tamanho de exibição | `Sizes::new(&str)` validado | gramática `(condition) length`, com `length = <n><unidade> \| auto \| calc(…) \| min(…) \| max(…) \| clamp(a, b, c)` |
| `Media<'a>` | `link media` / preload condicional | `Media::print()`, `Media::new(&str)` | vazia/inválida não emite o atributo |
| `OpenGraphKind` | `og:type` | `OpenGraphKind::{Website, Article, Product}` | enum fechado; `Website` é o padrão |
| `Loading` | `loading` de imagem | `Loading::{Eager, Lazy}` | presença explícita; `Img::lazy()` é o atalho |
| `MediaPreload` | `preload` de vídeo/áudio | `MediaPreload::{None, Metadata, Auto}` | enum fechado |
| `TrackKind` | `kind` de `<track>` | `TrackKind::{Subtitles, Captions, Descriptions, Chapters, Metadata}` | `subtitles` exige `srclang` (`Track::subtitles`) |
| `Integrity<'a>` | SRI de preload/stylesheet | `Integrity::parse(&str)` | só `sha256/384/512` + base64; o resto não constrói |
| `JsonLd` | dado em `<script type="application/ld+json">` | `JsonLd::graph(&Graph)` | escapa `<` `>` `&` `'` como `\u003c` `\u003e` `\u0026` `\u0027` |
| `Alt<'a>` | texto alternativo | `Alt::new(&str)` · `Alt::decorative()` | `decorative()` é explícito; `new("")` equivale a `decorative()` |
| `Class` | classes CSS | `Class::of(&[Token])` · `Class::new("a b")` · `class="a b"` | divide, ordena e deduplica; o enum de tokens continua disponível para o consumidor |
| `Tone` | cor por identidade | `Tone::Primary` … (enum) | nunca hex; ver [`06-shell-e-tema.md`](06-shell-e-tema.md) |
| `SectionId` | identidade de seção | enum por módulo | chave do mapa de tokens |
| `WidthPx`/`HeightPx` | dimensão | `WidthPx::new(u32)` | > 0; emite `width`/`height` como número |
| `Timestamp` | `<time datetime>` | `Timestamp::new(i64, offset)` | `RFC 3339` com offset preservado |
| `Label<'a>` | nome acessível | `Label::text(m)` · `Label::sr_only(m)` — `m: impl Msg` | não pode ser vazio |

## 3. Escaping: tabela normativa

| Contexto | Caracteres escapados | Forma |
|---|---|---|
| texto | `& < > " '` | `&amp; &lt; &gt; &quot; &#x27;` |
| valor de atributo | os 5 + `\r \n \t` | os 5 + `&#13; &#10; &#9;` |
| JSON-LD | `< > & '` (e aspa não é problema) | `\u003c \u003e \u0026 \u0027` |
| URL (query) | percent-encoding por segmento | `%XX` maiúsculo |
| `class` | nada (tokens já validados) | — |
| comentário HTML | não existe | — |

Nunca há escape duplo: `Text` já sai escapado; inserir `Text` dentro de `Text` é erro de tipo.

Texto e atributo têm escapes **distintos**: o texto escapa `&` `<` `>` `"` `'`; o valor de atributo
escapa os mesmos cinco e converte `\r`/`\n`/`\t` em entidades numéricas; o `JsonLd` escapa `<` `>` `&`
`'` e U+2028/U+2029 como `\uXXXX`. Um escape único "para todos os contextos" é mais curto e erra o
caso que importa (`"` fechando atributo).

`/` **não** é escapado, ao contrário do Jinja/Askama (`&#x2f;`): dentro de texto ou atributo a barra
não fecha nada, e a única sequência perigosa — `</script>` dentro de JSON — é fechada pelo escape do
`JsonLd` (que trata `/` explicitamente). Escapar `/` em todo texto custa bytes e não protege nada.

## 4. Atributo opcional (o que o Blade fazia com `@if (filled(...))`)

```rust
html! { img(src: s, alt: a, width?: maybe_w, height?: maybe_h); }
```

`attr?: Option<T>` significa: **se `None`, o atributo não é emitido**; se `Some`, é emitido com o
escape do tipo de `T`. Não existe "atributo com string vazia" — o caso `""` é sempre um bug de
intenção, e o tipo força a escolha.

## 5. Classes e merge

```rust
// no CRATE: o contrato. `Tone` (as cores semânticas) é do crate e implementa o trait.
pub trait ClassToken: Copy { fn as_str(self) -> &'static str; }
impl Class { pub fn of<T: ClassToken>(t: &[T]) -> Self; pub fn extend(self, o: Class) -> Self; }

// no CONSUMIDOR: o enum é gerado de `tokens.css` (D29: tema é dado do consumidor)
html! { div class=(Class::of(&[Token::Card, Token::Flat])) { … } }
```

- `ClassToken` segue o mesmo desenho de `Msg` (D31): **o crate não conhece o vocabulário do consumidor**,
  só o contrato. `Tone` (primária/secundária/…) é do crate porque é identidade visual da biblioteca.
- Merge é feito em `Class`, com deduplicação e ordem estável (bytes determinísticos).
- `class="a b"` (literal estático) é aceito e canonicalizado no codegen; `Class::new("a b")`
  faz o mesmo em runtime. `Class::of` continua sendo o caminho do enum gerado de `tokens.css`.
- O que não existe é `class` vindo de dado não estático: a lista é `&'static str` ou `Class`; o
  escape e a ordem são do crate.

## 6. Texto, i18n e nomes acessíveis

```rust
// no CRATE: só o contrato. Nenhum locale, nenhum catálogo, nenhum global.
pub trait Msg: Copy { fn as_str(self) -> &'static str; }
impl Text<'static> { pub fn msg<M: Msg>(m: M) -> Self; }

// no CONSUMIDOR: o enum dele, resolvido no locale da request
//   enum Msg { NavHome, LatestNews, … }
//   impl html::Msg for Msg { fn as_str(self) -> &'static str { … } }
```

- Texto visível pode ser `Msg` (i18n), `Text::new(&dado)` (dinâmico) ou o literal `"Olá"` no
  `html!` (escapado). `Msg` continua sendo o caminho quando há catálogo/tradução.
- `Label::text(m)` e `Label::sr_only(m)` recebem `M: Msg` e produzem nomes acessíveis traduzidos.
- A paridade de tradução é gate **do consumidor**: variante sem string nos 4 locales = erro lá.
- **Por que trait e não `enum Msg` no crate (D31):** o crate não pode resolver locale — isso exige
  estado ambiente, que o rigor proíbe ([`22`](22-rigor.md) §3). E `impl<M: Msg> From<M> for Text` está
  fora: colidiria com `From<&str>` se o consumidor implementasse `Msg` para `&str`.

Ganho colateral: o lint de "link ambíguo" (WCAG 2.4.4) opera **por variante de `Msg`**, não por
comparação de string.

## 7. Números, datas e listas

| Caso | Tipo | Regra |
|---|---|---|
| largura/altura de imagem | `WidthPx`/`HeightPx` | inteiro > 0, sem unidade no valor |
| `datetime` (perfil *instante*) | `Timestamp` | RFC 3339 **com offset** (`Z` ou `+03:00`); nunca sem offset nem em UTC forçado — o fuso local é do dado |
| `datetime` (perfis *data*, *mês*, *semana*, *hora*, *duração*) | `DateOnly`/`YearMonth`/`Week`/`TimeOfDay`/`Duration` | um tipo por perfil do `<time datetime>`; converter entre perfis é erro de compilação |
| contagem/estatística | `Count` | formatação localizada, agrupamento determinístico |
| dinheiro | `Money` | inteiro em centavos + `Msg` de moeda; **nunca** `f64` |
| lista de `srcset` | `SrcSet` | largura crescente, tipo MIME em ordem lexical |

## 7.1 Atributo booleano

O tipo é `bool`, a emissão é presença: `<button disabled>`. Nada de `disabled="false"` — em HTML esse
valor é **verdadeiro**, e é a origem do clássico "o botão continua desabilitado". A lista fechada vive
em `spec/elements.toml` → `[meta] boolean_attributes`; atributo booleano fora dela não compila
([`13`](13-decisoes.md) D19).

## 8. O que não existe (por decisão)

| Ausente | Motivo |
|---|---|
| `Raw`, `PreEscaped`, `unsafe_html` | bypass de escape é o defeito que o projeto elimina |
| `style=` | CSP sem inline; estilo é token |
| `on*` (event handlers) | CSP; comportamento é arquivo JS declarado |
| atributo arbitrário `attrs(…)` | validação que aceita "qualquer coisa" não valida |
| `String` como conteúdo | quebra o invariante de `Markup` |
| `f64` para dinheiro | precisão e determinismo |

## 9. Erros de tipo: mensagens (todas cobertas por `trybuild`)

```
erro: `img` exige `alt`
  --> src/views/news.rs:12:9
   = ajuda: use `Alt::new("Descrição objetiva da imagem")`
            ou `Alt::decorative()` se a imagem é puramente decorativa

erro: valor de `href` não permitido: `javascript:`
   = ajuda: use `Href::internal(LUrl::…)` para rotas do site,
            `Href::external("https://…")` para fora, `Href::mailto`/`Href::tel`

erro: atributo desconhecido `hreff` em `a`
   = nota: atributos válidos em `a`: href, hreflang, rel, target, download, referrerpolicy
```
