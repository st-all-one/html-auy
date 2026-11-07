<!-- knudge:start -->
<!-- knudge:version: 4 -->
# knudge — project memory

`notas/` is the source of truth; index/graph/embeddings are **derived** — never edit `notas/` by hand.

Loop: `kd prime` (protocol; `--long` = full) → `kd ask "<q>"` before acting → `kd write` each
learning → `kd rewind` for handoff (`--with-activity` adds recent agent activity).
- `kd ask` dedups: score <0.75 create · 0.75–0.92 merge · ≥0.92 reject.
- One assertion per note; anchor code with `--anchor PATH` (glob `src/**`).
- Work lives in `kd task`: `epic ⊃ {issue ⊃ task | task}`; `--scope epic|issue|task`,
`--kind task|error|question|risk|decision`; `kd task list --ready`; close with `--outcome`.
- `kd write --type task` is rejected; `ask` excludes work unless `--with-task`.
- Health: `kd doctor` (`--fix`); `kd maintenance learn|compact|prune` propose only.
- Embeddings are derived: `kd drain --status` / `--digest`.
- Agent memory shards: `kd ai --memory|--session "<q>"` (read-only NDJSON,
`.knudge/ai/<area>/<agent_id>.jsonl`); `--sort time`, `--agent-id/--agent-role`,
`--recap` (compact recap); filter `ask` with `--agent/--activity/--agent-id/--agent-role`.
- stdout = data, stderr = logs; `--json` = {success, command, data?, error?, warnings?}.
- If `kd` is missing on the host: `curl -fsSL https://raw.githubusercontent.com/st-all-one/knudge/main/install.sh | bash` (installs `~/.local/bin`), then `kd prime`.
Config: `.knudge/config.toml`.
<!-- knudge:end -->

<!-- akribie:start -->
## akribie

Diagramas são `.akrc` versionados; SVG/PNG/scene/excalidraw são derivados — nunca edite o derivado.

Fluxo: `akr check chart.akrc` → `akr fmt chart.akrc --write` → `akr render chart.akrc > chart.svg`.
- `akr check` sai 8 em erro; corrija **só** a linha `arquivo:linha:col: E0xx … hint:` e reenvie
  (`--strict` promove aviso a erro; `--docs` resolve `doc=`).
- `akr fmt --check` sai 1 quando a forma canônica difere; `--write` grava.
- `akr render --format png|jpg|avif --out chart.png [--scale 2]` (raster exige `--out`).
- `akr render --format scene|excalidraw` e `--dry-run` depuram sem escrever.
- `akr diff OLD NEW [--json]` mostra `+`/`-`/`~` por id; exit 1 quando há mudança.
- `akr check --batch lote.jsonl` e `akr render --batch lote.jsonl --out-dir dist/` processam N charts;
  `--params '<json>'` substitui integralmente as flags do verbo.
- `akr import desenho.excalidraw > desenho.akrc`; `akr schema` lista chaves e valores fechados;
  `akr prime [--long]` é o guia de gramática (BNF, tabelas e temas no `--long`).

Configuração: `akribie.toml` (`layout`, `direction`, `theme`, `fontsize`, `pad`, `lang`, `sketch`,
`font`); precedência: opção no `.akrc` > arquivo > default. `sketch=off font=sans` = traço técnico.

Acessibilidade é requisito: título sempre presente (`<title>`), `<desc>` com a topologia,
`aria-label` por elemento, contraste garantido pelos tokens; nunca codifique sentido só por cor.

MCP: `akribie-mcp` expõe `chart_check`, `chart_render` e `chart_diff` (JSON-RPC por stdio).
<!-- akribie:end -->

# html-auy — guia de evolução

> Os blocos `knudge` e `akribie` acima são gerados; não edite o conteúdo entre os marcadores. As regras abaixo são específicas deste repositório e vêm antes de qualquer convenção genérica.

## Missão

`html-auy` é uma biblioteca Rust que trata HTML como **artefato**: o macro `html!` valida o fragmento em compilação, valores tipados escolhem o escape, `Page` monta o documento, e `audit` verifica o documento final. O projeto prioriza correção, bytes determinísticos, acessibilidade por construção e uma fronteira explícita entre crate, consumidor e navegador.

Não transforme o crate em framework, template engine de runtime, VDOM, sanitizador genérico ou camada HTTP. O consumidor é dono de I/O, rotas, sessão, headers, locale, CSS e gates de navegador.

## Arquitetura e fontes de verdade

- `crates/html-auy-macros-core/`: parser, validação e codegen; sem `proc_macro` e testável isoladamente.
- `crates/html-auy-macros/`: proc-macro `html!`; superfície mínima.
- `crates/html-auy/`: API pública, valores, `Markup`, shell, componentes e `audit` opcional.
- `spec/elements.toml`: fonte normativa de elementos, atributos, conteúdo-modelo e invariantes.
- `spec/data.toml`: tabela de `data-*` do consumidor usada por `tools/data_check.py`.
- `tools/gen_spec.py`: gera as duas cópias Rust da tabela.
- `tools/validate_spec.py`: valida spec, escopos, contagens e rastreabilidade com a documentação.
- `docs/`: decisões, arquitetura e limites. `docs/usage/`: wiki operacional.
- `llms.txt` e `llms-full.txt`: contexto para LLM, em inglês.
- `SKILL.md`: instruções de uso correto para agentes, em inglês.
- `docs/acessibility.md`: arquitetura de acessibilidade, em português.

Nunca edite `crates/html-auy/src/spec/table.rs` ou `crates/html-auy-macros-core/src/table.rs` à mão. Edite `spec/elements.toml`, rode `python3 tools/gen_spec.py` e valide.

## Regras duras que não podem regredir

1. `Markup` não tem construtor público a partir de `String`; não reintroduza `Raw`, `PreEscaped`, `|safe` ou equivalente.
2. `html!` não emite elementos de documento (`html`, `head`, `body`, `title`, `meta`, `link`, `script`, `noscript`, `main`); isso é responsabilidade de `Page`.
3. `<form>` não existe em `html!`; use `Form::open`/`Form::open_with_submit` e mantenha o CSRF no tipo do método.
4. Texto literal não entra em `html!`; use `Msg`, `Text::new` ou `(markup)`.
5. Use valores tipados (`Href`, `LUrl`, `Src`, `Alt`, `Class`, `Label`, `AttrVal`, `Timestamp`, `Autocomplete`) em vez de `String`/`&str` para atributos.
6. Não adicione `on*`, `<style>`, `srcdoc`, atributos arbitrários, nomes de elemento/atributo dinâmicos ou `<script>` inline.
7. `style=` é escape hatch auditado: não é sanitização CSS; qualquer uso precisa de justificativa e acomodação de CSP.
8. `accesskey` é proibido. O crate fornece skip link e landmarks; `Alt+1` é decisão do consumidor, não feature do crate.
9. O crate público é `no_std` + `alloc`; `audit` é a única feature opcional e não pode vazar para a árvore normal.
10. Não use `unsafe`. Em código de biblioteca, os lints negam `unwrap_used`, `expect_used`, `panic`, `unreachable`, indexação e aritmética não verificada; propague `Option`/`Result` e use operações saturantes.
11. A saída é canônica: não dependa de `HashMap`, locale, indentação ou ordem de declaração para atributos/classes/assets.
12. Não existe camada advisory: uma regra é erro de compilação, teste que falha ou gate de navegador do consumidor.
13. Não adicione dependência normal ao crate público; o padrão é `core`/`alloc`. Dependência externa exige decisão arquitetural e feature explícita. Não introduza `build.rs`: a tabela é gerada por `tools/gen_spec.py` e versionada.

## Acessibilidade

A ordem de garantia é: tipo/ausência de API → primitivo → macro → `audit` → gate do consumidor. Preserve:

- `Img` exige `Alt` e dimensões; decoração exige `Alt::decorative()`.
- `Video::new` exige `Track::captions`; `Track::descriptions` cobre audiodescrição.
- `Transcript`, `Audio::transcript` e `Video::transcript` cobrem alternativas textuais; uma transcrição não substitui a faixa descritiva de 1.2.5.
- `Field`/`FieldState` derivam label, id, ajuda, erro e `aria-*`; não escreva esses ids à mão.
- `Page` emite skip link, `<main>`, região viva persistente e o slot opcional `ShellData::help`.
- `aria-current` é único por `<nav>` mais próximo; `false` não conta.
- `audit` cobre 13 regras de documento; não o trate como sanitizador nem como prova de contraste, alvo, foco ou layout.
- Contraste, foco visível, oclusão, reflow, animação e ordem de tabulação continuam gates de `axe`/`sniffCSS`/e2e do consumidor.

Leia `docs/05-acessibilidade.md`, `docs/acessibility.md` e `docs/23-acessibilidade-e-css.md` antes de mexer em primitivos de acessibilidade.

## Evolução: vocabulário e invariantes

Para adicionar ou mudar elemento, atributo, valor ou invariante:

1. Altere `spec/elements.toml` (fonte única).
2. Se criar invariante, declare o escopo em `[invariants]` e vincule-a ao(s) elemento(s); `block`, `type` e `output` têm donos diferentes.
3. Regenerate: `python3 tools/gen_spec.py`.
4. Adicione/ajuste testes de macro, golden, `trybuild` ou `audit` conforme o escopo.
5. Atualize `docs/04-validacao.md` e outras docs afetadas, incluindo contagens se mudarem.
6. Rode `python3 tools/validate_spec.py` e `python3 tools/gen_spec.py --check`.

Contagens atuais: **111 elementos**, **71 invariantes** (59 `block`, 4 `type`, 8 `output`) e **13 regras de `audit`**. Se mudarem, atualize `docs/04`, `docs/12`, `docs/21`, `docs/23`, README e `llms*.txt` quando aplicável.

Novas regras que dependem do documento inteiro pertencem a `output`/`audit`; não tente fazê-las no macro. O macro só vê um bloco.

## API pública e compatibilidade

- A API pública é a de `html-auy`; `html-auy-macros` e `html-auy-macros-core` existem para a macro e não devem virar dependências de aplicação.
- Mudança que quebra código previamente válido é breaking; em `0.x`, sobe o minor. Siga `docs/11-estabilidade.md`.
- MSRV é **1.92**, edition 2024. O toolchain de desenvolvimento é fixado em 1.99; não confunda os dois.
- Mantenha a licença dual **MIT OR Apache-2.0** e os arquivos `LICENSE-MIT`/`LICENSE-APACHE` na raiz e nos três crates.
- O projeto não publica binário. Releases publicam os três crates em ordem de dependência.

## Validação obrigatória

Antes de concluir uma mudança não trivial:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
cargo test -p html-auy --features audit --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
python3 tools/gen_spec.py --check
python3 tools/validate_spec.py
python3 tools/check_links.py
python3 -m unittest discover -s tools/tests
```

MSRV no alvo do CI:

```bash
cargo +1.92 test --workspace --lib --locked
```

Os `.stderr` de `trybuild` são artefatos do rustc 1.99 e não são comparados na MSRV. Não os regenere para “consertar” uma diferença de compilador sem intenção.

Quando mexer em dependências ou supply chain, rode também:

```bash
cargo deny check
cargo audit
cargo machete
cargo package -p html-auy-macros-core --locked
```

O CI também roda Miri e fuzz do parser; preserve os alvos existentes.

## Documentação e idioma

- `README.md`, `docs/`, `docs/usage/` e `docs/acessibility.md`: português.
- `SKILL.md`, `llms.txt` e `llms-full.txt`: inglês.
- Os blocos gerados de `knudge`/`akribie` neste arquivo não devem ser editados manualmente.
- Mudança de API pública deve atualizar a wiki de uso, o quick reference, `llms.txt`/`llms-full.txt` e, quando for instrução de uso, `SKILL.md`.
- Mudança de acessibilidade deve atualizar `docs/05-acessibilidade.md` e `docs/acessibility.md` quando a fronteira ou os mecanismos mudarem.
- Verifique links relativos com `python3 tools/check_links.py`.

## Checklist de aceitação

- [ ] A mudança preserva a filosofia: HTML artefato, sem runtime, sem raw HTML, sem template dinâmico.
- [ ] O código compila com `--all-features` e sem `audit`; a árvore normal não puxa `html5ever`.
- [ ] Testes cobrem o caminho alterado: golden/`insta`, `trybuild`, `audit` ou unidade, conforme o caso.
- [ ] Spec, tabelas geradas, contagens e documentação estão coerentes.
- [ ] Acessibilidade e segurança não foram trocadas por conveniência.
- [ ] A saída continua canônica e determinística.
- [ ] MSRV, clippy, docs, links e ferramentas passam.
- [ ] Licença dual e arquivos de licença continuam presentes.
