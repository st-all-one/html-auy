[← Primeira página](02-primeira-pagina.md) · [Índice](README.md) · [Próximo →](04-valores-tipados.md)

# 03 — O macro `html!`

**Em uma frase.** `html!` é um bloco de marcação validado em tempo de compilação; ele devolve
`Markup`, nunca `String` e nunca um template em runtime.

## Forma básica

```rust
use html_auy::prelude::*;

fn card(title: &str, href: LUrl<'_>) -> Markup {
    html! {
        article class="card" data_kind="news" {
            h2 { Text::new(title) }
            a href=(Href::internal(href)) { M::Read }
        }
    }
}
```

Cada elemento usa `<tag atributo=valor …> { filhos }`; elementos vazios terminam em `;`:

```rust
html! {
    br;
    img src="/media/a.avif"
        alt=(Alt::new("Descrição"))
        width=(WidthPx::new(640)) height=(HeightPx::new(480));
}
```

## Texto, mensagens e composição

O macro distingue três entradas; a diferença é sintática e semântica:

| Fonte | Sintaxe | O que entra |
|---|---|---|
| mensagem traduzida | `p { M::Title }` | expressão `Msg`, escapada como texto |
| dado dinâmico | `p { Text::new(&title) }` | `Text`, escapado como texto |
| markup já pronto | `div { (card) }` | expressão entre parênteses, composta como `Render` |

```rust
html! {
    h2 { M::LatestNews }              // Msg
    p { Text::new(&item.summary) }    // String/&str dinâmico
    div { (article_body) }            // Markup de um componente
}
```

Um literal de texto **não** é permitido:

```rust
html! { p { "Olá" } }
// error: texto literal não é permitido; use uma variante de `Msg`
```

Para texto estático, use `Msg`; para texto que veio de fora, `Text::new`; para markup que já é
confiável, `(markup)`. A regra completa está em [Texto e i18n](05-texto-e-i18n.md).

## Atributos

### Literal estático

```rust
html! {
    a href="/noticias" { M::News }
    a href="#conteudo" { M::SkipToContent }
    img src="/media/a.avif" alt=(Alt::decorative()) width=(WidthPx::new(1)) height=(HeightPx::new(1));
    a href=(Href::external(url)) target="_blank" rel="external" { M::Site }
    button type="button" disabled=(true) { M::Close }
}
```

- `href` e `src` literais aceitam **caminho interno** (`/…`) e, no `href`, fragmento (`#…`). O macro
  valida o caminho; `href="javascript:…"`, `src="https://…"` e `..` são erro de compilação.
- URL externa continua tipada (`Href::external`/`Src::parse`): o literal não esconde a decisão.
- O nome do atributo é do vocabulário (`spec/elements.toml`); `hreff`, `onclick`, `srcdoc` e
  `hx-*` não compilam.
- `target="_blank"` sem `rel` ganha `rel="noopener noreferrer"`; se você declarar um `rel` literal
  sem `noopener`, o macro recusa.
- `disabled`, `checked`, `required`, `inert` e demais booleanos usam **presença**: `true` emite o
  atributo, `false` o omite. Não existe `disabled="false"`.

### Expressão tipada

```rust
use html_auy::Id;

let id: Option<Id<'_>> = Some(Id::new("relatorio"));
let href = Href::internal(LUrl::new("/relatorios/1").expect("rota"));

html! {
    a href=(href) { M::Open }
    section id?=(id) { … }
}
```

`atributo=(expr)` exige um tipo que implemente `AttrValue` (`Href`, `Src`, `Alt`, `Class`,
`ClassList`, `Timestamp`, `WidthPx`, …). `&str` e `String` **não** implementam; use `Text`,
`AttrVal` ou o tipo de contexto. `datetime` é a exceção tipada: aceita `Date` ou `Timestamp` via
`DateTimeValue`.

`atributo?=(Option<T>)` só emite quando o `Option` é `Some`. `None` não vira atributo vazio.

### Classes

```rust
html! {
    div class="card card--wide" { … }          // lista estática; ordenada e deduplicada
    div class=(Class::of(&[Tone::Primary])) { … }
    div class=(classes) { … }                  // ClassList dinâmica
}
```

`class="a b"` é aceito, mas é canonizado (ordem alfabética, sem duplicata). Para tokens do design
system, `Class::of`; para valores calculados, `ClassList::try_new`. `class` vazio não é emitido.

### `data-*` e `aria-*`

```rust
use html_auy::AttrVal;

html! {
    nav aria_label=(Label::text(M::MainNav)) {
        a href=(home) aria_current="page" { M::Home }
    }
    div data_kind="news" data_state=(AttrVal::new(&state)) { … }
}
```

- O identificador Rust usa `_`; o atributo HTML usa `-` (`aria_label` → `aria-label`,
  `data_kind` → `data-kind`).
- `aria_label` exige `Label` (`Label::text`/`Label::sr_only`), nunca `&str`; é o último ponto de
  texto visível no markup.
- `data-*` usa valores escalares; não serialize JSON no atributo. Nomes e valores são validados.
- `role` só existe onde a spec permite e com valor enumerado.

### `style=` inline

`style` é o **escape hatch** do vocabulário; existe, mas não é o caminho de apresentação:

```rust
html! {
    div style="margin-top: 1rem" { … }
}
```

O valor é escapado; `audit` reporta `no_style_attribute`, e a CSP precisa de `style-src-attr`.
Prefira `Class`/token. `<style>` não existe. Veja [Segurança](10-seguranca.md).

## Controle de fluxo

```rust
html! {
    @if items.is_empty() {
        p { M::Empty }
    } @else if loading {
        p { M::Loading }
    } @else {
        ul {
            for item in &items {
                li { a href=(Href::internal(route(&item.slug))) { Text::new(&item.title) } }
            }
        }
    }
}
```

- `@if`, `@else if`, `@else` e `for` são as construções do bloco.
- O `for` é Rust normal; `&items`, `items.iter().enumerate()` e padrões funcionam.
- Não há `@match`, `@let` ou template de runtime; use `match` fora do macro para escolher um
  `Markup`.
- `@if let Some(x) = …` é apenas uma condição Rust; funciona quando o `let` está no escopo.

## Primitivos e componentes

O crate tem componentes que devolvem `Markup` (`Form::open`, `Field::*`, `Img::render`,
`Video::render`, …). Eles entram como composição:

```rust
use html_auy::{CsrfToken, Field, FieldName, Form, Method};

let form = Form::open(&action, Method::Post(csrf), html! {
    (Field::text(FieldName::new("nome"), M::Name, &state))
    button type="submit" { M::Send }
});

let view = View::new(html! {
    section { h2 { M::Title } (form) }
});
```

`Form::open` também aceita a forma `Form::open(...) { filhos }` dentro do macro, mas a forma com
`html!` + `(form)` é mais legível e evita confundir o bloco do formulário com o bloco da view.

> **Vários `Markup` irmãos:** use `Markup::concat([a, b])`. `(a) (b)` é lido como a chamada
> `(a)(b)` e o erro aponta o `concat`.

## Elementos e aninhamento

O macro valida o conteúdo-modelo:

| Escrita | Resultado |
|---|---|
| `p { div { … } }` | erro: `flow` dentro de `phrasing` |
| `a { button { … } }` | erro: interativo dentro de interativo |
| `table { tr { … } }` | erro: `tr` só em `thead`/`tbody`/`tfoot` |
| `picture { img … }` | erro se faltar `source` ou se o `img` não for o fallback final |
| `select { option value="x" { … } }` | ok; `option` exige `value` |
| `button { … }` | erro se faltar `type` |
| `th { … }` | erro se faltar `scope` |
| `label for="x"` | erro se não houver `id="x"` no mesmo bloco |
| `main { … }` | erro: `main` é do `Page` |
| `form { … }` | erro: só `Form::open` |

`h1` é o caso especial: existe no vocabulário, mas no fluxo normal o `Page` já emite o `<h1>`.
Use `h2`–`h6` dentro da view.

## Saída canônica

O macro **ignora whitespace** e emite na ordem da spec:

```rust
html! {
    div data_zeta="z" data_alpha="a" data_mid="m" class="b a" { }
}
```

```html
<div class="a b" data-alpha="a" data-mid="m" data-zeta="z"></div>
```

- A ordem dos atributos não depende da ordem em que você os escreveu.
- `class` é ordenada/deduplicada.
- `data-*` é ordenado.
- `target="_blank"` pode sintetizar `rel`.
- `svg` sem nome acessível ganha `aria-hidden="true" focusable="false"`.
- `svg` shapes usam `/>`; elementos HTML vazios não.
- Não há formatação de markup: a saída é uma string contínua.

## Erros de compilação que você vai ver

| Mensagem (resumo) | Correção |
|---|---|
| `elemento 'blink' não existe no vocabulário` | use um elemento da spec |
| `atributo desconhecido 'hreff' em 'a'` | corrija o nome ou veja a lista válida na mensagem |
| `img exige alt` | `Alt::new("…")` ou `Alt::decorative()` |
| `img exige width/height` | `WidthPx::new(…)`/`HeightPx::new(…)` |
| `texto literal não é permitido` | `M::X` (estático) ou `Text::new(&valor)` (dinâmico) |
| `duas expressões (…) seguidas` | `Markup::concat([a, b])` |
| `href`/`src` literal inválido | `/caminho` interno ou forma tipada |
| `aria-label literal` | `aria_label=(Label::text(M::X))` |
| `button exige type` | `type="button"`, `"submit"` ou `"reset"` |
| `th exige scope` | `scope="col"`/`"row"` |
| `label for sem id` | use `Field::*` ou adicione o controle com id |
| `main só existe no Page` | remova `main`; o `Page` o emite |
| `form` | use `Form::open` com `CsrfToken` |
| `` `a` não pode conter interativo `` | reestruture o link/botão |

O texto exato e os spans são cobertos por `trybuild`; veja [Testes e diagnóstico](13-testes-e-diagnostico.md).

## O que não existe

| Não existe | Consequência |
|---|---|
| `Markup::from(String)` / `From<&str>` | um `String` de fora não vira HTML |
| `Raw` / `PreEscaped` / `\|safe` | bypass de escape não é proibido; é inexprimível |
| atributo arbitrário `attrs(…)` | validação que aceita tudo não valida |
| `on*` / `<style>` / script inline | comportamento é asset; estilo é token |
| template em runtime | a view é função Rust compilada |
| `@match` | use `match` para escolher um `Markup` |

---

[← Primeira página](02-primeira-pagina.md) · [Índice](README.md) · [Próximo →](04-valores-tipados.md)
