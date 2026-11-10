# 27 — Lições do `eta` para o 0.1.0

> **Documento histórico.** Registra a leitura feita para o 0.1.0; o demo citado foi removido no
> 0.2.0. O exemplo executável atual é `crates/html-auy/examples/portal_governamental.rs`.

Este documento registra a leitura do `eta/` (templating engine em TypeScript, v4) e o que foi
absorvido — e recusado — no `html-auy` 0.1.0. O objetivo não é copiar a arquitetura de templates em
runtime, e sim roubar o que ela faz bem: **superfície pequena, defaults explícitos e erro com
contexto**.

## 1. O que o `eta` faz

| Peça | O que faz | Lição |
|---|---|---|
| `src/config.ts` | um `EtaConfig` com **todos** os defaults num único objeto | o consumidor configura uma vez; não existe “modo invisível” espalhado |
| `src/core.ts` | `class Eta extends EtaCore {}` — a fachada pública é quase vazia | API pública pequena; implementação fica em `internal.ts` |
| `src/parse.ts` → `compile.ts` → `render.ts` | parse gera AST, compile gera função, render executa | passes separados, cada um testável isoladamente |
| `src/err.ts` | `EtaParseError`/`EtaRuntimeError` com linha, coluna e trecho do template | erro aponta o lugar e o contexto, não só “falhou” |
| `src/storage.ts` | armazenamento de templates é substituível | I/O fica fora do núcleo |
| `test/*.spec.ts` | um arquivo de teste por módulo | a suíte cresce junto com a superfície |

Números do próprio projeto: ~3,5 KB minzipped, zero dependências, um único ponto de entrada.

## 2. O que foi aplicado no `html-auy`

### 2.1 Uma forma canônica para cada destino

`Href` ganhou `Fragment` e `Href::fragment`. Antes o demo inventava `Href::internal(LUrl("/#"))`
para escrever `/#`; agora `href="#"` e `href="#conteudo"` são tipos validados (`#` + alfabeto de
`Id`). O link de skip, o índice e o cartão de serviço usam a mesma regra.

### 2.2 Conteúdo transparente em vez de exceção

`<a>` passou a declarar `transparent = true`. O validador herda o content model do ancestral não
transparente mais próximo:

- um `<a>` dentro de `<div>` pode conter `<div>` (o cartão de serviço);
- um `<a>` dentro de `<p>` não pode (herda `phrasing`);
- `<a>` continua proibindo **qualquer** interativo descendente (`button`, `input`, outro `a`, …),
  mesmo através de `<div>`.

O `spec` agora descreve o HTML real; o demo não precisa de string crua para o grid.

### 2.3 `title` é global; `h1` é markup comum

- `title` entrou em `[globals]` como `Text`. Ele escapa como qualquer atributo e **não** conta como
  nome acessível (`control_has_accessible_name` continua exigindo texto/`aria-labelledby`).
- `h1` saiu de `only = "shell"`. O `Page` continua emitindo o heading por padrão, mas um shell
  próprio pode declarar o seu; `single_h1` passou para `output` e o `audit` cobra exatamente um.
  A promessa “é tipo, não teste” foi trocada por “o documento montado é testado” — o preço da
  flexibilidade, explicitado em `docs/design/04` §4.0.

### 2.4 Dados + loop no demo, não quatro `Render`

O demo tinha quatro implementações de `Render` (`HeroTitle`, `ServiceGrid`, `HeaderControls`,
`Marker`) e 8,6 KB de HTML cru. Agora:

- `SERVICES: &[Service]` e `CONTROLS: &[Control]` são dados;
- `service_grid()` e `header_controls()` são `html!` com `for`;
- `Marker` virou `div title="…"`;
- `HeroTitle` virou `h1`.

Além disso, a superfície do `html!` ficou mais próxima do `eta` no caso comum:

- `class="a b"` é aceito como lista **estática** e o codegen divide, ordena e deduplica; `Class::of`
  continua para o enum de `tokens.css` e `Class::new("a b")` é equivalente, sem hierarquia entre
  as formas.
- texto visível continua rigoroso: `Msg` para catálogo/i18n e `Text::new` para dado dinâmico; o
  parser recusa literal `"Olá"` no `html!`.
- classes dinâmicas têm porta opt-in em `ClassList::try_new`; assets preservam a ordem declarada,
  aceitam CDN por construtor explícito e `html`/`body` recebem classe/`data-*` via `RootAttributes`.
- os dados estáticos saíram de `page.rs` para `content.rs` (`SERVICES`, `CONTROLS`, `NAV`,
  `EXEC_QUICK`, `LEG_QUICK`, `GAZETTE`, `Msg`); `page.rs` é só marcação + loops.

Resultado no corpo: nenhum `Render` de consumidor, nenhum `<a href="#">` sem passar pelo tipo e
nenhum `/#` na saída. O `<head>` continua sendo o `head.html` do exemplo (Tailwind CDN + config
inline), uma concessão **do demo**, não do crate; ele é o único trecho fora da validação.

### 2.5 Nomes que dizem o que fazem

`rules::children` virou `rules::descendants`: a função sempre percorreu netos, e o nome antigo
sugeria filho direto. `validate::check_children` continua sendo o passe de filhos diretos. A
distinção agora está no nome, não no comentário.

## 3. O que foi recusado

| Ideia do `eta` | Por que não |
|---|---|
| template em runtime + `Storage` | o `html-auy` existe para o HTML ser o artefato e o erro ser de compilação; template em runtime devolve o erro para a request |
| `useWith` / execução de JS no template | o crate proíbe `script` executável e `on*`; `style=` existe só como escape hatch auditado, e executar código no template é o modelo de ameaça que ele evita |
| plugins que reescrevem a AST | a AST é fechada e validada contra `spec/elements.toml`; plugin é uma segunda linguagem para manter |
| `autoEscape` configurável | escape é contexto, não opção global: `Href`, `Src`, `Text`, `JsonLd` já são o “config” |
| cache de templates | o `Markup` já é o resultado pré-compilado; cache é do consumidor |

## 4. Números do 0.1.0

| Métrica | Antes | Depois |
|---|---:|---:|
| `demo/html-auy/src/page.rs` | 95,5 KB / 1.415 linhas | **72,8 KB / 1.132 linhas** |
| `demo/html-auy/src/content.rs` | — | 7,5 KB / 200 linhas (dados) |
| HTML cru no corpo | 8,6 KB | 0 |
| `impl Render` no demo | 4 | 0 |
| fonte total do demo | 95,5 KB / 1.415 linhas | 80,3 KB / 1.332 linhas |
| comparação | — | maud: 86,8 KB / 1.389 linhas; askama: 58,7 KB (template de 2 linhas + main de 53) |
| `href="/#…"` na saída | 52 | 0 |
| `<div>` dentro de `<a>` no demo | string crua | validado pelo macro |
| erros iniciais ao portar o exemplo | 20 | 0 |

## 5. O que ainda não está pronto (e é decisão, não esquecimento)

O `head.html` do demo usa Tailwind CDN e um `<script>` inline. Isso é o que o `EXAMPLE.html`
original faz e é **incompatível** com `no_inline_script`/CSP do crate. O `audit` do `html-auy` não é
aplicado a esse documento de demonstração; um consumidor de produção usa `Asset::style`/`Asset::script`
e um CSS compilado. A biblioteca em si continua auditável: os testes de `Page` e de `audit` não
dependem do demo.
