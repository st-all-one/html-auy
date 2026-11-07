# 22 — Rigor de compilação: o que absorver do `sniff-css`

> **Proveniência.** Auditado no projeto `sniff-css` deste repositório (8 crates, ~28,6k LOC Rust,
> `docs/quality.md`, `docs/phase-3.md`, `clippy.toml`, `deny.toml`, `rustfmt.toml`,
> `.github/workflows/ci.yml`). **Absorção:** integral na configuração; **nada** do domínio (CDP,
> Flutter, MCP, tokio) vem junto.
> **Quando:** **antes** de qualquer código — é a fase **F-1** de [`21`](21-implementacao.md).

## 1. A ideia central: duas camadas, não uma

O `sniff` separa **níveis** (`deny`/`warn`/`allow`, no `[workspace.lints]` do `Cargo.toml`) de
**valores** (limiares e allowlists, no `clippy.toml`). Isso importa por dois motivos concretos:

1. as lints `restriction` são `allow` por padrão — **`clippy.toml` sozinho não liga nada**;
2. cada crate precisa de `[lints] workspace = true`, senão herda o default e o gate vira ficção.

O `sniff` documenta a adoção em **fases** (o débito legado fica em `allow` com TODO, e é promovido a
`deny` quando pago). Aqui não há débito: o crate nasce com o padrão final, o que só é possível porque
nenhuma linha foi escrita antes desta decisão.

## 2. Toolchain: dois números, quatro lugares

Há **duas** coisas distintas, e confundi-las é a origem do bug clássico ("minha máquina compila, o CI
não" invertido: o CI passa na stable e quebra na MSRV).

| Número | Lugar | Valor | Se divergir |
|---|---|---|---|
| **pin** (reprodutibilidade) | `rust-toolchain.toml` → `channel` | `1.99` + `clippy` + `rustfmt` (D32) | máquina e CI compilam com compiladores diferentes no dia a dia |
| **MSRV** (contrato) | `Cargo.toml` → `rust-version` | `1.92` ([`11`](11-estabilidade.md) §4, D32) | a lint `incompatible_msrv` mente |
| **MSRV** | `clippy.toml` → `msrv` | `1.92` | sugestões propõem API mais nova que a MSRV |
| **MSRV** | job de CI `msrv` (`cargo build --locked` em 1.92) | `1.92` | MSRV declarada é ficção |

Regra: **o pin é local ao repositório; a MSRV é o contrato**. O pin pode subir com a stable; a MSRV só
sobe em major. `resolver = "3"` (MSRV-aware, Edition 2024) faz a resolução de dependências respeitar o
`rust-version` — sem ele, `cargo update` pode puxar crate que exige o compilador do pin.

## 3. Os bans, e por que cada um **importa neste crate**

O `sniff` é um cliente de rede; este crate é um **macro que produz bytes**. Os bans coincidem porque o
inimigo é o mesmo: **comportamento não determinístico e falha sem diagnóstico**.

| Lint | Nível | Por que aqui |
|---|---|---|
| `unwrap_used`, `expect_used`, `panic`, `unreachable`, `indexing_slicing` | `deny` | **um panic dentro de proc-macro não é erro útil**: vira mensagem genérica sem span. Toda falha tem de ser `compile_error!` com span. `indexing_slicing` fecha a porta mais comum (`.get()`/`split_at`/`first()` no parser) |
| `disallowed_types`: `HashMap`, `HashSet`, `LinkedList` | `deny` | ordem de iteração não determinística → **bytes diferentes** para a mesma view, ETag instável, golden flaky. Ordem explícita de declaração ou `BTreeMap`/`BTreeSet` |
| `disallowed_methods`: `SystemTime::now`, `Instant::now` | `deny` | golden sem horário; nada no crate precisa de relógio (se precisar, é injetado) |
| `disallowed_methods`: `std::env::var*` | `deny` | o macro **não lê ambiente**: configuração entra por `ShellData`/feature/registro, nunca por `cfg!(env)` |
| `as_conversions` + `cast_possible_truncation` / `cast_sign_loss` / `cast_possible_wrap` / `cast_precision_loss` | `deny` | conversão de pixel/percentual com saturação **auditada** num único módulo (`numeric`), como no `sniff`; `as` silencioso em `usize → u32` é bug de largura |
| `arithmetic_side_effects` | `deny` | `saturating_*`/`checked_*` explícitos; contagem de nós e largura de buffer não estouram em silêncio |
| `print_stdout`, `print_stderr` | `deny` | o crate **não escreve em stream nenhum**; diagnóstico é do consumidor (e stdout poluído quebra teste) |
| `allow_attributes_without_reason` | `deny` | exceção sem justificativa é supressão silenciosa |
| `todo`, `unimplemented`, `dbg_macro`, `mem_forget` | `deny` | não há "depois"; `mem_forget` é vazamento |
| `let_underscore_must_use`, `let_underscore_drop` | `deny` | descarte explícito com `drop(…)`, não `let _ =` |
| `wildcard_imports`, `unused_qualifications`, `trivial_casts` | `deny` | leitura previsível |
| `unsafe_code` | **`forbid`** (mais forte que o `deny` do `sniff`) | não existe motivo para `unsafe` aqui; `forbid` impede até `#[allow]` local |
| `missing_docs` + `broken_intra_doc_links` | `warn`/`deny` | a documentação é o produto (mensagens de erro citam regras); link quebrado é doc quebrada |
| `multiple_unsafe_ops_per_block`, `undocumented_unsafe_blocks` | `deny` | irrelevantes com `forbid`, mantidos por simetria se o `forbid` algum dia cair |

## 4. `clippy.toml` — os valores que valem aqui

Mesmo formato do `sniff`, com os números ajustados a um parser/codegen:

```toml
msrv = "1.92"          # = rust-version; o pin do toolchain é outro número
cognitive-complexity-threshold = 15
type-complexity-threshold = 150
too-many-arguments-threshold = 5
excessive-nesting-threshold = 3         # parser aninhado é onde a complexidade se esconde
max-fn-params-bools = 1                 # >1 bool vira enum
struct-field-name-threshold = 3
enum-variant-size-threshold = 128
pass-by-value-size-limit = 128
array-size-threshold = 4096
stack-size-threshold = 65536
failure-size-threshold = 8192
allow-unwrap-in-tests = true            # `.unwrap()` no teste é o mecanismo de falha
allow-print-in-tests = true
allow-indexing-slicing-in-tests = true
allow-expect-in-consts = false
disallowed-names = ["foo", "baz", "quux", "tmp", "thing", "stuff", "misc", ".."]
doc-valid-idents = ["html", "HTML", "CSS", "WCAG", "DOM", "ARIA", "SVG", "JSON", "ETag", "URL", "URI",
                    "srcset", "oklch", "BCP-47", "CLDR", "TOML", "MSRV", "TTI", ".."]
```

Divergência deliberada em relação ao `sniff`: **não** banimos `vec!`/`format!` em teste, e não usamos
`await_holding_*` (não há `async` no crate — e isso também é uma proibição: ver §7).

## 5. `rustfmt.toml` — só opções estáveis

O `sniff` documenta o porquê: em toolchain estável, opções `unstable` **são ignoradas com aviso**, então
configurá-las dá a falsa impressão de padrão. Aqui, além do óbvio:

```toml
edition = "2024"
style_edition = "2024"        # `cargo fmt` infere da edition; `rustfmt` direto assume 2015 sem isto
max_width = 100
tab_spaces = 4
newline_style = "Unix"
hex_literal_case = "Lower"    # ids/hashes/ETag saem em hex minúsculo — é **saída**, não estilo
use_field_init_shorthand = true
use_try_shorthand = true
```

`hex_literal_case = "Lower"` não é cosmético: o crate emite hex em `id`/hash/`srcset`; a caixa do literal
no fonte é a primeira coisa que denuncia divergência entre o literal e o byte emitido.

## 6. Perfis: falhar em vez de mentir

```toml
[profile.release]
overflow-checks = true      # aritmética não estoura em silêncio no caminho de render
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"

[profile.dev]
overflow-checks = true
debug-assertions = true
```

`overflow-checks` em release é o item que o `sniff` mantém e que aqui é ainda mais barato (o crate não tem
caminho numérico quente): se uma largura estourar, a falha aparece **no teste**, não no HTML.

## 7. Supply chain: `deny.toml` com a mesma política de licença do projeto pai

```toml
[advisories]
version = 2
yanked = "deny"

[licenses]
version = 2
allow = ["MIT", "Apache-2.0", "Apache-2.0 WITH LLVM-exception", "BSD-2-Clause", "BSD-3-Clause",
         "ISC", "Unicode-3.0", "Unicode-DFS-2016", "Zlib", "BSL-1.0", "CC0-1.0", "MPL-2.0"]
confidence-threshold = 0.8

[bans]
multiple-versions = "warn"
wildcards = "deny"
allow-wildcard-paths = true

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

A allowlist é a **mesma** da constituição de origem (permissivas apenas; MPL só build-only): o `deny.toml`
passa a ser a execução automática de uma regra que hoje é convenção em documento.

## 8. CI: cinco jobs, cada um cobrindo o que o outro não vê

| Job | Comando | O que ele pega que os outros não pegam |
|---|---|---|
| `check` | `cargo fmt --all -- --check` · `clippy --workspace --all-targets --all-features -- -D warnings` · `test --workspace` · `test -p html --features audit` | estilo, lints, unidade+integração+**doc-tests** (doc-test roda em `cargo test`) |
| `supply-chain` | `cargo deny check` · `cargo audit` · `cargo machete` | licença proibida, advisory, dependência morta (`typos` saiu: dicionário inglês × corpus pt-BR) |
| `miri` | `cargo miri test -p html --test miri_smoke` (nightly) | UB/leitura inválida no núcleo puro (barato com `forbid(unsafe_code)`) |
| `fuzz` | `cargo +nightly fuzz run parser` (smoke 30 s) | **o parser entrando em pânico com input adverso** — a única falha que o macro não pode ter |

O alvo de fuzz deste crate é o **parser do macro**, não rede: qualquer `TokenStream` malformado deve virar
`compile_error!`, nunca pânico. `msrv`/`miri`/`fuzz` fixam `RUSTUP_TOOLCHAIN` no job: sem isso o
`rust-toolchain.toml` (1.99) venceria o toolchain instalado e o job testaria o compilador errado. É o mesmo motivo pelo qual o `sniff` extraiu `decode_incoming` como função
pura e total.

## 9. Golden: byte-for-byte, regenerado por intenção

Padrão do `sniff` (`crates/sniff-engine/tests/golden.rs`), aplicado aqui:

- os arquivos de ouro são **versionados** (`crates/html/tests/golden/*.html`);
- a comparação é **byte-for-byte** (não normalizada);
- a regeneração é explícita: `UPDATE_GOLDEN=1 cargo test` — nunca automática;
- a mensagem de falha mostra o diff **e** o comando de regeneração.

O `insta` faz o mesmo com `INSTA_UPDATE=always`; o valor do padrão é a **regra**, não a biblioteca.

## 10. Política de exceção

1. Todo `#[allow]` exige `reason` (`allow_attributes_without_reason = deny`).
2. Prefira `#[expect]` (Edition 2024): quando a lint deixa de disparar, o `expect` **vira erro** — a
   supressão não fica zumbi depois de um refactor.
3. Exceção é **local** (função/statement), nunca de workspace.
4. "Desligar para limpar o CI" é violação: ou o código se adequa, ou entra `#[expect(…, reason = "…")]`
   com justificativa revisável.

## 11. O que **não** vem junto

| Item do `sniff` | Por que não |
|---|---|
| `tokio`, async, `await_holding_*` | o crate não faz I/O; `async` aqui seria dependência sem uso |
| CDP/Flutter/MCP, `serde`, `clap`, `tracing` | domínio do `sniff`; o crate não tem CLI nem transporte |
| faseamento de débito (`docs/phase-3.md`) | não há débito: o crate nasce no padrão final |
| `criterion` como gate | benchmark informativo; orçamento de bytes é teste, tempo de parede não é contrato |

## 12. Efeito mensurável esperado

| Antes (sem o rigor) | Depois |
|---|---|
| panic em macro → erro sem span, difícil de diagnosticar | `compile_error!` sempre, com span, coberto por `trybuild` |
| ordem de `HashMap` → golden instável | `deny` de `HashMap`/`HashSet`; saída byte-estável |
| `as` em largura → bug silencioso | conversões saturadas e auditadas em `numeric` |
| licença nova entra sem revisão | `cargo deny` falha no CI |
| exceção `#[allow]` esquecida no código | `#[expect]` vira erro quando deixa de ser necessária |
