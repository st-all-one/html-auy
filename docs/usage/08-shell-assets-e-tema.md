[← Mídia](07-midia.md) · [Índice](README.md) · [Próximo →](09-conteudo-dinamico.md)

# 08 — Shell, assets e tema

**Em uma frase.** `Page` é o documento; `ShellData` são os dados que só o documento conhece;
`Assets` é a lista ordenada de CSS/JS; classes e `data-*` do `<html>`/`<body>` são atributos de
root, não markup livre.

## `ShellData`: o que o documento sabe

```rust
let data = ShellData::new(
    Text::msg(M::Title),   // <title>, obrigatório
    Text::msg(M::Heading), // <h1>, obrigatório
    lang,                  // Lang, obrigatório
)
.description(Text::msg(M::Description))
.canonical(ExternalUrl::parse("https://example.org/noticias").expect("URL"))
.skip(Text::msg(M::Skip));
```

| Builder | Efeito |
|---|---|
| `new(title, heading, lang)` | documento mínimo |
| `.skip(Text)` | skip link para `#main`; primeiro focável |
| `.description(Text)` | `<meta name="description">` |
| `.canonical(ExternalUrl)` | `<link rel="canonical">` e `og:url` |
| `.manifest(LUrl)` | `<link rel="manifest">` |
| `.dir(Dir)` | `dir` do `<html>` |
| `.schemas(Vec<JsonLd>)` | JSON-LD no fim do `<head>` (substitui a lista) |
| `.alternates(Vec<Alternate>)` | `link rel="alternate"` (substitui a lista) |
| `.open_graph(OpenGraph)` | `og:*` derivado do documento |
| `.hints(Vec<Hint>)` / `.hint(Hint)` | resource hints, deduplicados |
| `.htmx_config(HtmxConfig)` | meta de configuração segura do htmx |
| `.assets(Assets)` | assets do documento (substitui o conjunto) |
| `.html_attributes(RootAttributes)` | classe/`data-*` do `<html>` |
| `.body_attributes(RootAttributes)` | classe/`data-*` do `<body>` |
| `.regions(Regions)` | header/breadcrumb/aside/footer |
| `.noscript(Markup)` | fallback sem JS, no início do `<body>` |

`hints`/`hint` **acrescentam** e deduplicam; `schemas`, `alternates`, `assets` e `regions`
substituem o valor anterior.

## `View`: o miolo e o que ele exige

```rust
let view = View::new(html! {
    section { h2 { M::Title } … }
})
.modal(html! { dialog aria_label=(Label::text(M::Modal)) { … } })
.asset(Asset::script("/assets/home.js").expect("asset"));
```

| Método | Efeito |
|---|---|
| `View::new(main)` | conteúdo do `<main>` |
| `.modal(Markup)` | modal emitido depois do footer, na ordem |
| `.asset(Asset)` | asset exigido pela tela; deduplica |
| `.merge(other)` | soma modais/assets; o `main` de `self` vence |

Os assets do `View` entram **depois** dos assets do `ShellData`, na ordem declarada; duplicata não
move posição.

## Ordem do documento

`Page::new(&data, view)` emite, nesta sequência:

```text
<!DOCTYPE html>
<html lang="…" [dir] [class] [data-*]>
  <head>
    charset → viewport → title → description → og:* → canonical → manifest → alternates
    → hints → htmx-config → assets → JSON-LD
  </head>
  <body [class] [data-*]>
    skip link → role=status → noscript → header → before_main
    → main#main[h1 + view] → after_main → footer → modals
  </body>
</html>
```

Consequências:

- `Page` sempre emite **um** `<h1>`; não escreva outro na view.
- `<main id="main" tabindex="-1">` é do shell; a view começa no seu conteúdo.
- O `<div role="status" aria-atomic="true">` é sempre emitido, vazio; é a região viva persistente.
- `noscript` fica depois do status e antes do header; o skip link continua sendo o primeiro focável.
- Modais ficam no fim do `<body>`, fora do `<main>`.

## Assets

```rust
use html_auy::{Asset, Assets, Media};

let assets: Assets = [
    Asset::style("/assets/app.css").expect("css"),
    Asset::style_media("/assets/print.css", Media::print()).expect("print"),
    Asset::script("/assets/app.js").expect("módulo"),
    Asset::classic_script("/assets/legacy.js").expect("clássico"),
].into_iter().collect();
```

| Construtor | Emite |
|---|---|
| `Asset::style("/…")` | `<link rel="stylesheet" href="/…">` |
| `Asset::style_media("/…", Media)` | o mesmo, com `media` (vazio ⇒ `None`) |
| `Asset::script("/…")` | `<script type="module" defer src="/…"></script>` |
| `Asset::classic_script("/…")` | `<script defer src="/…"></script>` |
| `Asset::external_style("https://…")` | CDN externo; URL `http(s)` validada |
| `Asset::external_script("https://…")` | módulo externo |
| `Asset::external_classic_script("https://…")` | clássico externo |

- A ordem é a de declaração; **não** há inferência “CSS antes de JS”. Se a cascata precisa dessa
  ordem, declare nessa ordem.
- `Assets` deduplica por igualdade de `(tipo, href, media, módulo)`; repetir não move a posição.
- `View::asset` e `ShellData::assets` seguem a mesma regra; o shell vem primeiro.
- O caminho interno precisa começar com `/`; `http(s)` só entra pelos construtores `external_*`.
- O caminho é `&'static str`: um nome com hash de build deve virar constante Rust no build do
  consumidor, não uma `String` de runtime.
- `Asset` não carrega `integrity`; SRI não é emitido para `link`/`script` por este tipo. CDN é aceito,
  mas self-host com CSP própria é o caminho recomendado.

### CDN e scripts clássicos

```rust
let assets: Assets = [
    Asset::external_style("https://cdn.example/reset.css").expect("cdn"),
    Asset::external_classic_script("https://cdn.example/bootstrap.bundle.js").expect("cdn"),
    Asset::script("/assets/app.js").expect("app"),
].into_iter().collect();
```

O uso é explícito para não esconder dependência externa. Se usar CDN, a CSP precisa allowlistar a
origem (`style-src`/`script-src`) e você perde o cache/controle de SRI do self-host. Não há
`latest`/URL mágica no crate.

## Atributos do root

```rust
use html_auy::{Class, DataAttr, RootAttributes};

let html_attrs = RootAttributes::new().class(Class::new("dark"));
let body_attrs = RootAttributes::new()
    .class(Class::new("app"))
    .data(DataAttr::new("data-bs-theme", "dark").expect("nome válido"));

let data = ShellData::new(title, heading, lang)
    .html_attributes(html_attrs)
    .body_attributes(body_attrs);
```

- `RootAttributes` aceita **classe** e **`data-*`**; nada de atributo arbitrário.
- A ordem é classe e depois os `data-*` na ordem de inserção; `data-*` com o mesmo nome é
  substituído.
- `DataAttr::new` exige `data-` + segmentos `[a-z0-9]` separados por `-`; `data-bs-theme` é válido,
  `data-BS-theme` não.
- O valor de `DataAttr` é `&'static str` e é escapado. Para um valor dinâmico, mapeie-o para um
  conjunto finito de constantes (`dark`/`light`, por exemplo) antes de construir; não invente nome
  de atributo em runtime.
- Exemplos: Tailwind `class="dark"`, Bootstrap `data-bs-theme="dark"`, tema próprio
  `data-theme="high-contrast"`.

## Tema é dado do consumidor

O crate **não emite CSS**, não resolve tokens e não conhece `theme.css`. O padrão é:

- tokens de design no consumidor (`tokens.css` ou dados);
- classes semânticas via `ClassToken`/`Tone`;
- o `<link>` do CSS entra em `Assets`;
- o estado do tema entra em `RootAttributes` (`class="dark"`, `data-theme="…"`);
- contraste e paleta são validados no consumidor.

Isso mantém o `Page` determinístico: o mesmo `ShellData` gera os mesmos bytes.

## Regiões

```rust
use html_auy::Regions;

let mut regions = Regions::new();
regions.header = Some(html! {
    header {
        nav aria_label=(Label::text(M::MainNav)) {
            a href=(home) aria_current="page" { M::Home }
        }
    }
});
regions.before_main = Some(html! {
    nav aria_label=(Label::text(M::Breadcrumb)) { … }
});
regions.after_main = Some(html! { aside aria_label=(Label::text(M::Related)) { … } });
regions.footer = Some(html! { footer { … } });
```

- Slot vazio não emite nada; não preencha “quase” para segurar espaço.
- `header`/`footer`/`aside`/`nav` são regiões; a ordem do DOM é a ordem visual.
- Com dois ou mais `nav`/`aside`, cada um precisa de nome acessível (`audit`).

## Metadados e hints

```rust
let data = ShellData::new(title, heading, lang)
    .description(Text::msg(M::Description))
    .canonical(canonical)
    .alternates(vec![
        Alternate::new(Lang::parse("en").expect("lang"), english_url),
        Alternate::x_default(default_url),
    ])
    .open_graph(
        OpenGraph::new()
            .kind(OpenGraphKind::Article)
            .image(og_image),
    )
    .hints(vec![
        Hint::Preconnect(ExternalUrl::parse("https://cdn.example").expect("URL")),
        Hint::Preload(Preload::font(Src::parse("/assets/font.woff2").expect("fonte"))),
        Hint::ModulePreload(Src::parse("/assets/app.js").expect("módulo")),
    ]);
```

- `OpenGraph` só emite tags se você chamar `.open_graph`; `title`, `description` e `canonical` são
  reaproveitados. `og:image` é declarado.
- `alternates` e `canonical` usam `ExternalUrl`, não caminho interno.
- `hints` são ordenados por espécie na saída (`preconnect`, `dns-prefetch`, `preload`,
  `modulepreload`, `prefetch`), independentemente da ordem do vetor.
- `Preload::font` já inclui `type="font/woff2"` e `crossorigin="anonymous"`.
- O orçamento de `<head>` é do projeto; o crate não corta nem reordena o que você declarou além da
  sequência fixa acima.

## htmx

O crate não emite `hx-*`. Se você vendoriza htmx, ligue só a configuração segura:

```rust
let data = ShellData::new(title, heading, lang)
    .htmx_config(HtmxConfig::safe());
```

Isso emite `<meta name="htmx-config">` antes dos assets, com `allowEval:false`,
`allowScriptTags:false`, `selfRequestsOnly:true`, `historyCacheSize:0` e `timeout:10000`. O
`<script src>` do htmx continua sendo um `Asset`; o comportamento de swap é do seu adaptador
(veja [HTTP e integrações](12-http-e-integracoes.md)).

## Limites que valem saber

- `Page` não carrega status HTTP nem cabeçalhos.
- `Page` não aceita atributos no `<main>`; o id `main` e `tabindex="-1"` são fixos.
- `Form::open` não aceita classe; envolva o formulário.
- `Asset` é `&'static str`; não há asset com nome gerado em runtime.
- `style=` inline é permitido, mas `audit` sinaliza e a CSP muda. Prefira `Class`.
- O crate não injeta CSS, não resolve tema e não mede contraste.

---

[← Mídia](07-midia.md) · [Índice](README.md) · [Próximo →](09-conteudo-dinamico.md)
