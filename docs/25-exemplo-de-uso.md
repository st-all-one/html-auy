# 25 — Como é usar o crate (demonstração teórica)

> **Critério de aceite de leitura de F0–F6.** Se o uso abaixo não compilar, não gerar os bytes
> mostrados e não falhar onde está dito, a fase correspondente não está pronta. O núcleo já está
> implementado; as extensões de formulário e mídia desta rodada aparecem em §5 e §5.1. Ele também é o
> "cheat sheet de 12 linhas" prometido em [`10`](10-ergonomia.md) §1, na versão longa.

## 1. A cena

Um portal municipal (o consumidor real: 222 views, 4 locales, WCAG 2.2 AA, CSP sem inline, funciona sem
JavaScript, `ETag`/304). Três telas:

| Tela | O que ela exercita |
|---|---|
| `/noticias` | lista paginada, imagem responsiva, data localizada, estado vazio |
| `/noticias/{slug}` | shell completo (`Page`), JSON-LD, HTML vindo do banco (texto não confiável) |
| `/solicitacoes/nova` | `Form` + CSRF, 3 `Field` com erro de validação, `POST` → `303` |

## 2. O que o consumidor escreve antes de qualquer view

```rust
// app/src/msg.rs — enum do consumidor; o crate só conhece o trait (D31)
#[derive(Clone, Copy)]
pub enum Msg { Home, News, LatestNews, ReadNews, Title, Required, InvalidEmail, Submit, EmptyList, BackToTop }

impl html::Msg for Msg {
    fn as_str(self) -> &'static str {
        match self {
            Msg::LatestNews => t!("news.latest"),      // `t!` é do consumidor: lê o catálogo do locale
            Msg::Required   => t!("form.required"),
            /* … */
        }
    }
}
```

```rust
// app/src/http.rs — 6 linhas, o crate não participa (docs/06 §1)
impl axum::response::IntoResponse for Responder {
    fn into_response(self) -> axum::response::Response {
        let etag = format!("\"{}\"", hex(sha256(self.page.markup().as_str())));
        (self.status, [("etag", etag), ("content-type", "text/html; charset=utf-8")],
         self.page.markup().as_str().to_owned()).into_response()
    }
}
```

## 3. A view de lista

```rust
use html::{html, Markup, Href, LUrl, Src, SrcSet, Sizes, Alt, WidthPx, HeightPx, Class, Text, Timestamp};
use crate::token::Token;   // enum do consumidor, gerado de `tokens.css` (ClassToken)

// helpers do consumidor (C2): o crate recebe `LUrl`; a tabela de rotas é do app
fn noticia(slug: &str) -> LUrl<'static> {
    LUrl::with_segments("/noticias", &[slug]).expect("slug do banco")
}
fn noticias() -> LUrl<'static> {
    LUrl::new("/noticias").expect("rota fixa")
}

pub fn news_list(data: &NewsList) -> Markup {
    html! {
        section class=(Class::of(&[Token::NewsList])) {
            h2 { Msg::LatestNews }

            @if data.items.is_empty() {
                p { Msg::EmptyList }
            } @else {
                ul class=(Class::of(&[Token::Cards])) {
                    for item in &data.items {
                        li {
                            a href=(Href::internal(noticia(&item.slug))) {
                                img src=(Src::media(&item.cover))
                                    srcset=(SrcSet::from_variants(&item.variants))
                                    sizes=(Sizes::new("(min-width: 48em) 33vw, 100vw"))
                                    alt=(Alt::new(&item.cover_alt))
                                    width=(WidthPx::new(720)) height=(HeightPx::new(480));
                                h3 { Text::new(&item.title) }
                                time datetime=(Timestamp::new(item.published_at, item.tz)) {
                                    Text::new(&item.date_label)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

O que já está decidido **pela forma** desta função:

- `item.title` vem do banco como `String`; `Text::new(&item.title)` é o **único** caminho para dentro do
  HTML, e ele escapa. Não existe `Markup::from(String)`.
- `alt` não tem default: `Alt::new` ou `Alt::decorative()`, escolhido pelo autor.
- `width`/`height` são `u32 > 0` — `CLS 0` deixa de ser recomendação.
- `item.cover` implementa `MediaPath`, cujo `path()` devolve `&LUrl`: `Src` não aceita `&str`.
- `SrcSet` ordena por largura e deduplica (bytes determinísticos — [`13`](13-decisoes.md) D14).
- `h2` da seção; neste exemplo o `h1` é do `Page`. O macro também aceita `h1` para shells próprios; o `audit` cobra `single_h1` na saída.

## 4. A view de detalhe — o shell

```rust
pub fn news_detail(ctx: &Ctx, n: &News) -> Page {
    let view = View::new(
        html! {
            article class=(Class::of(&[Token::Article])) {
                h2 { Text::new(&n.title) }
                p { time datetime=(Timestamp::new(n.published_at, n.tz)) { Text::new(&n.date_label) } }
                div class=(Class::of(&[Token::Body])) { n.body.render() }   // `Markup` do RichText (ADR-028 do consumidor)
                a href=(Href::internal(noticias())) { Msg::BackToTop }
            }
        },
    );

    Page::new(&shell(ctx, Msg::Title, Msg::News, n.schemas()), view)
}

// helper do consumidor — o crate recebe DADOS, não conhece locale, tema nem request
fn shell(ctx: &Ctx, title: impl html::Msg, heading: impl html::Msg, schemas: Vec<JsonLd>) -> ShellData<'static> {
    ShellData::new(
        Text::msg(title),      // NÃO opcional
        Text::msg(heading),    // NÃO opcional → `<h1>` único por construção
        ctx.locale,            // tipo `Lang`, não string
    )
    .schemas(schemas)          // JSON-LD entra no `<head>` na posição fixa (docs/06 §1)
    .assets(ctx.assets())
    .skip(Text::msg(Msg::SkipToContent))   // skip link do shell (SC 2.4.1); sem texto, não emite
}
```

`ArticleBody::render()` (primitivo do módulo) devolve `Markup` — é assim que o crate compõe sem
introduzir um runtime de template.

## 5. O formulário

```rust
pub fn solicitacao_form(ctx: &Ctx, f: &FormState) -> Page {
    let view = View::new(
        html! {
            h2 { Msg::Title }
            Form::open(&Action::new(solicitacoes_nova()), Method::Post(ctx.csrf())) {
                Field::text(FieldName::new("nome"), Msg::Title, f.field("nome"))
                Field::input(FieldName::new("email"), Msg::InvalidEmail, InputType::Email,
                             &f.field("email").clone().autocomplete(Autocomplete::parse("email").expect("token")))
                Field::input(FieldName::new("nascimento"), Msg::Title, InputType::Date, f.field("nascimento"))
                Field::textarea(FieldName::new("descricao"), Msg::Required, f.field("descricao"))
                Field::select(FieldName::new("assunto"), Msg::Title, &assuntos(), f.field("assunto"))
                    .expect("opções do catálogo")
                Field::file(FieldName::new("anexo"), Msg::Title, Some(AttrVal::new("image/png, application/pdf")),
                            true, f.field("anexo"))
                Field::checkbox(FieldName::new("aceite"), Msg::Required, &f.field("aceite").clone().checked(true))
                button type="submit" { Msg::Submit }
            }
        },
    );
    Page::new(&shell(ctx, Msg::Title, Msg::Title, vec![]), view)
}
```

- `<form>` **não** está no vocabulário de `html!` (`only = "component"`): o único jeito de emitir um é
  `Form::open`. O token mora no **método** (`Method::Post(token)`): esquecer o CSRF não é omissão — é
  erro de compilação, e `Method::Get` não pede token (busca não inventa CSRF).
- `Form::open` emite `<form>`, o `_token` e, para `Put`/`Patch`/`Delete`, o `_method` oculto. Os filhos
  entre `{ … }` são passados como `Markup` (o mesmo vale para qualquer primitivo do consumidor).
- `Field::text`/`input`/`textarea`/`select`/`file` derivam `id`, `for`, `aria-describedby` (com os
  `<span>` de ajuda/erro), `aria-invalid`, `required` e o `*` fora do `<label>`. O autor não escreve
  `aria-*` de campo. `input` cobre os 14 tipos nativos; `file` não emite `value`; `select` devolve
  `None` sem opção selecionável. `autocomplete` só é emitido a partir de um token validado.
- `Field::checkbox_group`/`radio_group` emitem `<fieldset>`+`<legend>`, ids únicos por opção,
  `name[]` só no checkbox e `aria-describedby`/`aria-invalid`/`aria-required` no grupo; `None` quando
  todas as opções estão `disabled`.
- Para mídia, os builders de `media` substituem a construção manual quando o dado é dinâmico:
- `f.field("email")` devolve `&FieldState` vindo de `FormState::from_request` — `old()` e `$errors` do
  Blade deixam de ser 302 leituras espalhadas.

### 5.1 Mídia como dado

```rust
let cover = Img::new(
    Src::media(&item.cover),
    Alt::new(&item.cover_alt),
    WidthPx::new(720),
    HeightPx::new(480),
)
.responsive(SrcSet::from_variants(&item.variants), Sizes::new("(min-width: 48em) 33vw, 100vw"))
.lazy();

let video = Video::new(
    Src::parse("/media/aula.mp4").expect("asset"),
    Track::captions(Src::parse("/media/aula.vtt").expect("legenda"), Text::new("Legendas")),
)
.poster(Src::parse("/media/aula.jpg").expect("poster"))
.preload(MediaPreload::Metadata)
.dimensions(WidthPx::new(1280), HeightPx::new(720));
```

`Img` exige `alt` e as duas dimensões; `srcset`/`sizes` só saem juntos. `Picture` preserva a ordem
dos `Source` e exige o `Img` de fallback. `Video::new` exige a faixa de legendas; `Audio` e `Video`
aceitam `Track` adicionais.

## 6. O que o compilador recusa

Quatro mensagens **normativas** (todas com `trybuild` em [`10`](10-ergonomia.md) §2):

```
error: `img` exige `alt`
   = regra: WCAG 1.1.1 / spec:element.img
   = ajuda: `Alt::new("Descrição objetiva")` ou `Alt::decorative()`

error: atributo desconhecido `hreff` em `p`
   = ajuda: atributos válidos em `p`: class, id

error: `<div>` não é permitido dentro de `<p>`
   = regra: conteúdo-modelo (spec:element.p)
   = ajuda: use `<span>` (phrasing) ou mova o `<div>` para fora do `<p>`

error: `label for="nome"` não tem `id="nome"` no mesmo bloco
   = regra: WCAG 1.3.1
   = ajuda: use `Field::text(FieldName::new("nome"), Msg::Name, &state)` (gera label+id+aria) ou envolva o controle
```

E as duas que a própria arquitetura exige (`shell_only_elements`, `form_requires_csrf`), no mesmo formato:

```
error: `<main>` só existe no `Page`
   = regra: spec:shell_only_elements
   = ajuda: use `ShellData::new(Text::msg(Msg::Title), heading, lang)` — o `<title>`, `<html>`, `<head>` e `<main>` são do shell

error: `<form>` não é elemento de `html!`
   = regra: spec:form_requires_csrf
   = ajuda: use `Form::open(&action, Method::Post(csrf), children)`
```

## 7. A saída

O fragmento de §3, com 2 itens, produz (uma linha, sem indentação — a marcação do fonte é irrelevante):

```html
<section class="news-list"><h2>Últimas notícias</h2><ul class="cards"><li><a href="/noticias/a-frota-nova"><img src="/media/abc.avif" srcset="/media/abc-360.avif 360w, /media/abc-720.avif 720w, /media/abc-1200.avif 1200w" sizes="(min-width: 48em) 33vw, 100vw" alt="Ônibus estacionados no pátio" width="720" height="480"><h3>Prefeitura entrega a frota nova</h3><time datetime="2026-11-05T09:00:00-03:00">5 de novembro de 2026</time></a></li>…
```

Propriedades que o consumidor passa a ter **por construção**:

| Propriedade | Como |
|---|---|
| `ETag` estável | a mesma entrada gera os mesmos bytes (ordem canônica de atributos, `srcset` ordenado, sem hash) |
| CSP sem `inline` | não há `style=`, `on*` nem `<script>` inline no vocabulário |
| prefixo do `<head>` ≤ 14 KB | ordem fixa de `docs/06` §1; `Link`/`preload` só do que está no caminho crítico |
| HTML válido | o aninhamento é validado antes de gerar código |
| sem `FOUC` | CSS antes de `script type="module"` |
| funciona sem JS | nada no vocabulário depende de JS para render |

## 8. O teste que roda no CI do consumidor

```rust
// app/tests/a11y_pages.rs — `html = { path = "…", features = ["audit"] }` (D30)
#[test]
fn paginas_publicas_passam_no_n4() {
    for page in render_all_fixtures() {
        let findings = html::audit::check(page.markup().as_str());
        assert!(findings.is_empty(), "{:#?}", findings);
    }
}
```

Três linhas, sem binário, sem crate extra — e é exatamente o que o `M3`
([`16`](16-roadmap.md) §2) exige medir: defeitos **reais**, no HTML que o consumidor gera.

## 9. O que não existe (e por isso não se escreve)

| Não existe | Consequência |
|---|---|
| `Markup::from(String)` / `From<&str>` / `Display` / `Deref` | um `String` de fora não vira HTML |
| `style=` / `on*` | CSP sem `inline` é possível; não há handler inline |
| `Raw` / `PreEscaped` / `\|safe` | o bypass de escape não é proibido: é inexprimível |
| `render_template(path)` / template em runtime | a view é função Rust, compilada |
| `class="string livre"` | `class="a b"` é aceito como lista **estática** e canonicalizado; para o vocabulário tipado, `Class::of(&[Token::…])` + `extend` |
| `<html>`, `<head>`, `<body>`, `<title>`, `<h1>` em `html!` | só o `Page` os emite |
| `<form>` em `html!` | só `Form::open` com `CsrfToken` |
| locale dentro do crate | `Msg` é trait; quem resolve é o consumidor |

## 10. Custo esperado

| Medida | Alvo | Onde está |
|---|---|---|
| linhas da view vs Blade equivalente | ≤ **1,4×** sem macro (F0); **menos** com `html!` (F5/F6) | `M1`/`M2` de [`16`](16-roadmap.md) |
| `cargo check` por `html!` | imperceptível em blocos < 200 linhas | [`04`](04-validacao.md) §8 |
| `audit` por página | ~5–15 ms | [`04`](04-validacao.md) §8 |
| bytes por página | orçamento versionado no consumidor | [`09`](09-desempenho-e-cache.md) §6 |

## 11. As lacunas que este documento fechou

Escrever o uso final obrigou a decidir três pontos que estavam **inconsistentes entre documentos**
(o corpus mostrava três gramáticas diferentes):

| Ponto | Antes | Agora (normativo em [`21`](21-implementacao.md) F2) |
|---|---|---|
| valor de atributo | `div(class: …)` em `docs/02`, `href = expr` em F2, `.list` em `docs/01` | `nome=(expr)` para cálculo; `nome="literal"` para enum da spec e para `class`; `href`/`src`/`alt` continuam tipados e texto visível literal não é aceito. Os parênteses **delimitam** a expressão no `TokenStream` (`href=x rel=y` é ambíguo) |
| elemento vazio | sem terminador definido | `img src=(…) alt=(…);` — o `;` fecha o `void` |
| `class` | `Class::of(&[Tone])` + atalho `.token` | `Class::of(&[Token::…])` com `trait ClassToken` ou `class="a b"` estático; ambos ordenam e deduplicam |

O que **continua** sendo trabalho de F2: os erros de sintaxe com span (tag não fechada, atributo sem
valor, texto depois de `}`) e o `trybuild` de cada um.
