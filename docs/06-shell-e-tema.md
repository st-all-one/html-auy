# 06 — Shell, tema e atualização em runtime

> **Em revisão:** [`19`](19-revisao-arquitetural.md) §4.2 propõe que **tema e `/theme.css` saiam do
> crate** (viram contrato do consumidor); do crate fica `Page` + `Class`/`data-*`. As §3–§7 abaixo
> passariam a ser documentação de consumidor, não de crate.

## 1. A base estrutural é impossível de contornar

O crate não conhece framework: nada de `http`, nada de `axum`. A base estrutural é um **tipo**, e os
elementos de documento **não existem** no vocabulário do `html!` (`only = "shell"` no spec):

```rust
// html-core::shell — sem dependência de framework, sem I/O
pub struct Page { markup: Markup, status: u16 }

impl Page {
    /// Único jeito de produzir `<html>`: recebe o miolo e o envolve no shell.
    /// `ShellData` tem `title` e `heading` NÃO opcionais — `single_h1`/`title` são tipo, não teste.
    pub fn new(data: &ShellData<'_>, main: Markup) -> Self { /* … */ }
    pub fn markup(&self) -> &Markup { &self.markup }
}

// consumidor (6 linhas, o crate não participa):
impl IntoResponse for Responder {          // Responder { status, page: Page }
    fn into_response(self) -> Response { /* status + headers + page.markup() */ }
}
```

Consequências:

- Um handler **não consegue** devolver HTML sem passar pelo shell: `document`/`head`/`body`/`title`/
  `meta`/`link`/`script`/`noscript` **não compilam** em `html!` (`shell_only_elements`, escopo
  `block` — é erro de compilação, não convenção).
- `Page` carrega `status`, cabeçalhos de área e o `ETag` do corpo já renderizado.
- Uma view de módulo devolve `Markup` do **miolo**; o shell é o único lugar que emite `<html>`,
  `<head>`, `<header>`, `<main>`, `<footer>`.
- `POST` que altera estado responde **`303 See Other`** com `Location`, sempre — sem isso o F5
  reenvia o formulário e o usuário não tem como saber se a operação aconteceu.

### Ordem fixa do `<head>`

`charset` → `viewport` → `title` → `meta name="description"` → `link rel="canonical"`/`hreflang` →
`link rel="manifest"` → `preconnect`/`dns-prefetch` → `preload` do que entra no caminho crítico →
`link rel="stylesheet"` (`app.css`, `/theme.css`) → `script type="module"` com `defer` → JSON-LD.

A ordem não é estilo: `charset` nas primeiras 1024 posições é detectado mais cedo, e o CSS antes do
script evita FOUC. O **prefixo do `<head>`** (até o último `<script>` do head, já com CSS inline se
houver) tem limite absoluto de **14 KB comprimidos**: acima disso o primeiro pacote TCP se divide e o
ganho de streaming se perde.

### O que o shell resolve (uma vez por request)

`ShellCtx` é montado pelo consumidor antes de renderizar e contém:

| Campo | Origem | Uso |
|---|---|---|
| `locale` | URL | `lang`/`dir`, `LUrl`, `Msg` |
| `preset` + `regions` | dados ([ADR-016] equivalente) | presença de header/footer/aside, grid |
| `theme` | tokens resolvidos (ver §3) | `data-theme`, link do `/theme.css` |
| `a11y` | cookies | `data-a11y-*` no `<html>` (sem FOUC) |
| `nav` | manifesto de módulos | `nav` nomeado, um `aria-current` |
| `meta` | SEO (title/description/canonical/hreflang) | `<head>` |
| `jsonld` | `JsonLd` | `<script type="application/ld+json">` |
| `slots` | registries | `Vec<Markup>` ordenado |
| `main` | `View::main` | `Markup` (um só) |
| `modals` | `View::modals` | `Vec<Markup>` — contribuição devolvida, nunca pilha (D24) |
| `assets` | `ctx.require(...)` | `<script>`/`<link>` com hash |
| `search` | query sanitizada | `<search>` do header |

- **Slot não preenchido não emite nada** — nem placeholder, nem fallback silencioso, nem comentário.
  A região colapsa e o grid se resolve em CSS (mobile-first `fr` + `gap`); preencher "*quase*" um slot
  para segurar espaço é o caminho mais curto para `CLS` e para leitor de tela anunciando vazio.

### Gate que fecha o cerco (`layout-contract`)

Todo corpo renderizado tem: exatamente um `<main>`, um `<header>`, `<html lang>` válido, `<title>`
não vazio; nada de `<html>`/`<head>`/`<style>`/`style=` dentro do `<main>`; nenhum `<script>` sem
`src` (exceto `application/ld+json`).

## 2. Precedência de layout

A resolução é uma cadeia de sobreposições, avaliada **uma vez** e em ordem fixa:

```
default → global → preset → URL (profile) → instance
```

Regras:

- O resultado é um conjunto **completo** de campos (sem `Option` vazando para a view).
- Valor inválido em qualquer camada é **ignorado** (a camada anterior vence). Nunca quebra a página.
- O token final é serializado em ordem canônica → o `ETag` do `/theme.css` só muda quando o valor muda.

## 3. Tema = dados, nunca CSS no banco

```toml
# settings.theme (jsonb, schema tipado)
[theme.tokens]
primary    = "oklch(0.55 0.14 250)"
surface    = "oklch(0.98 0.01 250)"
on_primary = "oklch(0.99 0 0)"
radius     = "0.5rem"
[theme.sections]
"home-hero"  = { background = "primary", text = "on_primary" }
"news-card"  = { accent = "primary" }
```

O que o schema **aceita**: escalas e tokens semânticos, valores `oklch()` validados. O que **não
aceita**: CSS, HTML, `style`, cor fora da paleta, unidade física.

## 4. O arquivo `/theme.css`

```
GET /theme.css              → 200 + ETag (sha256 do CSS canônico) | 304
Cache-Control: public, no-cache      # revalida sempre; 304 é barato
```

Conteúdo (ordem canônica, tokens em `BTreeMap`):

```css
:root{--tenant-primary:oklch(0.55 0.14 250);--tenant-surface:oklch(0.98 0.01 250);…}
[data-section="home-hero"]{--section-bg:var(--tenant-primary)}
[data-section="news-card"]{--card-accent:var(--tenant-primary)}
.tone-primary{background:var(--section-bg);color:var(--tenant-on-primary)}
```

Propriedades garantidas:

- **sem inline** → CSP intacto e página cacheável;
- se não há override, o `<link>` **não é emitido** (zero request extra);
- determinístico: dois tenants com o mesmo tema têm o mesmo `ETag`;
- `304` por revalidação, não por `immutable` (o tema muda sem deploy).

### 4.1 Contrato do CSS emitido

Vale para `app.css` e para `/theme.css` — os dois únicos arquivos de estilo do shell.

```css
@layer tokens, reset, base, components, utilities, a11y, print;
```

1. **Nada fora de camada.** Regra sem `@layer` vence qualquer camada; o lint do CSS gerado exige zero
   declarações fora de camada e zero `@import`.
2. **`a11y` antes de `print`, e ambas depois de `utilities`.** A camada de acessibilidade nunca é
   sobrescrita pelo tema do tenant; `print` pode, porque só existe em mídia de impressão.
3. **`@property` para todo token que tenha sintaxe possível**, sempre com `initial-value` — sem isso o
   token é string opaca (não anima, não valida, não herda com tipo). E o motivo é concreto: `var()`
   com valor inválido cai no **valor inicial**, não no herdado — sem `@property`, um token digitado
   errado vira preto silenciosamente em vez de erro no save.
4. **Cor canônica:** `oklch(L C H[ / A])`, sem notação ad hoc. `hex`, nome de cor, `hsl()`/`rgb()` são
   rejeitados no schema do tema.
5. **Unidade:** `rem`, `ch`, `%`, `fr`, `dvh`, `clamp()` com mínimo em `rem`. `px` só em `@media print`;
   unidade física nunca.
6. **No máximo dois `<link rel="stylesheet">` render-blocking** (`app.css` + `/theme.css` quando há
   override). Preload/prefetch só com `crossorigin` explícito.
7. **Padrões proibidos no CSS emitido:** `order` e `flex-direction: *-reverse` (a ordem visual deixa de
   ser a ordem do DOM — WCAG 1.3.2/2.4.3), `!important`, `display: contents` em elemento focável,
   `content-visibility: auto` sem `contain-intrinsic-size` (a altura colapsa e o CLS estoura),
   **`display: grid`/`flex` em `<table>`** e **`list-style: none` em `ul`/`ol`** — os dois destroem a
   semântica nativa (tabela deixa de ser tabela, lista deixa de ser lista) e não há `role` no
   vocabulário para repor o que se perdeu (D4).

## 5. Cor de elemento individual: identidade, não CSS

O elemento nunca recebe uma cor. Recebe um **nome**:

```rust
html! { section(data_section: SectionId::HomeHero) { … } }
html! { div(class: Class::of(&[Token::Card]), data_tone: Tone::Primary) { … } }
```

- `Tone` é enum fechado; `SectionId` é enum por módulo.
- O mapa `seção → token` é dado, validado; o CSS é derivado e entra no mesmo `/theme.css`.
- Tentar `style="background:#ff0"` **não compila** (o atributo não existe).
- Hex vindo do banco é rejeitado no schema e o valor é ignorado (fail-safe).

Ganho de manutenção: "cor do bloco da home", "acento dos cards", "cor do rodapé" são três chaves de um
mapa — não N folhas inline, não cache fragmentado por página.

## 6. Preview (interatividade real, sem publicar)

| Endpoint | Cache | Fonte |
|---|---|---|
| `GET /theme.css` | `public, no-cache` + `ETag` | tema **publicado** |
| `GET /theme.css?draft=<id>` | `private, no-store` | rascunho da sessão |

- O editor altera o rascunho, recarrega o iframe de preview (mesmo shell, mesmo CSS) e vê a mudança.
- Publicar = trocar a fonte do endpoint público e invalidar o cache do tema; a próxima request recebe
  outro `ETag`. **Sem deploy, sem restart.**
- O preview nunca contamina o cache público: `no-store` + sessão.

## 7. Contraste é verificado no save

- A paleta é ampla, mas **bounded**. Cada par (texto, fundo) é validado em 4.5:1 (texto) e 3:1
  (não-texto) **ao salvar**.
- Valor que quebra o mínimo é rejeitado com a razão (par + taxa medida).
- A camada de acessibilidade (`@layer a11y`, sempre última) continua por cima: alto contraste P&B,
  escala de fonte, espaçamento. O tema do tenant nunca ganha da a11y.

## 8. O que o shell garante, em lista

0. `<!DOCTYPE html>` é o primeiro byte, `<html>` é o primeiro elemento e `<head>` o primeiro filho —
   é o único gatilho de modo "no-quirks"; sem ele o box model e o `height: 100%` mudam.
1. Nenhuma página existe sem base estrutural (tipo, não lint).
2. `data-a11y-*` renderizado no servidor a partir de cookie → sem FOUC.
3. `lang`/`dir`/`hreflang`/`canonical` do locale ativo.
4. Um único `<h1>` (o shell é dono; módulo não declara heading de página).
5. Ordem do DOM = ordem visual (sidebar por CSS lógico).
6. `<script>`/`<link>` só com `src`/`href` e hash; ordem determinística por `BTreeSet`.
7. `ETag` do corpo calculável a partir do `Markup` final.
8. Somente o shell conhece `Ctx`; módulos recebem apenas os dados do seu domínio.

## 9. Modos de falha conhecidos

| Falha | Efeito | Mitigação |
|---|---|---|
| Tema com valor inválido | campo ignorado; camada anterior vence | validação no save + fail-safe |
| `SectionId` sem entrada no mapa | cai no token global | default explícito no resolver |
| Asset declarado e não existente | falha no teste de assets do consumidor | gate de assets |
| `ctx.require` chamado depois do shell | impossível: `shell()` consome o `Ctx` | assinatura |
