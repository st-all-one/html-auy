[← O macro `html!`](03-macro-html.md) · [Índice](README.md) · [Próximo →](05-texto-e-i18n.md)

# 04 — Valores tipados e contexto

**Em uma frase.** O escape é escolhido pelo **tipo do valor**, não pelo ponto onde ele entra; cada
contexto tem um tipo com um construtor seguro e nenhum construtor cru.

## O mapa

| Contexto | Tipo | Construtor | Garantia principal |
|---|---|---|---|
| texto de conteúdo | `Text<'a>` | `Text::new(&str)` · `Text::msg(M)` | escapa `& < > " '` e NUL |
| valor de atributo comum | `AttrVal<'a>` | `AttrVal::new(&str)` | escapa os 5 + `\r\n\t` |
| rota interna | `LUrl<'a>` | `LUrl::new` · `from_string` · `with_segments` | começa com `/`; sem `//`, `:`, `..`, controle |
| URL absoluta | `ExternalUrl<'a>` | `ExternalUrl::parse` | só `http`/`https`; esquema normalizado |
| link | `Href<'a>` | `Href::internal` · `fragment` · `external` · `mailto` · `tel` | une os destinos permitidos |
| fragmento | `Fragment<'a>` | `Fragment::new` | `#` ou `#id` com alfabeto seguro |
| e-mail | `MailAddr<'a>` | `MailAddr::parse` | `local@dominio.tld` conservador |
| telefone | `PhoneNumber<'a>` | `PhoneNumber::parse` | E.164 (`+` + 1..15 dígitos) |
| fonte de mídia | `Src<'a>` | `Src::media` · `Src::asset` · `Src::parse` | caminho validado ou `http(s)` |
| `srcset` | `SrcSet<'a>` | `SrcSet::from_variants` | ordena por largura, deduplica, valida cada `src` |
| `sizes` | `Sizes<'a>` | `Sizes::new` · `try_new` | gramática de lista; inválida vira vazio |
| texto alternativo | `Alt<'a>` | `Alt::new` · `decorative` | vazio só por declaração explícita |
| dimensão | `WidthPx` / `HeightPx` | `new(u32)` | nunca zero (0 satura para 1) |
| data/hora | `Timestamp` | `new(unix_seconds, Offset)` | RFC 3339 com offset |
| fuso | `Offset` | `UTC` · `from_seconds` · `from_hms` · `parse` | até ±23:59, em minutos |
| idioma | `Lang` | `Lang::parse` · `Lang::EN` | BCP-47 canônico (`pt-BR`) |
| direção | `Dir` | `Ltr` · `Rtl` · `Auto` | `dir` fechado |
| nome acessível | `Label<'a>` | `Label::text(M)` · `sr_only(M)` | texto traduzido; nunca vazio por `Msg` |
| id | `Id<'a>` | `Id::new` · `try_new` | normaliza para `[A-Za-z0-9_-]` |
| nome de campo | `FieldName<'a>` | `FieldName::new` · `try_new` | normaliza e deriva `f-…` |
| classes estáticas | `Class` | `Class::of` · `new` | ordena/deduplica; vazio = omitido |
| classes dinâmicas | `ClassList` | `ClassList::try_new` | token único por item; ordena/deduplica |
| cor/token | `Tone` | variantes | identidade semântica, não hex |
| consulta de mídia | `Media<'a>` | `print` · `screen` · `new` · `try_new` | estrutura válida; vazia = omitido |
| SRI | `Integrity<'a>` | `Integrity::parse` | `sha256/384/512` + base64 |
| autocomplete | `Autocomplete<'a>` | `Autocomplete::parse` | vocabulário HTML fechado |
| JSON-LD | `JsonLd` | `JsonLd::graph(&G)` | serializa e escapa `< > & '` |

## URLs: cada destino tem um tipo

```rust
use html_auy::{ExternalUrl, Fragment, Href, LUrl, MailAddr, PhoneNumber};

let home = LUrl::new("/").expect("rota constante");
let article = LUrl::with_segments("/noticias", &["a frota", "ônibus"]).expect("segmentos");
let external = ExternalUrl::parse("https://example.org/noticia").expect("https");
let mail = MailAddr::parse("contato@example.org").expect("e-mail");
let phone = PhoneNumber::parse("+5511999998888").expect("E.164");
let anchor = Fragment::new("#conteudo").expect("fragmento");

html! {
    a href=(Href::internal(home)) { M::Home }
    a href=(Href::internal(article)) { M::Article }
    a href=(Href::external(external)) { M::External }
    a href=(Href::mailto(mail)) { M::Contact }
    a href=(Href::tel(phone)) { M::Call }
    a href=(Href::fragment(anchor)) { M::Skip }
}
```

- `javascript:`, `data:`, `vbscript:` e `file:` não constroem `ExternalUrl`/`Src`.
- `LUrl` recusa `//host`, `..` e `%2e%2e`; query e fragmento são aceitos.
- `LUrl::with_segments` faz percent-encoding de cada segmento (`%XX` maiúsculo).
- `Href` não tem `Href::new(&str)`; não há URL arbitrária.

## Texto e atributos

```rust
let title = "Notícia <urgente>";
let text = Text::new(title);          // conteúdo, escapado
let attr = AttrVal::new(title);       // valor de atributo, escapado
```

`Text` e `AttrVal` não são `String`: são empréstimos para o dado já decidido. `Text::new` não
aloca; o escape acontece na escrita. `AttrVal` é para atributos cujo nome é conhecido mas cujo tipo
não tem um valor mais específico (`value`, `placeholder`, `accept`, `style` dinâmico).

## Imagem e mídia

```rust
struct Avif { src: &'static str, width: u32 }

impl Variant for Avif {
    fn src(&self) -> &str { self.src }
    fn width(&self) -> u32 { self.width }
}

let variants = [
    Avif { src: "/media/a-360.avif", width: 360 },
    Avif { src: "/media/a-720.avif", width: 720 },
];
let set = SrcSet::from_variants(&variants); // ordena e deduplica
let sizes = Sizes::new("(min-width: 48em) 33vw, 100vw");
let alt = Alt::new("Ônibus no pátio");
let width = WidthPx::new(720);
let height = HeightPx::new(480);
```

- `SrcSet` descarta largura zero e `src` inválido; empate de largura mantém o menor `src`.
- `Sizes` inválida vira vazia; `Img::responsive` só emite `srcset` e `sizes` quando ambos têm
  conteúdo.
- `WidthPx::new(0)` satura para `1`; dimensão zero não existe.
- `Alt::new("")` é `Alt::decorative()`; o vazio só existe por declaração.
- Para `Src` de um objeto do consumidor, implemente `MediaPath`:

```rust
struct Cover { url: LUrl<'static> }

impl MediaPath for Cover {
    fn path(&self) -> &LUrl<'_> { &self.url }
}

let src = Src::media(&cover);
```

## Datas e fusos

```rust
let offset = Offset::from_hms(-3, 0).expect("UTC-3");
let stamp = Timestamp::new(1_704_164_645, offset);

html! {
    time datetime=(stamp) { Text::new("5 de novembro de 2026") }
}
```

- `Timestamp` é RFC 3339 **com offset**; não há modo “data sem fuso”.
- `Offset::from_seconds` exige múltiplo de 60; `parse` aceita `Z`, `+HH:MM` e `-HH:MM`.
- A formatação visível (“5 de novembro”) continua sendo do consumidor; o crate só emite o valor de
  máquina.

## Classes: estática, dinâmica e tokens

```rust
#[derive(Clone, Copy)]
enum Token { Card, Wide }
impl ClassToken for Token {
    fn as_str(self) -> &'static str {
        match self { Token::Card => "card", Token::Wide => "card--wide" }
    }
}

let classes = Class::of(&[Token::Wide, Token::Card]); // "card card--wide"
let literal = Class::new("card card--wide");         // mesma saída
let dynamic = ClassList::try_new(["col-6", "is-open"]).expect("tokens");
```

- `Class` guarda `&'static str`; use para tokens de design system.
- `ClassList` guarda `String`; use para valores calculados.
- Ambos ordenam e deduplicam; a ordem é alfabética, não a ordem de declaração.
- `ClassList` recusa item vazio, espaço, tab, newline e controle. Ele **não** conhece o seu design
  system: se o token vier de entrada não confiável, valide contra uma allowlist antes.
- `class` vazio não emite o atributo.

## Ids e nomes de campo

```rust
let id = Id::new("relatorio 2026");   // "relatorio-2026" (normaliza)
let same = Id::try_new("relatorio-2026").expect("já válido");
let field = FieldName::new("sections[0][campo]"); // name válido; id "f-sections-0-campo"
```

- `Id::new` nunca falha: caracteres fora de `[A-Za-z0-9_-]` viram `-`; vazio vira `"id"`.
- `FieldName::new` também normaliza e aceita `[`, `]`, `.` no `name`.
- Não derive `id` de conteúdo do usuário; o `audit` tem `no_content_derived_identifiers`.

## Atributo opcional e vazio

```rust
let maybe: Option<Id<'_>> = None;

html! {
    div id?=(maybe) { }  // nada é emitido
    div id=(maybe.unwrap_or_else(|| Id::new("x"))) { }
}
```

`attr?=(Option<T>)` só existe para valores computados. Para valores literais, escreva o atributo ou
não o escreva.

| Valor | Efeito |
|---|---|
| `Class::default()` | `class` omitido |
| `Sizes::new("inválido")` | `sizes` omitido |
| `Media::new("(")` | `media` omitido |
| `AttrVal::new("")` | atributo **emitido vazio**; não use como “sem valor” |
| `Text::new("")` | conteúdo vazio; é diferente de omitir |

## `AttrValue` e `Render`: as duas extensões

O crate expõe dois traits para composição:

- `Render` escreve HTML confiável em um `String` (`render_to`) e pode produzir `Markup`.
- `AttrValue` escreve um valor de atributo já escapado.

Implementar qualquer um dos dois é assumir responsabilidade pela confiança. Um `Render` que escreve
`&str` cru pode reintroduzir injeção; use `Text`, `escape::text` ou os tipos de valor. O detalhe está
em [Segurança](10-seguranca.md) e [Conteúdo dinâmico](09-conteudo-dinamico.md).

## O que não usar

| Em vez de | Use |
|---|---|
| `String` em `class` | `Class`/`ClassList` |
| `String` em `href` | `Href` + `LUrl`/`ExternalUrl` |
| `f64` para dinheiro | inteiro em centavos + `Msg` de moeda (do consumidor) |
| `String` em `style` vindo do usuário | classe/token; `style=` só com valor confiável e auditado |
| `format!` para montar HTML | `html!`/`Render`/`Markup::from_render` |
| `i64` direto em `datetime` | `Timestamp`/`Offset` |

---

[← O macro `html!`](03-macro-html.md) · [Índice](README.md) · [Próximo →](05-texto-e-i18n.md)
