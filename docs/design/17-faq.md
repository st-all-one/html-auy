# 17 — Perguntas difíceis

**Por que publicar como crate?**
Porque o projeto passou de artefato interno a biblioteca pública em 2026-11-07. O crate `html-auy`
segue semver, tem MSRV declarada e é instalável pelo crates.io; o custo é suporte a consumidores
desconhecidos. Ver D35 em [`13-decisoes.md`](13-decisoes.md).

**Por que não usar arquivos de template?**
Porque template em arquivo precisa de lógica (`config()`, `route()`, normalização de variantes) e essa
lógica, sem tipos, vira bug silencioso. O valor do arquivo (revisão, formatação, edição por não-Rust)
é preservado por **snapshot** e **catálogo de tradução** (ver [`10-ergonomia.md`](10-ergonomia.md) §8).

**Por que não usar Web Components / Lit?**
Porque o valor deles é ciclo de vida e composição **no cliente**: `customElements.define` por tag,
Shadow DOM por host (~0,5 ms), e para "funcionar sem JS" é preciso SSR que emite `<style>` dentro de
`<template shadowroot>` — `style` inline, contra a CSP sem inline. <slot>, `part`, `::slotted` e
`:host` só existem com shadow root, então a dependência vem em cadeia, não em parte. E o corpus não
tem uma linha sobre elemento indefinido piscando. O que CE resolveria aqui já está resolvido por
`Markup` + `enum Slot` + `/theme.css`. Ver D20 em [`13-decisoes.md`](13-decisoes.md).

**Por que não há warnings do compilador?**
Porque proc-macro em Rust stable só emite erro — `proc_macro::Diagnostic` é instável. Recomendação não
virou aviso: virou **teste que falha** (`N4` atrás de `feature = "audit"`, D30) ou erro de compilação. A
camada advisory com ratchet foi cortada (D28/A4): aviso que não bloqueia ninguém é documentação de pior
qualidade.

**Como alguém que não é de Rust edita o site?**
Texto: arquivo de tradução. Estrutura: snapshot HTML revisável. Layout complexo: primitivo + snapshot,
com revisão de quem é de Rust. Ver [`10-ergonomia.md`](10-ergonomia.md) §8.

**E se eu precisar de um atributo que não está na spec?**
PR em `spec/elements.toml` + entrada de decisão. Não há escape em tempo de execução — de propósito
(D4). O custo é o preço de o typo ser impossível.

**Nunca vou precisar de HTML pronto (ex.: AST de texto rico)?**
Precisa, e é suportado: renderização **nó a nó** no vocabulário (um `match` sobre o tipo de nó). O que
não existe é `Raw` para despejar string (D2).

**Isso substitui `axe`/`sniffCSS`/teste de teclado?**
Não. Substitui a parte que pode ser decidida antes da página existir. Contraste real, alvo medido e
ordem de foco continuam no navegador. Ver [`05-acessibilidade.md`](05-acessibilidade.md) §8.

**Funciona com axum/actix/rocket/warp?**
Sim: `Markup`/`Page` são valores. O `impl IntoResponse` (axum) ou equivalente tem **6 linhas** e é
escrito pelo consumidor; o crate não conhece framework nem `http` ([`01-arquitetura.md`](01-arquitetura.md) §3).

**E páginas grandes, streaming?**
`Render::render_to(&mut String)` permite montar sem alocação intermediária; o `ETag` exige o corpo
completo, então streaming de resposta fica no consumidor (com cache desligado).

**Como usar htmx?**
Opcional e fora do vocabulário: asset declarado + `Render::{Full, Partial}` + `Vary: HX-Request` +
`axe` depois do swap. Ver [`07-js-e-assets.md`](07-js-e-assets.md) §7.

**Por que todo texto precisa ser `Msg`?**
Porque nome acessível, `title`, `alt` e link precisam existir nos 4 locales, e porque o lint de 2.4.4
(link ambíguo) fica trivial por variante. Custo: catálogo gerado (2.702 entradas no estudo de caso).

**Por que `Tone`/`SectionId` e não cor?**
Cor livre quebra acessibilidade (contraste), CSP (inline) e cache (ETag por página). Identidade +
token validado resolve os três. Ver [`06-shell-e-tema.md`](06-shell-e-tema.md) §5.

**O tema pode mudar em produção sem deploy?**
Sim: tokens são dados; o `/theme.css` muda de `ETag` e a próxima request recebe o novo. Preview usa
endpoint privado `no-store`.

**E se o projeto morrer?**
O código é MIT OR Apache-2.0 e o crate `html-auy` 0.1.0 continua disponível no crates.io; o consumidor pode
vendorizar a versão que usa, mantém os snapshots como contrato e estes documentos como decisão
registrada. Não há dependência de serviço ou runtime externo.

**Quão frágil é depender de proc-macro?**
É o ponto frágil real. Mitigação: MSRV declarada, CI na MSRV e na stable, apenas `proc-macro2`/`quote`,
parser sem detalhes internos do compilador, fuzz para não gerar `panic`. Se quebrar, quebra em
`cargo check`, não em produção.

**Isso não é reinventar o maud?**
É o maud **mais** três coisas que ele não tem: atributo validado, escape por contexto e
acessibilidade por construção. Se você não precisa das três, use maud — está dito em
[`14-alternativas.md`](14-alternativas.md) §3.

**Licença e nome.**
MIT OR Apache-2.0. `html-auy` é o nome proposto originalmente, sem significado técnico interno.
