[← Arquitetura](15-arquitetura.md) · [Índice](README.md) · [Próximo →](17-guia-de-implementacao.md)

# 16 — Projeto mínimo com Axum

**Em uma frase.** Um projeto pequeno precisa de Cargo, um adaptador `IntoResponse`, views que
devolvem `Page`, um diretório de assets e decisões explícitas de formulário, erro e headers. Não há
`build.rs`, diretório de templates nem codegen.

Este é o esqueleto operacional. Cache, `ETag`, `303`/`422`, resposta parcial e `Vary` estão em
[HTTP e integrações](12-http-e-integracoes.md); formulários em [Formulários](06-formularios.md).

## 1. Cargo

```toml
[package]
name = "site"
version = "0.1.0"
edition = "2024"
rust-version = "1.92"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }
serde = { version = "1", features = ["derive"] } # se usar formulários
html-auy = "0.1"

# opcionais
tower-http = { version = "0.7", features = ["fs"] } # arquivos estáticos
sha2 = "0.11"                                       # ETag
```

O crate público é `html-auy`; em Rust, `html_auy`. Não adicione `html-auy-macros` nem
`html-auy-macros-core` como dependência de aplicação. A aplicação Axum é `std` normalmente; o
`no_std` do crate não muda isso.

## 2. O adaptador HTTP

`Page` não implementa `IntoResponse` de propósito: o crate não conhece `http`, status nem headers.
O adaptador é escrito uma vez:

```rust
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

struct PageResponse {
    status: StatusCode,
    page: html_auy::Page,
}

impl IntoResponse for PageResponse {
    fn into_response(self) -> Response {
        (
            self.status,
            Html(self.page.into_markup().into_string()),
        )
            .into_response()
    }
}
```

`Html` do Axum define `Content-Type: text/html; charset=utf-8`. Para `ETag`, CSP, compressão ou
`Cache-Control`, troque o corpo por uma `Response` própria; veja [HTTP e integrações](12-http-e-integracoes.md).

## 3. `main.rs` mínimo

Este exemplo compila com Axum 0.8.9 e `html-auy` 0.1:

```rust
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use html_auy::{html, Href, Lang, LUrl, Page, ShellData, Text, View};

struct PageResponse {
    status: StatusCode,
    page: Page,
}

impl IntoResponse for PageResponse {
    fn into_response(self) -> Response {
        (
            self.status,
            Html(self.page.into_markup().into_string()),
        )
            .into_response()
    }
}

fn lang() -> Lang {
    Lang::parse("pt-BR").expect("idioma constante")
}

fn error_page(status: StatusCode, message: &str) -> PageResponse {
    let data = ShellData::new(Text::new("Erro"), Text::new("Erro"), lang());
    let view = View::new(html! {
        p { Text::new(message) }
    });
    PageResponse {
        status,
        page: Page::new(&data, view),
    }
}

async fn index() -> PageResponse {
    let data = ShellData::new(Text::new("Início"), Text::new("Início"), lang());
    let view = View::new(html! {
        p { Text::new("Olá, mundo.") }
    });
    PageResponse {
        status: StatusCode::OK,
        page: Page::new(&data, view),
    }
}

async fn noticia(Path(slug): Path<String>) -> PageResponse {
    let Some(url) = LUrl::with_segments("/noticias", &[&slug]) else {
        return error_page(StatusCode::BAD_REQUEST, "Slug inválido");
    };

    let data = ShellData::new(Text::new("Notícia"), Text::new("Notícia"), lang());
    let view = View::new(html! {
        article {
            h2 { Text::new(&slug) }
            p {
                a href=(Href::internal(url)) { Text::new("Link canônico") }
            }
        }
    });

    PageResponse {
        status: StatusCode::OK,
        page: Page::new(&data, view),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/noticias/{slug}", get(noticia))
        .fallback(|| async {
            error_page(StatusCode::NOT_FOUND, "Página não encontrada")
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
```

O que esse esqueleto já resolve por construção:

- um `<title>` e um `<h1>` por página, emitidos pelo `Page`;
- `lang` canônico no `<html>`;
- `<main id="main" tabindex="-1">` e região viva persistente;
- escape de todo texto dinâmico por `Text`;
- URL interna validada e percent-encoded por `LUrl`/`Href`;
- 404 com a mesma estrutura de documento, não uma string solta.

## 4. Views e dados

O handler busca dados e só então monta a view. A view não faz I/O:

```rust
struct Article {
    title: String,
    slug: String,
}

fn article_page(article: &Article) -> Page {
    let data = ShellData::new(
        Text::new(&article.title),
        Text::new(&article.title),
        lang(),
    );

    let view = View::new(html! {
        article {
            h2 { Text::new(&article.title) }
            p { Text::new("Conteúdo vindo do banco.") }
        }
    });

    Page::new(&data, view)
}
```

Para UI fixa, prefira um `enum Msg` implementando `Msg` em vez de `Text::new` em todos os rótulos.
`Msg` dá catálogo, autocomplete e impede texto literal no markup. Conteúdo dinâmico continua entrando
por `Text::new`; markup confiável, por `(markup)`.

## 5. Formulários, CSRF e 422

Há duas armadilhas de nomes:

```rust
use axum::Form as AxumForm;
use html_auy::Form as HtmlForm;
```

- `html_auy::Form::open` exige `Action` (`LUrl`) e, em métodos de escrita, `CsrfToken` dentro de
  `Method::Post/…`.
- O crate **emite** o token; validar sessão, expiração e replay é do servidor. Use sua sessão
  (`tower-sessions`, cookie assinado etc.); não invente um `CsrfToken` a partir de query.
- Erro de validação: `422` com o **mesmo** formulário, `FieldState` preenchido e um resumo
  `role="alert"`.
- Sucesso de escrita: `303` com `Location`; não responda `200` com o formulário renderizado.

`FieldState` deriva `id`, `for`, `aria-describedby`, `aria-invalid` e `required`; não escreva esses
atributos à mão. O fluxo completo está em [Formulários](06-formularios.md).

## 6. Assets e arquivos estáticos

O crate declara `<link>`/`<script>`; o Axum serve os bytes:

```rust
use html_auy::{Asset, Assets};
use tower_http::services::ServeDir;

let assets: Assets = [
    Asset::style("/assets/app.css").expect("css"),
    Asset::script("/assets/app.js").expect("js"),
]
.into_iter()
.collect();

let data = ShellData::new(title, heading, lang()).assets(assets);

let app = Router::new()
    .route("/", get(index))
    .nest_service("/assets", ServeDir::new("assets"));
```

- `Asset` usa `&'static str`; assets com hash de build precisam virar constantes no build do
  consumidor, não `String` de runtime.
- CSS continua em arquivo; o crate não emite `<style>` inline como caminho normal.
- A ordem dos assets é a declarada; CSS antes de JS só acontece se você declarar nessa ordem.
- CSP, `Cache-Control`, compressão e headers de segurança ficam no middleware/adaptador.

## 7. Erros, ETag e headers

- `PageResponse { status, page }` cobre `200`, `400`, `404` e `422`.
- `500`: use um `AppError` que implementa `IntoResponse` ou um middleware de captura; renderize uma
  `Page` de erro, não `format!` de HTML.
- `ETag`: calcule sobre os bytes finais; `Html` não define `ETag`. `Page` é determinístico, mas
  dados e ordem de widgets precisam ser determinísticos também.
- `Vary`, `HX-*`, `303`/`422`, compressão e cache de assets estão detalhados em
  [HTTP e integrações](12-http-e-integracoes.md).
- CSP e `style=` são decisão de segurança; veja [Segurança](10-seguranca.md).

## 8. Testes mínimos

- Teste os **bytes** de páginas representativas com `insta`/snapshot.
- Teste status e `Content-Type` com `tower::ServiceExt::oneshot` e `axum::body::Body`.
- Rode `audit::check` em fixtures com a feature `audit`; não em produção.
- Teste formulário: `422` preserva valores, `303` redireciona e CSRF inválido falha.
- Deixe `axe`, teclado e contraste para o gate de navegador; a crate não mede isso.

```rust
#[cfg(feature = "audit")]
#[test]
fn paginas_publicas_passam() {
    for page in fixtures() {
        let findings = html_auy::audit::check(page.markup().as_str());
        assert!(findings.is_empty(), "{findings:#?}");
    }
}
```

## 9. Checklist do mínimo

- [ ] `html-auy` é a única dependência do crate; os outros dois pacotes não entram na aplicação.
- [ ] `PageResponse` é o único ponto que converte `Page` em resposta HTTP.
- [ ] Toda view recebe dados prontos e devolve `Markup` ou `Page`.
- [ ] Todo texto dinâmico passa por `Text`; todo `href`/`src` passa por `LUrl`/`Href`/`Src`.
- [ ] `Page` emite o `<h1>`; a view começa no conteúdo e usa `h2`–`h6`.
- [ ] Assets são externos e servidos por `ServeDir`; não há `<style>`/`on*` inline.
- [ ] Formulários usam `Form::open`, `FieldState` e CSRF de sessão; `422` e `303` estão definidos.
- [ ] 404/500 usam `Page`; não há `Html(format!("<h1>…"))`.
- [ ] `audit` roda no CI sobre fixtures; `axe`/teclado/contraste rodam no gate do consumidor.
- [ ] Se houver `ETag`, ele é calculado sobre os bytes finais e o cache respeita a política de
      compressão e personalização.

## O que não precisa fazer

- Não crie `build.rs`, diretório `templates/` ou gerador de views.
- Não habilite `no_std` na aplicação; o Axum usa `std`.
- Não implemente `IntoResponse` para `Page`; use o wrapper.
- Não use `html-auy-macros` diretamente.
- Não habilite `audit` em produção; use como dev-dependency/CI.

## Limites que continuam valendo

- `Page` não carrega status, headers, cookies nem `ETag`.
- `Form::open` não valida CSRF, sessão ou rate limit.
- `Asset` não tem `integrity`/SRI e não escolhe cache.
- O crate não serve arquivos, não comprime e não define CSP.
- Contraste, alvo de toque, foco e layout continuam gates de navegador.

---

[← Arquitetura](15-arquitetura.md) · [Índice](README.md) · [Próximo →](17-guia-de-implementacao.md)
