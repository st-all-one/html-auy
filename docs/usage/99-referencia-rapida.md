[← Arquitetura](15-arquitetura.md) · [Índice](README.md)

# 99 — Referência rápida

**Uma página para consultar assinatura, construção e limite.** Para explicação, siga os links.

## Importações

```rust
use html_auy::{
    html, Action, Alt, Asset, Assets, AttrVal, Autocomplete, Class, ClassList, ClassToken,
    CsrfToken, DataAttr, ExternalUrl, Field, FieldName, FieldState, Form, HeightPx, Href,
    Img, InputType, Integrity, JsonGraph, JsonLd, Label, Lang, LUrl, Markup, Media, Method,
    Msg, Offset, Page, Partial, Picture, Regions, Render, RootAttributes, SelectOption,
    ShellData, Sizes, Src, SrcSet, Swap, Text, Timestamp, Tone, Track, Variant, View,
    WidthPx, HtmxConfig, OpenGraph, OpenGraphKind, Hint, Preload,
};
```

## Página mínima

```rust
let data = ShellData::new(
    Text::msg(M::Title),
    Text::msg(M::Heading),
    Lang::parse("pt-BR").expect("lang"),
).skip(Text::msg(M::Skip));

let view = View::new(html! {
    p { M::Welcome }
});

let page = Page::new(&data, view);
let html = page.markup().as_str();
```

`Page` emite `<!DOCTYPE html>`, `<html>`, `<head>`, `<body>` e `<main>`; sempre um `<h1>`.

## Macro

| Construção | Sintaxe |
|---|---|
| elemento | `div { … }` |
| vazio | `br;` / `img …;` |
| mensagem | `p { M::Title }` |
| texto dinâmico | `p { Text::new(&s) }` |
| markup | `div { (markup) }` |
| atributo literal | `type="button"` |
| atributo tipado | `href=(Href::internal(url))` |
| opcional | `id?=(Option<Id>)` |
| booleano | `disabled=(true)` |
| classe | `class="a b"` / `class=(Class)` |
| dado | `data_kind="news"` |
| ARIA | `aria_label=(Label::text(M::X))` |
| condicional | `@if c { … } @else { … }` |
| loop | `for item in &items { … }` |
| componente | `(form)` / `(field)` (ambos são `Markup`) |

Não existem `@match`, `@let`, template runtime, `on*`, `<style>` nem `<form>` em `html!`.

## Valores

| Necessidade | Construção |
|---|---|
| texto | `Text::new(&s)` / `Text::msg(M)` |
| atributo | `AttrVal::new(&s)` |
| rota | `LUrl::new("/x")` / `LUrl::with_segments("/x", &["a b"])` |
| link | `Href::internal(LUrl)` / `fragment` / `external` / `mailto` / `tel` |
| URL externa | `ExternalUrl::parse("https://…")` |
| mídia | `Src::parse("/x")` / `Src::media(&obj)` / `Src::asset(&asset)` |
| `srcset` | `SrcSet::from_variants(&variants)` |
| `sizes` | `Sizes::new("(min-width: 48em) 50vw, 100vw")` |
| alt | `Alt::new("…")` / `Alt::decorative()` |
| dimensões | `WidthPx::new(720)` / `HeightPx::new(480)` |
| data | `Timestamp::new(unix, Offset::UTC)` |
| idioma | `Lang::parse("pt-BR")` |
| classes | `Class::of(&[Token::X])` / `Class::new("a b")` |
| classe dinâmica | `ClassList::try_new(tokens)?` |
| id | `Id::new("x")` / `Id::try_new("x")?` |
| campo | `FieldName::new("nome")` |
| SRI | `Integrity::parse("sha384-…")?` |
| autocomplete | `Autocomplete::parse("email")?` |
| JSON-LD | `JsonLd::graph(&graph)` |

## Formulários

```rust
let action = Action::new(LUrl::new("/x").expect("rota"));
let token = CsrfToken::new(csrf).expect("csrf");
let form = Form::open(&action, Method::Post(token), children);

let state = FieldState::new(AttrVal::new(&value))
    .required(true)
    .help(Text::new("ajuda"))
    .error(Text::new("erro"));

let field = Field::text(FieldName::new("nome"), M::Name, &state);
```

| Método | Retorno |
|---|---|
| `Form::open` / `open_with_submit` | `Markup` |
| `Method::{Get, Post, Put, Patch, Delete}` | `Method` |
| `Field::text/input/textarea/file/checkbox` | `Markup` |
| `Field::select` | `Option<Markup>` |
| `Field::checkbox_group/radio_group` | `Option<Markup>` |
| `InputType` | 14 tipos nativos |

`Field` deriva `f-<nome>`, `label for`, `aria-describedby`, `aria-invalid`, `required`. `select` usa
`SelectOption::selected`; grupos usam `ChoiceOption`.

## Mídia

```rust
let img = Img::new(src, Alt::new("…"), WidthPx::new(720), HeightPx::new(480))
    .responsive(srcset, sizes)
    .lazy();

let video = Video::new(src, Track::captions(vtt, Text::new("Legendas")))
    .poster(poster)
    .dimensions(WidthPx::new(1280), HeightPx::new(720));
```

`Picture::new(fallback).source(Source::new(srcset, sizes)?)`; `Audio::new(src)`;
`Track::subtitles(vtt, Text, Lang)`.

## Shell e assets

```rust
let assets: Assets = [
    Asset::style("/assets/app.css").expect("css"),
    Asset::script("/assets/app.js").expect("js"),
].into_iter().collect();

let data = ShellData::new(title, heading, lang)
    .assets(assets)
    .html_attributes(RootAttributes::new().class(Class::new("dark")))
    .body_attributes(RootAttributes::new().data(
        DataAttr::new("data-bs-theme", "dark").expect("attr"),
    ))
    .regions(regions);
```

| Asset | Emissão |
|---|---|
| `style` / `style_media` | `<link rel="stylesheet">` |
| `script` / `classic_script` | `<script defer>` (module ou clássico) |
| `external_*` | CDN `http(s)` |

Ordem = declaração; dedupe sem mover. `View::asset` entra depois do shell.

## `Page` / `View`

| Método | Efeito |
|---|---|
| `View::new(main)` | `<main>` |
| `.modal(Markup)` | depois do footer |
| `.asset(Asset)` | asset da view |
| `.merge(other)` | soma; `main` de `self` vence |
| `Page::new(&data, view)` | documento |
| `.markup()` | `&Markup` |
| `.into_markup()` | consome |

## Resposta parcial

| Método | Efeito |
|---|---|
| `Partial::new(main)` | substitui alvo |
| `Partial::keep_main()` | só updates |
| `.update(Id, Swap, Markup)` | ordem de aplicação |
| `.focus(Id)` | foco pós-swap |
| `.select(Id)` / `.reselect(Id)` | seleção |
| `.select_oob(Vec<Id>)` | OOB |

`Swap`: `Inner`, `Outer`, `BeforeBegin`, `AfterBegin`, `BeforeEnd`, `AfterEnd`, `Delete`, `None`.
`RequestClass::vary()`: `""`, `HX-Request`, `HX-Request, HX-Target`, etc.

## Audit e testes

```rust
// features = ["audit"]
let findings = html_auy::audit::check(document);
assert!(findings.is_empty(), "{findings:#?}");
```

Regras: `main_unique`, `single_h1`, `heading_levels_no_skips`, `ids_unique`,
`landmark_names_distinct`, `landmark_names_required`, `aria_refs_resolve`,
`popover_target_resolves`, `no_content_derived_identifiers`, `no_inline_script`,
`no_style_attribute`, `no_event_handler`.

## Não existe

| Não existe | Consequência |
|---|---|
| `Markup::from(String)` | dado não vira HTML |
| `Raw`/`PreEscaped`/`\|safe` | bypass de escape é inexprimível |
| `Href::new(&str)` | URL arbitrária não constrói |
| `String` em `class`/`href` | use `Class`/`Href` |
| `on*` / `<style>` / script inline | comportamento é asset |
| `<form>` em `html!` | use `Form::open` |
| `<main>`/`head`/`body` em `html!` | use `Page` |
| template em runtime | view é função Rust |
| locale no crate | `Msg` é trait do consumidor |
| CSS/tema no crate | tokens são do consumidor |

---

[← Arquitetura](15-arquitetura.md) · [Índice](README.md)
