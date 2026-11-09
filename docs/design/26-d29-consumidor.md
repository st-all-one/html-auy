# 26 — D29 na prática: tema e shell de consumidor

> **Decisão:** o crate gera HTML puro e não faz I/O; tema, CSS servido, `ETag`, cache e shell de
> tenant ficam no consumidor ([`13`](13-decisoes.md) D29). Este documento mostra **como o consumidor
> implementa** e **como seria** se a responsabilidade voltasse para o crate — para que a recusa seja
> auditável, não dogmática.

## 1. O que fica de cada lado

| Responsabilidade | Onde fica | Por quê |
|---|---|---|
| `Page`, `ShellData`, `View`, `Form`, `Field`, `Img`/`Video`/`Audio` | crate | é geração de bytes determinística, sem ambiente |
| tokens de tema (cor, espaço, tipografia) | consumidor | são dados do produto, mudam por tenant |
| CSS compilado e `/theme.css` | consumidor | exige build, hash, `Content-Type`, compressão |
| `ETag`, `304`, `Cache-Control` | consumidor | depende de HTTP e da política de sessão |
| `manifest.webmanifest` | consumidor | arquivo servido; o crate só emite o `link` via `ShellData::manifest` |
| service worker (`/sw.js`) | consumidor | estratégia de cache é do produto |
| locale, catálogo `Msg`, `old()`/`$errors` | consumidor | o crate não lê ambiente (D31) |

O crate só conhece **dados já decididos**: `Class`, `data-*` tipados e `Markup`. A fronteira é a
mesma de [`20`](20-fronteira-de-aplicacao.md).

## 2. Implementação no consumidor

### 2.1 Tokens → CSS

```rust
// app/src/theme.rs — dados do produto, não do crate
pub struct Theme {
    pub bg: &'static str,
    pub fg: &'static str,
    pub accent: &'static str,
    pub radius: u16,
}

impl Theme {
    /// CSS determinístico: a ordem dos tokens é a ordem do `BTreeMap`/array, nunca a de um `HashMap`.
    pub fn css(&self) -> String {
        format!(
            ":root{{--bg:{bg};--fg:{fg};--accent:{accent};--radius:{r}px}}",
            bg = self.bg,
            fg = self.fg,
            accent = self.accent,
            r = self.radius,
        )
    }
}
```

O consumidor compila esse CSS no build, grava `theme.<hash>.css` e registra o asset em
`ShellData::assets`:

```rust
let assets: Assets = [Asset::style("/assets/theme.9f2c1a.css").expect("asset")]
    .into_iter()
    .collect();
ShellData::new(title, heading, lang).assets(assets)
```

O `Page` emite `<link rel="stylesheet" href="/assets/theme.9f2c1a.css">` na posição fixa do `<head>`;
o hash no nome permite `Cache-Control: public, max-age=31536000, immutable` ([`09`](09-desempenho-e-cache.md) §2.1).

### 2.2 `ETag` e cache

```rust
use sha2::{Digest, Sha256};

fn respond(page: Page) -> Response {
    let body = page.into_markup();
    let etag = format!("\"{}\"", hex(Sha256::digest(body.as_str().as_bytes())));
    Response::new(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .header("etag", etag)
        .header("cache-control", "public, no-cache")
        .body(body.as_str().to_owned())
}
```

Regras que o consumidor não pode relaxar:

- o hash é dos **bytes finais**, não da entrada;
- `If-None-Match` igual devolve `304` sem corpo;
- HTML autenticado usa `private, no-cache`, nunca `no-store` (quebra `bfcache`);
- `/sw.js` e `/manifest.webmanifest` usam URL fixa + `no-cache` + `ETag`;
- assets com hash no nome podem ser imutáveis.

### 2.3 Manifest e tema no `<head>`

```rust
ShellData::new(title, heading, lang)
    .manifest(LUrl::new("/manifest.webmanifest").expect("caminho interno"))
    .assets(assets)
```

O `link rel="manifest"` sai entre `canonical` e `alternates` ([`06`](06-shell-e-tema.md)). Ícones,
`theme-color` e `apple-mobile-web-app-capable` ainda são pós-MVP; quando entrarem, serão campos
tipados do `ShellData`, não string solta.

## 3. Como seria dentro do crate (e por que não)

O desenho possível seria:

```rust
// hipotético: NÃO implementado
pub struct Theme<'a> { /* tokens já validados */ }
impl<'a> Theme<'a> {
    pub fn css(&self) -> ThemeCss;              // Markup de CSS? o crate não emite <style>
    pub fn asset(&self) -> Asset;               // exigiria hash de conteúdo no crate
}

pub enum AssetKind { Style, Script, Theme }     // `Theme` exigiria I/O de arquivo
pub struct ShellData<'a> { /* …, theme: Option<Theme<'a>> */ }
```

O problema não é a API: é o que ela arrasta.

1. **Hash de conteúdo** exige ler o arquivo CSS ou receber o CSS inteiro; o crate passaria a
   depender de `std::fs`/build script, contra `#![no_std]` + sem I/O.
2. **`ETag`/`304`/`Cache-Control`** são HTTP, e o crate não conhece status nem headers
   ([`20`](20-fronteira-de-aplicacao.md)).
3. **Tema por tenant** viraria estado ambiente (request → tema), exatamente o que D29/D31 recusam.
4. **Teste de bytes** ficaria dependente do filesystem do consumidor; hoje o crate testa apenas
   `Markup` puro.

Se um dia o crate absorver tema, a forma defensável é só **dados**: `Theme` com tokens e
`ThemeCss` construído a partir deles, sem ler arquivo e sem escolher `Cache-Control`. O consumidor
continuaria dono do asset servido e do `ETag`. Enquanto essa necessidade não aparecer, a fronteira
atual é a que mantém o crate vendorizável e testável sem ambiente.

## 4. Gate do consumidor

| Teste | Prova |
|---|---|
| render duas vezes → mesmos bytes | `ETag` estável |
| `theme.css` tem hash no nome e `immutable` | cache de longo prazo |
| HTML autenticado responde `private, no-cache` | sem vazamento por cache compartilhado |
| `manifest` e `/sw.js` respondem `no-cache` + `ETag` | atualização sem URL nova |
| página sem tema não emite `link` de tema | ausência é ausência, não placeholder |
