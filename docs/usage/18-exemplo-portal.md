# 18 — Exemplo completo: portal governamental

Uma página inteira de portal municipal, montada com a API pública: shell, metadados, navegação,
aviso, serviços, notícias e rodapé. O código executável está em
[`crates/html-auy/examples/portal_governamental.rs`](../../crates/html-auy/examples/portal_governamental.rs).

## Como rodar

```bash
cargo run -p html-auy --example portal_governamental
```

O programa escreve o documento completo em `stdout`; não há servidor, template em runtime nem
dependência de Axum. O exemplo compila sem a feature `audit`.

Para ver a versão com CSS de demonstração, use a
[`preview`](../../examples/portal_governamental_preview/README.md) e sirva a pasta localmente.

## O que a página exercita

| Parte | Mecanismo |
|---|---|
| documento, `lang`, `title`, `h1`, skip link | `Page` + `ShellData` |
| navegação com item atual | `nav aria_label` + `aria_current="page"` |
| aviso antes do `main` | `Regions::before_main` |
| cards de serviço | `html!` com `a` + `h3` + `p` |
| notícias com data | `Timestamp::new(unix, Offset::from_hms(-3, 0))` |
| CSS e JS externos | `Asset::style`/`Asset::script` + `Assets` |
| metadados sociais e descrição | `description`, `canonical` |
| JSON-LD | `JsonGraph` + `JsonLd::graph` |
| rodapé com segunda navegação | `Regions::footer` + `Label::text` |

O `h1` é emitido pelo `Page` a partir do `ShellData`; a view começa no conteúdo do `<main>`. O
`Page` também emite skip link, região viva e a ordem fixa de `<head>`/`<body>`.

## O esqueleto

```rust
let mut regions = Regions::new();
regions.header = Some(header());
regions.before_main = Some(notice());
regions.footer = Some(footer());

let data = ShellData::new(
    Text::msg(PortalMsg::Title),
    Text::msg(PortalMsg::Heading),
    lang(),
)
.description(Text::msg(PortalMsg::Description))
.canonical(ExternalUrl::parse("https://prefeitura.exemplo.gov.br/").expect("URL canônica"))
.skip(Text::msg(PortalMsg::Skip))
.assets(assets())
.schemas(vec![JsonLd::graph(&GovernmentOrganization)])
.regions(regions);

Page::new(&data, View::new(main))
```

O `main` compõe seções independentes; como dois `Markup` seguidos seriam lidos como uma chamada, os
componentes são emitidos por um `for`:

```rust
let main = html! {
    section class=(Class::new("hero")) { /* … */ }
    for part in [services(), news()] {
        (part)
    }
};
```

## Trecho da saída

O documento é uma única linha canônica. Formatado para leitura, o início fica assim:

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Portal da Prefeitura de Exemplo</title>
  <meta name="description" content="Serviços, notícias e canais de atendimento da Prefeitura de Exemplo.">
  <link rel="canonical" href="https://prefeitura.exemplo.gov.br/">
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
`Raw`/`PreEscaped`. Texto dinâmico passa por `Text`; URL por `Href`/`LUrl`; data por `Timestamp`.

## O que adaptar no consumidor

- Troque as rotas `/`, `/servicos`, `/noticias` etc. pela tabela de rotas da aplicação.
- Sirva `/assets/portal.css` e `/assets/portal.js` com `ServeDir` ou CDN e ajuste o CSP.
- Resolva locale e catálogo de mensagens no seu `Msg`; o crate não lê ambiente.
- Calcule status, headers, `ETag` e `Cache-Control` no adapter HTTP.
- Rode `axe`, teclado e `audit` sobre a página real no CI do consumidor.

O exemplo é deliberadamente estático: a página demonstra o contrato de renderização, não um framework
de rotas, banco ou sessão.

---

[← Guia de implementação](17-guia-de-implementacao.md) · [Índice](README.md) · [Próximo →](99-referencia-rapida.md)
