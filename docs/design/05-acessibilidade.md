# 05 — Acessibilidade (WCAG 2.2 AA e eMAG)

## 1. Estratégia

Acessibilidade não é verificada no fim: ela é **emitida**. A ordem de eficácia adotada:

1. **ausência de API** — o errado não compila;
2. **primitivo que já vem certo** — o dev escolhe o componente, não os atributos;
3. **teste de página (N4)** — o que só o HTML final revela vira teste que falha no CI do consumidor;
4. **gate de CI** — axe, `sniffCSS`, teclado, e2e.

Um aviso que dispara 20 vezes por página é desligado na segunda semana. Por isso a regra 6 da
[§6](#6-ergonomia-as-sete-regras).

## 2. Requisitos de nível A/AA e o mecanismo que os sustenta

| SC | Requisito | Mecanismo |
|---|---|---|
| 1.1.1 | conteúdo não textual tem alternativa | **tipo**: `img` exige `Alt`; decorativa = `Alt::decorative()` explícito; `svg` exige `title` ou `aria-hidden` |
| 1.2.1 | alternativa para áudio pré-gravado | **tipo**: `Audio::transcript(Transcript)` envolve o player em `figure` com link para a transcrição; `Track` não substitui transcrição |
| 1.2.2 | legendas em vídeo pré-gravado | **tipo**: `Video::new(src, Captions)` — sem `Caption`, não compila |
| 1.2.3 | audiodescrição ou alternativa | **tipo**: `Video::track(Track::descriptions(…))` ou `Video::transcript(Transcript)` |
| 1.2.5 | audiodescrição (AA) | **tipo**: `Track::descriptions`; `Transcript` cobre 1.2.3, não substitui a faixa descritiva de 1.2.5 |
| 1.3.1 | informação e relações | **tipo** (semântica + `label`/`for` + `scope`) + **N4** (estrutura de página) |
| 1.3.2 | sequência significativa | **estrutural**: a ordem de leitura é a ordem do DOM (não existe `style`, logo não existe `order` visual divergente) |
| 1.3.4 | orientação | **CSS** (`@media (orientation)`) + gate de design |
| 1.3.5 | propósito do campo identificado | **tipo**: `Field::text(FieldName::new("nome"), Msg::Name, &FieldState)` — id/label/`aria-*` derivados (D25) |
| 1.4.1 | uso da cor | **tipo**: cor é token + texto/ícone; **N4**: "estado só por cor" na página renderizada |
| 1.4.3 | contraste mínimo (4.5:1) | **save-time** (paleta validada) + **axe** |
| 1.4.4 | redimensionar texto 200% | **tokens** (`rem`/`clamp`) + `sniffCSS` |
| 1.4.10 | reflow | **tokens** + `sniffCSS` (1366×768 e 390×844) |
| 1.4.11 | contraste de não-texto | **save-time** (bordas/foco como token) + axe |
| 1.4.13 | conteúdo em hover/focus | **tipo**: `Tooltip` com `role="tooltip"` e o gatilho apontando **`aria-describedby`** (não `aria-expanded` — tooltip não expande); fecha com `Escape` e não captura foco |
| 2.1.1/2.1.2 | teclado, sem armadilha | **e2e**: `02-teclado-e-foco` |
| 2.2.2 | pausar/parar | **composição** de elementos da spec (sem tag nova): rotação só com controle de pausa e parada em `hover`/`focus`; sem isso, sem rotação |
| 2.3.1 | sem flash | **CSS** + gate de animação |
| 2.4.1 | saltar blocos | **shell**: `ShellData::skip(Text)` emite o link para `#main` como primeiro focável |
| 2.4.2 | título de página | **shell**: `<title>` obrigatório em `Page` |
| 2.4.3 | ordem de foco | **shell**: ordem do DOM = ordem visual (posição de sidebar por CSS lógico) |
| 2.4.4 | propósito do link | **N4**: link ambíguo por variante de `Msg` (página inteira) |
| 2.4.6 | cabeçalhos e rótulos descritivos | **N4**: `<h1>` único, sem pular nível; o `Page` o emite ou a view o declara |
| 2.4.7 | foco visível | **CSS** `@layer a11y` + `sniffCSS` |
| 2.4.11 | foco não obscurecido (2.2, AA) | **CSS**: `scroll-padding-top` = altura da barra fixa; nenhum focável sob barra `sticky`/`fixed`; `sniffCSS` mede. 2.4.12 (AAA) fica fora da meta |
| 2.5.3 | rótulo no nome acessível | **N4**: texto visível do controle contido no nome acessível |
| 2.5.7 | movimentos de arrasto (2.2) | **composição**: ordenação sempre exposta também por botões "mover para cima/baixo" (sem `draggable`) |
| 2.5.8 | alvo mínimo 24×24 CSS px (2.2, **AA**) | **tipo**: `Button`/`Link` já trazem a classe de alvo; **sniffCSS** mede. O projeto adota **44 px** como regra interna mais estrita (2.5.5 é AAA); links em prosa são isentos |
| 3.1.1 | idioma da página | **tipo**: `html lang` obrigatório e BCP-47 válido |
| 3.1.2 | idioma de partes | **tipo**: `lang=(Lang)` em qualquer elemento (atributo global BCP-47) |
| 3.2.1/3.2.2 | sem mudança por foco/entrada | **estrutural**: não existe `on*`; mudança só por submit |
| 3.2.6 | ajuda consistente (2.2, **A**) | **shell**: `ShellData::help(Markup)` fixa o bloco entre header e `<main>`; o consumidor decide conteúdo e presença |
| 3.3.1 | identificação de erro | **tipo**: `Field` exige `erro: Option<Text>` e emite `aria-invalid` |
| 3.3.2 | rótulos ou instruções | **tipo**: `Field` exige `label`; `ajuda: Option<Text>` vira `aria-describedby` |
| 3.3.3 | sugestão de correção | **tipo**: `Field` recebe o texto de sugestão em `erro: Option<Text>` |
| 3.3.7 | entrada redundante (2.2, **nível A**) | **tipo**: `Autocomplete` com tokens de identidade; o reuso de valor no mesmo fluxo é do consumidor |
| 3.3.8 | autenticação acessível — mínimo (2.2, **AA**) | **tipo**: `input type=password` com `Autocomplete` (`current-password`/`new-password`/`username webauthn`), sem CAPTCHA e sem bloquear colar/gerenciador; o fluxo de passkey é do consumidor. 3.3.9 (AAA) fora da meta |
| 4.1.2 | nome, papel, valor | **tipo**: `a`/`button` exigem nome acessível; `Dialog` gera `aria-labelledby` (pós-MVP); **N4**: com dois ou mais `nav`/`aside`, cada um exige nome (`landmark_names_required`) |
| 4.1.3 | mensagens de status | **tipo + shell**: região viva (`role="status"` + `aria-atomic="true"`) **persistente no shell**, vazia no primeiro render e preenchida por swap/navegação. Sem `aria-atomic` só o trecho alterado é anunciado; `role="alert"` **nunca** pré-existente estático (não é anunciado) |

> **Defeito registrado no corpus `_RUST/web-accessibility/`:** `12-wcag-quick-reference.md:89-90`
> rotula 3.3.7 como *Accessible Authentication* (AA) e 3.3.8 como *(No Exception)* (AAA), e marca
> 3.3.9 como "não usado". O texto do W3C é: **3.3.7 Redundant Entry (A)**, **3.3.8 Accessible
> Authentication — Minimum (AA)**, **3.3.9 — Enhanced (AAA)**. Esta tabela segue o W3C.

**eMAG:** os itens de recomendações (1.1 a 6.x) mapeiam 1:1 nos primitivos e nas regras acima; a
diferença prática é a exigência de `lang="pt-BR"` (coberto por 3.1.1) e de rótulos em português nos
primitivos, garantidos pelo catálogo de `Msg`.

## 3. Primitivos (o que o dev não escreve)

**Estado:** entregues `Page`, `View`, `Form`, `Field::text`, `Field::input` (todos os 14 tipos nativos),
`Field::textarea`, `Field::select`, `Field::file`, `Field::checkbox`, `Field::checkbox_group`,
`Field::radio_group` e `autocomplete` validado; `Img`, `Picture`, `Video`, `Audio`, `Track` e `Transcript` vivem em
`media`. As demais linhas desta tabela seguem o catálogo pós-MVP; a coluna “Emite” é o contrato delas.

| Primitivo | Emite |
|---|---|
| `Field::text/input/textarea/select/file` | `<label for>`, `id`, `aria-describedby` com os `<span>` de ajuda/erro, `aria-invalid`, `required` e o `*` fora do `<label>`; `input` cobre os 14 tipos nativos; `file` não emite `value` e aceita `accept`/`multiple`; `autocomplete` só sai de um `Autocomplete` validado; `select` com opções como dado (`selected`/`disabled`) e `None` sem opção selecionável |
| `Field::checkbox`/`checkbox_group`/`radio_group` | `<fieldset>`+`<legend>`, ids `f-<nome>-<índice>`, `name[]` só em checkbox, `aria-describedby`/`aria-invalid`/`aria-required` no grupo e `checked`/`disabled` por opção; `None` quando todas as opções estão `disabled` |
| `Dialog` | `<dialog>`, `aria-labelledby` com id gerado, botão de fechar com nome, foco inicial |
| `Table` | `<caption>`, `<th scope>`, agrupamento `thead/tbody` |
| `Img`/`Picture`/`Video`/`Audio`/`Track`/`Transcript` | `Img` com `src`/`alt`/`width`/`height` obrigatórios e `srcset`+`sizes` acoplados; `Picture` com `source` ordenado e `img` de fallback; `video` exige `track kind="captions"`, `Track::descriptions` cobre 1.2.3/1.2.5; `Audio::transcript` cobre 1.2.1; `Track` com `srclang` em `subtitles` |
| `Icon` | `<svg aria-hidden="true" focusable="false">` por padrão; nomeável só por `Icon::labelled` |
| `Button`/`LinkButton` | `type`, alvo ≥ 2.75 rem, estado (`aria-pressed`/`aria-expanded`) quando aplicável |
| `Alert`/`Toast` | `role="status"` **sem** `aria-live` adicional (o papel já implica `polite`); `role="alert"` só para erro que interrompe, e também sem `aria-live` (duplicar faz o VoiceOver iOS anunciar duas vezes) |
| `Pagination` | `<nav>` nomeado, `aria-current="page"`, `rel=prev/next`, alvo mínimo |
| `Breadcrumb` | `<nav>`+`<ol>`, separador `aria-hidden`, um único `aria-current` |
| `SkipLinks` | primeiro focável do documento |
| `Tooltip` | `role="tooltip"` + gatilho com `aria-describedby`; abre no hover (1 s) e no foco, fecha com `Escape` |

## 4. Regras de página (N4) — racional

A camada advisory (N3) foi cortada (D28/A4). O que sobrevive é o que **só a página inteira revela**, e
que agora é **teste que falha** no CI do consumidor, não aviso com ratchet:

| Regra (N4) | Racional |
|---|---|
| `div` com `aria-label` | ARIA sem papel é ruído |
| link com `Msg` ambíguo | 2.4.4 |
| `alt` igual ao nome do arquivo | alt inútil (1.1.1) |
| `tabindex` positivo | 2.4.3 (ordem artificial) |
| `title` como único nome acessível | 4.1.2 (não aparece em toque/teclado) |
| estado só por cor | 1.4.1 |
| `id` duplicado / `h1` duplicado / salto de nível | 1.3.1, 2.4.6 (as 8 invariantes de `output`) |
| nomes de landmark repetidos | 1.3.1 |

As demais regras que eram advisory **não** viraram teste porque não precisavam de teste: viraram tipo,
erro de macro ou default do gerador — a tabela regra→destino está em
[`04-validacao.md`](04-validacao.md) §5.

## 5. Elementos crus: relatório, não gate

O ratchet (`a11y-budget.toml`) **saiu** junto com o N3 (D28/A4): um limite que só desce é um gate de
processo, não uma garantia de acessibilidade — e quem cobra a estrutura é o próprio vocabulário (o que
não está na spec não compila).

- A medição continua como **saída informativa** do teste de `N4`: percentual por view, para leitura.
- A meta de ≥ 90% do markup vindo de primitivos continua sendo critério de projeto, medido no `M2`
  ([`16`](16-roadmap.md) §2), não um gate de CI.

## 6. Ergonomia: as sete regras

1. **Default correto** — o primitivo emite toda a plumbing; o dev escolhe o componente.
2. **Mensagem acionável** — formato `o que` → `por quê (SC/regra)` → `como corrigir`; teste `trybuild`
   obrigatório.
3. **Escapulidas visíveis, nunca silenciosas** — `Alt::decorative()`, `Label::sr_only`,
   `Dialog::labelled_by`. Esquecer ≠ decorativa.
4. **Não existe advisory** (N3 cortado, D28/A4): ou é erro de compilação, ou é teste que falha.
5. **Tudo aparece uma vez** — skip link (com `ShellData::skip`), `<main id="main">` e a região viva vêm do shell;
   landmarks vêm das `Regions`; `data-a11y-*` é do consumidor (pós-MVP).
6. **Regra ruidosa é bug do componente** — se dispara > 3× na mesma página, corrige-se o primitivo.
7. **O mecânico é default do gerador** (D28/A3): `type="button"`, `aria-hidden` em ícone, `<figure>` em
   torno de imagem com legenda, `scope` em `th` — o autor não escreve, não corrige, não precisa saber.

## 7. Vantagem específica de `Msg` ser enum

- nome acessível traduzido e **garantido** nos 4 locales (paridade é gate);
- link ambíguo detectado por variante, sem parsear string;
- `aria-label` nunca fica vazio por tradução faltando.

## 8. O que continua fora do crate

| Preocupação | Ferramenta |
|---|---|
| contraste real medido (composição de ancestrais) | `axe` |
| alvos de toque medidos | `sniffCSS` |
| uniformidade, animação, foco visível | `sniffCSS` |
| ordem de tabulação em diálogo/popover | e2e de teclado |
| comportamento pós-swap (htmx) | `axe` **depois** do swap (obrigatório, ver [`07`](07-js-e-assets.md)) |

O crate reduz o custo de chegar até esses gates; não os substitui. Quando o teste roda, as violações
estruturais já não existem.
