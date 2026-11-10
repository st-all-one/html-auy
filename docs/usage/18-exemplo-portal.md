# 18 — Exemplo completo: portal governamental

Uma página inteira de portal municipal, montada com a API pública: shell, metadados, navegação,
aviso, serviços, notícias e rodapé. O exemplo é multilíngue — `pt-BR`, `en`, `es` e `fr` — e mostra
como resolver o locale no consumidor sem entregar catálogo, plural ou formatação ao crate. O código
executável está em
[`crates/html-auy/examples/portal_governamental.rs`](../../crates/html-auy/examples/portal_governamental.rs).

## Como rodar

```bash
cargo run -p html-auy --example portal_governamental            # pt-BR
cargo run -p html-auy --example portal_governamental -- en      # inglês
cargo run -p html-auy --example portal_governamental -- es      # espanhol
cargo run -p html-auy --example portal_governamental -- fr      # francês
```

O programa escreve o documento completo em `stdout`; não há servidor, template em runtime nem
dependência de Axum. O exemplo compila sem a feature `audit`. Um argumento desconhecido retorna
`InvalidInput` em vez de cair silenciosamente no idioma padrão.

Para ver as quatro versões com CSS de demonstração, use a
[`preview`](../../examples/portal_governamental_preview/README.md) e sirva a pasta localmente.

## O que a página exercita

| Parte | Mecanismo |
|---|---|
| documento, `lang`, `dir`, `title`, `h1`, skip link | `Page` + `ShellData` |
| navegação com item atual | `nav aria_label` + `aria_current="page"` |
| aviso antes do `main` | `Regions::before_main` |
| cards de serviço | `html!` com `a` + `h3` + `p` |
| notícias com data | `Date::new(2026, 11, 5).expect("data fixa")` (data-only) |
| CSS e JS externos | `Asset::style`/`Asset::script` + `Assets` |
| descrição e canonical | `description`, `canonical` |
| JSON-LD | `JsonGraph` + `JsonLd::graph` |
| rodapé com segunda navegação | `Regions::footer` + `Label::text` |
| quatro locales e catálogo | `Message { locale, key }` + `Msg` + `Lang` |
| seletor de idioma | `language_switch` com `nav` nomeado, `hreflang`, `rel="alternate"` e `aria-current` |
| SEO multilíngue | `ShellData::canonical` + `Alternate::new`/`x_default` |

`ShellData::dir(locale.dir())` emite `dir="ltr"` para os quatro locales atuais; um locale RTL usaria
`Dir::Rtl` no mesmo ponto. O `h1` é emitido pelo `Page` a partir do `ShellData`; a view começa no
conteúdo do `<main>`. O `Page` também emite skip link, região viva e a ordem fixa de `<head>`/`<body>`.

## O esqueleto

```rust
fn page(locale: Locale) -> Page {
    let regions = Regions::new()
        .header(header(locale))
        .before_main(notice(locale))
        .footer(footer(locale));

    let data = ShellData::new(
        Message::text(locale, Key::Title),
        Message::text(locale, Key::GovernmentName),
        locale.lang(),
    )
    .description(Message::text(locale, Key::Description))
    .canonical(ExternalUrl::parse(locale.canonical()).expect("URL canônica"))
    .alternates(alternates())
    .skip(Message::text(locale, Key::Skip))
    .assets(assets())
    .schemas(vec![JsonLd::graph(&GovernmentOrganization)])
    .regions(regions);

    Page::new(&data, View::new(main))
}
```

O `main` compõe seções independentes com `Markup::concat`; dois `Markup` seguidos (`(a)(b)`)
seriam lidos como uma chamada:

```rust
let main = html! {
    section class=(Class::new("hero")) { /* … */ }
    (Markup::concat([services(), news()]))
};
```

## Como o locale entra

`Locale` é um `enum` do consumidor; o crate nunca o vê. O valor de `Msg` carrega o locale já
resolvido e a chave da mensagem:

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
                Key::Title => "Portal da Prefeitura de Exemplo",
                Key::Services => "Serviços",
                Key::Skip => "Pular para o conteúdo",
                Key::MainNav => "Navegação principal",
            },
            Locale::En => match self.key {
                Key::Title => "City Hall of Example Portal",
                Key::Services => "Services",
                Key::Skip => "Skip to content",
                Key::MainNav => "Main navigation",
            },
            Locale::Es => match self.key {
                Key::Title => "Portal del Ayuntamiento de Ejemplo",
                Key::Services => "Servicios",
                Key::Skip => "Saltar al contenido",
                Key::MainNav => "Navegación principal",
            },
            Locale::Fr => match self.key {
                Key::Title => "Portail de la Mairie d'Exemple",
                Key::Services => "Services",
                Key::Skip => "Aller au contenu",
                Key::MainNav => "Navigation principale",
            },
        }
    }
}

impl Message {
    fn text(locale: Locale, key: Key) -> Text<'static> {
        Text::msg(Self { locale, key })
    }
}
```

O `match` é exaustivo: se uma `Key` nova não existir em todos os locales, o código não compila.
`Message::text` é um caminho (`Type::fn`); dentro de um corpo de `html!`, uma chamada solta como
`tr(locale, key)` seria interpretada como o elemento `<tr>`. Use `Message::text(...)` ou
`(tr(...))`.

O locale é escolhido antes da view (aqui, pelo argumento de `main`; em produção, pelo adapter HTTP).
`lang` e `canonical` saem do mesmo valor:

```rust
fn alternates() -> Vec<Alternate<'static>> {
    let mut links: Vec<_> = Locale::ALL
        .iter()
        .map(|locale| {
            Alternate::new(
                locale.lang(),
                ExternalUrl::parse(locale.canonical()).expect("URL canônica"),
            )
        })
        .collect();
    links.push(Alternate::x_default(
        ExternalUrl::parse(Locale::PtBr.canonical()).expect("URL canônica"),
    ));
    links
}
```

O snippet usa `Alternate`, que não está no prelude; o exemplo importa `use html_auy::Alternate;`.

As quatro páginas têm `hreflang` recíproco e `x-default` para `pt-BR`; cada uma mantém seu próprio
`canonical`. No exemplo, `Locale` também define `ALL`, `tag`, `lang` e `canonical`; só o `match` do
catálogo muda de um idioma para outro. A data das notícias também é uma mensagem do catálogo, porque
a ordem e o formato são do locale; `Date` continua sendo o valor de máquina em `datetime`.

## Trecho da saída

O documento é uma única linha canônica. Formatado para leitura, o início fica assim:

```html
<!DOCTYPE html>
<html lang="pt-BR" dir="ltr">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Portal da Prefeitura de Exemplo</title>
  <meta name="description" content="Serviços, notícias e canais de atendimento da Prefeitura de Exemplo.">
  <link rel="canonical" href="https://prefeitura.exemplo.gov.br/">
  <link rel="alternate" hreflang="pt-BR" href="https://prefeitura.exemplo.gov.br/">
  <link rel="alternate" hreflang="en" href="https://prefeitura.exemplo.gov.br/en/">
  <link rel="alternate" hreflang="es" href="https://prefeitura.exemplo.gov.br/es/">
  <link rel="alternate" hreflang="fr" href="https://prefeitura.exemplo.gov.br/fr/">
  <link rel="alternate" hreflang="x-default" href="https://prefeitura.exemplo.gov.br/">
  <link rel="stylesheet" href="/assets/portal.css">
  <script type="module" defer src="/assets/portal.js"></script>
  <script type="application/ld+json">{"@context":"https://schema.org","@type":"GovernmentOrganization",…}</script>
</head>
<body>
  <a class="skip-link" href="#main">Pular para o conteúdo</a>
  <div role="status" aria-atomic="true"></div>
  <header class="site-header">…</header>
  <aside class="notice" aria-label="Aviso de atendimento">…</aside>
  <main id="main" tabindex="-1">
    <h1>Prefeitura de Exemplo</h1>
    <section class="hero">…</section>
    <section class="section">…</section>
    <section class="section section-alt">…</section>
  </main>
  <footer class="site-footer">…</footer>
</body>
</html>
```

Não há `<style>`, `on*`, `<script>` inline de comportamento, `String` de HTML vindo de dado ou
`Raw`/`PreEscaped`. Texto dinâmico passa por `Text`; rotas internas literais são validadas no macro;
data-only passa por `Date` e instantes por `Timestamp`.

## O que adaptar no consumidor

- Troque as rotas `/`, `/servicos`, `/noticias` etc. pela tabela de rotas da aplicação.
- Sirva `/assets/portal.css` e `/assets/portal.js` com `ServeDir` ou CDN e ajuste o CSP.
- Resolva o locale no adapter HTTP e passe `Locale` (ou o catálogo já resolvido) para a view; o crate
  não lê ambiente, `Accept-Language` nem sessão.
- Traduza `Key` em todos os locales; o `match` exaustivo é o gate de compilação. Datas, números,
  plural e rich text continuam no catálogo do consumidor.
- Calcule status, headers, `ETag`, `Cache-Control` e o `Content-Language` no adapter HTTP.
- Rode `axe`, teclado e `audit` sobre cada locale no CI do consumidor.

O exemplo é deliberadamente estático: a página demonstra o contrato de renderização, não um framework
de rotas, banco ou sessão.

---

[← Guia de implementação](17-guia-de-implementacao.md) · [Índice](README.md) · [Próximo →](99-referencia-rapida.md)
