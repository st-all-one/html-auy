# lit — v3

`_RUST/lit/` é um guia de 18 arquivos (5.796 linhas) sobre **Lit 3**, a biblioteca de Web Components que
produz custom elements reativos com `lit-html`, Shadow DOM e SSR via `@lit-labs/ssr`. Ele cobre ciclo de
update, propriedades/atributos, templates, CSS encapsulado e compartilhado, slotted content, eventos,
composição (controllers/mixins), Context/Signals/Task, performance, interoperabilidade, SSR, tooling,
custom directives e localização. Serve aqui como a **única fonte do corpus sobre custom elements**: é
ela que decide se `<slot>`, Shadow DOM, `attributeChangedCallback` e `::part` podem existir no
vocabulário fechado de `html-auy`, e a que preço. Versão declarada em `VERSION` — `3`.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-index.md` | 38 | índice, princípios centrais ("componentes são elementos HTML nativos", "Shadow DOM por padrão") | mapa de leitura; nada normativo |
| `01-arquitetura-fundamentos.md` | 173 | hierarquia `HTMLElement→ReactiveElement→LitElement`, ciclo de update, `@property`/`@state`, light DOM | define *o que* um custom element é: classe JS + ciclo assíncrono → A1, A2, A19 |
| `02-componentes-propriedades.md` | 227 | `@customElement`, decorators, conversão tipo↔atributo, `reflect`, `hasChanged`, setters | conversão `JSON.parse` em atributo e `reflect` são o canal de estado → A3, A4, A5, A6 |
| `03-templates-renderizacao.md` | 499 | template literals, expressões, listas, ~14 directives, formas inválidas, well-formed HTML | directives e `unsafeHTML` são API de cliente; nomes dinâmicos proibidos confirmam D4 → A7–A11 |
| `04-estilos-css.md` | 497 | `css` tag, `:host`, compartilhamento, tokens, `part`/`exportparts`, light DOM, `@layer` | tema atravessa shadow **só** por custom property; `part` é API pública → A12–A17 |
| `05-shadow-dom.md` | 319 | modo open/closed, slots, `::slotted`, `slotchange`, retargeting, `delegatesFocus`, scoped registry | o núcleo da decisão: slot/composição só existem com shadow root → A18–A23 |
| `06-eventos.md` | 211 | listeners declarativos, `composed`, cleanup, teclado, delegação | contrato duplex (props ↓, eventos ↑) inexistente na crate → A25 |
| `07-composicao.md` | 436 | component composition, reactive controllers, mixins, `VisibilityController` | controllers são estado por instância no cliente; sem análogo server-side |
| `08-gerenciamento-dados.md` | 472 | `@lit/context`, Signals, `Task` (fetch com abort/race) | estado ambiente + fetch no cliente conflitam com render puro → §6 |
| `09-performance.md` | 412 | `shouldUpdate`/`willUpdate`, `cache`/`guard`/`live`, lazy import, Lit Compiler, checklist | números escassos (só "~0.5ms"); otimização é toda client-side → A16, A36 |
| `10-interoperabilidade.md` | 366 | React 19 nativo, wrapper React<19, Angular `[attr.value]`, Vue, design system, regras de API | interop é problema do consumidor JS; a API recomendada é props+eventos → A39 |
| `11-ssr.md` | 266 | `@lit-labs/ssr`, declarative shadow DOM, hydration, limitações | a única rota para "funciona sem JS" emite `<style>` e shadow tree → A26–A29 |
| `12-ferramentas.md` | 451 | Vite/Rollup, ESLint lit-plugin, Web Test Runner, `to.be.accessible()`, publicação | testes dependem de `shadowRoot` + `updateComplete` → A23, A24 |
| `13-custom-directives.md` | 401 | directives function/class/async, `noChange`, PartType, `setValue`, overhead | reintroduz exatamente a API de manipulação de DOM que D3/D4 excluem → A33, A34 |
| `14-localizacao.md` | 373 | `@lit/localize`, `msg`/`str`, runtime vs transform, XLIFF, `lit-localize-status` | i18n no cliente × `Msg` resolvido por request → A30–A32 |
| `general-cross-lang-usage.md` | 572 | consumo de Lit de PHP/Go/Rust via CDN + importmap; "backend como API" | é a alternativa arquitetural completa, com custos declarados → A35, A36, A37 |
| `SKILL.md` | 82 | resumo operacional: 9 "Core Rules", padrão `x-counter`, file map | confirma as regras que a crate rejeita (`reflect`, `unsafeHTML` no SSR) |
| `VERSION` | 1 | `3` | versão do conjunto |

## 2. Achados

### A1. Reatividade mora na classe, não no HTML
- Citação: `01-arquitetura-fundamentos.md:16` — "Implementa **propriedades reativas** com getters/setters"
- Semântica: o comportamento vem de `ReactiveElement`, que instala getter/setter por propriedade e agenda `performUpdate()` em microtask.
- Consequência: o HTML que a crate emite seria **estado inicial**, não o artefato; o mesmo byte passaria a ter dois contratos (bytes e upgrade).
- Veredito: confirma P1/P5 — o valor da crate é justamente não ter segunda fase.

### A2. Só o atributo declarado em `observedAttributes` chega ao componente
- Citação: `01-arquitetura-fundamentos.md:113` — "attribute: 'my-prop',   // nome do atributo (false desativa)"
- Semântica: `attributeChangedCallback` existe por atributo declarado; sem declaração, o atributo é decorativo.
- Consequência: um CE no vocabulário exigiria uma tabela `elemento → atributos observados → conversor` — exatamente o que `spec/elements.toml` já é, mas para uma finalidade nova (estado em runtime).
- Veredito: refina D5 — a spec continuaria fonte única, mas passaria a descrever comportamento, não só marcação.

### A3. Objeto em atributo é `JSON.parse`, ou seja, atributo com blob
- Citação: `02-componentes-propriedades.md:104` — "| `Object` | `JSON.parse(valor)` | `JSON.stringify(valor)` |"
- Semântica: passar dado estruturado a um CE só é possível por string JSON no atributo (ou por `.property` via JS).
- Consequência: contraria a regra da crate de que `data-*` é token enumerado; `data='{…}'` reintroduz "string crua com semântica" na porta da frente.
- Veredito: refuta o padrão de `general-cross-lang-usage.md:523` ("data='{ \"items\": [...] }'").

### A4. Booleano é presença, e default `false`
- Citação: `02-componentes-propriedades.md:109` — "Propriedades booleanas **devem** default para `false`:"
- Semântica: `true` emite atributo vazio, `false` remove; nunca `disabled="false"`.
- Consequência: alinha 1:1 com a adição já aceita em `README.md` (atributo booleano é presença, nunca valor).
- Veredito: confirma D1/D14 (emissão determinística de booleano).

### A5. A reatividade depende de uma flag de compilador TypeScript
- Citação: `02-componentes-propriedades.md:60` — "`useDefineForClassFields: false` é crítico. Com `true` (default ES2022+), class fields sobrescrevem os getters/setters reativos, quebrando a reatividade."
- Semântica: um erro de configuração de `tsconfig` quebra silenciosamente todo componente; a doc dedica uma seção inteira ao caso.
- Consequência: adotar CE significa adicionar uma toolchain (TS + bundler) cuja falha é *silenciosa* — o oposto do "erro de marcação é erro de compilação".
- Veredito: refuta — P7 (erro acionável) não tolera falha muda.

### A6. `reflect` é o único caminho atributo↔CSS; estado visual vai por `CustomStateSet`
- Citação: `02-componentes-propriedades.md:164` — "Preferir `:state()` pseudo-class (CustomStateSet) para estados visuais"
- Semântica: `reflect: true` existe para permitir `:host([attr])` e `querySelector`; para estado interno, a recomendação é `:state()`, invisível ao seletor externo.
- Consequência: qualquer tema/estado de CE endereçável por CSS exige atributo refletido e, portanto, uma regra de reflexão por atributo na spec.
- Veredito: refina D12 — tokens continuariam sendo o canal, mas o CE abriria um segundo canal (`:state()`), fora do `/theme.css`.

### A7. O ganho do Lit é client-side: parse único, update por expressão
- Citação: `09-performance.md:6` — "**Template parsing único** — HTML é parseado uma vez, re-renders só atualizam valores"
- Semântica: o modelo é "parse uma vez, atualizar partes"; não existe "renderizar uma vez e pronto".
- Consequência: nenhum ganho para o HTML servido; o benefício só aparece em páginas que vivem no cliente e re-renderizam.
- Veredito: irrelevante para a crate (mas explica por que Lit não resolve o problema dela).

### A8. Nome de atributo dinâmico é inválido até no Lit
- Citação: `03-templates-renderizacao.md:444` — "// ❌ Inválido — nome de atributo dinâmico"
- Semântica: `html`<div ${this.attrName}=${value}></div>`` não compila em runtime; a única saída é `literal` (template novo).
- Consequência: nem a biblioteca mais permissiva aceita atributo por variável — evidência externa de que D4 não é excesso de zelo.
- Veredito: confirma D4.

### A9. `unsafeHTML`/`unsafeSVG` são o `Raw` do Lit
- Citação: `03-templates-renderizacao.md:221` — "return html`<div>${unsafeHTML(this.trustedHtml)}</div>`"
- Semântica: a porta de escape é explícita e documentada com aviso de XSS ("⚠️ APENAS para conteúdo confiável").
- Consequência: a existência da porta prova que o modelo de tipos do Lit não conhece contexto; a crate fecha essa porta por construção.
- Veredito: confirma D2.

### A10. HTML mal formado é erro, não autocorreção
- Citação: `03-templates-renderizacao.md:459` — "Lit requer HTML **bem formado** no template. Tags não fechadas ou mal aninhadas causam erro:"
- Semântica: o parser do template não aceita aninhamento inválido.
- Consequência: confirma que exigir conteúdo-modelo e estrutura correta (P8 + N2) é prática consolidada, não invenção.
- Veredito: confirma P8.

### A11. Tag dinâmica custa template novo
- Citação: `03-templates-renderizacao.md:395` — "⚠️ **Performance**: Mudar valores `literal` cria um novo template do zero."
- Semântica: trocar `<button>` por `<a>` via `literal` recria o template inteiro; é aceitável só para configuração rara.
- Consequência: se algum dia houver "componente dinâmico" na crate, o custo de variar o nome do elemento não é zero em nenhum runtime.
- Veredito: irrelevante hoje (D4 impede nome dinâmico) · confirma o desenho.

### A12. Estilo de componente é objeto parseado uma vez, não markup
- Citação: `04-estilos-css.md:45` — "Retorna `CSSResult` — não é string"
- Semântica: `css`...`` produz objeto compartilhado entre instâncias; `unsafeCSS` é a única porta para string dinâmica ("⚠️ risco XSS").
- Consequência: confirma que CSS de componente é asset/derivado (postura de `/theme.css`), não `<style>` no documento.
- Veredito: confirma D3/D12.

### A13. `:host-context()` é a única "herança de ancestral" que o shadow oferece
- Citação: `04-estilos-css.md:57` — ":host-context(.dark-theme) { /* ancestral com classe dark-theme */ }"
- Semântica: para reagir a um ancestral, o componente precisa desse seletor; sem ele, só custom property herdada atravessa.
- Consequência: se CE entrasse, `data-theme` do `<html>` (docs/06) deixaria de ser suficiente: cada componente precisaria de `:host-context` ou de token herdado — uma regra a mais por primitivo.
- Veredito: refina D12 — o tema por token já resolve; `:host-context` seria dívida nova.

### A14. Tema vive fora do shadow, em custom properties globais
- Citação: `04-estilos-css.md:123` — "// tokens.css — global, não no Shadow DOM"
- Semântica: a recomendação explícita é manter tokens no documento e consumir via `var(--…)` dentro do componente.
- Consequência: é *o mesmo* desenho de `docs/06-shell-e-tema.md` (tokens → `/theme.css` → `var()`), o que explica por que a crate não precisa de CE para tematizar.
- Veredito: confirma D12.

### A15. `part`/`exportparts` são API pública de estilo
- Citação: `04-estilos-css.md:344` — "<div class="header" part="header"><slot name="header"></slot></div>"
- Semântica: estilização externa só acontece por partes exportadas; `exportparts` re-expõe partes de filhos aninhados (`04-estilos-css.md:365`).
- Consequência: cada `part` publicada é contrato que precisa ser versionado, documentado e testado — e a crate hoje tem **zero** partes.
- Veredito: refuta a necessidade (o token cobre o caso) · mas registra o custo se CE entrar.

### A16. Shadow root custa ~0,5 ms por componente
- Citação: `04-estilos-css.md:427` — "**Shadow DOM adiciona ~0.5ms por componente** — irrelevante para maioria"
- Semântica: o número é por componente e considerado irrelevante *porque* uma página tem poucos componentes.
- Consequência: o portal tem 222 views e 6.765 tags; "componente por primitivo" multiplicaria esse custo por centenas de hosts — o próprio número do corpus deixa de ser irrelevante nessa escala.
- Veredito: refuta adoção ampla · [INFERENCE] a extrapolação é minha, o dado é deles.

### A17. Sem shadow, `id` é global e estilo vaza
- Citação: `04-estilos-css.md:283` — "**Colisão de IDs** — sem Shadow DOM, IDs são globais"
- Semântica: light DOM elimina encapsulamento; IDs colidem e `<style>` vaza.
- Consequência: é exatamente por isso que a crate precisa de `id` derivado do call site e de `Class` com token fechado, em vez de confiar em encapsulamento do browser.
- Veredito: confirma a adição 4 de `README.md` (`id` do call site) e D1/`Class`.

### A18. Slot é superfície frágil: fallback, `slotchange` e `::slotted` raso
- Citação: `05-shadow-dom.md:71` — "**`::slotted(selector)`**: Estiliza conteúdo projetado (⚠️ apenas seletores simples)"
- Semântica: `::slotted(.container span)` e `::slotted([slot="header"])` **não funcionam** (`05-shadow-dom.md:82-85`); projeção muda exige `slotchange` + `assignedElements({flatten:true})`.
- Consequência: o "conteúdo slotted" que a crate teria de emitir (`<h2 slot="header">`) só seria estilizável por seletor direto — o layout do primitivo não poderia estilizar o que o chamador projeta.
- Veredito: refuta `<slot>` no vocabulário.

### A19. Sem shadow root não existe slot
- Citação: `01-arquitetura-fundamentos.md:142` — "**Slots não funcionam** — sem Shadow DOM, não há `<slot>`"
- Semântica: a afirmação é repetida em `04-estilos-css.md:285` ("sem Shadow DOM, `<slot>` é ignorado"); projeção é inseparável de shadow root.
- Consequência: aceitar `<slot>` obriga todo primitivo a ter shadow root — e shadow root, hoje, exige JS de registro (a crate não pode definir `customElements`).
- Veredito: refuta — a dependência é em cadeia, não opcional.

### A20. Evento de conteúdo projetado perde o alvo
- Citação: `05-shadow-dom.md:196` — "e.composedPath(); // [button, #shadow-root, my-element, body, html, document]"
- Semântica: `e.target` fora do componente é retargetado para o host (`05-shadow-dom.md:195`); só `composed: true` atravessa.
- Consequência: medição de interação, analytics de clique e testes de teclado passariam a depender de `composedPath`, que a crate não emite nem testa.
- Veredito: irrelevante (a crate não emite eventos) · registra custo de gate.

### A21. `delegatesFocus` é o único ganho de a11y "de graça"
- Citação: `05-shadow-dom.md:222` — "delegatesFocus: true  // foco no primeiro focoável do Shadow DOM"
- Semântica: sem ele, clicar no host não move foco para dentro; com ele, o foco "pula" a fronteira.
- Consequência: seria preciso declarar `delegatesFocus` por primitivo interativo (botão, campo, link) — uma linha de spec por elemento, e um comportamento a testar no e2e de teclado.
- Veredito: refina D10 — o custo de a11y aumentaria, não diminuiria.

### A22. Nome de custom element é global; registry escopado é experimental
- Citação: `05-shadow-dom.md:295` — "## Scoped Custom Element Registry (Experimental)"
- Semântica: `@lit-labs/scoped-registry-mixin` resolve colisão de nomes, mas é Labs.
- Consequência: um `<search>`/`<dialog>`/`<time>` do vocabulário conviveria com tags `x-*` globais no mesmo documento; colisão vira bug de página, não de compilação.
- Veredito: refuta — P7 exige erro em compilação; colisão de tag não é detectável por macro.

### A23. Testes passam a exigir `renderRoot` e `updateComplete`
- Citação: `12-ferramentas.md:219` — "const paragraph = element.shadowRoot!.querySelector('p');"
- Semântica: asserção só é válida depois de `await element.updateComplete` (`12-ferramentas.md:225`).
- Consequência: todo golden/e2e da crate passaria a precisar de espera assíncrona e de traversal de shadow — o snapshot `.snap` deixaria de ser a verdade inteira.
- Veredito: refuta — quebra o contrato "bytes são o artefato" (D14).

### A24. O único sinal de a11y do corpus é `to.be.accessible()`
- Citação: `12-ferramentas.md:249` — "await expect(el).to.be.accessible();"
- Semântica: a11y é verificada por asserção de teste no componente, não por tipo.
- Consequência: confirma que "acessibilidade por construção" (P4/D10) é diferencial da crate, não padrão do ecossistema.
- Veredito: confirma P4 · [INFERENCE] o corpus não afirma que essa asserção penetra shadow root.

### A25. O corpus ensina um padrão que a crate proíbe
- Citação: `06-eventos.md:142` — "role="menuitem""
- Semântica: o exemplo emite `role="menuitem"` + `tabindex` + `aria-disabled` em `<div>`, com `@keydown` manual para `Enter`/`Espaço`.
- Consequência: é precisamente o "widget ARIA reconstruído em `div`" que `docs/05-acessibilidade.md` §4 trata como advisory; a crate empurraria para `<button>`/`<a>` reais.
- Veredito: refuta o padrão; confirma P3/P4.

### A26. O SSR do Lit materializa o shadow tree em `<template shadowroot>`
- Citação: `11-ssr.md:77` — "<template shadowroot="open">"
- Semântica: a saída inclui `<style>` dentro do template (`11-ssr.md:78-79`: `<style>` / `:host { display: inline-block; }`).
- Consequência: ou a crate emite `<style>` inline (proibido por D3 e reprovado no N4) ou o CE chega sem estilo e pisca; além disso o HTML dobra de tamanho.
- Veredito: refuta — é o conflito mais duro entre CE e CSP sem inline · [INFERENCE] `style-src 'self'` bloquearia esse `<style>`; o corpus não discute CSP.

### A27. O caminho SSR é Labs e tem limitações declaradas
- Citação: `11-ssr.md:3` — "**Status: Lit Labs** — `@lit-labs/ssr` (em desenvolvimento ativo)"
- Semântica: `@query` "pode não funcionar" no SSR (`11-ssr.md:253`) e `updateComplete` "pode não resolver" (`11-ssr.md:255`).
- Consequência: a única rota que daría "funciona sem JS" para CE é a menos estável do ecossistema — e a crate tem prazo de 2 semanas por marco.
- Veredito: refuta — risco desproporcional a M1/P10.

### A28. O corpus converge para o nosso modelo: render é função pura de props
- Citação: `11-ssr.md:94` — "**Renderização pura** — `render()` deve ser função apenas de propriedades"
- Semântica: para SSR, proíbe `window`/`document`/`localStorage` em `render()` e exige `static styles`.
- Consequência: é a mesma disciplina de P5/P6 (sem I/O, sem estado ambiente, CSS derivado), com outro nome.
- Veredito: confirma P5 e P6.

### A29. Hydration não recria DOM: sem JS o HTML do SSR fica como veio
- Citação: `11-ssr.md:173` — "**DOM existente não é recriado** — Lit attaches ao shadow root existente"
- Semântica: `firstUpdated` roda após hydration (`11-ssr.md:174`) e listeners são apenas adicionados.
- Consequência: a paridade sem-JS de um CE depende inteiramente do que o servidor emitiu (A26). Sem DSD, o host fica vazio até o módulo carregar.
- Veredito: confirma o diagnóstico de FOUC estrutural (ver A40 e §7).

### A30. i18n em runtime tem custo declarado de SEO
- Citação: `14-localizacao.md:370` — "| SEO por locale | ⚠️ Requer setup | ✅ Bundles separados |"
- Semântica: modo runtime troca locale sem reload com ~5 KB de overhead (`14-localizacao.md:369`); transform gera um bundle por locale.
- Consequência: `Msg` resolvido na request (docs/02 §6) entrega o resultado do "transform mode" sem bundle por locale e sem negociar SEO.
- Veredito: confirma a decisão de i18n da crate (e a `LUrl` com locale).

### A31. Plural e formatação ficam no cliente
- Citação: `14-localizacao.md:108` — "msg(str`${count} item${count === 1 ? '' : 's'}`);"
- Semântica: pluralização é "manual", expressa em JS dentro da mensagem.
- Consequência: a crate já resolve isso em Rust (`Count`, `Money`, `Timestamp` formatados no servidor), com paridade garantida por gate.
- Veredito: confirma docs/02 §7 · refuta mover i18n para o cliente.

### A32. Troca de locale é um evento global na `window`
- Citação: `14-localizacao.md:188` — "window.addEventListener('lit-localize-status', this._onStatus as EventListener);"
- Semântica: o locale é estado global mutável, com status `ready|loading|error` e listeners para re-render.
- Consequência: oposto do modelo da crate (locale vem da URL, está no `ShellCtx`, e a página é cacheável por locale).
- Veredito: refuta — estado global é justamente o que P1/P5 excluem.

### A33. Directives reabrem a manipulação de DOM com estado
- Citação: `13-custom-directives.md:121` — "| `noChange` | Não altera o DOM existente — o valor anterior permanece |"
- Semântica: `update(part, …)` decide se o DOM muda; `setValue()` (`13-custom-directives.md:256`) escreve fora do ciclo; `disconnected()` existe para não vazar timer (`13-custom-directives.md:386`).
- Consequência: é uma API de mutação imperativa com ciclo de vida próprio — o oposto do "HTML é o artefato"; nada disso tem análogo server-side.
- Veredito: refuta qualquer importação do conceito.

### A34. `PartType` é a prova externa de que "contexto é tipo"
- Citação: `13-custom-directives.md:163` — "case PartType.CHILD:"
- Semântica: o Lit separa `CHILD`/`ATTRIBUTE`/`BOOLEAN_ATTRIBUTE`/`PROPERTY`/`EVENT`/`ELEMENT` como tipos de part distintos.
- Consequência: a taxonomia de contextos da crate (`Text`, `AttrVal`, `Href`, `Src`, `JsonLd`…) é a mesma ideia, mais fina e sem runtime.
- Veredito: confirma D1/P2.

### A35. A promessa do corpus é a arquitetura inteira trocada
- Citação: `general-cross-lang-usage.md:31` — "**Alternativa real a templates server-side** (Blade, Twig, Tera, html/template) — a UI vive no cliente, o backend só serve dados"
- Semântica: Lit não é melhoria incremental de template; é mover a UI para o cliente e reduzir o backend a API JSON.
- Consequência: adotar CE não é "aceitar uma tag a mais"; é sair do modelo "HTML é o artefato" — e o consumidor tem 222 views em Blade para migrar com prazos.
- Veredito: refuta — a mudança é de categoria, não de escopo.

### A36. O próprio corpus admite os custos contra o caso do consumidor
- Citação: `general-cross-lang-usage.md:561` — "| Performance inicial (FCP) | ✅ SSR direto | ⚠️ Depende de JS |"
- Semântica: na mesma tabela, "Página estática ou com pouco JS | ✅ | ⚠️ Overhead" (`general-cross-lang-usage.md:554`).
- Consequência: portal municipal = páginas estáticas, pouco JS, SEO, CSP `'self'`, ETag — exatamente a coluna onde Lit aparece com ⚠️.
- Veredito: refuta — evidência do corpus contra a própria adoção neste consumidor.

### A37. Swap de parcial + CE = upgrade tardio, admitido de passagem
- Citação: `general-cross-lang-usage.md:304` — "// Os componentes Lit hidratam automaticamente"
- Semântica: o comentário assume que HTML trocado por HTMX com tags `<my-*>` "hidrata" sozinho — o que só ocorre quando o módulo já está carregado e o elemento é atualizado.
- Consequência: é a descrição literal do FOUC em partial swap: tag desconhecida, conteúdo do slot visível, shadow tree ausente até o upgrade.
- Veredito: refuta CE em região swappable · registra a lacuna de A40.

### A38. O padrão "dado em atributo" volta pela porta dos fundos
- Citação: `general-cross-lang-usage.md:523` — "<my-data-table" seguido de `data='{ "items": [...] }'`
- Semântica: o corpus recomenda serializar dados no atributo porque o backend "não sabe" que existe propriedade JS.
- Consequência: conflita com `docs/03-vocabulario-de-elementos.md` §6 (todo `data-*` enumerado) e com a proibição de string com semântica.
- Veredito: refuta o padrão · confirma a regra de `data-*` tipado.

### A39. O contrato de um CE é duplex; o da crate é unidirecional
- Citação: `10-interoperabilidade.md:357` — "**Propriedades reativas** para input (não métodos)"
- Semântica: entrada por props/atributos, saída por eventos `composed` (`10-interoperabilidade.md:358`); em Angular, atributo é sempre string e propriedade exige `[property]` (`10-interoperabilidade.md:166`).
- Consequência: para a crate, "enviar dado a um componente" é passar `Markup`/`Msg` em Rust; criar o equivalente em CE significaria manter dois sistemas de props em paralelo.
- Veredito: refuta — duplicação de contrato sem ganho.

### A40. O corpus não tem uma única linha sobre `:defined`, `whenDefined`, `upgrade` ou FOUC
- Citação: `_RUST/lit/` inteiro — zero ocorrências desses termos em 5.796 linhas (`00-index.md` a `general-cross-lang-usage.md`; `[sem linha]`).
- Semântica: o único tratamento do problema é o DSD do SSR (`11-ssr.md:77`) e a frase "hidratam automaticamente" (`general-cross-lang-usage.md:304`).
- Consequência: a documentação não oferece defesa para o flash de elemento indefinido — que é o defeito visível de toda adoção; quem adota fica sem receita.
- Veredito: irrelevante para a crate hoje · é lacuna registrada em §7.

## 3. Confirmações do desenho

1. **P1 (HTML é o artefato)** — o Lit existe para o oposto: re-render por expressão (`09-performance.md:6`, `01-arquitetura-fundamentos.md:16`). Nada no corpus permite HTML determinístico com comportamento.
2. **P2/D1 (contexto é tipo)** — `PartType` (`13-custom-directives.md:163`) separa contexto por tipo de binding; a crate faz o mesmo por tipo de valor.
3. **P3/P4 (a11y por construção)** — o ecossistema testa a11y com `to.be.accessible()` (`12-ferramentas.md:249`) e constrói widgets ARIA em `div` (`06-eventos.md:142`); emitir semântica por primitivo continua sendo diferencial.
4. **P5/P6 (sem runtime, determinismo)** — `render()` puro de props é regra explícita para SSR (`11-ssr.md:94`); `reflect`/`JSON.parse` em atributo (`02-componentes-propriedades.md:104`) mostram o caminho que a crate evita.
5. **P7 (erro de compilação)** — nome de atributo dinâmico é inválido até no Lit (`03-templates-renderizacao.md:444`); a crate só torna isso estático.
6. **P8 (um parser)** — "Lit requer HTML bem formado" (`03-templates-renderizacao.md:459`).
7. **D2 (sem `Raw`)** — `unsafeHTML`/`unsafeSVG` são o `Raw` do ecossistema, com aviso de XSS (`03-templates-renderizacao.md:221`).
8. **D3 (sem `style`/`on*`)** — CSS de componente é objeto (`04-estilos-css.md:45`), nunca markup inline; a única string dinâmica (`unsafeCSS`) é marcada como risco.
9. **D4 (sem atributo livre)** — atributo novo = linha na spec; o corpus confirma que nem nome de atributo dinâmico pode existir.
10. **D12 (tema é dado)** — tokens globais consumidos por `var()` (`04-estilos-css.md:123`) são exatamente `/theme.css` + `--tenant-*`.
11. **`data-*` tipado** — `data='{json}'` (`general-cross-lang-usage.md:523`) é o anti-padrão que a regra de enum evita.
12. **i18n no servidor** — runtime mode custa SEO e ~5 KB (`14-localizacao.md:369-370`); `Msg` por request é superior em SEO, cache e plural (`14-localizacao.md:108`).
13. **`id` derivado + `Class` fechada** — sem shadow, IDs colidem e estilo vaza (`04-estilos-css.md:283-285`); encapsulamento não está disponível de graça.
14. **Funciona sem JS** — o corpus só alcança isso com SSR Labs + DSD (`11-ssr.md:3`, `11-ssr.md:77`), que emite `<style>` inline.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata + arquivo) |
|---|---|---|
| `docs/00-principios.md` §3 lista não-objetivos, sem mencionar custom elements | CE é "a UI vive no cliente, o backend só serve dados" (`general-cross-lang-usage.md:31`); sem JS o host fica sem shadow tree (A29, A40) | Inserir em `docs/00-principios.md` §3, tabela de não-objetivos: `\| Elementos personalizados (`<my-*>`, `customElements.define`, Shadow DOM, `<slot>`) \| o comportamento passaria a viver no cliente; o HTML deixaria de ser o artefato (P1) e o FOUC de elemento indefinido não é representável no vocabulário \|` |
| A spec tem 65 elementos e nenhum `slot`/`template` — a ausência é acidental, não decidida | slot só existe com shadow root (`01-arquitetura-fundamentos.md:142`) e `::slotted` é raso (`05-shadow-dom.md:82-85`) | Inserir em `docs/03-vocabulario-de-elementos.md` §8 (governança), após a lista de passos: "`slot`, `template` e qualquer elemento com hífen no nome (`<x-…>`, `<my-…>`) são rejeitados por princípio: projeção exige Shadow DOM e comportamento exige `customElements.define`, ambos fora de P1/P5." |
| `docs/03-vocabulario-de-elementos.md` §5 proíbe `style`, `on*`, `srcdoc`, `@*`, `hx-on*` | `part`/`exportparts` são o canal de estilo externo de CE (`04-estilos-css.md:344`, `:365`); `slot` é o canal de projeção | Acrescentar à tabela de §5: `\| slot, part, exportparts, is \| atributos de Shadow DOM/custom elements: só têm sentido com árvore encapsulada e `customElements.define` (ver `18-aprendizados/lit.md` §2) \|` |
| `docs/07-js-e-assets.md` §10 tem 6 regras e nenhuma sobre o que o asset **não** pode fazer | Lit substitui conteúdo por render próprio (`01-arquitetura-fundamentos.md:16`) e "hidrata" tags trocadas por HTMX (`general-cross-lang-usage.md:304`) | Acrescentar regra 7 em `docs/07-js-e-assets.md` §10: "7. Nenhum asset define custom element, anexa Shadow Root ou substitui conteúdo do documento: o que saiu no HTML é o que o usuário vê (P1); o JS só lê `data-*` e melhora o que já existe." |
| `docs/04-validacao.md` §6 lista regras N4 e nenhuma barra `<template shadowroot>`/`<slot>`/tag com hífen | DSD do SSR emite `<template shadowroot="open">` + `<style>` (`11-ssr.md:77-79`) | Acrescentar em `docs/04-validacao.md` §6: "ausência de `<template shadowroot>`/`<slot>`/elemento com hífen (indício de custom element ou de saída de SSR de terceiro)" |
| `docs/12-testes.md` §9 reusa gates do consumidor sem gate de "nada de CE" | o corpus recomenda fixture com `shadowRoot.querySelector` + `updateComplete` (`12-ferramentas.md:219-225`) — teste que a crate não quer precisar | Acrescentar em `docs/12-testes.md` §9: "`grep` de `customElements.define`, `attachShadow` e `<template shadowroot` sobre a saída renderizada e sobre os assets; ocorrência = falha do gate, não advisory." |
| `docs/13-decisoes.md` vai de D0 a D15; a exclusão de Web Components não está registrada | todo o conjunto `lit` é a evidência de que CE = runtime + toolchain + SSR Labs (`11-ssr.md:3`) | Acrescentar **D16 — Sem custom elements** (texto completo em §5 abaixo) em `docs/13-decisoes.md`. |
| `docs/17-faq.md` não responde "por que não Web Components/Lit?" | o corpus oferece a alternativa pronta e seus custos (`general-cross-lang-usage.md:554`, `:561`) | Acrescentar a pergunta/resposta de §5 em `docs/17-faq.md`, depois de "Como usar htmx?". |

## 5. Adições prontas para colar

**5.1 — Linha para `docs/00-principios.md` §3 (não-objetivos):**
`| Elementos personalizados (`<my-*>`, `customElements.define`, Shadow DOM, `<slot>`) | comportamento passaria a viver no cliente; o HTML deixaria de ser o artefato (P1), o FOUC de elemento indefinido não é representável no vocabulário, e a paridade sem-JS só existe via SSR Labs com `<style>` inline |`

**5.2 — D16 para `docs/13-decisoes.md`:**

```
## D16 — Sem custom elements

**Contexto.** Web Components (Lit 3) resolvem reatividade, composição por slot e tema por `::part`;
o custo é uma classe JS registrada por tag (`customElements.define`), Shadow DOM por host
(~0,5 ms), toolchain TypeScript cuja falha de configuração é silenciosa
(`useDefineForClassFields`) e, para funcionar sem JS, SSR em Labs que emite `<style>` inline.

**Decisão.** Nenhum elemento com hífen, `<slot>`, `<template shadowroot>`, `slot`/`part`/`exportparts`
ou asset que chame `customElements.define`/`attachShadow` existe no vocabulário. Composição é
primitivo com `Markup`; tema é token servido em `/theme.css`; i18n é `Msg` resolvido na request.

**Consequência.** O HTML servido é a página final em qualquer estágio de carregamento (sem FOUC
estrutural); `ETag`/CSP/gates valem sem exceção. Perde-se: widgets ricos de cliente (tabela
virtualizada, editor colaborativo) — que passam a ser script de domínio declarado por `ctx.require`,
não componente de markup.
```

**5.3 — Pergunta para `docs/17-faq.md`:**

```
**Por que não Web Components / Lit?**
Porque o valor deles está no cliente: slots, eventos `composed`, directives e `updateComplete` são API
de runtime (`_RUST/lit`). Para funcionar sem JS, o caminho é SSR em Labs com Declarative Shadow DOM —
que emite `<style>` inline e o shadow tree serializado, contra D3 e contra o lint de saída. Composição,
tema e i18n já são resolvidos aqui por primitivo + `Markup`, token e `Msg` (D16).
```

**5.4 — Agenda negativa para `spec/elements.toml` (requer `schema = 2`; sem isso, é apenas lint):**

```toml
[forbidden]
# nomes de elemento que nunca existirão, mesmo por proposta
elements = ["slot", "template"]        # projeção e conteúdo inerte pertencem ao Shadow DOM
name_pattern = "^[a-z][a-z0-9-]*$"     # hífen = custom element = rejeitado
# atributos recusados em qualquer elemento
attributes = ["slot", "part", "exportparts", "is"]
```

**5.5 — Linha para a tabela de `docs/07-js-e-assets.md` §3 (`data-*`):**
`| `data-*` | **somente tokens enumerados** — nunca JSON, nunca chave livre (`data='{"items":[…]}` é o anti-padrão de `general-cross-lang-usage.md:523`) | o asset lê o token, nunca desserializa estado |`

**5.6 — Tabela comparativa de i18n (para `docs/02-tipos-e-contextos.md` §6 ou `README.md`):**

| Critério | `@lit/localize` runtime | `@lit/localize` transform | `Msg` da crate |
|---|---|---|---|
| Locale por request | ❌ (global, mutável) | ⚠️ (bundle por locale) | ✅ (`ShellCtx.locale`) |
| SEO por locale | ⚠️ requer setup | ✅ | ✅ |
| Plural | ❌ lógica em JS | ❌ lógica em JS | ✅ `Count`/`Money` em Rust |
| Cache/`ETag` | ⚠️ re-render muda DOM | ✅ | ✅ bytes fixos |
| Paridade de tradução | ❌ runtime | ⚠️ build | ✅ gate por variante |

**5.7 — Item para `docs/05-acessibilidade.md` §8 (o que fica fora):**
`| a11y **dentro** de Shadow Root (se um dia houver CE) | nenhum gate atual cobre; exigiria traversal explícito — motivo adicional de D16 |`

## 6. Armadilhas de complexidade

1. **"Só um `<slot>` para o primitivo"** — arrasta shadow root, `customElements.define`, `slotchange`,
   `assignedElements` e `::slotted` raso: 4 mecanismos para um ganho que `Table::new(header, body)` já dá.
   Custo: um runtime por primitivo e um modo de falha (conteúdo não projetado) invisível em compilação.
2. **`unsafeHTML`/`unsafeSVG` como "só para rich text"** — é a porta que D2 fecha; e `setValue()` em
   `AsyncDirective` (`13-custom-directives.md:256`) reabre a mesma porta de forma assíncrona.
3. **Directives** (`noChange`, `PartType`, `disconnected`) — trazem ciclo de vida e estado por instância
   para manipular DOM; a crate teria de modelar "part", "part type" e "cleanup" sem nunca emitir JS.
4. **Context API / Signals / Task** — estado ambiente, pinpoint updates e `fetch` com abort são
   client-side por definição; `args`+`AbortSignal` não têm representação em `Markup`.
5. **`@lit-labs/ssr` + DSD** — o SSR do Lit emite `<style>` inline e shadow tree serializado
   (`11-ssr.md:77-79`); viola D3/N4 e dobra os bytes da página, exatamente onde o orçamento de 14 KB
   (docs/09) morde.
6. **`reflect: true` como atalho de estado** — refletir objeto/array significa `JSON.stringify` em
   atributo a cada update (`02-componentes-propriedades.md:104`, `:163`); serialização cara e atributo
   ilegível no snapshot.
7. **`:host-context` / `::part` como sistema de tema** — troca tokens validados por seletores
   publicados; cada `part` vira contrato versionado (`04-estilos-css.md:344`) e o `/theme.css` deixa de
   ser a única fonte.
8. **Toolchain TypeScript/bundler no CI** — `useDefineForClassFields: false` (`02:60`), `lit-plugin`,
   Web Test Runner, `custom-elements.json`: uma segunda cadeia de build cuja falha não aparece em
   `cargo check`.
9. **`literal`/tag dinâmica** — custa template novo (`03-templates-renderizacao.md:395`); qualquer
   "componente por nome" é caro em qualquer runtime.

## 7. Lacunas do corpus

1. **`:defined`/`whenDefined`/`upgrade`/FOUC: zero ocorrências** em 5.796 linhas. A única defesa
   oferecida é o DSD do SSR — e ela cobre first paint, não partial swap (A37, A40).
2. **axe dentro de Shadow Root** — nada é afirmado. `to.be.accessible()` (`12-ferramentas.md:249`) é
   citado sem dizer o que atravessa; não há registro de limitação nem de cobertura.
3. **Form-associated custom elements / `ElementInternals`** — o corpus só diz que `<form>` não vê
   inputs no shadow (`01-arquitetura-fundamentos.md:171`); não cobre `static formAssociated`.
4. **CSP × Declarative Shadow DOM** — nunca mencionado. A saída de `11-ssr.md:78` é `<style>` dentro de
   template, o que colide com `style-src 'self'` (docs/08 §6) — [INFERENCE] minha, e é decisiva.
5. **Tamanho absoluto do bundle Lit** — não há número; só `~5KB` do `@lit/localize` runtime
   (`14-localizacao.md:369`) e URLs de `lit-core.min.js`. Docs/09 pede orçamento de JS ≤ 150 KB.
6. **Baseline de browser** — nenhum dado de suporte a `adoptedStyleSheets`, DSD ou `::part`; o corpus
   assume browsers modernos sem declarar.
7. **Custo medido de slot/composição** — só existe "~0,5 ms por Shadow DOM" (`04-estilos-css.md:427`);
   não há número para `slotchange`, retargeting ou `assignedElements`.
8. **Hidratação com mismatch** — não há tratamento de divergência entre HTML do servidor e render do
   cliente; é o modo de falha que a crate evita não tendo segunda renderização.

## 8. Veredito: custom elements **não** entram no vocabulário

Cinco razões, todas com evidência no corpus:

1. **O CE não é markup, é ciclo de vida.** O comportamento mora em getters/setters e em
   `performUpdate()` (`01-arquitetura-fundamentos.md:16`, `:52-55`); o HTML vira estado inicial e o
   artefato deixa de ser final (P1).
2. **Sem JS o host é vazio; com JS o HTML pode violar a CSP.** A paridade sem-JS exige SSR Labs
   (`11-ssr.md:3`) que emite `<style>` inline dentro de `<template shadowroot="open">`
   (`11-ssr.md:77-79`) — proibido por D3 e reprovado pelo N4.
3. **Todo o valor de CE é API de cliente, não de documento.** Slots (A18), `part` (A15), eventos
   `composed` (A20), controllers (A33) e locale em `window` (A32) não têm análogo server-side; a crate
   é unidirecional por construção (A39).
4. **O custo aparece declarado no próprio corpus.** FCP "⚠️ Depende de JS" (`general-cross-lang-usage.md:561`),
   página estática com "⚠️ Overhead" (`:554`), ~0,5 ms por shadow root (`04-estilos-css.md:427`),
   SSR em Labs com `@query`/`updateComplete` quebrados (`11-ssr.md:253-255`), e uma toolchain cuja
   falha é silenciosa (`02-componentes-propriedades.md:60`).
5. **O que CE ofereceria, a crate já entrega mais barato.** Composição por `Markup` + slots tipados
   (`enum Slot`), tema por custom property global (`04-estilos-css.md:123` ≡ `/theme.css`), i18n por
   `Msg` em vez de runtime (`14-localizacao.md:369-370`), a11y por primitivo em vez de
   `to.be.accessible()` (`12-ferramentas.md:249`).

Cinco condições em que a decisão mudaria (todas observáveis, nenhuma hipótese de gosto):

1. **O consumidor deixar de exigir página cacheável e funcionamento sem JS** — se houver uma área
   autenticada e dinâmica (dashboard do CMS) onde FOUC e JS obrigatório são aceitáveis, CE passa a ser a
   opção natural naquele escopo.
2. **DSD deixar de emitir `<style>` inline** (estilos por `adoptedStyleSheets` externa, ou política que
   aceite hash de style) **e** `@lit-labs/ssr` sair de Labs — as duas condições juntas, porque uma só
   não remove o custo de bytes nem o risco.
3. **Aparecer uma necessidade de UI que o HTML nativo não cobre** e que não possa ser script de domínio
   declarado — o corpus lista exatamente esse nicho (`09-performance.md` §11, tabela virtualizada).
4. **`M1` falhar e o plano B for adotado** (`16-roadmap.md` §2, "parar o projeto e adotar maud"): se a
   camada tipada morrer, a alternativa registrada no corpus é "backend como API, UI em CE"
   (`general-cross-lang-usage.md:31`) — não outro template engine.
5. **Um gate de a11y que atravesse Shadow Root** entrar no CI do consumidor (hoje inexistente, §7.2):
   sem ele, CE entraria sem cobertura de acessibilidade — a condição é ter o gate antes do componente.
