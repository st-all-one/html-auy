# 09 — Desempenho e cache

## 1. Modelo de alocação

- `html!` produz um único `String` dentro de `Markup` (uma alocação por bloco, não por nó).
- O escape é feito **durante** a escrita (`push_str`/`extend_from_slice`), sem buffer intermediário.
- `Render::render_to(&mut String)` existe para reaproveitar buffer — usado pelo shell para montar a
  página inteira sem alocações por fragmento.
- O macro estima o tamanho do bloco (`tags` + atributos + folga por interpolação) e o `Writer` já
  nasce com `String::with_capacity(…)`: o custo por bloco vira **uma** alocação, e um atributo
  literal reserva o pior caso do escape.
- Nenhum valor tipado aloca ao ser construído além do necessário (`Text<'a>` empresta; `Alt<'a>`
  empresta; `SrcSet` é montado uma vez e reusado).

Metas declaradas (não dogmas):

| Métrica | Meta |
|---|---|
| render de uma view típica (< 300 nós) | < 20 µs, < 3 alocações |
| render de lista de 100 itens | < 500 µs, alocações proporcionais a 1 + itens |
| página completa (shell + main) | < 1 ms de CPU |

## 2. `ETag` canônico

- `ETag` do corpo = `sha256` dos bytes finais (não do dado de entrada).
- Para que o hash seja estável entre requests, a emissão é **determinística**:
  - ordem de atributos = ordem declarada na spec;
  - `SrcSet`: largura crescente, tipo MIME em ordem lexical;
  - classes: deduplicadas e em ordem estável (`Class` ordena por `Token` derivado);
  - assets: lista na ordem de declaração, com dedupe;
  - tokens de tema e regras de CSS: `BTreeMap`.
- `304` quando `If-None-Match` casa. `Cache-Control: public, no-cache` para HTML com formulário em
  área própria; HTML público pode ser cacheável (decisão do consumidor).
- **Nenhum identificador de build ou de estado entra no HTML.** Sem hash do binário, sem id de
  sessão, sem nonce, sem `data-version`: qualquer um deles muda com o que não é conteúdo e o `ETag`
  passa a invalidar a cada deploy (ou a cada login).
- **Knobs que mudam byte são proibidos.** Não existe configuração de `trim`, `rmWhitespace`,
  auto-escape, delimitador customizado, resolução de caminho ou cache de template: qualquer um deles
  reescreve a saída sem mudar a entrada tipada, e o `ETag` deixa de significar "este conteúdo".
- **Nunca** minificar HTML em runtime: quebra `ETag` estável, dificulta depuração e não paga o custo.
### 2.1 Regimes de cache por tipo de recurso

| Recurso | `Cache-Control` | Por quê |
|---|---|---|
| asset com hash no nome (`.a1b2c3.css/.js`, fonte, ícone) | `public, max-age=31536000, immutable` | o nome muda com o conteúdo |
| `/sw.js`, `/manifest.webmanifest` | `no-cache` + `ETag` | URL fixa por protocolo |
| HTML público | `public, no-cache` + `ETag` | revalida barato; conteúdo muda sem deploy |
| HTML autenticado / formulário | `private, no-cache` + `ETag` | **`no-store` é proibido**: bloqueia `bfcache` e o "voltar" fica lento |
| fragmento de swap | mesma política da página de origem | herda, não inventa |

- Fontes: `woff2` com `font-display: swap` e `preload` **do único arquivo de texto**. Use
  `Hint::Preload(Preload::font(...))`: o construtor emite `as="font" type="font/woff2"
  crossorigin="anonymous"` juntos — sem `crossorigin` o preload faz só DNS lookup e baixa duas
  vezes.
- Imagem LCP: `Img` exige `width`/`height` (CLS 0) e `srcset`+`sizes` saem juntos; o preload usa a
  **mesma** `SrcSet`/`Sizes` (`ImagePreload::responsive`), senão o navegador baixa duas variantes.
  Fora da dobra, `Img::lazy()`; para a LCP, `fetchpriority="high"` via `Img::fetchpriority`.
- Vídeo/áudio: `preload="metadata"` para não baixar o corpo inteiro na primeira pintura; `poster`
  ocupa o espaço antes do primeiro frame.
- `.br` é servido com `Content-Encoding: br` e `Vary: Accept-Encoding`; `.gz` é fallback, nunca o
  padrão.

- Exceções à regra de cache longo: `/sw.js` e `/manifest.webmanifest` com `no-cache` + `ETag` (URL
  fixa por necessidade do protocolo — [`07`](07-js-e-assets.md) §8); fragmento de swap **nunca**
  responde `304` na primeira carga (ver §7 daquele doc).

## 3. Compressão

- `.br` e `.gz` gerados no **build**, com hash de conteúdo no nome, servidos direto (sem compressão
  por request).
- Determinismo da compressão faz parte do contrato de build (mesma entrada → mesmos bytes).

## 3.1 Isolamento de render

`content-visibility: auto` só é emitido junto de `contain-intrinsic-size` (com altura estimada real):
sem o par, a região contribui com altura zero até ser renderizada e o `CLS` estoura — o ganho de
pintura vira regressão de layout.

## 4. Orçamento de bytes (gate)

- Teste de orçamento sobre fixtures: HTML de página de lista não pode crescer além de N% entre
  versões (orçamento de bytes por página, como o do projeto consumidor).
- CSS do `/theme.css` limitado por tamanho; número de regras por seção limitado pelo schema.
- Atributo condicional vazio não é emitido: `Option` (ausente), `SrcSet`/`Sizes` vazios (sem variante
  válida) e `Class::of(&[])` são omitidos pelo `Writer`. `AttrVal::new("")` computado é emitido — o
  consumidor só constrói um quando o valor vazio é intencional.

## 5. Custo de compilação

| Componente | Estratégia |
|---|---|
| `html-spec` | gerado no `build.rs`, cacheado; só recompila se `spec/elements.toml` mudar |
| `html-macro` | parser linear sobre tokens; sem análise semântica de Rust |
| views | um crate por área (ou por módulo grande) para paralelizar `cargo check` |
| `fmt` | roda sobre o AST em memória; sem reparse |

Regra: bloco `html!` acima de ~200 linhas deve ser dividido em primitivos. Não por estética — porque
um erro em bloco grande recompila uma unidade grande.

## 6. O que **não** se otimiza

- Micro-otimização de `push_str` em prejuízo de clareza.
- Concatenação de arquivos de view.
- Interning de string.
- Cache de render de fragmento em memória (o consumidor tem cache HTTP; fragmento é barato).

## 7. Medição

- `criterion` para: escaper, `SrcSet` de 6 variantes, view de tabela com 100 linhas, shell completo.
- Os números entram no README do repositório do consumidor, não aqui (este repositório é a
  especificação; o consumidor publica a medição).
- Regressão de desempenho > 20% em qualquer benchmark bloqueia merge no consumidor.
