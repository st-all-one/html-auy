[← Acessibilidade](11-acessibilidade.md) · [Índice](README.md) · [Próximo →](13-testes-e-diagnostico.md)

# 12 — HTTP, cache e resposta parcial

**Em uma frase.** O crate entrega bytes e um modelo tipado de resposta parcial; status, headers,
`ETag`, `303` e `Vary` são do adaptador HTTP do consumidor.

## `Page` não é `Response`

`Page` carrega apenas o documento. Um handler decide:

```rust
struct Response {
    status: u16,
    content_type: &'static str,
    etag: String,
    body: String,
}

fn respond(page: Page) -> Response {
    let body = page.markup().as_str().to_owned();
    let etag = format!("\"{}\"", hex_sha256(&body));
    Response {
        status: 200,
        content_type: "text/html; charset=utf-8",
        etag,
        body,
    }
}
```

- `Content-Type` deve incluir `charset=utf-8`; o crate emite UTF-8.
- `Page::markup()` empresta; `into_markup().into_string()` evita cópia se você já vai consumir.
- O crate não define status, cookies, HSTS, `X-Content-Type-Options` ou `Referrer-Policy`.

## Status que importam

| Situação | Status | Corpo |
|---|---|---|
| leitura | `200` | `Page` |
| escrita concluída | `303` + `Location` | vazio/redirect |
| validação de formulário | `422` | o **mesmo** formulário re-renderizado |
| não encontrado | `404` | página de erro do consumidor |
| conflito/replay divergente | `409` | erro do consumidor |
| corpo malformado | `400` | erro do consumidor |
| `HEAD` | `200` sem corpo | headers do `GET` |

`POST` que altera estado responde `303`; sem isso o F5 reenvia o formulário. Em `422`, preserve os
valores, marque erros por campo e inclua um resumo `role="alert"`.

## `ETag`, `304` e compressão

```text
ETag = '"' + sha256(bytes finais) + '"'
```

- O mesmo `ShellData`/`View`/dados geram os mesmos bytes; a ordem canônica é o que torna o `ETag`
  confiável.
- `If-None-Match` igual → `304`, sem corpo. Só depois de um `200`; não invente `304` para fragmento
  na primeira carga.
- `Vary` deve casar com o que o handler lê. Se a resposta muda entre página e fragmento:
  `Vary: HX-Request`; se lê o alvo, `Vary: HX-Request, HX-Target`.
- O crate não comprime. O adaptador pode aplicar `br`/`gzip` sobre os bytes finais; o `ETag` deve
  representar a representação servida (bytes comprimidos ou não, conforme sua política).
- Assets imutáveis com hash: `Cache-Control: public, max-age=31536000, immutable`. Manifest e
  service worker: URL fixa + `ETag`/`no-cache`.

## Resposta parcial: o modelo

O crate **não** emite `hx-*`; ele modela a intenção para o seu adaptador:

```rust
use html_auy::{Id, Partial, Swap, html};

let partial = Partial::new(html! {
    section id="cart" {
        h2 id="cart-title" { M::Cart }
        p { Text::new("Atualizado") }
    }
})
.update(
    Id::new("cart-count"),
    Swap::Inner,
    html! { Text::new("3") },
)
.focus(Id::new("cart-title"));
```

| Tipo | Uso |
|---|---|
| `Partial::new(main)` | substitui o alvo principal |
| `Partial::keep_main()` | não toca o alvo; só atualiza regiões |
| `.update(Id, Swap, Markup)` | atualização secundária, na ordem de chamada |
| `.focus(Id)` | foco pós-swap (título/controle, nunca o gatilho) |
| `.select(Id)` | seleção do corpo (`hx-select`/`HX-Reselect` no adaptador) |
| `.reselect(Id)` | seleção imposta pelo servidor |
| `.select_oob(Vec<Id>)` | regiões do corpo para OOB |

`Swap` cobre `Inner`, `Outer`, `BeforeBegin`, `AfterBegin`, `BeforeEnd`, `AfterEnd`, `Delete` e
`None`. `MainSwap` é `Replace(Markup)` ou `Keep`. `RegionUpdate` expõe `target()`, `swap()` e
`markup()`.

`Partial` não tem `Render` nem serializador: o adaptador do consumidor escolhe OOB, `<hx-partial>`
ou outro mecanismo. A ordem de `updates()` é a ordem de aplicação; preserve-a.

## Concorrência e classes de requisição

`Sync` é fechado:

| Valor | Uso |
|---|---|
| `Drop` | ignora o novo pedido enquanto houver um em voo |
| `Replace` | aborta o pedido em voo e começa o novo |
| `Abort` | ignora o novo se já houver um em voo; caso contrário, aborta este quando outro chegar |

`Queue` não existe: fila pode aplicar resposta antiga depois da nova.

`RequestClass::vary()` devolve o `Vary` que o handler deve emitir:

| Classe | `Vary` |
|---|---|
| `Full` | `""` |
| `Partial` | `HX-Request` |
| `Targeted` | `HX-Request, HX-Target` |
| `HistoryRestore` | `HX-Request, HX-History-Restore-Request` |
| `Boosted` | `HX-Request, HX-Boosted` |

## htmx, sem atributos no markup

O vocabulário não compila `hx-*`. Se você vendoriza htmx:

```rust
let data = ShellData::new(title, heading, lang)
    .htmx_config(HtmxConfig::safe())
    .assets(assets); // inclua /assets/htmx.min.js
```

`HtmxConfig::safe()` emite `<meta name="htmx-config">` antes dos assets com `allowEval:false`,
`allowScriptTags:false`, `includeIndicatorStyles:false`, `selfRequestsOnly:true`,
`withCredentials:false`, `historyCacheSize:0` e `timeout:10000`.

Regras de integração:

- duas formas, uma rota: página completa e fragmento;
- `Vary` coerente com o que o handler lê;
- `aria-busy="true"` na região durante a requisição, removido no swap;
- foco movido para o título/controle após o swap;
- `axe` depois do swap;
- `id` único no DOM; fragmento com `HX-Retarget` não repete o id do alvo;
- `4xx`/`5xx` não substituem o conteúdo principal; erro dentro da região com `role="alert"`;
- nada de `hx-on`, `js:` ou expressão avaliada.

## Cache de assets

| Asset | URL | Cache |
|---|---|---|
| CSS/JS com hash | `/assets/app.a1b2c3.css` | `public, max-age=31536000, immutable` |
| manifest | `/manifest.webmanifest` | `no-cache` + `ETag` |
| service worker | `/sw.js` | `no-cache` + `ETag` |
| tema | `/theme.css` | `public, no-cache` + `ETag` (se aplicável) |

O crate só declara o `<link>`/`<script>`; o servidor de arquivos e os headers são do consumidor.

## Exemplo de fluxo

```text
GET /carrinho
  → handler consulta dados
  → view = html! { … }
  → page = Page::new(&data, View::new(view))
  → 200 + ETag + text/html

POST /carrinho/itens
  → validação falha → 422 + mesmo formulário + erros
  → sucesso → 303 + Location: /carrinho

GET /carrinho (HX-Request)
  → Partial::new(fragmento).update(...).focus(...)
  → adaptador serializa + Vary: HX-Request
```

## Limites

- O crate não escolhe transporte, não lê `HX-*`, não escreve `Vary` e não define status.
- `Partial` não é uma resposta HTTP; é um valor para o adaptador.
- `Page` não tem `ETag`; calcule sobre os bytes finais.
- `Asset` não carrega `integrity`; SRI de `link`/`script` não é emitido no `0.1`.
- `style=` inline muda a CSP; trate como decisão de segurança, não como detalhe de HTTP.

Para o esqueleto completo de uma aplicação — Cargo, `PageResponse`, rotas, assets, erros e
formulários — veja [Projeto mínimo com Axum](16-projeto-minimo-axum.md).

---

[← Acessibilidade](11-acessibilidade.md) · [Índice](README.md) · [Próximo →](13-testes-e-diagnostico.md)
