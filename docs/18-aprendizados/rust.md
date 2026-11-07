# `rust` — 1.99.1 (Edition 2024)

Guia de linguagem e de ferramental — 20 arquivos, 4.901 linhas — que cobre Rust 1.99.1 / Edition 2024
do `let` até Cargo, macros, FFI e CI. Não é um livro de desenho de API: não há capítulo sobre newtype
validante, `TryFrom`, spans de proc-macro ou revisão de API pública. O valor para o html-auy está em
**(a)** mecanismos que a crate usa diretamente (campo privado, `PhantomData`, coerência, lints de
workspace, `proc-macro2`/`quote`), **(b)** a disciplina de toolchain que o guia prescreve e que a
crate pode copiar literalmente e **(c)** as lacunas — que a crate decide sozinha, e que são
exatamente onde mora o risco de escape hatch.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-introduction.md` | 45 | filosofia, tabela de editions, checklist de projeto | fixa `edition`/`rust-version`; origem do conflito de MSRV |
| `01-toolchain.md` | 182 | rustup, cargo, clippy, rustfmt, tabela de ferramentas, CI mínimo, layout | fonte do bloco `[lints]` e da lista de ferramentas de CI |
| `02-fundamentals.md` | 301 | escalares, floats algébricos, `NumBuffer`, controle de fluxo, doc comments | proíbe `algebraic_*`; `format_into` é isca de MSRV |
| `03-ownership.md` | 223 | move/Copy, borrowing, lifetimes, elision, RAII | argumentos `&str`/`&[T]`; tensão com `Text<'a>` |
| `04-structs-enums.md` | 243 | structs, newtype, enums, `#[derive]`, `match`, `repr(transparent)` | confirma newtype; `Default` derive é armadilha; nota de `non_exhaustive` |
| `05-collections.md` | 202 | `Vec`, `String`, `HashMap`, `BTreeMap`, ordenação | `sort_unstable`/`HashMap` proibidos na emissão canônica |
| `06-error-handling.md` | 250 | `panic!`, `Result`, `?`, `From`, `thiserror`, `anyhow`, tipo de validação | núcleo de §2.1/§2.2: `From` infalível e `Guess` |
| `07-generics-traits.md` | 326 | generics, bounds, `impl Trait`, associated types, newtype, `PhantomData`, coerência | APIT e `PhantomData` de fase |
| `08-closures-iterators.md` | 258 | `Fn` traits, adaptadores, zero-cost | justifica `render_to(&mut String)` sem coleções intermediárias |
| `09-modules-crates.md` | 264 | pacotes, módulos, visibilidade, workspaces, features, `cfg` | campo privado + construtor público (o padrão de `Markup`) |
| `10-smart-pointers.md` | 266 | `Box`, `Rc`, `Arc`, `RefCell`, `Weak`, `OnceLock` | irrelevante por decisão: `Markup(String)`, sem ponteiro compartilhado |
| `11-concurrency.md` | 256 | threads, canais, `Mutex`, atomics, `Send`/`Sync` | só a regra `Send + Sync` para erros sobrevive |
| `12-async.md` | 306 | futures, tokio, `select!`, cancellation | irrelevante: P5 proíbe runtime; registrar como armadilha |
| `13-testing.md` | 271 | unit/integration/doc tests, asserções, `criterion`, mocks | `criterion`/`nextest`/`proptest` aceitos; `trybuild`/`insta` **ausentes** |
| `14-unsafe-macros-ffi.md` | 399 | unsafe, FFI, C-variadics, `macro_rules!`, proc-macro, asm | única fonte de proc-macro; não cobre spans nem `compile_error!` |
| `15-cargo-deep.md` | 402 | manifesto completo, targets, profiles, workspaces, features, lints, publishing | melhor arquivo do conjunto: `[workspace.lints]`, `publish`, profiles |
| `16-project-practical.md` | 428 | arquitetura, builder, newtype, repository, state pattern, CI completo | `PhantomData` de estado e CI sem job de MSRV |
| `CHANGELOG.md` | 184 | 1.97.0 → 1.99.1, APIs estabilizadas, compatibility notes | prova que `format_into`/`VaList` estão acima da MSRV alvo |
| `SKILL.md` | 94 | resumo operacional, regras, mapa de arquivos | regras 22–33; mapa promete `Cow` que o arquivo não tem |
| `VERSION` | 1 | `1.99.1` | fixa a versão do cabeçalho |

## 2. Achados
### 2.1 Construtor, newtype e a porta do `From`
#### A1. Newtype com campo privado e construtor público é o mecanismo do `Markup`
- Citação: `09-modules-crates.md:94` — "height: u32,         // privado — só criado internamente" (construtor em `09-modules-crates.md:98`: "pub fn new(w: u32, h: u32) -> Window {").
- Semântica: campo privado + único construtor público = o tipo só existe em estado válido; não há literal de struct fora do módulo.
- Consequência: é literalmente `pub struct Markup(String)` com construção `pub(crate)`; a linguagem cobra o invariante sem runtime.
- Veredito: confirma P3 / D2.
#### A2. O "tipo de validação" devolve `Result` — e o erro é `String`
- Citação: `06-error-handling.md:216` — "pub fn new(value: i32) -> Result<Guess, String> {"
- Semântica: o construtor validante é o padrão canônico (erro na linha 218: `Err("must be 1-100".into())`); o tipo do erro fica como `String` no exemplo.
- Consequência: adotar a forma (`new -> Result<Self, E>`), nunca o `String`; cada valor de contexto precisa de erro próprio (`AltError`, `HrefError`) para o `trybuild` afirmar mensagem.
- Veredito: confirma P3; refina o tipo do erro.
#### A3. `#[derive(Default)]` é recomendado como boa prática genérica
- Citação: `04-structs-enums.md:240` — "6. **Usar `Default` derive** para structs com defaults sensatos"
- Semântica: `Default` é tratado como conveniência sempre que houver default razoável.
- Consequência: em newtype validado, `Default` é **construtor infalível público** — `Alt::default()` seria `Alt::new("")` por outro nome. Proibido derivar `Default` em qualquer tipo de contexto, e também `FromStr`/`Deserialize`.
- Veredito: refuta a prática, no nosso caso.
#### A4. Newtype também serve a *type safety*, não só a coerência
- Citação: `04-structs-enums.md:239` — "5. **Newtype pattern** (`struct Email(String)`) para segurança de tipo"
- Semântica: dois `String` de mesmo layout não são intercambiáveis; a troca é erro de compilação.
- Consequência: justifica os ~15 tipos de contexto de `docs/02` e impede que `Src` e `Href` sejam o mesmo tipo (um `Src::media` nunca vira link).
- Veredito: confirma D1.
#### A5. `From` é infalível, e `?` o invoca implicitamente
- Citação: `06-error-handling.md:57` — "Err(e) => return Err(e.into()),  // conversão automática via From"
- Semântica: `impl From<X> for Y` é função total: não pode falhar, não pode validar. E `?` usa `From` sem aparecer no código.
- Consequência: `impl From<String> for Markup` (ou `From<&str>`) destruiria o escape por tipo em uma linha, e passaria em revisão que só olhasse call sites. Regra dura: nenhum `From` para dentro de tipo de contexto.
- Veredito: confirma D2 com mecanismo explícito.
#### A6. `#[from]` do `thiserror` gera `From` — e portanto um bypass
- Citação: `06-error-handling.md:160` — "- `#[from]` gera `From<io::Error>`"
- Semântica: `#[from]` é açúcar para `impl From<E>` no enum de erro.
- Consequência: `#[from]` só em erro de domínio vindo de erro de terceiro (`io::Error` no `cq`). Em erro de HTML, `#[error(...)]` sem `#[from]` — senão `?` reabre a porta pela qual o `TryFrom` foi criado.
- Veredito: refina D9 (erros por domínio).
#### A7. `TryFrom` não existe no corpus
- Citação: `06-error-handling.md` [sem linha] — as 250 linhas cobrem `From` (linha 133), `?`, `thiserror`, `anyhow`; `TryFrom` não aparece uma vez.
- Semântica: a conversão falível só aparece como `fn new() -> Result`.
- Consequência: a regra "conversão falível nunca é `From`" tem de ser escrita na crate, não importada; o `trybuild` precisa de fixture provando que `Markup::try_from("…")` não existe.
- Veredito: lacuna — decisão própria (§5).
### 2.2 Erros: domínio, não caixa-preta
#### A8. `Box<dyn Error>` é explicitamente resto
- Citação: `06-error-handling.md:238` — "4. **Use `Box<dyn Error>`** só para protótipos ou quando os erros não importam"
- Semântica: trait object de erro custa indireção e apaga a variante; serve a binário descartável.
- Consequência: `Markup`/`Render`/validador não devolvem `Box<dyn Error>`; erro de macro é `compile_error!`, erro de spec é enum fechado. `anyhow` só no binário `cq`.
- Veredito: confirma D9.
#### A9. Erros de biblioteca precisam ser `Send + Sync`
- Citação: `06-error-handling.md:245` — "9. **Error types devem ser `Send + Sync`** para async e threading"
- Semântica: o erro atravessa `JoinHandle`, `spawn`, `Box<dyn Error + Send + Sync>`.
- Consequência: `SpecError`/`EscapeError` não podem conter `Rc`, `RefCell` ou `&dyn` não-`Sync`; derivar só `Debug` e checar com teste estático (`fn assert_send_sync<T: Send + Sync>() {}`).
- Veredito: confirma.
#### A10. `thiserror` na lib, `anyhow` no app — e o app é só o `cq`
- Citação: `06-error-handling.md:235` — "2. **Use `anyhow`** para aplicações, **`thiserror`** para bibliotecas"
- Semântica: erro tipado onde o chamador discrimina; erro opaco onde só se imprime.
- Consequência: `html-core`/`html-spec`/`html-macro` com `thiserror` ou `Display` à mão (dependência zero é melhor); `cq` pode usar `anyhow`. Nenhuma dependência de erro em `html-core`.
- Veredito: confirma D9; refina.
#### A11. "Não abuse de panic" — e `panic` é lint `deny` no workspace
- Citação: `06-error-handling.md:244` — "8. **Não abuse de panic** — Result é mais composto e testável"
- Semântica: `panic!` é para bug, não para entrada inválida; teste é a exceção declarada.
- Consequência: no parser da macro, `panic!` é bug (deve virar `compile_error!` com span); em `html-core` é proibido por lint (§2.5). Mais forte que o guia, que libera panic em teste.
- Veredito: confirma; endurece.
### 2.3 Fronteiras de API: APIT, `dyn`, coerência, `Cow`
#### A12. APIT (`impl Trait` em argumento) é recomendado como ergonomia
- Citação: `07-generics-traits.md:312` — "1. **Preferir `impl Trait`** em argumentos para funções simples" (forma em `07-generics-traits.md:84`: "fn print_summary(item: &impl Summary) {}").
- Semântica: `impl Trait` em posição de argumento é genérico anônimo; aceita qualquer `T: Trait`, inclusive tipos futuros do chamador.
- Consequência: em elemento/primitivo é **escape hatch**: `fn img(src: impl Into<Src>)` reabre `From<String> for Src` sem tocar no construtor. Todo parâmetro de elemento é concreto e nominal.
- Veredito: **refuta** na API pública de `html-spec`.
#### A13. `impl Trait` em retorno é outra coisa (e muda com edition)
- Citação: `07-generics-traits.md:88` — "fn returns_summarizable() -> impl Summary {" ; e `SKILL.md:29` — "RPIT lifetime capture changes".
- Semântica: RPIT esconde o tipo concreto; na edition 2024 a captura de lifetimes do corpo é automática.
- Consequência: RPIT em retorno é aceitável (`fn children(...) -> impl Render`), mas fixa o tipo para sempre; em `Render` público, preferir o tipo nomeado.
- Veredito: irrelevante para o macro; decisão livre.
#### A14. Genéricos para dispatch estático, `dyn` só para heterogeneidade
- Citação: `SKILL.md:27` — "Traits: small and cohesive; use generics for static dispatch, `dyn Trait` for heterogeneity."
- Semântica: `dyn` serve a coleção de tipos diferentes em runtime; fora disso, generics.
- Consequência: a árvore da macro é enum fechado (filhos heterogêneos, mas conhecidos); `Vec<Markup>` é o contêiner de filhos. `Box<dyn Node>` não se justifica.
- Veredito: confirma enum fechado.
#### A15. Coerência: trait seu em tipo alheio e trait alheio em tipo seu são permitidos
- Citação: `07-generics-traits.md:306` — "// impl Display for Vec<String> {} // ERROR!"
- Semântica: orphan rule impede trait externo em tipo externo; o resto passa.
- Consequência: `impl From<Markup> for String` é **legal** (tipo nosso) e é a saída de bytes que o `Page` precisa — mas `Deref<Target = str>` ou `Display` público fariam o conteúdo voltar como texto e ser re-escapado. `into_string` nomeado, não trait implícito.
- Veredito: refina D2 (a porta não é `From`, é `Deref`/`Display`).
#### A16. `Cow<'a, str>` é prometido e nunca explicado
- Citação: `SKILL.md:81` — "| `10-smart-pointers.md` | Box, Rc, Arc, RefCell, Cow, Weak |"; o arquivo (266 linhas) não contém `Cow`; `SKILL.md:20` lista "`Box`/`Cow`" como critério de escolha.
- Semântica: o índice afirma cobertura que o texto não tem; não há uma linha sobre `Cow`, `ToOwned` ou quando copiar.
- Consequência: `Text<'a>`/`Alt<'a>`/`Class<'a>` são a resposta ao caso "emprestar quando possível"; generalizar `Cow` contamina primitivo e `Markup` com lifetime. Só onde houver dúvida real — hoje não há.
- Veredito: lacuna + armadilha (§6).
#### A17. `&str`/`&[T]` em argumento; lifetime raramente em struct
- Citação: `03-ownership.md:219` — "4. **structs raramente precisam de lifetimes** — a maioria deve owned" (e `03-ownership.md:217`: "**Usar `&str` em vez de `&String`**").
- Semântica: empréstimo na borda, ownership no armazenamento; lifetime em struct só para dado genuinamente emprestado.
- Consequência: construtores recebem `&str`/`&[Variant]`; `Text<'a>` é emprestado por natureza (escape na escrita), mas `SrcSet`, `JsonLd` e `Markup` são owned. `Cow` no struct produziria `Markup<'a>` — proibido.
- Veredito: confirma `Text<'a>`; proíbe lifetime em `Markup`.
### 2.4 Fases do pipeline e enumeração fechada
#### A18. `PhantomData` marca o que só existe no nível de tipo
- Citação: `07-generics-traits.md:320` — "8. **PhantomData** para parâmetros genéricos que só existem no type level"
- Semântica: o parâmetro não aparece em campo de dados; `PhantomData` dá ownership/variance sem custo de runtime.
- Consequência: `Parser<Raw>` → `Parser<Validated>` → `Parser<Formatted>` expressa P8 (um parser, três passes) **no tipo**: `fmt` só aceita `Validated`, `codegen` só aceita `Validated`. Zero bytes em runtime.
- Veredito: confirma P8 e refina o desenho.
#### A19. O state pattern mostra o custo real: métodos duplicados por fase
- Citação: `16-project-practical.md:283` — "pub fn publish(self) -> Post<published::Published> {"
- Semântica: cada transição consome `self` e devolve outro parâmetro; `impl` separados por estado.
- Consequência: `validate(self) -> Result<Parser<Validated>, SpecError>` é ótimo; mas método comum exige `impl<S> Parser<S>` — se isso passar de 2 blocos, a fase virou abstração. Duas fases (`Raw`, `Validated`), no máximo.
- Veredito: confirma com limite.
#### A20. `repr(transparent)` não garante nada além de layout — e ficou mais estrito
- Citação: `04-structs-enums.md:141` — "privados, e `#[non_exhaustive]` não são mais considerados "triviais"."
- Semântica: layout é contrato de ABI, não de API; a 1.98 endureceu o que conta como transparente.
- Consequência: nunca depender de `repr(transparent)` para justificar um newtype. `Markup(String)` é opaco por privacidade, não por layout.
- Veredito: confirma (armadilha já registrada em `18-aprendizados.md`).
#### A21. `#[non_exhaustive]` reduz breaking change, mas obriga `_` no `match`
- Citação: `04-structs-enums.md:241` — "7. **Evitar `match` com `_` quando variantes podem ser adicionadas** — prefira match exaustivo"
- Semântica: o guia quer `match` exaustivo para o compilador apontar variante nova; `#[non_exhaustive]` faz o oposto na borda do crate.
- Consequência: o atributo só se paga com consumidor **externo**; aqui o consumidor é o mesmo workspace, então `#[non_exhaustive]` em `Token`/`Attr` remove a exaustividade sem ganhar nada. Usar em `Error`, onde a variante é aberta.
- Veredito: refuta o uso generalizado; decisão por tipo.
### 2.5 Manifesto, lints e toolchain
#### A22. `unsafe_code = "forbid"` no nível do workspace
- Citação: `15-cargo-deep.md:190` — "rust.unsafe_code = "forbid"" (sob `[workspace.lints]`, linha 189); forma por crate em `01-toolchain.md:103`.
- Semântica: `forbid` não pode ser revertido por `allow` abaixo na árvore; é a única forma de garantir "zero unsafe" sem revisão.
- Consequência: em `[workspace.lints]`, cobre os quatro crates de uma vez e impede que `html-spec` gerado introduza `unsafe` em silêncio.
- Veredito: confirma D9.
#### A23. `unwrap_used`/`expect_used`/`panic` como `deny`
- Citação: `15-cargo-deep.md:382` — "clippy::unwrap_used = "deny"" (com `expect_used` na 383 e `panic` na 384).
- Semântica: os três lints de "abortar o processo" viram erro de compilação.
- Consequência: casa com A11 e com P5. Exceção: `#[cfg(test)]` e `compile_error!` (que não é panic). `#[allow]` por módulo de teste, nunca no crate.
- Veredito: confirma; refina (escopo de teste).
#### A24. O guia se contradiz sobre onde vai `clippy::*`
- Citação: `15-cargo-deep.md:381` — "clippy::pedantic = "warn"" aparece dentro de `[lints.rust]` (linha 377), enquanto `01-toolchain.md:106` usa "[lints.clippy]" com "pedantic = "warn"" (linha 107).
- Semântica: a tabela `[lints.<tool>]` recebe o nome do lint sem prefixo; `[lints.rust]` com chave `clippy::x` gera `rust::clippy::x`. [INFERENCE: lint desconhecido; a forma correta é `[workspace.lints.clippy]`.]
- Consequência: copiar `01-toolchain.md`, não `15-cargo-deep.md`; e não ligar `pedantic` em `html-spec` (código gerado).
- Veredito: conflito interno do corpus — corrigir na cópia.
#### A25. `publish = false` era o cadeado do D0 (histórico; D35 o removeu)
- Citação: `15-cargo-deep.md:21` — "publish = false   # impede publish acidental"
- Semântica: o manifesto recusa `cargo publish` sem depender de disciplina.
- Consequência: em **todos** os crates (o root do workspace não tem `[package]`), mais `publish = false` no `cq`. É o que torna "interno" verificável.
- Veredito: histórico; D35 substitui por publicação pública com semver.
#### A26. `Cargo.lock` versionado é para aplicação, não para biblioteca
- Citação: `15-cargo-deep.md:393` — "1. **Commit `Cargo.lock`** para aplicações (não para bibliotecas)"
- Semântica: quem entrega binário fixa a árvore; quem é dependência não impõe versões.
- Consequência: o html-auy é vendorizado por consumidor único e `docs/11` §3 exige `cargo build --locked` → commitar o lock, mesmo sendo lib. O critério real é "consumidor único", não "tem `main.rs`".
- Veredito: refina a regra para o nosso caso.
#### A27. `resolver = "2"`; o resolver 3 é nightly
- Citação: `15-cargo-deep.md:114` — "| 3 (nightly) | MSRV-aware |"
- Semântica: resolver 3 (MSRV-aware) só em nightly; em stable o resolver é 2.
- Consequência: `resolver = "2"` no workspace; proteção de MSRV tem de ser **CI** (matriz), não resolução de features.
- Veredito: confirma `docs/11` §3.
#### A28. Features: `dep:` e `default` mínimo — e aqui, zero features
- Citação: `09-modules-crates.md:210` — "serde = ["dep:serde"]      # dep: prefix evita feature implícita" (e `15-cargo-deep.md:398`: "**Features `default` mínimas** — não force dependências extras").
- Semântica: feature que só liga dependência opcional deve usar `dep:`; default deve ser o mínimo.
- Consequência: a crate não tem funcionalidade opcional — nem `trybuild` (dev-dependency), nem `serde`. Zero features; qualquer `cfg(feature)` futura é decisão nova em `docs/13`.
- Veredito: confirma a adição "zero features".
#### A29. `rust-version` no manifesto é a MSRV declarada
- Citação: `15-cargo-deep.md:394` — "2. **`rust-version`** no Cargo.toml para MSRV claro" (valor do guia: `00-introduction.md:40` — "`rust-version = "1.85"` no `Cargo.toml`").
- Semântica: o campo é a única declaração machine-readable de MSRV; Cargo recusa dependência com MSRV maior.
- Consequência: **conflito aberto** — o conjunto é 1.99.1, sugere 1.85, e `docs/11-estabilidade.md:77` diz "Rust ≥ MSRV, edição 2021+" sem número. Ver nota em §4.
- Veredito: refina; exige decisão explícita.
#### A30. Edition 2024 é o default a partir do rustc 1.85
- Citação: `00-introduction.md:34` — "edition = "2024"  # default a partir de rustc 1.85+"
- Semântica: edição e MSRV são independentes; 2024 não obriga APIs novas.
- Consequência: adotar 2024 é seguro na MSRV; muda `unsafe extern`, captura de RPIT e `gen` — nada que a crate use. O que não pode acontecer é `docs/11` §7 prometer "edição 2021+" enquanto o manifesto diz 2024.
- Veredito: corrige `docs/11` §7.
#### A31. `[profile.dev.build-override]` acelera proc-macro e build script
- Citação: `15-cargo-deep.md:158` — "[profile.dev.build-override]" com "opt-level = 3" na linha 159.
- Semântica: crates compiladas para o host (proc-macro, `build.rs`) rodam no compilador; otimizá-las acelera a expansão.
- Consequência: útil em quem **compila** a macro (o consumidor vendorizado), não na crate; e só depois de medir — o parser é 1,2k LOC lineares. [INFERENCE]
- Veredito: irrelevante por enquanto; registrado.
### 2.6 Proc-macro: o guia para no "hello world"
#### A32. `proc-macro = true` e os três tipos de macro
- Citação: `14-unsafe-macros-ffi.md:227` — "proc-macro = true" (sob `[lib]`, linha 226); tipos nas linhas 230/234/241.
- Semântica: crate proc-macro é especial: só exporta `#[proc_macro*]` e não pode ser dependência normal.
- Consequência: `html-macro` é crate separado (já é, por `docs/01` §7) e não pode ser dependência de `html-core`; a fronteira é imposta pelo Cargo.
- Veredito: confirma `docs/01`.
#### A33. `parse_macro_input!` + `quote!` — sem uma palavra sobre spans
- Citação: `14-unsafe-macros-ffi.md:251` — "let ast = parse_macro_input!(input as DeriveInput);" e `14-unsafe-macros-ffi.md:253` — "let gen = quote! {"
- Semântica: o exemplo converte tokens em AST e reemite com `quote!`; spans não são mencionados em nenhuma das 399 linhas.
- Consequência: P7 ("span no ponto exato") não tem apoio no corpus. Regra própria: todo token reemitido carrega span de origem (`quote_spanned!` ou interpolação de `Ident`/`TokenStream` originais); `Span::call_site()` nunca para diagnóstico.
- Veredito: lacuna crítica para P7.
#### A34. `compile_error!` não aparece
- Citação: `14-unsafe-macros-ffi.md` [sem linha] — nenhuma ocorrência de `compile_error!` no arquivo, nem no conjunto.
- Semântica: o guia mostra macros que *geram* código, não macros que *recusam* código.
- Consequência: a única forma estável de erro de macro (`docs/04` §2) precisa ser decidida e testada aqui: um `compile_error!` por erro (não concatenar), span do token, mensagem em três partes (`docs/10` §2).
- Veredito: lacuna.
#### A35. `cargo-expand` é a ferramenta de depuração de macro
- Citação: `01-toolchain.md:136` — "| `cargo-expand` | `cargo install cargo-expand` | Expandir macros |"
- Semântica: mostra o código depois da expansão.
- Consequência: útil ao desenvolver `html-macro`; nunca no CI (lento, saída muda entre versões). Alternativa barata: teste que compara `TokenStream::to_string()` — determinístico e sem instalar nada.
- Veredito: irrelevante para a spec; útil no processo.
### 2.7 Testes: o que existe, o que falta
#### A36. `#[should_panic]` é recomendado — e nós o proibimos
- Citação: `13-testing.md:257` — "2. **`#[should_panic]`** para testar condições de erro"
- Semântica: o guia trata panic como resultado testável.
- Consequência: com `clippy::panic = deny` (A23), o teste de erro é `compile_error!` via `trybuild`, não panic. Manter `#[should_panic]` só onde o panic é contrato — e não há nenhum.
- Veredito: refuta (já em `18-aprendizados.md`).
#### A37. `criterion` com `harness = false`
- Citação: `13-testing.md:151` — "use criterion::{black_box, criterion_group, criterion_main, Criterion};" (e `15-cargo-deep.md:57`: "harness = false  # desativa harness padrão").
- Semântica: benchmark estável exige harness próprio; o nativo é nightly.
- Consequência: `benches/render.rs` com `harness = false` cobre o que `docs/09` §7 pede (escaper, `SrcSet` de 6 variantes, tabela de 100 linhas, shell). Bench não é gate de CI; o gate é o orçamento de bytes.
- Veredito: confirma `docs/09` §7.
#### A38. `proptest` e `nextest` existem; só o nome
- Citação: `13-testing.md:265` — "9. **Property-based testing com `proptest`** — testa casos aleatórios"; `13-testing.md:262` — "7. **`cargo nextest`** é mais rápido que `cargo test` para CI".
- Semântica: uma linha cada, sem exemplo de estratégia nem de `proptest!`.
- Consequência: a tabela de propriedades de `docs/12` §3 é nossa, não importada; `nextest` no CI é escolha operacional (e não roda doc tests — o guia omite [INFERENCE]).
- Veredito: irrelevante além do nome.
#### A39. Doc tests são testes de verdade (compilam contra a API pública)
- Citação: `13-testing.md:104` — "/// ```rust,should_panic" (variantes `ignore` e `no_run` no mesmo bloco).
- Semântica: o bloco do comentário é compilado como crate externo que importa a lib.
- Consequência: doc test de `html-core` usando `html!` exige dev-dependency em `html-macro`, que depende de `html-spec`, que depende de `html-core` — ciclo de dev que o Cargo aceita mas confunde MSRV. Doc test sem macro; exemplo com macro em `examples/`.
- Veredito: refina `docs/12`.
#### A40. `trybuild` e `insta` não existem no corpus — e são os dois pilares de `docs/12`
- Citação: `13-testing.md` [sem linha] — 271 linhas sobre testes, zero ocorrências de `trybuild` ou `insta` em todo o conjunto.
- Semântica: o guia cobre o harness nativo; teste de mensagem de compilação e snapshot não são do interesse dele.
- Consequência: `docs/12` §2/§5 dependem inteiramente de ferramentas que o corpus não descreve — formato `tests/ui/*.stderr`, variação da mensagem entre versões do rustc, `INSTA_FORCE_UPDATE` proibido no CI.
- Veredito: lacuna — a maior do conjunto, porque P7 é um dos pilares.
### 2.8 Determinismo: as três armadilhas que o guia recomenda
#### A41. `algebraic_*` é explicitamente não-determinístico
- Citação: `02-fundamentals.md:86` — "- **Não-determinísticos**: o compilador escolhe as otimizações, então o"
- Semântica: `f64::algebraic_add` permite reordenar (`-ffast-math`), com resultado variável entre builds.
- Consequência: D14 proíbe. Nenhum float em caminho de emissão; não há float na crate (dimensão é `u32`, dinheiro é inteiro). Registrar a proibição, porque o guia a recomenda.
- Veredito: **refuta** o uso; confirma D14.
#### A42. `format_into`/`NumBuffer` é bom — e está acima da nossa MSRV
- Citação: `02-fundamentals.md:107` — "Todos os inteiros primitivos têm `format_into`, que escreve a forma" (uso em `02-fundamentals.md:116`, estabilizado na 1.98).
- Semântica: formata inteiro sem alocação, com performance de `itoa`.
- Consequência: é o que `WidthPx`/`Count` querem — e é **proibido** com MSRV anterior a 1.98. Exemplo concreto do custo: reimplementar com `write!` num buffer `[u8; 20]`, ou subir a MSRV.
- Veredito: conflito direto com A29 (§4).
#### A43. `sort_unstable` é recomendado por velocidade
- Citação: `05-collections.md:194` — "3. **`sort_unstable()`** é mais rápido que `sort()` — use a menos que"
- Semântica: ordenação instável não preserva ordem relativa de chaves iguais (linha 195 completa: "precise de estabilidade").
- Consequência: `SrcSet`/`Class`/atributos têm ordem canônica; com chave repetida, `sort_unstable` pode gerar bytes diferentes entre builds. Usar `sort_by_key` (estável) ou garantir chave total única.
- Veredito: refuta na emissão; confirma D14.
#### A44. `HashMap` não tem ordem — `BTreeMap` tem
- Citação: `05-collections.md:139` — "let mut scores = HashMap::new();" (iteração sem ordem garantida; `BTreeMap` só na tabela de coleções).
- Semântica: hash map itera em ordem arbitrária, dependente da semente.
- Consequência: nada de `HashMap` em caminho de emissão (`docs/09` §2 já exige `BTreeMap`/`BTreeSet`); idem para deduplicação de assets e de `Class`.
- Veredito: confirma D14 e `docs/09` §2.

## 3. Confirmações do desenho
1. **`Markup` sem construtor público** — A1 (campo privado + construtor), A5 (`From` é total, logo é a porta).
2. **Escape por tipo** — A4 (type safety por newtype), A17 (`&str` na borda), A16 (sem generalizar `Cow`).
3. **Uma porta de entrada por contexto** — A2 (`new -> Result`), A6 (`#[from]` reabre), A7 (`TryFrom` ausente ⇒ regra nossa).
4. **Nenhum escape hatch de atributo** — A12 (APIT aceita qualquer `T: Trait`), A14 (enum fechado).
5. **Erros por domínio, `Send + Sync`, `anyhow` só no binário** — A8, A9, A10.
6. **`forbid(unsafe_code)` e dependências mínimas** — A22, A32 (proc-macro é crate separado), A28 (zero features).
7. **Determinismo de bytes** — A41, A43, A44; e o custo de A42.
8. **Um parser, três passes, fases no tipo** — A18 (`PhantomData`), A19 (limite de duas fases).
9. **P7 (erro de compilação com span)** — sem apoio no corpus: A33, A34, A40.
10. **publicação versionada / lock versionado** — A25 (histórico), A26.
11. **Traits pequenos; `dyn` só onde há heterogeneidade real** — A14.
12. **`repr(transparent)` não é contrato de API** — A20.

## 4. Conflitos e correções
| O que a crate diz hoje | O que o corpus diz | Correção (frase exata a inserir) |
|---|---|---|
| `docs/11-estabilidade.md:77`: "Rust ≥ MSRV, edição 2021+." | `00-introduction.md:34` — "edition = "2024"  # default a partir de rustc 1.85+" | Em `docs/11-estabilidade.md` §7, substituir por: "Rust ≥ 1.92 (`rust-version = "1.92"`), edição 2024." |
| `docs/11` não fixa número de MSRV | `00-introduction.md:40` — "`rust-version = "1.85"` no `Cargo.toml`"; `18-aprendizados.md` decidiu 1.92 | Em `docs/11-estabilidade.md` §3, inserir: "MSRV = 1.92 (edição 2024). `algebraic_*`, `format_into`/`NumBuffer`, `VaList`, `Box::into_non_null`, `Vec::into_parts`, `String::from_utf8_lossy_owned` são proibidos, e um job de CI com 1.92 prova isso." |
| `docs/02` §8 lista o que não existe, mas não cita construtores derivados | `04-structs-enums.md:240` recomenda `Default` derive | Em `docs/02-tipos-e-contextos.md` §1: "Nenhum tipo de contexto deriva `Default`, `FromStr` ou `Deserialize`: os três são construtores infalíveis e reabrem o bypass." |
| `docs/02` não fala de `TryFrom` | `06-error-handling.md` [sem linha]: só `From` (linha 133) | Em `docs/02` §2, coluna "Construção": "conversão falível é `TryFrom`/`new -> Result`; `From` para dentro de valor de contexto é proibido (é total)." |
| `docs/01` §6 e `docs/10` não fixam o tipo dos parâmetros de elemento | `07-generics-traits.md:312` recomenda APIT | Em `docs/02` §1: "Parâmetro de elemento e de primitivo é **tipo concreto**; `impl Trait`/`impl Into<T>` em assinatura pública de `html-spec` é proibido." |
| `docs/04` §4 não fixa profundidade de aninhamento | `14-unsafe-macros-ffi.md` [sem linha]: nada sobre recursão ou limite de tokens | Em `docs/04` §4, novo item: "Profundidade de aninhamento limitada a N níveis (erro de macro acima disso); parser iterativo ou com orçamento de pilha explícito." |
| `docs/09` §2 exige ordem canônica sem citar a coleção | `05-collections.md:194` recomenda `sort_unstable` | Em `docs/09` §2: "`sort_unstable` e `HashMap` são proibidos em caminho de emissão; ordem vem de `BTreeMap`/`BTreeSet` e de chave total única." |
| `docs/12` §7 exige determinismo sem citar float | `02-fundamentals.md:86` admite resultado não-determinístico | Em `docs/09` §2: "Nenhum `f64` em caminho de emissão; `algebraic_*` é proibido (resultado varia entre builds)." |
| `docs/11` §3 exige matriz de CI, sem job escrito | `16-project-practical.md:358` lista Check/Test/Coverage/Security/Build e nenhum é MSRV | Em `docs/11` §3: "Job `msrv` roda `cargo build --locked` e `cargo test --locked` com 1.92; jobs `stable` e `beta` idem; nenhum job usa nightly." |
| `docs/02` §2 não diz o que é público em `Markup` | `07-generics-traits.md:304` — "impl Display for MyStruct {}" é legal | Em `docs/02` §1: "`Markup` expõe `as_bytes`/`into_string` nomeados; não implementa `Deref<Target = str>`, `Display`, `From<Markup> for String` nem `Into<String>`." |

> **Nota explícita sobre o conflito de MSRV.** Três números convivem: o conjunto é **1.99.1**
> (`VERSION`, `SKILL.md`), o guia manda declarar **`rust-version = "1.85"`**
> (`00-introduction.md:40`), e `docs/11-estabilidade.md:77` promete "Rust ≥ MSRV, edição 2021+"
> **sem número**, enquanto `18-aprendizados.md` já registrou **1.92 / edição 2024**. Não são
> equivalentes: 1.85 aceita edition 2024 mas **não** tem `format_into` (1.98), `algebraic_*` (1.98)
> nem `VaList` (1.99); 1.92 também não. A crate cabe em 1.85 se renunciar explicitamente a essas
> APIs — o que ela já faz (sem float, sem FFI, e `u32` cabe em `[u8; 20]`). **Decisão a aplicar:**
> `rust-version = "1.92"`, `edition = "2024"`, e job de CI compilando com 1.92 para que nenhuma API
> de 1.98 entre por conveniência.

## 5. Adições prontas para colar
### 5.1 `[workspace.lints]` (Cargo.toml raiz)
```toml
[workspace]
members = ["crates/html-core", "crates/html-spec", "crates/html-macro", "crates/html-lint", "crates/cq"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.92"
publish = false

[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
unused_imports = "deny"
unused_qualifications = "warn"
non_ascii_idents = "deny"

[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
dbg_macro = "deny"
print_stdout = "deny"
pedantic = "warn"
```
Cada crate herda com `[lints] workspace = true`; `html-spec` gerado recebe
`#![allow(clippy::all, clippy::pedantic)]` no topo do arquivo.

### 5.2 Manifesto de `html-core`
```toml
[package]
name = "html-core"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish = false

[dependencies]          # vazio: só core/alloc

[lints]
workspace = true
```

### 5.3 Regras de API (para `docs/02-tipos-e-contextos.md` §1)
| Regra | Proibido | Obrigatório |
|---|---|---|
| Construção | `From`, `Default`, `FromStr`, `Deserialize`, campo `pub` | `new(...) -> Result<Self, E>`, `TryFrom`, `decorative()` explícito |
| Parâmetro | `impl Trait`, `impl Into<T>`, `&String`, `&Vec<T>`, genérico livre | tipo concreto, `&str`, `&[T]` |
| Saída | `Deref`, `Display`, `Into<String>`, `AsRef<str>` | `as_bytes(&self) -> &[u8]`, `into_string(self) -> String` |
| Enum público | `_ =>` interno; `#[non_exhaustive]` em `Token`/`Attr` | `match` exaustivo; `#[non_exhaustive]` só em `Error` |
| Erro | `Box<dyn Error>`, `anyhow`, `String` como erro, `panic!`/`unwrap`/`expect` | enum fechado + `thiserror` (ou `Display` à mão), `Send + Sync`, `source()` |
| Fase | `bool` de estado, campo `validated: bool` | `PhantomData` com no máximo duas fases |
| Layout | `repr(transparent)` como contrato | opaco por privacidade |

### 5.4 Harness de mensagem e snapshot (para `docs/12`)
```toml
[dev-dependencies]
trybuild = "1"
insta = { version = "1", features = ["filters"] }
proptest = "1"
```
```
crates/html-macro/tests/
├── insta/     # um .snap por elemento da spec
├── ui_fail/   # trybuild: um .rs + .stderr por mensagem de docs/10 §2
└── ui_pass/   # trybuild: caso que DEVE compilar (mensagem é contrato nos dois sentidos)
```
- `trybuild` roda na MSRV **e** na stable: a mensagem do rustc muda entre versões — `.stderr` é
  comparado por versão, ou o teste é `#[ignore]` fora da MSRV.
- `insta`: `INSTA_FORCE_UPDATE` proibido no CI; snapshot alterado exige leitura no PR.
- `proptest` só nas propriedades de `docs/12` §3; o parser vai para `cargo-fuzz`, não para `proptest`.

### 5.5 Regra de span para `html-macro` (nova)
```rust
// todo token reemitido carrega span de origem
let span = tok.error_span();                 // span do token HTML culpado
quote_spanned! { span => compile_error!(#msg) }
```
- Um `compile_error!` por erro, nunca concatenado.
- `Span::call_site()` só para código gerado sem origem (helpers), jamais para diagnóstico.
- Mensagem em três partes (`docs/10` §2), com `regra:` e `ajuda:` no mesmo `compile_error!`.

## 6. Armadilhas de complexidade
| Armadilha | Por que não entra | Custo se entrar |
|---|---|---|
| APIT (`impl Into<T>`) em elemento | aceita qualquer `T: Trait`; reabre `From<String>` sem tocar em construtor | a porta de escape volta, invisível em revisão de call site |
| `Cow<'a, str>` generalizado | A16: prometido e não explicado; lifetime em struct contamina primitivo | `Markup<'a>`, `Page<'a>`, assinatura com 2 lifetimes |
| `Box<dyn Node>`/`Box<dyn Render>` na árvore | heterogeneidade é conhecida em compilação; enum fechado dá exaustividade | alocação por nó, perda de exaustividade, vtable em hot path |
| `PhantomData` com 3+ fases | cada fase duplica `impl`; A19 mostra o custo | tipo ilegível, erro de tipo em vez de erro de validação |
| `#[non_exhaustive]` em `Token`/`Attr` | consumidor é o mesmo workspace: não há variante futura desconhecida | `_ =>` obrigatório dentro de casa; compilador deixa de apontar variante nova |
| `Default`/`Deserialize` derivados em newtype | construtor infalível público (A3) | `Alt::default()` = alt vazio, sem linha suspeita |
| `#[from]` em erro de HTML | `?` converte sozinho (A6) | bypass de validação por operador |
| `algebraic_*`/`f64` na emissão | resultado varia entre builds (A41) | `ETag` instável, snapshot irreprodutível |
| `sort_unstable`/`HashMap` na emissão | ordem não especificada (A43/A44) | bytes diferentes para o mesmo dado; 304 quebrado |
| tokio/axum/serde no `core` | P5/P9; arrasta runtime para o binário do consumidor | MSRV acoplada, `cq check` lento, superfície de auditoria |
| FFI/`unsafe`/`#[unsafe(naked)]` | nada disso é necessário; `forbid(unsafe_code)` (A22) | revisão de UB num projeto com marco de 2 semanas |
| `#[should_panic]` como teste de erro | o contrato é `compile_error!` (A36) | teste que passa quando a macro entra em pânico em vez de diagnosticar |
| `pedantic` em `html-spec` gerado | milhares de linhas geradas, zero leitura humana (A24) | CI vermelho permanente ou `#[allow]` global que anula o lint |
| `[profile.*]` definidos na crate | profile é do workspace que compila [INFERENCE] | expectativa de `lto`/`strip` que o consumidor sobrescreve |

## 7. Lacunas do corpus
1. **`TryFrom`** — zero menções; "conversão falível nunca é `From`" é regra nossa.
2. **APIT como armadilha** — o guia recomenda `impl Trait` em argumento (`07:312`); nada sobre
   `impl Into<T>` reabrindo `From`.
3. **Spans** — nenhuma linha sobre `Span`, `quote_spanned!`, `syn::Error` ou `compile_error!` com
   span; P7 depende inteiramente disso.
4. **`#[non_exhaustive]`** — só a nota de `repr(transparent)` (A20/A21); nenhuma política de uso.
5. **`Cow`** — prometido no mapa (`SKILL.md:81`) e inexistente no texto.
6. **`trybuild`/`insta`** — ausentes; `docs/12` §2/§5 são 100% decisão própria, inclusive o problema
   de a mensagem de erro variar por versão do rustc.
7. **Política de MSRV** — o conjunto é 1.99.1 e sugere 1.85; nada sobre matriz de CI,
   `cargo build --locked` ou como proibir API acima da MSRV.
8. **Revisão de API pública** — nenhuma menção a `cargo-semver-checks`, `cargo public-api`,
   `#[doc(hidden)]`, `#[must_use]` ou `pub(crate)` em assinatura pública ("private in public").
   Para crate interna a decisão provável é "não instalar nada" — mas é decisão.
9. **Recursão do parser** — nada sobre profundidade de aninhamento, orçamento de pilha ou
   `TokenStream` recursivo; o fuzz acha o caso, não fixa o limite.
10. **`no_std`/`alloc`** — `docs/01` §1 pede `core`/`alloc` em `html-core`; o guia só cita
    `extern crate alloc` numa linha de `cfg`.
11. **Custo de expansão** — nenhum número sobre tempo de `cargo check` por `html!`; `docs/04` §8
    assume "imperceptível" sem medição no corpus.
12. **Determinismo de hash** — `BTreeMap` resolve ordem, mas o guia não discute `BuildHasher`
    determinístico, `IndexMap`, nem custo de ordenar por chave textual.
13. **`Send`/`Sync` do contexto** — o guia afirma que "a maioria dos tipos são `Send + Sync`
    automaticamente" (`11-concurrency.md:183`); se `Ctx` carrega `Rc`/`Cell`, a suposição quebra em
    axum sem aviso.
