# 23 — Acessibilidade e CSS: a fronteira do que é decidível

> **Proveniência.** O catálogo de regras vem do `sniff-css` (`crates/sniff-css-check/src/rules.rs`, 25
> checks medidos sobre estilo computado, mais `--uniform`), e a doutrina de julgamento de
> `docs/accessibility.md` (grade de acessibilidade, `is_user_noticeable`, contraste composto in-page).
> **Absorção:** o **conhecimento de fronteira** (o que é estático, o que exige browser) entra na spec e no
> `N4`; a **execução** continua no consumidor, como gate separado.

## 1. A pergunta que o catálogo força

Cada regra de acessibilidade responde a uma de três perguntas — e essa divisão decide **onde** ela mora:

| Decidível em | Pergunta | Onde mora no html-auy | Custo |
|---|---|---|---|
| bloco `html!` | "este elemento tem atributo/estrutura coerente?" | invariante `block` (macro) | zero em runtime |
| página montada | "o conjunto de fragmentos é coerente?" | invariante `output` (`N4`) | barato, offline |
| **só com estilo computado** | "como o navegador **pintou** isto?" | gate do consumidor (`sniff`) | um browser por execução |

O erro que o catálogo expõe: é tentador anunciar regra de **contraste** ou **alvo mínimo** como
"acessibilidade embutida". Não é — depende de `color`, `background` composto, `font-size`, `opacity` e de
quem cobre quem. Nenhum tipo e nenhum parse estático responde isso.

## 2. As 25 regras medidas, por decidibilidade

### 2.1 Estáticas (cabem em `html!` ou no `N4`)

| Regra do `sniff` | WCAG | Decidível | No html-auy |
|---|---|---|---|
| `empty-alt-image` | 1.1.1 | bloco | `alt_empty_means_decorative` + tipo `Alt` (sem vazio implícito) |
| `control-without-name` | 1.1.1 / 2.4.4 / 4.1.2 | bloco | **novo**: `control_has_accessible_name` (`a`, `button`, `input`) |
| `aria-hidden-focusable` | 4.1.2 | bloco (mesmo elemento) / saída (wrapper) | **novo**: `no_aria_hidden_on_focusable` (`a`, `button`, `input`) |
| ordem de títulos (`h2 → h4`) | 1.3.1 | página | **novo**: `heading_levels_no_skips` (`output`) |
| `id` duplicado entre fragmentos | 1.3.1 / 4.1.1 | página | **novo**: `ids_unique` (`output`) |
| landmark/`main`/`h1` únicos | 1.3.1 / 2.4.1 | tipo + página | `main_unique`, `single_h1`, `shell_owns_h1`, `landmark_names_distinct` |

Os quatro **novos** entraram na spec nesta revisão (73 invariantes: 62 `block`, 6 `type`, 5 `output`). Os
três primeiros são de bloco porque o macro vê o conteúdo do controle e os atributos dele; os dois últimos
são de página porque dependem do conjunto que nenhum fragmento vê.

### 2.2 Só com estilo computado (gate do consumidor, nunca do crate)

| Regra do `sniff` | WCAG | O que mede |
|---|---|---|
| `contrast-aa` / `contrast-aaa` | 1.4.3 / 1.4.6 | razão WCAG com **fundo efetivo composto** (transparentes até o canvas) |
| `target-size` | 2.5.8 | área de toque < 24×24 CSS px |
| `focus-indicator` | 2.4.7 / 2.4.11 | `outline`/`box-shadow` removidos no `:focus-visible` |
| `hidden-focusable` | 2.4.3 / 4.1.2 | focável dentro de container `display:none`/`visibility:hidden` |
| `small-text` | 1.4.4 | `font-size` abaixo do limiar legível |
| `line-height-below-font-size` | 1.4.12 | entrelinha menor que o corpo |
| `occluded` / `backdrop-over-modal` | 2.4.11 / 1.4.3 | elemento coberto por outro (inclusive o scrim sobre o próprio diálogo) |
| `sticky-in-overflow-hidden` | — | `position:sticky` quebrado por ancestral com `overflow≠visible` |
| `fixed-broken-by-transform` | — | `position:fixed` relativo a ancestral com `transform`/`filter`/`contain` |
| `z-index-on-static` | — | `z-index` sem efeito (`position:static`) |
| `absolute-without-insets` | — | `position:absolute` sem nenhum inset |
| `width-100-with-padding` | — | estouro horizontal por `box-sizing:content-box` |
| `ellipsis-without-clip` | — | `text-overflow:ellipsis` sem `overflow:hidden`/`nowrap` |
| `text-not-selectable` | — | `user-select:none` em conteúdo textual |
| `transition-all` | — | `transition-property:all` (custo imprevisível) |
| `infinite-fast-animation` | 2.3.3 | animação infinita sem `prefers-reduced-motion` |
| `horizontal-overflow` / `overflow-x-hidden-on-body` | 1.4.10 | rolagem lateral de página (ou escondida no `body`) |
| `interactive-pointer-events-none` | 2.1.1 | controle com `pointer-events:none` |
| `small-thumbnail` | 1.4.4 | miniatura renderizada pequena demais |
| `unmeasurable` (estado) | — | geometria zero — **não** é falha de visibilidade |

**Nada nesta tabela pode virar tipo.** Mover qualquer uma delas para o crate seria prometer o que o
compilador não vê — o mesmo erro que D21 corrigiu nas regras de página.

## 3. Por que isso importa para o desenho

1. **`Class` fechado é necessário, não suficiente.** Token fechado impede classe inventada; não impede
   contraste ruim, alvo pequeno ou `transition:all` no CSS por trás do token. A garantia de CSS é do gate.
2. **`data-*` e JS**: o catálogo mostra que metade dos erros "comuns de CSS" só aparece com o navegador
   pintando — logo, o consumidor precisa de **um** gate de browser, e o crate não deve duplicá-lo.
3. **`is_user_noticeable`** (do `sniff`) é a lição de vocabulário que vale copiar: separar
   `display_visible` (está renderizado) de `accessibility_grade` (`NONE`/`AA`/`AAA`) evita a confusão
   clássica "abaixo da dobra = invisível = falha". O `N4` não precisa disso porque só vê HTML, mas o
   relatório do consumidor precisa — e isso é documentação de integração, não código do crate.

## 4. Como integrar (separadamente, no consumidor)

```bash
# 1) o crate já garantiu o que é estático (compile-time + N4)
cargo test                       # inclui N4 sobre as páginas capturadas

# 2) o consumidor mede o que só o browser sabe (sniff, fora do crate)
sniff capture "$URL" -s "main" --depth 5 > main.jsonl
sniff check main.jsonl --rules | jq -r 'select(.status=="fail") | [.check,.selector]|@tsv'
sniff check main.jsonl --uniform            # "o card estranho" entre irmãos
```

Regra de ouro da integração: **nenhum check do `sniff` substitui uma invariante do crate, e nenhuma
invariante do crate dispensa o `sniff`.** Eles cobrem camadas diferentes, e a fronteira está em §2.

## 5. O que foi absorvido nesta revisão

| Absorvido | Onde |
|---|---|
| divisão por decidibilidade (bloco / página / computado) | este documento + [`04`](04-validacao.md) §4.0 |
| 4 invariantes estáticas novas | `spec/elements.toml` (`control_has_accessible_name`, `no_aria_hidden_on_focusable`, `heading_levels_no_skips`, `ids_unique`) |
| catálogo de 25 regras computadas como **contrato de gate** do consumidor | §2.2 |
| vocabulário `display_visible` × `accessibility_grade` | §3.3 |
| doutrina "medir, não chutar" (evidência no relatório) | [`22`](22-rigor.md) §9 e [`12`](12-testes.md) |

**Não absorvido:** as regras em si, o CDP, o parser de propriedades CSS, os bundles de captura. O crate
não abre browser; o consumidor já tem ferramenta para isso.
