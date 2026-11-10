[← Testes e diagnóstico](13-testes-e-diagnostico.md) · [Índice](README.md) · [Próximo →](15-arquitetura.md)

# 14 — Receitas

**Em uma frase.** Cada receita resolve um caso concreto em poucos bytes; a teoria fica nas páginas
indicadas.

## Página com header, breadcrumb e footer

```rust
use html_auy::prelude::*;

let regions = Regions::new()
    .header(html! {
        header {
            nav aria_label=(Label::text(M::MainNav)) {
                a href=(home) aria_current="page" { M::Home }
                a href=(news) { M::News }
            }
        }
    })
    .before_main(html! {
        nav aria_label=(Label::text(M::Breadcrumb)) {
            a href=(home) { M::Home }
            span aria_hidden="true" { M::Separator }
            M::News
        }
    })
    .footer(html! { footer { M::Copyright } });

let data = ShellData::new(title, heading, lang).regions(regions);
```

A view não escreve `main`/`h1`; o `Page` faz isso. Veja [Shell](08-shell-assets-e-tema.md).

## Card + estado vazio

```rust
html! {
    section class="cards" {
        @if items.is_empty() {
            p class="empty" { M::Empty }
        } @else {
            for item in items {
                article class="card" {
                    h3 { Text::new(&item.title) }
                    p { Text::new(&item.summary) }
                    a href=(Href::internal(route(&item.slug))) { M::Read }
                }
            }
        }
    }
}
```

## Tabela com caption e escopo

```rust
html! {
    table {
        caption { M::Fleet }
        thead {
            tr {
                th scope="col" { M::Plate }
                th scope="col" { M::Status }
            }
        }
        tbody {
            for row in rows {
                tr {
                    td { Text::new(&row.plate) }
                    td { Text::new(&row.status) }
                }
            }
        }
    }
}
```

`caption` é o primeiro filho; `th` exige `scope`. O macro valida a estrutura.

## Diálogo como modal

```rust
let modal = html! {
    dialog id="confirm" aria_label=(Label::text(M::ConfirmTitle)) {
        h2 { M::ConfirmTitle }
        p { M::ConfirmBody }
        button type="button" { M::Close }
    }
};

let view = View::new(main).modal(modal);
```

`Page` emite o modal depois do footer. `dialog` exige nome acessível; o foco inicial e o fechamento
são do JS do consumidor. Veja [Acessibilidade](11-acessibilidade.md).

## Paginação

```rust
html! {
    nav aria_label=(Label::text(M::Pagination)) {
        @if let Some(prev) = &page.prev {
            a href=(Href::internal(prev.clone())) rel="prev" { M::Previous }
        }
        a href=(Href::internal(page.current.clone())) aria_current="page" {
            Text::new(&page.number.to_string())
        }
        @if let Some(next) = &page.next {
            a href=(Href::internal(next.clone())) rel="next" { M::Next }
        }
    }
}
```

`rel="prev"`/`"next"` são permitidos; `aria-current="page"` é enum. Cada `nav` tem nome.

## Dark mode (Tailwind) e Bootstrap

```rust
let html_attrs = RootAttributes::new().class(Class::new("dark"));
let body_attrs = RootAttributes::new()
    .class(Class::new("app"))
    .data(DataAttr::new("data-bs-theme", "dark").expect("nome"));

let data = ShellData::new(title, heading, lang)
    .html_attributes(html_attrs)
    .body_attributes(body_attrs);
```

- Tailwind: `class="dark"` no `<html>`.
- Bootstrap: `data-bs-theme="dark"` no `<html>` ou `<body>`.
- `DataAttr` exige nome válido e valor `'static`; para alternar em runtime, mapeie para
  `"dark"`/`"light"` (ou use uma classe) antes de construir.

## CDN + script clássico

```rust
let assets: Assets = [
    Asset::external_style("https://cdn.example/reset.css"),
    Asset::external_classic_script("https://cdn.example/bootstrap.bundle.js"),
    Asset::classic_script("/assets/legacy.js"),
    Asset::script("/assets/app.js"),
].into_iter().collect();
```

A ordem é a declarada. CDN exige CSP allowlist; prefira self-host. `Asset` não carrega SRI.
Veja [Shell e assets](08-shell-assets-e-tema.md).

## Link externo seguro

```rust
let external = ExternalUrl::parse("https://example.org").expect("https");

html! {
    a href=(Href::external(external))
      target="_blank"
      rel="noopener noreferrer" {
        M::ExternalSite
    }
}
```

`target="_blank"` sem `rel` ganha `noopener noreferrer`; se você declarar `rel` sem `noopener`, o
macro recusa.

## JSON-LD

```rust
struct Article { title: String }

impl JsonGraph for Article {
    fn write_json(&self, out: &mut String) {
        // serialização do consumidor; `JsonLd` escapa o resultado
        out.push_str(r#"{"@context":"https://schema.org","@type":"Article","headline":"#);
        out.push_str(&serde_json::to_string(&self.title).expect("string"));
        out.push('}');
    }
}

let ld = JsonLd::graph(&article);
let data = ShellData::new(title, heading, lang).schemas(vec![ld]);
```

`JsonLd` escapa `< > & '` na construção; `</script>` é impossível.

## Componente próprio

```rust
struct Badge<'a> { text: Text<'a> }

impl Render for Badge<'_> {
    const SIZE_HINT: usize = 32;

    fn render_to(&self, out: &mut String) {
        out.push_str("<span class=\"badge\">");
        self.text.render_to(out);
        out.push_str("</span>");
    }
}

let badge = Markup::from_render(&Badge { text: Text::msg(M::New) });
html! { div { (badge) } }
```

Tags são confiáveis; todo dado dinâmico passa por `Text`/`AttrVal`. Veja
[Conteúdo dinâmico](09-conteudo-dinamico.md).

## Classe dinâmica com allowlist

```rust
const ALLOWED: &[&str] = &["col-3", "col-6", "col-9", "is-open"];

let classes = ClassList::try_new(
    user_tokens
        .iter()
        .filter(|token| ALLOWED.contains(&token.as_str()))
        .cloned(),
).expect("tokens válidos");
```

`ClassList` não conhece o seu design system; a allowlist é sua.

## Resposta parcial

```rust
let count = 3;
let partial = Partial::new(html! {
    section id="cart" { h2 id="cart-title" { M::Cart } … }
})
.update(Id::new("cart-count"), Swap::Inner, html! { Text::new(&count.to_string()) })
.focus(Id::new("cart-title"));
```

`Partial` é um valor; o adaptador serializa e define `Vary`. Veja
[HTTP e integrações](12-http-e-integracoes.md).

## Formulário completo

A receita de `Form`/`Field`/CSRF/422 está em [Formulários](06-formularios.md); não repita a montagem
de `label`/`id`/`aria-*` à mão.

---

[← Testes e diagnóstico](13-testes-e-diagnostico.md) · [Índice](README.md) · [Próximo →](15-arquitetura.md)
