# 21 — Plano de implementação

> **Documento histórico.** O plano F0–F6 foi concluído no 0.1.0; os nomes de pacote abaixo seguem D35. O estado atual está em [`01`](01-arquitetura.md).
>
> **O que é:** a ordem executável para escrever o crate, já com as decisões fechadas
> (**D21–D26**) e as decisões fechadas (**D27–D32**). Cada fase é um verde de `cargo test`.
> **O que não é:** cronograma. Os números são de código, não de calendário.

## 0. Para começar

> **Critério de leitura:** o uso final está escrito em [`25`](25-exemplo-de-uso.md). Se o exemplo de lá
> não compilar, não gerar os bytes mostrados e não falhar onde está dito, a fase correspondente não está
> pronta — é a definição de "aceito" que vale para F0–F6.
>
> **MVP = F-1 → F6 + F6.1.** O corpus de conformidade (**F7**) e a exportação para outros ecossistemas
> ([`24`](24-exportacao.md)) são **pós-MVP**: estão documentados para não se perderem, mas **não
> bloqueiam nenhuma fase e não entram no primeiro ciclo**. Tema/shell de consumidor (**D29**) também fica
> fora — o crate nasce sem I/O e sem cache.

### 0.1 Pré-requisitos (estado verificado)

| Pré-requisito | Como se verifica | Estado |
|---|---|---|
| `spec/elements.toml` (111 elementos, 71 invariantes) | `python3 tools/validate_spec.py` → `0 erro(s)` | ✅ pronto |
| `docs/00`–`25` + `tools/` (validador de spec, checador de links) | `python3 tools/check_links.py` → `0 quebrado(s)` | ✅ pronto |
| `sniff-css/` **fora** do workspace (workspace próprio, `resolver` próprio) | `ls sniff-css/Cargo.toml` | ✅ isolado |
| `.gitignore` cobrindo `target/`, `*.snap.new`, `fuzz/artifacts/` | `cat .gitignore` | ✅ pronto (corpus do fuzz é versionado) |
| **repositório git** | `git rev-parse` | ✅ inicializado |
| toolchain **1.99.0** (o pin que [`22`](22-rigor.md) §2 fixa) | `rustc -V` → `1.99.0` | ✅ instalado e igual ao pin |
| `clippy` + `rustfmt` no 1.99 | `rustup component list --installed` | ✅ instalados |
| **MSRV 1.92** (job de CI da matriz) | `rustup toolchain list` | ✅ instalado (`cargo +1.92 test`) |
| `miri` no nightly (job `miri`) | `rustup component list --installed --toolchain nightly` | ✅ instalado |
| `cargo-deny` · `cargo-audit` · `cargo-machete` · `cargo-fuzz` (nightly) | `--version` | ✅ no CI; `typos` saiu (dicionário inglês × corpus pt-BR) |
| decisões **D27–D32** | [`13`](13-decisoes.md) | ✅ decididas em 2026-11-06 (§0.4) |

**Estado:** o ambiente atende ao [`22`](22-rigor.md); o workspace tem `html-auy`, `html-auy-macros` e
`html-auy-macros-core`, e o `fuzz/` compila com `cargo +nightly build --manifest-path fuzz/Cargo.toml`.

### 0.2 Primeiro commit (árvore exata)

Nada de código de domínio ainda: este commit é o **rigor** ([`22`](22-rigor.md)), e ele só é aceitável se
o workspace compilar **vazio**.

```
Cargo.toml                 # [workspace] members = crates/*, resolver = "3", lints, rust-version
rust-toolchain.toml        # channel 1.99 (pin) + clippy + rustfmt
clippy.toml                # msrv = "1.92" + limiares + allowlists
rustfmt.toml               # só opções estáveis (+ hex_literal_case = "Lower")
deny.toml                  # advisories/yanked deny, licenças permissivas, wildcard/registry deny
.github/workflows/ci.yml   # jobs: check, supply-chain, miri, fuzz
crates/html-auy/Cargo.toml     # publicável, rust-version = 1.92, zero deps por padrão (feature `audit`)
crates/html-auy/src/lib.rs     # no_std + extern crate alloc + doc de topo (nada mais)
crates/html-auy-macros/Cargo.toml
crates/html-auy-macros/src/lib.rs   # proc-macro vazio + doc de topo
```

Aceitação do primeiro commit (rodar **duas vezes**, ADR-001):

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### 0.3 O que **não** fazer no início

| Não | Por quê |
|---|---|
| `syn` no `html-auy-macros` | o fluxo é HTML, não Rust; `proc-macro2` + `quote` bastam ([`01`](01-arquitetura.md) §1) |
| `build.rs` / gerador / esquema de spec | D27 corta: a tabela vive em Rust (`macro_rules!`/`const`) |
| binário `cq` (`fmt`/`fix`/`audit`/`spec`) | D28 corta; o gate é `cargo` puro |
| tag/release sem gate | D35: o workflow `release.yml` verifica versão, testes e empacotamento antes de publicar |
| F7 (corpus) e [`24`](24-exportacao.md) (exportação) | pós-MVP; sem eles o crate funciona |
| tema, shell de tenant, cache | D29: o crate não faz I/O |
| `#[allow]` para "depois eu tiro" | `allow_attributes_without_reason` é `deny`; exceção é `#[expect(…, reason)]` |

### 0.4 Decisões fechadas (2026-11-06)

Todas as pendências foram decididas e estão registradas em [`13`](13-decisoes.md):

| ID | Decisão | Efeito em §1.1 e nas fases |
|---|---|---|
| **D27** | TOML normativo + `tools/gen_spec.py` + teste de paridade | sem `build.rs`; `spec/table.rs` versionado |
| **D28** | sem binário `cq` | gate = `cargo fmt/clippy/test` |
| **D29** | tema/shell de consumidor fora do crate ([`26`](26-d29-consumidor.md)) | F5 expõe `Page`+`ShellData`, nada de CSS/`ETag` |
| **D30** | `N4` como módulo atrás de `feature = "audit"` | html5ever opcional; consumidor roda no CI dele |
| **D31** | `Msg` é trait do consumidor; `Text::msg(m)` | F0 não conhece locale |
| **D32** | pin `1.99`; MSRV `1.92` em três lugares | F-1 fixa os dois números |

**Nada disso reverte o critério de parada:** o `M1`/`M3` de [`16`](16-roadmap.md) continua valendo como
teste de que o desenho menor paga — se o `M3` não achar ≥ 3 defeitos reais, o corte foi caro e a reversão
é da peça específica, não da tese.

### 0.5 Ordem do primeiro dia

1. `git init` (o **humano** commita) e o primeiro commit de §0.2.
2. **F-1** fechada: gates verdes duas vezes no workspace vazio.
3. **F0** — tipos e valores, **sem macro**; é a fase do **M1** ([`16`](16-roadmap.md)) e o **ponto de
   parada**: se F0 não pagar, o plano B é maud + estes mesmos valores (`html-auy` de ~600 LOC), e F2–F6 não
   começam.

## 1. Regras de execução

| Regra | Consequência prática |
|---|---|
| workspace com **3 crates** (`html-auy`, `html-auy-macros`, `html-auy-macros-core`) | `html-auy` só depende de `core`/`alloc`; o núcleo do macro depende de `proc-macro2`+`quote` (nunca `syn`) |
| gate único: `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` + `cargo test` | não há binário de tarefas: o `cq` não existe (D28 pendente; até lá, nada depende dele) |
| **1 mensagem de erro = 1 caso `trybuild`** | `crates/html-auy-macros/tests/ui/*.rs` + `.stderr`; o texto de [`10`](10-ergonomia.md) §2 é contrato |
| **1 saída canônica = 1 snapshot `insta`** | `crates/html-auy/tests/snapshots/`; bytes determinísticos são testados, não prometidos |
| nenhum `HashMap` em ordem de saída | `BTreeMap`/`Vec` ordenado; sem tempo, sem `rand`, sem ordem de FS (ADR-001 do projeto pai) |
| uma fase por vez, sempre verde | fase 4 não começa com fase 3 vermelha |
| `sniff-css/` é workspace **independente** | tem `Cargo.toml` próprio e `resolver` próprio: não entra em `members`, não é tocado por `cargo fmt --all`/`clippy --workspace` daqui |

### 1.1 `Cargo.toml` de partida

```toml
# Cargo.toml (workspace)
[workspace]
members = ["crates/html-auy", "crates/html-auy-macros", "crates/html-auy-macros-core"]
resolver = "3"
# `exclude` explícito: `sniff-css/` e `spec/` não são membros


[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
```

```toml
# crates/html-auy/Cargo.toml
[package]
name = "html-auy"
edition = "2024"
rust-version = "1.92"          # MSRV; o pin do toolchain vive no rust-toolchain.toml

[lints]
workspace = true

[dependencies]                 # nenhuma, por padrão: só core/alloc
html5ever = { version = "0.27", optional = true }   # `output` (D30)

[features]
audit = ["dep:html5ever"]      # `N4` chamável pelo consumidor, fora do binário sem a feature

[dev-dependencies]
insta = "1"
proptest = "1"
```

```toml
# crates/html-auy-macros/Cargo.toml
[package]
name = "html-auy-macros"
edition = "2024"
rust-version = "1.92"

[lib]
proc-macro = true

[lints]
workspace = true

[dependencies]
proc-macro2 = "1"
quote = "1"                    # nunca `syn`: o fluxo é HTML, não Rust

[dev-dependencies]
trybuild = "1"
```

## 2. Estado de partida

- `spec/elements.toml`: **111 elementos**, **71 invariantes** (59 `block` / 4 `type` / 8 `output`),
  `only`: 9 `shell` (`html head body title meta link script noscript main`) + 1 `component` (`form`).
- `docs/00`–`24`: contrato completo (o que é garantia, o que é gate, e por quê).
- `tools/validate_spec.py`: **a especificação executável do validador** — categorias, conteúdo-modelo,
  globais, escopos, `only`, rastreabilidade com `docs/04`, contagem nos docs. É a referência a portar;
  quando o passe de validação existir em Rust, o script sai.

## 3. Fases

### F-1 — rigor antes de qualquer linha (≈0 LOC de código, 6 arquivos)

Não é código; é o chão. Absorvido de [`22`](22-rigor.md), derivado do `sniff-css`:

| Arquivo | Conteúdo |
|---|---|
| `rust-toolchain.toml` | `channel = "1.99"` (pin), `components = ["clippy", "rustfmt"]` |
| `Cargo.toml` | `[workspace.package] rust-version = "1.92"` (**MSRV**, ≠ pin), `resolver = "3"`, `[workspace.lints.rust]`/`[workspace.lints.clippy]`/`[workspace.lints.rustdoc]` |
| `clippy.toml` | limiares + allowlists (incl. `msrv` e `doc-valid-idents`) |
| `rustfmt.toml` | só opções estáveis + `hex_literal_case = "Lower"` |
| `deny.toml` | advisories/`yanked` deny, allowlist de licença permissiva, wildcard/registry deny |
| `.github/workflows/ci.yml` | jobs `check`, `supply-chain`, `miri`, `fuzz` |

**DoD.** `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` e `cargo deny check` verdes num
crate vazio; nenhum `#![allow]` no workspace; `unsafe_code = "forbid"`.

### F0 — valores tipados, sem macro (≈700 LOC)

**Objetivo:** o plano B formal. Se o macro falhar, isto já vale ([`16`](16-roadmap.md) M1).

| Arquivo | Conteúdo |
|---|---|
| `html/src/markup.rs` | `Markup`, `Render`, buffer único (sem alocação intermediária) |
| `html/src/escape.rs` | tabela dos 5 caracteres + entidades numéricas de controle |
| `html/src/value/*.rs` | `Text`, `AttrVal`, `Href`, `Src`, `SrcSet`, `Alt`, `WidthPx`, `HeightPx`, `Class`, `Tone`, `Label`, `Id`, `FieldName`, `JsonLd` |
| `html/src/msg.rs` | `pub trait Msg: Copy { fn as_str(self) -> &'static str; }` + `Text::msg(m)` (D31: o enum é do consumidor; o crate não conhece locale) |

**API (contrato do crate):**

```rust
pub struct Markup(/* buffer */);
impl Markup { pub fn render_to(&self, out: &mut String); }
pub trait Render { fn render_to(&self, out: &mut String); }

impl Alt<'_>  { pub fn new(s: &str) -> Self; pub fn decorative() -> Self; }        // sem vazio implícito
impl SrcSet<'_> { pub fn add(self, url: Src<'_>, w: WidthPx) -> Self; }           // ordena e deduplica
impl Class { pub fn of<T: ClassToken>(t: &[T]) -> Self; pub fn extend(self, o: Class) -> Self; } // bitset, ordem canônica
impl Href { pub fn internal(LUrl<'_>) -> Self; pub fn external(ExternalUrl<'_>) -> Self; /* mailto, tel */ }
```

**Testes.** `proptest` para escape (`<>&"'` + controle), `SrcSet` (ordem, dedupe, largura > 0), `Class`
(dedupe, ordem). `insta` para 3 saídas.
**DoD.** `cargo test` verde; `Markup` não implementa `IntoResponse` (não há dependência de `http`);
`grep -r http crates/html-auy` vazio.

### F1 — tabela de elementos + porte do validador (≈400 LOC)

**Objetivo:** a spec deixa de ser um arquivo lido só por Python. As **regras** (o que é válido) são as
mesmas em qualquer caminho — o que muda é onde a tabela mora:

| Caminho | Tabela | Gate |
|---|---|---|
| vigente (até D27) | `html-spec` gerado por `build.rs` a partir de `spec/elements.toml` | divergência = build quebrado (registro histórico) |
| **decidido (D27)** | `spec/elements.toml` **normativo** → `tools/gen_spec.py` (manual) → `crates/html-auy/src/spec/table.rs` **versionado** | **teste de paridade**: TOML ≠ tabela = CI vermelho; nenhum `build.rs` |

**Artefatos de F1:** `tools/gen_spec.py` (o gerador), `crates/html-auy/src/spec/table.rs` (saída, commitada) e
`crates/html-auy/tests/spec_parity.rs` (a prova de que a saída não está velha). Esquecer de rodar o gerador
deixa o CI vermelho, nunca o produto errado.

```rust
// html/src/spec/mod.rs
pub struct Element { pub name: &'static str, pub categories: &'static [Cat], pub required: &'static [&'static str],
                     pub allowed: &'static [&'static str], pub children: &'static [Cat], pub only: Option<Only>,
                     pub invariants: &'static [&'static str], pub void: bool }
pub static ELEMENTS: &[Element] = &[ /* 111 */ ];
pub enum Only { Shell, Component }
```

**Testes.** Os mesmos do `validate_spec.py`: 111 elementos; cada invariante usada tem escopo; `only`
exige a invariante correspondente (`shell_only_elements` / `form_requires_csrf`); `children` alcançável
a partir de `body`; `docs/04` cobre 100% das invariantes; contagem declarada em `docs/03`/`docs/04` bate.
**DoD.** `cargo test spec` cobre cada regra do script Python; o script passa a ser redundante (só é
removido quando o macro começar a validar — F3).

### F2 — parser (≈900 LOC, inclui congelar a gramática de atributo)

**Sintaxe** (a de [`10`](10-ergonomia.md) §1 e do exemplo de [`25`](25-exemplo-de-uso.md) §3–§5):
elemento `nome { … }`, elemento vazio `nome attr=(expr);` (o `;` fecha o `void`), atributo
**`nome=(expr)`** para valor calculado e
**`nome="literal"`** só quando a spec declara o atributo como enumerado (`type`, `method`, `rel`,
`target`, `inputmode`…), texto como **expressão nua** (`Msg::X`, `Text::new(&s)`) ou `{ expr }` quando vem
de um bloco, `@if/@else`, `for x in itens { … }`, e **primitivo do consumidor** como
`Caminho::funcao(args)` (sem filhos, devolve `Markup`) ou `Caminho::funcao(args) { filhos }` (o macro
passa os filhos como `Markup` — é como `Form::open`, `Field` e `EmptyState` entram na view). Os parênteses do valor **não são estilo**: delimitam a expressão no `TokenStream` (sem eles,
`href=x rel=y` é ambíguo).
**Erros com span:** tag não fechada, tag fechada errada, atributo sem valor, texto depois de `}`.
**Testes.** 20 snippets parseados; `trybuild` para 4 erros de sintaxe.

### F3 — validação (≈500 LOC) — **o coração**

Porte 1:1 das regras de `validate_spec.py`, agora com span no token:

| Regra | Mensagem (contrato de [`10`](10-ergonomia.md) §2) |
|---|---|
| elemento desconhecido | `error: elemento desconhecido \`dev\`; você quis dizer \`div\`?` |
| atributo desconhecido | `error: atributo desconhecido \`hreff\` em \`a\`` + lista de válidos |
| atributo obrigatório ausente | `error: \`img\` exige \`alt\`` + `regra:`/`ajuda:` |
| aninhamento ilegal | `error: \`<div>\` não é permitido dentro de \`<p>\`` + alternativa |
| valor fora do enum | `error: \`type="texto"\` não é válido em \`input\`` + valores aceitos |
| elemento de documento | `error: \`<html>\` só existe no shell: use \`Page::new(…)\`` |
| `<form>` à mão | `error: \`<form>\` não é elemento de \`html!\`` |

**Testes.** `trybuild`: **um caso por mensagem** (as 7 acima + os enums/aria + `label`/`id` + `srcset` +
`role`), `insta` para os `.stderr` (a mensagem é artefato versionado).
**DoD.** Nenhuma regra existe só no Python; `N4` (F6) não é necessário para `block`.

### F4 — codegen (≈250 LOC)

Ordem de atributos = ordem da spec; boolean atributo (`checked`) por presença; `Class` em ordem canônica;
`SrcSet` já ordenado; escape na escrita.
**Testes.** `insta` com 10 fragmentos; um teste de **bytes**: renderizar duas vezes dá o mesmo `String`.
**DoD.** `ETag` do consumidor é estável por construção.

### F5 — `Page`, `View`, `Form`, `Field` (≈430 LOC; D22–D25)

```rust
// html/src/shell.rs  — `ShellData` é DADO; tema/CSS/ETag ficam no consumidor (D29)
pub struct Regions { pub header: Option<Markup>, pub before_main: Option<Markup>,
                     pub after_main: Option<Markup>, pub footer: Option<Markup> }
pub struct ShellData<'a> { /* campos privados */ }
impl<'a> ShellData<'a> {
    pub fn new(title: Text<'a>, heading: Text<'a>, lang: Lang) -> Self;   // title/heading NÃO opcionais
    pub fn description(self, Text<'a>) -> Self;
    pub fn canonical(self, ExternalUrl<'a>) -> Self;                   // absoluto (SEO)
    pub fn manifest(self, LUrl<'a>) -> Self;                           // link rel="manifest"
    pub fn alternates(self, Vec<Alternate<'a>>) -> Self;                // hreflang absoluto
    pub fn open_graph(self, OpenGraph<'a>) -> Self;                     // og:* derivado do documento
    pub fn dir(self, Dir) -> Self;
    pub fn schemas(self, Vec<JsonLd>) -> Self;
    pub fn assets(self, Assets) -> Self;
    pub fn hints(self, Vec<Hint<'a>>) -> Self;                          // resource hints (dedup)
    pub fn hint(self, Hint<'a>) -> Self;                                  // um hint isolado
    pub fn regions(self, Regions) -> Self;
    pub fn skip(self, Text<'a>) -> Self;                                  // skip link (SC 2.4.1)
    pub fn help(self, Markup) -> Self;                                    // ajuda fixa entre header e main (SC 3.2.6)
    pub fn noscript(self, Markup) -> Self;                                // fallback sem JS
    pub fn htmx_config(self, HtmxConfig) -> Self;                         // meta seguro, se houver htmx
}
pub struct View { /* campos privados */ }
impl View { pub fn new(m: impl Into<Markup>) -> Self; pub fn modal(self, m: impl Into<Markup>) -> Self;
            pub fn asset(self, a: Asset) -> Self; pub fn merge(self, o: View) -> Self; }
impl Page { pub fn new(data: &ShellData<'_>, view: View) -> Self; }

// html/src/hint.rs — `link` é shell-only; este é o único caminho para hints
pub enum Hint<'a> { Preconnect(ExternalUrl<'a>), DnsPrefetch(ExternalUrl<'a>),
                    Preload(Preload<'a>), ImagePreload(ImagePreload<'a>),
                    ModulePreload(Src<'a>), Prefetch(Src<'a>) }
pub struct Preload<'a> { /* href, as, type, crossorigin, integrity, referrerpolicy, media,
                            fetchpriority */ }
impl<'a> Preload<'a> {
    pub fn new(Src<'a>, PreloadAs) -> Self;
    pub fn font(Src<'a>) -> Self;      // type="font/woff2" + crossorigin="anonymous"
    pub fn image(Src<'a>) -> ImagePreload<'a>; // única porta para imagesrcset/imagesizes
    pub fn style(Src<'a>) -> Self;
    pub fn script(Src<'a>) -> Self;
    pub fn with_type(self, &'static str) -> Self;
    pub fn crossorigin(self, CrossOrigin) -> Self;
    pub fn integrity(self, Integrity<'a>) -> Self;
    pub fn referrerpolicy(self, ReferrerPolicy) -> Self;
    pub fn media(self, Media<'a>) -> Self;
    pub fn fetchpriority(self, FetchPriority) -> Self;
}
pub struct ImagePreload<'a> { /* Preload + imagesrcset/imagesizes */ }
impl<'a> ImagePreload<'a> {
    pub fn responsive(self, SrcSet<'a>, Sizes<'a>) -> Self;
    pub fn with_type(self, &'static str) -> Self;
    pub fn crossorigin(self, CrossOrigin) -> Self;
    pub fn integrity(self, Integrity<'a>) -> Self;
    pub fn referrerpolicy(self, ReferrerPolicy) -> Self;
    pub fn media(self, Media<'a>) -> Self;
    pub fn fetchpriority(self, FetchPriority) -> Self;
}

// html/src/shell.rs
impl Asset { pub fn style_media(href: &'static str, Media<'static>) -> Option<Asset>; }

// html/src/form.rs — o token mora no MÉTODO: `Post`/`Put`/`Patch`/`Delete` não existem sem ele
pub struct CsrfToken<'a>(&'a str);
pub enum Method<'a> {
    Get,
    Post(CsrfToken<'a>), Put(CsrfToken<'a>), Patch(CsrfToken<'a>), Delete(CsrfToken<'a>),
}
pub struct SubmitKey<'a>(&'a str);
impl Form {
    pub fn open(action: &Action<'_>, method: Method<'_>, children: Markup) -> Markup;
    pub fn open_with_submit(action: &Action<'_>, method: Method<'_>, submit: &SubmitKey<'_>, children: Markup) -> Markup;
}

// html/src/field.rs
pub struct FieldState<'a> { /* value, required, checked, error, help, autocomplete */ }
impl<'a> FieldState<'a> {
    pub const fn new(value: AttrVal<'a>) -> Self;
    pub const fn required(self, yes: bool) -> Self;
    pub const fn checked(self, yes: bool) -> Self;
    pub const fn error(self, text: Text<'a>) -> Self;
    pub const fn help(self, text: Text<'a>) -> Self;
    pub const fn autocomplete(self, token: Autocomplete<'a>) -> Self;
}
pub enum InputType { Text, Email, Tel, Url, Search, Password, Number, Date, Time,
                     DateTimeLocal, Month, Week, Color, Range }
pub struct SelectOption<'a> { /* valor, rótulo, selected, disabled */ }
pub struct ChoiceOption<'a> { /* valor, rótulo, checked, disabled */ }
impl Field { pub fn text<M: Msg>(n: FieldName<'_>, label: M, s: &FieldState<'_>) -> Markup;
             pub fn input<M: Msg>(n: FieldName<'_>, label: M, kind: InputType, s: &FieldState<'_>) -> Markup;
             pub fn textarea<M: Msg>(n: FieldName<'_>, label: M, s: &FieldState<'_>) -> Markup;
             pub fn file<M: Msg>(n: FieldName<'_>, label: M, accept: Option<AttrVal<'_>>, multiple: bool, s: &FieldState<'_>) -> Markup;
             pub fn checkbox<M: Msg>(n: FieldName<'_>, label: M, s: &FieldState<'_>) -> Markup;
             pub fn checkbox_group<M: Msg>(n: FieldName<'_>, legend: M, options: &[ChoiceOption<'_>], s: &FieldState<'_>) -> Option<Markup>;
             pub fn radio_group<M: Msg>(n: FieldName<'_>, legend: M, options: &[ChoiceOption<'_>], s: &FieldState<'_>) -> Option<Markup>;
             pub fn select<M: Msg>(n: FieldName<'_>, label: M, options: &[SelectOption<'_>], s: &FieldState<'_>) -> Option<Markup>; // None: sem opção selecionável }

// html/src/response.rs — o contrato de resposta parcial é dado, não `hx-*`
pub enum Swap { Inner, Outer, BeforeBegin, AfterBegin, BeforeEnd, AfterEnd, Delete, None }
pub enum MainSwap { Replace(Markup), Keep }
pub struct RegionUpdate<'a> { /* Id, Swap, Markup */ }
pub struct Partial<'a> { /* main, updates, focus, select, reselect, select_oob */ }
pub enum Sync { Drop, Replace, Abort }
pub enum RequestClass { Full, Partial, Targeted, HistoryRestore, Boosted }

// crates/html-auy/src/media.rs — builders de mídia; `srcset`+`sizes` andam juntos
pub struct Img<'a> { /* src, alt, width, height, srcset, sizes, loading, fetchpriority */ }
impl<'a> Img<'a> {
    pub const fn new(Src<'a>, Alt<'a>, WidthPx, HeightPx) -> Self;
    pub fn responsive(self, SrcSet<'a>, Sizes<'a>) -> Self;
    pub const fn loading(self, Loading) -> Self;
    pub const fn lazy(self) -> Self;
    pub const fn fetchpriority(self, FetchPriority) -> Self;
    pub fn render(&self) -> Markup;
}
pub struct Picture<'a> { /* sources, fallback */ }
impl<'a> Picture<'a> { pub fn new(Img<'a>) -> Self; pub fn source(self, Source<'a>) -> Self; pub fn render(&self) -> Markup; }
pub struct Source<'a> { /* srcset, sizes, media, mime */ }
impl<'a> Source<'a> { pub fn new(SrcSet<'a>, Sizes<'a>) -> Option<Self>; pub const fn media(self, Media<'a>) -> Self;
                       pub const fn mime(self, AttrVal<'a>) -> Self; pub fn render(&self) -> Markup; }
pub struct Video<'a> { /* src, tracks, controls, preload, poster, muted, looping, playsinline, dims, transcript, fallback */ }
impl<'a> Video<'a> {
    pub fn new(Src<'a>, Track<'a>) -> Self;                 // captions obrigatórias
    pub fn track(self, Track<'a>) -> Self;
    pub const fn controls(self, bool) -> Self;
    pub const fn preload(self, MediaPreload) -> Self;
    pub const fn poster(self, Src<'a>) -> Self;
    pub const fn muted(self, bool) -> Self;
    pub const fn looping(self, bool) -> Self;
    pub const fn playsinline(self, bool) -> Self;
    pub const fn dimensions(self, WidthPx, HeightPx) -> Self;
    pub const fn fallback(self, Text<'a>) -> Self;
    pub fn transcript(self, Transcript<'a>) -> Self;
    pub fn render(&self) -> Markup;
}
pub struct Audio<'a> { /* src, tracks, controls, preload, looping, transcript, fallback */ }
impl<'a> Audio<'a> { pub fn new(Src<'a>) -> Self; pub fn track(self, Track<'a>) -> Self;
                     pub const fn controls(self, bool) -> Self; pub const fn preload(self, MediaPreload) -> Self;
                     pub const fn looping(self, bool) -> Self; pub const fn fallback(self, Text<'a>) -> Self;
                     pub fn transcript(self, Transcript<'a>) -> Self;
                     pub fn render(&self) -> Markup; }
pub struct Track<'a> { /* src, kind, label, srclang, default */ }
impl<'a> Track<'a> {
    pub const fn new(Src<'a>, TrackKind, Text<'a>) -> Self;
    pub const fn captions(Src<'a>, Text<'a>) -> Self;
    pub const fn descriptions(Src<'a>, Text<'a>) -> Self;
    pub const fn subtitles(Src<'a>, Text<'a>, Lang) -> Self;
    pub const fn srclang(self, Lang) -> Self;
    pub const fn is_default(self, bool) -> Self;
    pub fn render(&self) -> Markup;
}
pub struct Transcript<'a> { /* href, label */ }
impl<'a> Transcript<'a> { pub const fn new(Href<'a>, Text<'a>) -> Self; pub fn render(&self) -> Markup; }
```

**Desvios do plano aceitos na implementação:** o `CsrfToken` mora no `Method` (fecha o `GET` sem token
inventado e torna “sem token” um erro de tipo, não de aridade); `FieldState` tem campos privados com
builders; parser/validação/codegen vivem em `html-auy-macros-core` (testáveis e fuzzáveis sem proc-macro);
o parser tem teto de **64 níveis** de aninhamento (fonte patológico vira erro, não estouro de pilha).

**Testes (aqui mora o valor das D23–D25):**

| Caso | Prova |
|---|---|
| `Form::open` com `Method::Post` sem `CsrfToken` | `trybuild` → não compila |
| `PUT`/`PATCH`/`DELETE` | `insta` → `_method` presente; `GET`/`POST` → ausente |
| `Page::new` sem `title`/`heading` | `trybuild` → não compila |
| `main` no fragmento | `trybuild` → `html!` recusa (F3) |
| `View` com 2 modais | `insta` → ambos na região, ordem estável; `merge` dedupe de `assets` |
| `Field` com erro | `insta` → `aria-invalid="true"` + `aria-describedby` com os dois ids |
| `Field` sem erro | `insta` → **sem** `aria-invalid` e **sem** o id de erro |

### F6 — `output` (`N4`) atrás de feature (≈250 LOC; D30)

Parse com `html5ever` (dependência **opcional**, `features = ["audit"]`) + as invariantes de `output`:
`main_unique`, `single_h1`, `heading_levels_no_skips`, `ids_unique`, `landmark_names_distinct`,
`landmark_names_required`, `aria_current_unique_per_nav`, `aria_refs_resolve`, `popover_target_resolves` e
`no_content_derived_identifiers`
([`23`](23-acessibilidade-e-css.md) §2.1), mais as três de conformidade (`no_inline_script`,
`no_style_attribute`, `no_event_handler`).

`RcDom` não guarda offset de origem: a ordem da saída é a **pré-ordem** do documento, com a ordem
fixa das regras desempatando dois findings no mesmo nó. `.parent`/`.children` do DOM já são a ordem
de documento, então o resultado é determinístico e não depende de mapa de posições.
**DoD.** `html_auy::audit::check(&page.markup())` roda no CI **do consumidor** em três linhas
(`html = { path = …, features = ["audit"] }`); sem binário e sem crate novo. Sem a feature, o crate tem
zero dependências — provado por `cargo tree -e normal` no job `check`.

### F6.1 — gate do consumidor para `data-*` (≈60 LOC, **fora** do crate)

D26 decide que o conjunto de `data-*` pertence ao JS do consumidor, não ao crate. O gate fecha o buraco:

| Checa | Falha quando |
|---|---|
| `spec/data.toml` ↔ uso no markup | nome usado e não declarado (ou declarado e não usado) |
| `spec/data.toml` ↔ leitor no JS | nome declarado que nenhum bundle lê (`[data-x]`, `dataset.x`) |
| valor | o valor do atributo não está no enum declarado (ex.: `data-mask` fora de `cpf\|cnpj\|phone\|money`) |

**DoD.** Uma tabela, três consultas, uma saída; mesmo formato do `validate_spec.py` (linha por problema,
código de saída 1).

### F7 — corpus de conformidade (exportação) — **pós-MVP, não bloqueia o crate**

Gerado **dos** `trybuild` de F3 + das regras de F6, no formato de
[`24`](24-exportacao.md) §3 (`id`, `input`, `expect`, `invariant`, `scope`). É o artefato que permite
qualquer outro ecossistema implementar as mesmas regras sem virar dialeto.

**DoD.** 100% dos casos `reject` citam `invariant`; todo `invariant` do corpus existe na spec; o corpus e os
`trybuild` não divergem (teste que falha quando um caso não tem par).

## 4. Ordem

```mermaid
graph LR
  FM1[F-1 rigor] --> F0[F0 valores]
  F0 --> F1[F1 spec+validador]
  F1 --> F2[F2 parser]
  F2 --> F3[F3 validação]
  F3 --> F4[F4 codegen]
  F3 --> F5[F5 Page/View/Form/Field]
  F4 --> F6[F6 output]
  F5 --> F6
  F6 -.-> F7[F7 corpus · pós-MVP]
```

F0 é o **ponto de parada**: se F0 não pagar (M1 de [`16`](16-roadmap.md)), o plano B é maud + valores
tipados, e as fases F2–F6 não começam.

## 5. Aceitação (antes de "pronto")

Portar os dois componentes reais ([`20`](20-fronteira-de-aplicacao.md) §5) com os critérios numéricos:

| Componente | Critério |
|---|---|
| `picture` (77 linhas Blade) | ≤ 1,3× LOC; **≥ 3** defeitos recusados (`alt` vazio, dims, `sizes`, classe string) |
| `select` (110 linhas Blade) | ≤ 1,3× LOC; **zero** `aria-*` escrito à mão |

Falhou o critério → **o DSL tem buraco**: conserta o buraco ou corta o item. Nunca "aceita porque é mais
seguro".

## 6. Riscos e ponto de parada

| Risco | Sinal | Ação |
|---|---|---|
| spans ruins | mensagem aponta linha errada em 2 casos | consertar antes de F3 fechar (span é o produto) |
| macro lento | > 200 ms em 200 views | medir antes de otimizar; `cargo build` incremental é o alvo |
| DSL ergonomicamente caro | porte de §5 > 1,3× | simplificar sintaxe (nunca a validação) |
| escopo crescendo | pedido de `style`/`on*`/bag | recusar: é a garantia (§4 de [`20`](20-fronteira-de-aplicacao.md)) |

## 7. DoD global

1. `cargo test` verde **2× seguidas** (sem flaky).
2. `clippy -D warnings` limpo; zero `allow` (só `#[expect(…, reason)]` pontual — [`22`](22-rigor.md) §10).
3. `cargo deny check` + `cargo audit` + `cargo machete` verdes; `cargo +nightly fuzz run parser` sem
   pânico em 30 s. `typos` **não** roda: o dicionário é inglês e o corpus (docs, comentários,
   identificadores) é pt-BR — todo "erro" seria falso positivo (`erro`→`error`, `ser`→`set`). A
   checagem ortográfica equivalente é revisão humana, registrada aqui em vez de fingida no CI.
4. Toda mensagem de erro de [`10`](10-ergonomia.md) §2 tem `trybuild`.
5. `html-auy` sem nenhuma dependência de runtime além de `core`/`alloc`.
6. `Page`, `Form`, `Field`, `View` com os testes de F5 na tabela.
7. Porte de §5 dentro dos critérios numéricos.
8. `tools/validate_spec.py` removido **somente** quando F3 cobrir 100% das suas regras (até lá, ele é o
   gate da spec).
