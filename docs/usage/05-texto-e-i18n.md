[← Valores tipados](04-valores-tipados.md) · [Índice](README.md) · [Próximo →](06-formularios.md)

# 05 — Texto, mensagens e i18n

**Em uma frase.** Todo texto visível entra por `Msg` (estático, traduzido) ou `Text::new` (dinâmico,
escapado); o crate não conhece locale, catálogo nem plural.

## O trait `Msg`

```rust
pub trait Msg: Copy {
    fn as_str(self) -> &'static str;
}
```

O `enum` é do consumidor. O crate só chama `as_str()`; o derive gera o `match` a partir do
atributo `#[msg("…")]`:

```rust
#[derive(Clone, Copy, Msg)]
enum M {
    #[msg("Notícias")]
    Title,
    #[msg("Últimas notícias")]
    Latest,
    #[msg("Ler notícia")]
    Read,
    #[msg("Pular para o conteúdo")]
    Skip,
}
```

O `impl Msg` manual continua válido quando o catálogo precisa de lógica própria. O derive exige
variantes unitárias e uma string literal por variante; variante sem `#[msg]` é erro de compilação.

Em uma aplicação real, `as_str` lê o catálogo já resolvido para o locale da request. O crate não
recebe `Locale`, não consulta ambiente e não formata plural. Paridade de tradução, CLDR e
`trans_choice` são do consumidor.

## `Text`: dois construtores, dois mundos

```rust
let from_message = Text::msg(M::Title);       // &'static str, mensagem
let from_data = Text::new(&item.title);       // &str, dado dinâmico
```

| Construtor | Uso | Escape |
|---|---|---|
| `Text::msg(M)` | mensagem estática/traduzida | texto |
| `Text::new(&str)` | título, nome, conteúdo vindo do banco | texto |

Os dois produzem `Text`, que é `Copy` e não aloca. O escape acontece quando o `Text` é renderizado.
Não há terceiro caminho.

## Dentro de `html!`

```rust
html! {
    h2 { M::Latest }                 // Msg, sem parênteses
    p { Text::new(&item.summary) }   // Text, sem parênteses
    div { (body_markup) }            // Markup já composto
}
```

- `M::Latest` é uma expressão de mensagem; o macro chama `Writer::msg`.
- `Text::new(&s)` é uma expressão de texto; o macro chama o mesmo caminho de texto.
- `(expr)` insere um `Render`/`Markup`; use para componentes, não para texto.
- `"literal"` sozinho é recusado. Para um texto constante, crie uma variante de `Msg`; para um dado
  dinâmico, `Text::new`.

## Nomes acessíveis

`Label` é o tipo de nome acessível e recebe `Msg`, não `&str`:

```rust
html! {
    button aria_label=(Label::text(M::Read)) { … }
    nav aria_label=(Label::text(M::MainNav)) { … }
    a href=(url) { (Label::sr_only(M::ExternalLink)) } // <span class="sr-only">…</span>
}
```

- `Label::text(M)` é um rótulo visível ou o valor de `aria-label`.
- `Label::sr_only(M)` renderiza `<span class="sr-only">…</span>`; o CSS de `.sr-only` é do
  consumidor.
- `aria_label` literal é erro: sem chave de tradução, a paridade dos locales quebraria.
- Não use `Label::sr_only` como valor de `aria-label`; ali o correto é `Label::text`.

O crate emite `aria-describedby` para ajuda/erro de campo, `aria-invalid` e `aria-required` nos
componentes de formulário; você não escreve esses atributos à mão.

## Título, `h1` e skip link

```rust
let data = ShellData::new(
    Text::msg(M::Title),   // <title>
    Text::msg(M::Heading), // <h1> dentro do <main>
    lang,
)
.skip(Text::msg(M::Skip));
```

- `title` e `heading` não são opcionais; o `Page` sempre emite `<title>` e `<h1>`.
- Se a página não tiver um título visual próprio, use o mesmo `Msg` para os dois.
- O skip link só é emitido com `.skip(Text)`; sem ele, não há link, mas a região `role="status"`
  continua no shell.
- O `<h1>` é do shell; a view começa no conteúdo.

## Texto dinâmico e escaping

```rust
use html_auy::prelude::*;

let title = "<script>alert(1)</script>";
let text = Text::new(title);

assert_eq!(text.render().as_str(), "&lt;script&gt;alert(1)&lt;/script&gt;");
```

`Text` escapa `&`, `<`, `>`, `"`, `'` e NUL. Ele não tenta “sanitizar” HTML: o dado deixa de ser
markup. Se você tem HTML confiável vindo de um editor, componha um `Markup` por um renderizador
confiável (veja [Conteúdo dinâmico](09-conteudo-dinamico.md)); não passe `String` para `Text` e
espere HTML.

## Onde o locale entra

O fluxo recomendado:

```text
request → locale → catálogo do consumidor → Msg resolvido → view
```

O `enum M` pode ser gerado do catálogo (`.ftl`, JSON, etc.) ou escrito à mão. O importante é que a
view não decida locale e que o texto não seja concatenado em runtime para formar uma mensagem:

```rust
// Evite: a ordem das palavras muda entre idiomas.
let label = format!("{} {}", M::Read.as_str(), item.title);

// Prefira: uma mensagem com placeholder resolvida pelo consumidor.
let label = catalog.read_item(&item.title); // String
html! { a href=(url) { Text::new(&label) } }
```

## Vários locales sem runtime

O crate não recebe `Locale`: ele recebe um `Msg` já resolvido. Para mais de um idioma, o valor do
`Msg` pode carregar o locale e a chave, sem alocar e sem perder a checagem do compilador:

```rust
#[derive(Clone, Copy)]
enum Locale { PtBr, En, Es, Fr }

#[derive(Clone, Copy)]
enum Key { Title, Services, Skip, MainNav }

#[derive(Clone, Copy)]
struct Message { locale: Locale, key: Key }

impl Msg for Message {
    fn as_str(self) -> &'static str {
        match self.locale {
            Locale::PtBr => match self.key {
                Key::Title => "Portal da Prefeitura",
                Key::Services => "Serviços",
                Key::Skip => "Pular para o conteúdo",
                Key::MainNav => "Navegação principal",
            },
            Locale::En => match self.key {
                Key::Title => "City Hall Portal",
                Key::Services => "Services",
                Key::Skip => "Skip to content",
                Key::MainNav => "Main navigation",
            },
            Locale::Es => match self.key {
                Key::Title => "Portal del Ayuntamiento",
                Key::Services => "Servicios",
                Key::Skip => "Saltar al contenido",
                Key::MainNav => "Navegación principal",
            },
            Locale::Fr => match self.key {
                Key::Title => "Portail de la Mairie",
                Key::Services => "Services",
                Key::Skip => "Aller au contenu",
                Key::MainNav => "Navigation principale",
            },
        }
    }
}

fn msg(locale: Locale, key: Key) -> Message {
    Message { locale, key }
}

impl Message {
    fn text(locale: Locale, key: Key) -> Text<'static> {
        Text::msg(Self { locale, key })
    }
}
```

A view recebe o locale já resolvido pelo adapter HTTP (path, `Accept-Language`, sessão ou cookie) e
usa `Message::text(locale, Key::…)`:

```rust
fn header(locale: Locale) -> Markup {
    html! {
        nav aria_label=(Label::text(msg(locale, Key::MainNav))) {
            a href="/servicos" { Message::text(locale, Key::Services) }
        }
    }
}
```

- O `match` é exaustivo: uma `Key` nova quebra a compilação até existir em todos os locales.
- O locale não é lido dentro de `html!`; a view recebe o valor já decidido.
- `Message::text` é um caminho (`Type::fn`). Uma chamada solta como `tr(locale, key)` dentro de um
  corpo de `html!` seria lida como o elemento `<tr>`; use um caminho ou `(tr(locale, key))`.
- `Lang` vem de `Lang::parse(locale.tag())`; não monte a etiqueta BCP-47 à mão.
- `dir` vem do locale (`ShellData::dir(locale.dir())`): `Dir::Ltr` para pt/en/es/fr; um locale RTL
  usa `Dir::Rtl` no mesmo ponto. O crate não infere direção a partir da etiqueta.
- Para SEO, `ShellData::alternates` recebe `Alternate::new(lang, url)` de cada locale e um
  `Alternate::x_default(url)`; o `canonical` continua sendo o da página atual.

O exemplo completo, com catálogo de quatro idiomas, está em
[18 — Exemplo completo: portal governamental](18-exemplo-portal.md).

## Plural, rich text e formatação

Ficam no consumidor:

| Necessidade | Onde resolver |
|---|---|
| plural/`trans_choice` | catálogo do consumidor; produza um `String` e use `Text::new` |
| rich text (negrito/link) | renderizador confiável que produza `Markup` |
| datas/números localizados | biblioteca do consumidor; o crate só emite `Date`/`Timestamp`/números de máquina |
| dinheiro | inteiro em centavos + `Msg` de moeda; nunca `f64` |
| paridade de locales | gate de CI do consumidor |

## Erros comuns

| Sintoma | Correção |
|---|---|
| `texto literal não é permitido` | `M::X` ou `Text::new(&s)` |
| `aria_label="Fechar"` | `aria_label=(Label::text(M::Close))` |
| texto aparece escapado no navegador | você passou HTML por `Text`; use um `Markup` confiável |
| tradução some em um locale | a variante de `Msg` não tem string; o gate é do consumidor |
| palavras fora de ordem | não concatene fragmentos; resolva a frase inteira no catálogo |

---

[← Valores tipados](04-valores-tipados.md) · [Índice](README.md) · [Próximo →](06-formularios.md)
