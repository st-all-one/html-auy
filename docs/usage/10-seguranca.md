[← Conteúdo dinâmico](09-conteudo-dinamico.md) · [Índice](README.md) · [Próximo →](11-acessibilidade.md)

# 10 — Segurança

**Em uma frase.** Texto, atributo, URL, JSON-LD e nome de elemento têm mecanismos distintos; o
crate elimina injeção por construção e deixa `style=` como único escape hatch explícito.

## Modelo de ameaças

| Vetor | Exemplo | Mecanismo |
|---|---|---|
| texto | `<script>` em um título | `Text` escapa os 5 caracteres; `String` não vira `Markup` |
| valor de atributo | `"` em `value`/`alt` | `AttrVal`/tipo escapa os 5 + `\r\n\t` |
| URL | `javascript:alert(1)` | `Href` só constrói `http(s)`, `mailto`, `tel`, `LUrl`, `Fragment` |
| JSON-LD | `</script>` em um campo | `JsonLd` escapa `< > & '` para `\uXXXX` |
| CSS | cor/classe do usuário | `Class`/token; `style=` auditado |
| nome de elemento/atributo | `alt` com `onmouseover` | nome vem do macro, nunca de variável |

Nomes de elemento e atributo vêm do texto do `html!` e são validados contra `spec/elements.toml`.
Não há caminho para emitir um atributo cujo **nome** venha de dado.

## Escape por contexto

| Contexto | Caracteres | Forma |
|---|---|---|
| texto (`Text`) | `& < > " '` + NUL | `&amp; &lt; &gt; &quot; &#x27;` |
| atributo (`AttrVal`, tipos) | os 5 + `\r \n \t` | os 5 + entidades numéricas |
| JSON-LD (`JsonLd`) | `< > & '` + U+2028/U+2029 | `\u003c` etc. |
| URL (`LUrl`) | segmentos | percent-encoding `%XX` |
| classe | tokens | sem escape; tokens validados |
| comentário | — | não existe no vocabulário |

Nunca há escape duplo: `Text` já sai escapado; inserir `Text` dentro de `Text` é erro de tipo.
`/` não é escapado em texto/atributo; o caso perigoso (`</script>`) é resolvido pelo `JsonLd`.

## URLs

```rust
use html_auy::{ExternalUrl, Href, LUrl};

assert!(ExternalUrl::parse("javascript:alert(1)").is_none());
assert!(ExternalUrl::parse("data:text/html,<script>").is_none());
assert!(LUrl::new("//evil.example").is_none());
assert!(LUrl::new("/../etc/passwd").is_none());
assert!(LUrl::new("/a/%2e%2e/b").is_none());

let href = Href::external(ExternalUrl::parse("https://example.org").expect("https"));
```

- `ExternalUrl` aceita só `http`/`https`; o esquema é normalizado na emissão.
- `LUrl` é caminho interno, começa com `/`, recusa `//`, `:`, `\`, controle, espaço, `"`, `<`, `>`,
  crase e segmentos `..`/`%2e%2e`.
- `Src` aceita caminho validado ou `http(s)`; não aceita `javascript:`/`data:`.
- `SrcSet` reaplica a validação a cada variante.
- `Href::mailto`/`Href::tel` validam o formato; não são strings livres.

## Atributos

```rust
let safe = html! {
    div data_kind="news" aria_label=(Label::text(M::News)) { … }
};
```

- `aria_label` exige `Label`; um literal é erro de compilação.
- `data-*` é escalar; não coloque JSON serializado no atributo.
- Atributos proibidos no vocabulário: `on*`, `srcdoc`, `@*`, `hx-on*`, `hx-*`, `formaction`, `ping`,
  `slot`/`part`/`is`, `xlink:href`, `accesskey`, `draggable`.
- `style` **não** está na lista proibida: é escape hatch; veja abaixo.

## `style=` inline

```rust
let markup = html! {
    div style="margin-top: 1rem" { … }
};
```

- O valor é escapado como atributo; isso impede quebrar a aspa, **não** sanitiza CSS.
- Valor dinâmico (`style=(AttrVal::new(&css))`) é possível, mas só com dado confiável; CSS aceita
  `url()`, `@import`, `expression` histórica e outras superfícies.
- `audit` reporta `no_style_attribute`.
- CSP padrão (`style-src 'self'`) não permite `style=`; você precisa de `style-src-attr
  'unsafe-inline'` ou `'unsafe-hashes'` com hashes dos valores.
- Prefira `Class`/token. `<style>` não existe.

## JSON-LD é dado, não script

```rust
impl JsonGraph for Article {
    fn write_json(&self, out: &mut String) {
        // serialização do consumidor (serde/schemars)
    }
}

let ld = JsonLd::graph(&article);
ShellData::new(title, heading, lang).schemas(vec![ld]);
```

- `JsonLd` escapa `<`, `>`, `&`, `'` na construção.
- `</script>` no dado vira `\u003c/script\u003e`; não fecha o bloco.
- O crate emite `<script type="application/ld+json">…</script>` no fim do `<head>`.
- Validar o grafo (parse/SEO) é do consumidor; o gate do crate cobre o escape.

## `Render` e `AttrValue`: extensões de confiança

O crate expõe `Render` e `AttrValue` para composição. Implementar qualquer um é escrever bytes
confiáveis:

```rust
impl Render for Badge<'_> {
    fn render_to(&self, out: &mut String) {
        out.push_str("<span class=\"badge\">");
        self.label.render_to(out); // Text escapado
        out.push_str("</span>");
    }
}
```

Nunca escreva um `&str` de fora direto no `out`; use `Text`, `escape::text` ou um tipo de valor.
`Markup::from_render` não “sanitiza” nada: ele confia no `Render`.

## CSP de referência

```text
default-src 'self';
script-src 'self';
style-src 'self';
img-src 'self' data:;
font-src 'self';
connect-src 'self';
object-src 'none';
base-uri 'none';
form-action 'self';
frame-ancestors 'none';
script-src-attr 'none';
```

- O caminho padrão não tem inline; não precisa de nonce.
- Se usar `style=`, ajuste `style-src-attr` explicitamente.
- CDN exige allowlist de origem em `script-src`/`style-src`.
- `Asset` não emite `integrity`; SRI para `link`/`script` não é suportado por esse tipo no `0.1`.
  Trate a ausência como limite e prefira self-host com CSP; `Integrity` existe para `Preload`.
- Trusted Types pode ser exigido (`require-trusted-types-for 'script'`); o crate não usa
  `innerHTML`, e scripts são arquivos.
- Headers (HSTS, `X-Content-Type-Options`, `Referrer-Policy`) são do consumidor.

## O que o `audit` cobre

Feature `audit`:

```rust
let findings = html_auy::audit::check(document.markup().as_str());
assert!(findings.is_empty(), "{findings:#?}");
```

| Regra | O que detecta |
|---|---|
| `main_unique` | zero ou mais de um `<main>` |
| `single_h1` | zero ou mais de um `<h1>` |
| `heading_levels_no_skips` | `h2` → `h4` |
| `ids_unique` | `id` repetido |
| `landmark_names_distinct` | dois landmarks com o mesmo nome |
| `landmark_names_required` | `nav`/`aside` repetidos sem nome |
| `aria_refs_resolve` | `aria-labelledby`/`describedby`/`controls` para id inexistente |
| `popover_target_resolves` | `popovertarget` sem `popover` correspondente |
| `no_content_derived_identifiers` | `id`/`name` fora do alfabeto estável |
| `no_inline_script` | `<script>` sem `src` e sem `application/ld+json` |
| `no_style_attribute` | qualquer `style=` |
| `no_event_handler` | qualquer `on*` |

O `audit` **não** mede contraste, alvo de toque, foco visível, sobreposição ou layout; isso é gate
de navegador. Ele também não substitui tipos e macro; é a última camada.

## Cadeia de suprimentos

| Política | Detalhe |
|---|---|
| `unsafe` | zero no workspace (`#![forbid(unsafe_code)]`) |
| dependências normais | nenhuma no default; `audit` puxa `html5ever`/`markup5ever_rcdom` |
| macro | `proc-macro2` + `quote`; sem `syn` |
| licenças | `cargo deny` |
| versões | `Cargo.lock` para o gate local |
| MSRV | 1.92 |

## O que continua sendo seu

- autenticação, CSRF de sessão, rate limit;
- validação de upload e conteúdo;
- headers de segurança;
- política de cookies/analytics;
- revisão de HTML confiável (rich text) e do seu `Render`;
- CSP, SRI e allowlist de terceiros.

O crate garante que **nada do que ele emite** vira injeção por texto, atributo, URL ou JSON-LD. Ele
não promete que o seu `Render` ou o seu CSS sejam seguros.

---

[← Conteúdo dinâmico](09-conteudo-dinamico.md) · [Índice](README.md) · [Próximo →](11-acessibilidade.md)
