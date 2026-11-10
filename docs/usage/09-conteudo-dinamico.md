[← Shell e assets](08-shell-assets-e-tema.md) · [Índice](README.md) · [Próximo →](10-seguranca.md)

# 09 — Conteúdo dinâmico e composição

**Em uma frase.** Dado de fora entra por `Text` (texto) ou por um valor tipado (URL, atributo,
classe); markup só entra por `html!` ou por um `Render` que você controla.

## O fluxo seguro

```text
banco/HTTP → String/campos → Text/AttrVal/Href/… → html! → Markup
editor confiável → Markup → (markup) → html!
```

| Origem | Entra como |
|---|---|
| título, nome, descrição do banco | `Text::new(&s)` |
| URL de rota | `Href::internal(LUrl::… )` |
| URL externa | `Href::external(ExternalUrl::…)` |
| atributo `value`/`placeholder` | `AttrVal::new(&s)` |
| classe calculada | `ClassList::try_new(…)` |
| lista de itens | `for item in &items` |
| HTML de editor confiável | `Markup` de um renderizador confiável |
| conteúdo do usuário | **nunca** `Markup`; use `Text` |

## Listas e estado vazio

```rust
fn list(items: &[News]) -> Markup {
    html! {
        section class="news-list" {
            h2 { M::Latest }
            @if items.is_empty() {
                p { M::Empty }
            } @else {
                ul {
                    for item in items {
                        li {
                            a href=(Href::internal(route(&item.slug))) {
                                Text::new(&item.title)
                            }
                        }
                    }
                }
            }
        }
    }
}
```

O `for` é Rust; `items.iter().enumerate()` e padrões funcionam. Evite construir `String` com
`format!` para “montar HTML”: o resultado não é `Markup` e não deve ser inserido por `Text`.

## Componentes com `Render`

Um componente é um tipo que escreve bytes confiáveis. Use os valores tipados para o dado:

```rust
use html_auy::prelude::*;

struct Badge<'a> {
    label: Text<'a>,
}

impl Render for Badge<'_> {
    const SIZE_HINT: usize = 32;

    fn render_to(&self, out: &mut String) {
        out.push_str("<span class=\"badge\">");
        self.label.render_to(out);
        out.push_str("</span>");
    }
}

let badge = Markup::from_render(&Badge { label: Text::msg(M::New) });
```

Regras:

- Tags e nomes de atributo no `Render` são **código confiável**; nunca interpole `&str` cru.
- Qualquer valor dinâmico deve passar por `Text`, `AttrVal` ou outro tipo que escape.
- `Markup::from_render(&value)` é o caminho para transformar um `Render` em `Markup`.
- `Render::SIZE_HINT` é opcional; ajuda a reservar o buffer, não altera a saída.
- `Markup` implementa `Render`; `&T` também, então `(badge)` compõe sem cópia.
- Um `Render` que escreve `String` de fora sem escape reintroduz injeção; a responsabilidade é sua.

## Opcional e composição

```rust
let badge = if item.is_new {
    html! { span class="badge" { M::New } }
} else {
    html! { }
};

let card = html! {
    article class="card" {
        h3 { Text::new(&item.title) }
        (badge)
    }
};
```

- `html! { }` produz um `Markup` vazio; é útil para ramos sem conteúdo.
- `Option<Markup>` pode ser resolvido com `match` antes do macro; evite `unwrap` no meio da view.
- `View::merge(other)` soma modais e assets; o `main` de `self` vence. Use para compor telas que
  retornam partes independentes.
- `View::new` aceita qualquer `impl Into<Markup>`; `Markup` entra direto.

## Classes dinâmicas

`ClassList` aceita `String`/`&str` calculados, mas **não** valida o seu design system:

```rust
const ALLOWED: &[&str] = &["col-3", "col-6", "col-9", "is-open"];

fn classes(user_tokens: &[String]) -> Option<ClassList> {
    ClassList::try_new(
        user_tokens
            .iter()
            .filter(|token| ALLOWED.contains(&token.as_str()))
            .cloned(),
    )
}
```

- `ClassList::try_new` recusa item vazio, espaço, tab, newline e controle.
- Ele ordena e deduplica; a ordem final é alfabética.
- Se o token vem de query/banco, use allowlist; o tipo não conhece `col-6` nem `is-open`.
- Para tokens estáticos do design system, prefira `Class::of(&[Token::…])`.

## Atributos dinâmicos

```rust
use html_auy::{AttrVal, Id};

let maybe_id: Option<Id<'_>> = item.anchor.as_deref().map(Id::new);
let value = AttrVal::new(&item.value);

html! {
    div id?=(maybe_id) data_value=(value) { … }
}
```

- `id?=(Option)` omite quando `None`.
- `data_*=(AttrVal)` é valor escalar; não coloque JSON.
- `style=(AttrVal)` é possível, mas é escape hatch auditado; dado não confiável **não** entra.
- Nome de atributo nunca vem de variável; o macro só aceita nomes do vocabulário.

## Conteúdo rico vindo de um editor

Se o consumidor tem um renderizador de Markdown/rich text que produz `Markup` confiável:

```rust
let body: Markup = render_markdown_trusted(&article.body);
let view = View::new(html! {
    article {
        h2 { Text::new(&article.title) }
        (body)
    }
});
```

O crate não sabe se o renderizador é seguro. Se o texto é do usuário, **não** o transforme em
`Markup`; mantenha `Text` e mostre o texto. A fronteira é a mesma de [Segurança](10-seguranca.md).

## Performance

- `Markup` é um `String`; composição com `(markup)` copia os bytes para o buffer da view.
- `Render::render_to` evita uma alocação intermediária quando você já tem um `&mut String`.
- `View::merge` não re-renderiza; junta modais e assets.
- `Class` não aloca; `ClassList` aloca `String`s.
- O shell reserva capacidade a partir de `title`/`main`; listas grandes ainda podem reservar com
  `String::with_capacity` no seu componente.

## O que não fazer

| Não faça | Faça |
|---|---|
| `format!("<p>{}</p>", user)` | `html! { p { Text::new(user) } }` |
| `Markup::from(String::from(...))` | não existe; use `html!`/`Render` |
| `Text::new("<b>x</b>")` esperando negrito | `Markup` de um renderizador confiável |
| `data_json=(serde_json::to_string(...))` | `data-*` escalar; payload é estado de hidratação |
| `class=(format!("col-{}", n))` | `ClassList::try_new` com allowlist |
| `id=(item.title.clone())` | `Id::new` com id estável, não conteúdo |

---

[← Shell e assets](08-shell-assets-e-tema.md) · [Índice](README.md) · [Próximo →](10-seguranca.md)
