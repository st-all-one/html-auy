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
