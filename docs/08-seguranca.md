# 08 — Segurança

## 1. Modelo de ameaças do HTML

Todo HTML gerado sofre cinco classes de injeção. O projeto trata cada uma com um mecanismo diferente,
e nenhuma depende de disciplina.

| Vetor | Exemplo | Mecanismo |
|---|---|---|
| **texto** | `<script>` em um título de notícia | `Text` escapa os 5 caracteres; `String` não vira `Markup` |
| **valor de atributo** | `"` em `alt`/`title` | `AttrVal` escapa os 5 + `\r\n\t` |
| **URL** | `javascript:alert(1)`, `data:text/html,…` | `Href` só constrói `https?`, `mailto`, `tel`, `LUrl`, `Fragment`; esquema normalizado |
| **script/data** | `</script>` dentro de JSON-LD | `JsonLd` escapa `<` `>` `&` `'` para `\uXXXX` |
| **CSS** | cor/classe controlada pelo usuário | `Token`/`Tone`/`SectionId` são enums; `style=` inline é escape hatch auditado, nunca com dado não confiável |

## 2. Nome de elemento e de atributo nunca vêm de dado

- Nome de elemento: vem do texto do `html!`, validado contra `spec/elements.toml`.
- Nome de atributo: idem. Não existe caminho para emitir atributo cujo nome venha de uma variável.
- Consequência: "attribute injection" (`alt` com `" onmouseover="`) é impossível por construção —
  mesmo o valor já é escapado, e o nome nunca é dinâmico.

## 3. URLs

```rust
pub enum Href {
    Internal(LUrl),              // caminho interno validado (`/noticias/…`)
    Fragment(Fragment),          // `#` ou `#id`; não sai da página
    External(ExternalUrl),       // http/https; esquema hostil não constrói
    Mailto(MailAddr),            // valida formato
    Tel(PhoneNumber),            // valida E.164
}
```

- **Não existe** `Href::new(&str)`. Não há como construir um `href` arbitrário.
- `javascript:`, `data:`, `vbscript:`, `file:` são recusados na construção — com erro de tipo, não de
  runtime.
- `ExternalUrl` aceita `http`/`https` (e recusa o resto: `javascript:`, `data:`, `vbscript:`,
  `file:`); a emissão normaliza o esquema para minúsculas.
- `LUrl` é caminho interno validado (`new`/`with_segments`, com `..` e `%2e%2e` recusados). O locale
  e a tabela de rotas são helpers do consumidor (C2): o `LUrl` é o tipo, não o roteador.
- `Src` aceita caminho interno validado (`Src::media`, `Src::asset`) **ou** URL absoluta `http(s)`
  (`Src::parse`); `javascript:`, `data:`, `vbscript:` e `//host` não constroem. `srcset` só via
  `SrcSet::from_variants`, que reaplica a mesma gramática a cada variante.

## 4. CSS

| Superfície | Permitido | Proibido |
|---|---|---|
| classe | `Token` (enum fechado), `Class::new`, `class="a b"` estático | classe vinda de string não estática |
| cor | `Tone`/`SectionId` → token do tema | hex, `rgb()`, nome de cor |
| estilo inline | `style=` com dado confiável, contra-recomendado e auditado | `<style>` |
| tema | tokens validados por schema, `oklch` | CSS, HTML, `url()`, `@import` |

`url()` no tema é recusado explicitamente: é o vetor clássico de exfiltração via CSS.

## 5. Cadeia de suprimentos

| Política | Detalhe |
|---|---|
| `unsafe` | **zero**, em todo o workspace (`#![forbid(unsafe_code)]`) |
| dependências de produção | `html-auy`: nenhuma externa; tabela gerada: nenhuma dep; `html-auy-macros`: `proc-macro2` + `quote` |
| dependências de desenvolvimento | `html5ever` (lint), `insta`, `trybuild`, `cargo-fuzz` |
| versões | declaradas no manifesto; `Cargo.lock` versionado para o gate local |
| auditoria | `cargo deny` (licenças permissivas, sem duplicata, sem yanked) |
| MSRV | declarada; CI tem job na MSRV (1.92) e na stable; beta não é matriz de suporte |

Observação de propósito: `html-auy-macros` **não** usa `syn` — o fluxo é HTML; um parser próprio de ~1,2k
LOC é menor e auditável, e evita a superfície de uma dependência grande em posição privilegiada
(dentro de macro de compilação).

## 6. Postura de CSP

```
default-src 'self';
script-src 'self';
style-src 'self';
img-src 'self' data:;          # data: só para ícone inline, se o consumidor precisar
font-src 'self';
connect-src 'self';
object-src 'none';
base-uri 'none';
form-action 'self';
frame-ancestors 'none';
```

- Sem `nonce`: o caminho padrão não tem inline, e página cacheável não pode carregar nonce.
- `style=` inline quebra a política padrão: exige `style-src-attr 'unsafe-inline'` (fraco) ou
  `'unsafe-hashes'` com hashes dos valores. É escape hatch, não o caminho recomendado.
- Sem `unsafe-eval`: nenhum caminho no vocabulário exige avaliação dinâmica (htmx só com atributos
  declarativos — ver [`07`](07-js-e-assets.md) §7).
- Trusted Types: como não há `innerHTML` no markup gerado e scripts são módulos próprios, a política
  pode ser `require-trusted-types-for 'script'` no consumidor — e com `default` policy **proibida**
  (uma `default` policy sanitiza em silêncio e esconde a origem do HTML não confiável).
- Diretivas que faltam nos guias e entram aqui: `script-src-attr 'none'` (bloqueia `on*` por atributo
  mesmo se algo escapar), `style-src-attr 'none'` (só enquanto não houver `style=` inline),
  `frame-src 'none'` quando não há iframe permitido,
  `form-action 'self'` (já acima) e `upgrade-insecure-requests`.
- Fetch Metadata: aceitar `Sec-Fetch-Site: same-origin`, **não** `same-site` — subdomínio comprometido
  é same-site e passaria na checagem.
- DOM clobbering: `id`/`name` nunca vêm do conteúdo (invariante `no_content_derived_identifiers`).
  Um `<a id="location">` no HTML de um artigo reescreve `window.location` para o script da página,
  e o defeito não aparece em nenhum scan de atributo.

## 7. O que o projeto **não** decide

- Cabeçalhos HTTP (HSTS, `X-Content-Type-Options`, `Referrer-Policy`) — do consumidor.
- Autenticação, CSRF, rate limit — do consumidor.
- Criptografia de dado em repouso — do consumidor.
- Conteúdo de texto (se um título é ofensivo) — do consumidor.
- Privacidade (consentimento, cookies, analytics, retenção) — do consumidor. O crate não abre
  slot para script de terceiro: o `<head>` é fechado e `Asset::script` só aceita caminho interno,
  então rastreamento não entra no markup por acidente.

O crate garante que **nada do que ele emite** vira injeção. O resto é fronteira do consumidor.

## 8. Regressões de segurança: como não voltar atrás

| Garantia | Teste |
|---|---|
| `Markup` não é construível de fora | `trybuild` (tentativa de `Markup::from(String)` falha) |
| `Raw`/`PreEscaped` não existem | `cargo doc` + grep no código exportado |
| escape dos 5 caracteres em texto | property test (proptest) sobre alfabeto hostil |
| escape de JSON-LD | fixture com `</script>` e `<` em título |
| `javascript:` recusado | `trybuild` + teste de unidade em `Href` |
| nenhum `on*` no vocabulário; `style=` só escape hatch | teste de conformidade da spec + `audit` |
| nenhum `<script>` inline na saída | `html-lint` (N4) |
| nenhuma origem externa fora da allowlist | `html-lint` + gate de assets |
| `Cargo.lock` sem `unsafe` novo em dep | `cargo geiger`/`cargo deny` no CI |
