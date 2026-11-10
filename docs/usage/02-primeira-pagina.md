[← Instalação](01-instalacao.md) · [Índice](README.md) · [Próximo →](03-macro-html.md)

# 02 — Primeira página

**Em uma frase.** Você monta o miolo com `html!`, declara os dados do documento em `ShellData` e
entrega os dois a `Page`; o `Page` devolve o HTML completo.

## O código completo

```rust
use html_auy::prelude::*;

#[derive(Clone, Copy, Msg)]
enum M {
    #[msg("Início")]
    Title,
    #[msg("Bem-vindo")]
    Heading,
    #[msg("Esta é a página inicial.")]
    Greeting,
    #[msg("Pular para o conteúdo")]
    Skip,
    #[msg("Descrição para buscadores")]
    Description,
}

fn page() -> Page {
    let lang = Lang::parse("pt-BR").expect("idioma constante");

    let data = ShellData::new(
        Text::msg(M::Title),   // <title>
        Text::msg(M::Heading), // <h1>
        lang,                  // <html lang="pt-BR">
    )
    .skip(Text::msg(M::Skip));

    let view = View::new(html! {
        p { M::Greeting }
    });

    Page::new(&data, view)
}
```

## A saída

O `Page` produz exatamente esta forma canônica (uma linha; o fonte pode estar indentado de qualquer
jeito):

```html
<!DOCTYPE html><html lang="pt-BR"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>Início</title></head><body><a class="skip-link" href="#main">Pular para o conteúdo</a><div role="status" aria-atomic="true"></div><main id="main" tabindex="-1"><h1>Bem-vindo</h1><p>Esta é a página inicial.</p></main></body></html>
```

Observe o que **não** está no seu código e está na saída:

| Trecho | Quem emite |
|---|---|
| `<!DOCTYPE html>`, `<html>`, `<head>`, `<body>` | `Page` |
| `<meta charset>` e viewport | `Page` |
| `<title>` e `<h1>` | `Page`, a partir dos dois `Text` de `ShellData::new` |
| skip link e `role="status"` vazio | `Page` (o skip só existe se você chamar `.skip`) |
| `<main id="main" tabindex="-1">` | `Page` |
| `<p>…</p>` | sua `View` |

## Por que `title` e `h1` são argumentos separados

`ShellData::new(title, heading, lang)` exige **dois** textos e **um** `Lang`:

- `title` é o nome do documento em `<title>` (WCAG 2.4.2);
- `heading` é o `<h1>` da página (hierarquia de títulos);
- `lang` é uma etiqueta BCP-47 validada, não uma `String` livre.

O `Page` sempre emite um `<h1>` dentro do `<main>`. Por isso a view começa em `<h2>` ou em conteúdo
sem título; **não** escreva `h1` em `html!` quando estiver usando `Page` (o `audit` acusaria
`single_h1` com dois). A exceção documentada da spec existe para shells próprios, não para o fluxo
normal.

## `Page` não é uma resposta HTTP

`Page` é o documento, não status nem cabeçalho. O adaptador da sua aplicação decide:

```rust
let body = page.markup().as_str();     // &str
let owned = page.into_markup().into_string(); // String, se precisar
```

Um handler típico responde `200`, `Content-Type: text/html; charset=utf-8` e o corpo. Formulários
que alteram estado respondem `303`; erros de validação respondem `422` com o formulário
re-renderizado. Os detalhes estão em [HTTP e integrações](12-http-e-integracoes.md).

## O que controlar no `ShellData`

O construtor mínimo já entrega um documento válido. Os builders cobrem o resto:

```rust
let data = ShellData::new(title, heading, lang)
    .description(Text::msg(M::Description))
    .canonical(ExternalUrl::parse("https://example.org/").expect("URL"))
    .skip(Text::msg(M::Skip))
    .assets(assets)
    .regions(regions);
```

A lista completa, a ordem do `<head>` e os assets estão em
[Shell, assets e tema](08-shell-assets-e-tema.md).

## Erros comuns

| Código | Erro | Correção |
|---|---|---|
| `html! { main { … } }` | `<main>` é `only = "shell"` | o `Page` já emite `<main>`; comece pela view |
| `html! { h1 { … } }` com `Page` | dois `h1` no documento | use `h2` na view; o `h1` é o `heading` |
| `html! { p { "Olá" } }` | texto literal não é permitido | `M::Greeting` (estático) ou `Text::new(&value)` (dinâmico) |
| `Lang::new("pt-BR")` | `Lang` não tem construtor livre | `Lang::parse("pt-BR")?` ou `Lang::EN` |
| `Page::new(&data, html! { … })` | `Page` espera `View` | `Page::new(&data, View::new(html! { … }))` |

## Verificação

```rust
#[test]
fn primeira_pagina_tem_um_h1_e_main() {
    let page = page();
    let html = page.markup().as_str();
    assert_eq!(html.matches("<h1>").count(), 1);
    assert_eq!(html.matches("<main").count(), 1);
    assert!(html.starts_with("<!DOCTYPE html><html lang=\"pt-BR\""));
}
```

Para o passe de acessibilidade, veja [Testes e diagnóstico](13-testes-e-diagnostico.md).

---

[← Instalação](01-instalacao.md) · [Índice](README.md) · [Próximo →](03-macro-html.md)
