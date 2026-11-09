# `leptos` 0.8 — composição sem reatividade

Leptos é um framework full-stack em Rust: componentes são funções, `view!` produz uma árvore reativa,
`ssr`/`hydrate` compilam o mesmo código para nativo e para WebAssembly, e `islands` reduz o WASM a
"ilhas" hidratadas. Este conjunto (23 capítulos + `SKILL.md`, ~3.200 linhas, MIT) é o corpus mais próximo
do nosso problema **na forma** — props tipadas, children como valor, slots, `view!` validado em
compilação — e o mais distante **no contrato**: o HTML deixa de ser o artefato.

Valor para `html-auy`: (a) a **camada de composição** é portável quase inteira e o corpus mostra os
nomes e as armadilhas; (b) a **camada de runtime** é o contraexemplo medido — e o único lugar do corpus
onde se lê, byte a byte, o que um framework moderno põe no HTML além do HTML (script de WASM,
`data-component`, `data-props`, fragmentos fora de ordem). Toda rejeição vem com a consequência exata em
CSP, `ETag` ou funcionamento sem JS.

> `VERSION` = `0.8`. O `spec/elements.toml` de html-auy foi revisado por outro fluxo durante esta
> leitura (`accessible_name`, `invariants`, `required_children`, `first_child`); os conflitos da §4
> consideram o **estado atual** do repositório.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---:|---|---|
| `VERSION` | 0 | literal `0.8`; sem data de release | fixa a versão; nada além disso aproveitável |
| `00-foreword.md` | 41 | posicionamento: reatividade de grão fino, WASM no cliente + SSR | enquadramento; só índice, não rende achado |
| `01-getting-started.md` | 111 | Trunk (CSR) vs `cargo-leptos` (SSR), `console_error_panic_hook`, `leptosfmt`, `erase_components` | **fonte** de A38 (`fmt` próprio, pressão de compilação) |
| `02-view-syntax.md` | 165 | `view!`, texto literal, `class:`/`style:`, `prop:`, `inner_html`, builder, custom element | **fonte** de A8, A9, A14, A15, A16 (catálogo de açúcar a recusar) |
| `03-components.md` | 167 | componente = setup fn, props tipadas, `optional`/`default`/`into`/`marker`, genéricas, spread | **fonte principal do porte** (A1–A7) |
| `04-reactivity.md` | 199 | `signal`/`RwSignal`, `_local`, memos, `Effect` (e "não roda no servidor"), auto-tracking | **fonte** de A27 (o que nunca entra) |
| `05-control-flow.md` | 247 | `if`/`match`/`Option`/`Result` como view, `<Show/>`, listas, `<For key>`, `into_any`, `Either` | **fonte** de A17–A20 |
| `06-forms-inputs.md` | 123 | controlado vs não-controlado, `prop:value`, `bind:`, `NodeRef`, `<textarea>`, `<select>` | **fonte mais útil de formulários** (A10–A13) |
| `07-parent-child.md` | 251 | `WriteSignal` como prop, callback, `on:` no componente, context, children/`ChildrenFragment`, slots, render props, `clone:` | **fonte** de A21–A25 (desenho de composição) |
| `08-async.md` | 156 | `spawn_local`, `Resource`/`LocalResource`/`OnceResource`, `Suspense`, `Await`, `Transition`, `Action` | **fonte** de A28 |
| `09-routing.md` | 196 | router, rotas aninhadas, `Outlet`, params/queries, `<A>` com `aria-current`, `<Form>` + `requestSubmit()` inline | **fonte** de A39 e da evidência de CSP |
| `10-global-state.md` | 126 | URL como estado global, context, `reactive_stores` com acesso por campo | **fonte** de A25–A26 |
| `11-styling.md` | 92 | "sem opinião" sobre CSS; Tailwind, stylers, stylance, `styled` em runtime | **fonte** de A34 |
| `12-metadata.md` | 55 | `Title` com `formatter` closure, `Link`, `Stylesheet`, `Style` inline, `<Script>` vs `<script>` | **fonte** de A33 (head durante o render) |
| `13-js-interop.md` | 80 | `wasm-bindgen`, `web-sys`, `NodeRef`, aviso de "duas fontes da verdade" | **fonte** de A13 e A34 |
| `14-testing.md` | 100 | lógica fora do componente, `wasm-bindgen-test` comparando `inner_html()`, Playwright, Gherkin | **fonte** de A38 (snapshot como contrato) |
| `15-ssr.md` | 152 | ciclo SSR, 4 modos de streaming, `Resource::new_blocking`, bugs de hidratação | **fonte** de A29, A30 (maior impacto) |
| `16-server-functions.md` | 194 | `#[server]`, `serde_qs` compatível com `<form>`, endpoints hasheados, extractors, `redirect`, tabela async | **fonte** de A35 e da nota de autz |
| `17-progressive-enhancement.md` | 103 | 5 princípios, `<ActionForm/>` com/sem JS, `on:submit:capture`, nomes `serde_qs` indexados | **fonte** de A36–A37; é o capítulo que confirma o desenho |
| `18-islands.md` | 131 | `#[island]`, `HydrationScripts islands=true`, children server-only, contexto entre ilhas, **HTML emitido** | **fonte** de A31–A32 (bytes exatos) |
| `19-deployment.md` | 192 | deploy CSR/SSR, Docker, non-root paths, `opt-level='z'`, compressão, `#[lazy]`/`#[lazy_route]` | **fonte** de A32 (orçamento de bytes) e non-root |
| `20-appendix-reactive-system.md` | 90 | grafo signal/memo/effect, diamante, push-pull, memo vs derived | irrelevante salvo como registro de custo (A27) |
| `21-appendix-lifecycle.md` | 125 | componentes "não existem em runtime", ownership tree, arena de signals, `panic` ao ler descartado | **fonte** de A27 (prova do runtime que P5 proíbe) |
| `SKILL.md` | 102 | resumo operacional: regras, patterns, mapa de arquivos, Rust 1.85+ | índice de conferência; as regras úteis estão nos capítulos |

## 2. Achados

### Tabela 2.0 — o que cada recurso põe no HTML (e o que isso custa)

| Recurso do Leptos | Entra no HTML? | CSP | `ETag` | Sem JS |
|---|---|---|---|---|
| `#[component]` + `view!` estático | só o markup equivalente | nenhum efeito | estável | funciona |
| `signal`/`RwSignal`/`Memo` (em `ssr`) | nada (estado em memória da request) | nenhum | nenhum | funciona |
| `Effect::new` | nada — "**Não roda no servidor**" (`04-reactivity.md:119`) | nenhum | nenhum | tudo que dependa dele é invisível ao HTML |
| `Resource` + `<Suspense/>` (síncrono) | fallback + conteúdo quando resolve | nenhum | **duas saídas para a mesma URL** | fica no fallback |
| Streaming OOO (padrão) | HTML em pedaços, fora de ordem | nenhum | **impossível**: `sha256` do corpo não existe antes do último flush | funciona, mas o documento não é atômico |
| `HydrationScripts` / `hydrate_body` | `<script type="module">` de bootstrap + URL do `.wasm` | exige `script-src 'self'` e `connect-src` | nome do bundle tem de ser fixo; sem hash de conteúdo no tag | página visível, interação morta |
| `#[island]` | `<leptos-island data-component="Counter_12345" data-props='{"initial":0}'>` | `<script>` externo; `data-props` é dado, não script | **hash de build** no atributo → `ETag` muda sem mudar dado | degrada para estático |
| `<ActionForm/>` | `<form method="post" action="<endpoint>">` | nenhum (form nativo) | endpoint hasheado por build entra no hash do HTML | **funciona** (POST + redirect) |
| `provide_context` / `Store` | nada | nenhum | nenhum | funciona (acopla ordem de execução) |
| `leptos_meta` `<Title>`/`<Meta>` | sim, no `<head>` | nenhum | ordem depende da ordem de execução do render | funciona |
| `leptos_meta` `<Style>` | `<style>` **inline** | **quebra** `style-src 'self'` | CSS inline muda o hash do HTML a cada ajuste | funciona |
| `on:click`/`on:input` | **nada** no HTML SSR — listeners vêm na hidratação | nenhum | nenhum | HTML idêntico, comportamento ausente |
| `inner_html` | bytes crus, sem escape | nenhum | nenhum | funciona (e é o defeito) |
| `style:` / `style=` | atributo `style` | **quebra** `style-src 'self'` | muda por valor dinâmico | funciona |
| `prop:value` / `bind:` | só o `value` **inicial** (attribute) | nenhum | estável | estado do usuário não persiste; pós-POST exige re-render |
| `<Router>` / `<A>` | `<a href>` (+ `aria-current` na rota ativa) | nenhum | estável se a rota ativa for determinística na request | funciona |
| children server-only de um island | conteúdo real no HTML, sem custo de WASM | nenhum | estável | funciona |

### A1. Componente é função que roda uma vez (setup, não render)
- Citação: `03-components.md:27` — "1. A função **roda uma vez** (setup function, não render function)".
- Semântica: a função monta a saída uma vez; não é chamada a cada mudança de estado.
- Consequência: é a nossa view (`fn nome(p: Dados) -> Markup`). Sem segunda passagem, `Markup` pode ser o `String` final.
- Veredito: confirma P5 · D7. Só registrar a equivalência.

### A2. Props são argumentos; a macro gera a struct de props
- Citação: `03-components.md:28` — "2. Props são argumentos da função"; `:29` — "3. O macro `#[component]` gera um struct Props automaticamente".
- Semântica: a assinatura é a API; a struct existe para a macro e para o builder.
- Consequência: primitivos com assinatura explícita, obrigatórios primeiro (`03-vocabulario` §7). Struct nomeada só acima de ~4 args — nunca builder fluente, que devolve `Option` a todo campo.
- Veredito: confirma P3 · D7.

### A3. Props não têm significado reativo "especial"
- Citação: `03-components.md:34` — "Props **não têm significado reativo especial**. Para que a UI responda a / mudanças, passe signals ou closures:".
- Semântica: reatividade é o *tipo do valor*, não uma propriedade da prop.
- Consequência: sem reatividade, "prop" é dado. Não existe prop viva: o que muda entre duas renderizações é uma **request nova** — e isso elimina a categoria "prop que precisa ser função".
- Veredito: confirma P1; base da recusa em A6.

### A4. Opcional e default são declarados no parâmetro
- Citação: `03-components.md:47` — "### `#[prop(optional)]`"; `:67` — "    #[prop(default = 100)]".
- Semântica: ausência significa `Default::default()` ou valor escolhido.
- Consequência: espelha `attr?: Option<T>` (não emite quando `None`) e "nunca atributo vazio" (`09` §4). Default nunca é `""`: para texto, `Option<Msg>`.
- Veredito: confirma D5 · `02` §4.

### A5. `#[prop(into)]` e `#[prop(marker)]`: açúcar que reabre a porta
- Citação: `03-components.md:75` — "Aplica `.into()` automaticamente no valor passado:"; `:94` — "    #[prop(marker)] _ty: PhantomData<T>,".
- Semântica: `into` deixa a prop aceitar qualquer tipo conversível; `marker` esconde um parâmetro de tipo.
- Consequência: `into` é a versão de props do `From<String>` infalível que destruiria `Markup` — com ele, o construtor seguro de `Text`/`Href` deixa de ser único. `marker` não tem função sem `#[component]`.
- Veredito: refuta (into) · irrelevante (marker). Registrar em `docs/design/02` §8.

### A6. Props de closure e props genéricas `Fn() -> T`
- Citação: `03-components.md:122` — "    progress: Option<Box<dyn Fn() -> i32 + Send + Sync>>,"; `:107` — "    progress: F,".
- Semântica: para reagir, passa-se código; a prop carrega comportamento.
- Consequência: a saída passa a depender de código executado no render — o oposto de bytes determinísticos (`00-principios.md:43-44`) — e força `'static + Send + Sync` e `impl Trait` na superfície, que `10-ergonomia.md:14` proíbe na view.
- Veredito: refuta P6 · D14. **Nenhuma prop aceita closure/callback.**

### A7. Spread de atributos: aplicado a *todos* os elementos de topo
- Citação: `03-components.md:147-148` — "Atributos adicionados a um componente são aplicados a **todos** os elementos / HTML de topo retornados pela view do componente:"; `:167` — "Para aplicar atributos seletivamente, use `AttributeInterceptor`."
- Semântica: `<Comp class:foo=true on:click=… attr:id="foo" {..} {..spread}/>` despeja atributos em cada raiz.
- Consequência: invalida a allowlist por elemento (um `aria-label` pode cair em `<div>` *e* em `<p>`) e viola conteúdo-modelo; `AttributeInterceptor` é o escape hatch de D4 com outro nome.
- Veredito: refuta P3 · D4. Atributo não se propaga: primitivo recebe props tipadas.

### A8. `inner_html`: o `Raw` do Leptos
- Citação: `02-view-syntax.md:88` — "### `inner_html` (injeção de HTML raw)"; `:92` — "view! { <div inner_html=html/> }".
- Semântica: aceita string e injeta sem escape; a doc diz "injeção" sem ironia.
- Consequência: é o que `Markup` privado elimina. A porta existe porque o tipo de contexto é fraco (mesma lição do maud issue #181).
- Veredito: refuta D2; a spec já reflete (`spec/elements.toml:82` — "script inline é proibido (D3)").

### A9. `style=` e `style:` dinâmicos
- Citação: `02-view-syntax.md:60` — "style:left=move || format!("{}px", count.get() + 100)"; `:62` — "style:max-width="400px"".
- Semântica: estilo é expressão Rust aplicada inline ou por propriedade.
- Consequência: inline exige `unsafe-inline` em `style-src`, e valor dinâmico muda o hash do HTML. Nosso caminho é `Tone`/`SectionId` → `/theme.css` (D12).
- Veredito: refuta D3 · D12.

### A10. Attribute ≠ property: `value` carrega só o estado inicial
- Citação: `06-forms-inputs.md:27` — "- `value` **attribute**: define valor inicial apenas"; `:18` — "        // Use prop:value, NÃO value (attribute)".
- Semântica: o atributo é o valor com que o campo nasce; a property é o valor vivo, que só existe no cliente.
- Consequência: **só emitimos atributos**, logo toda página é "não-controlada por natureza". Estado do usuário não tem onde morar no HTML; o pós-POST vem de re-render (PRG). Não é limitação: é o que torna a página cacheável — e precisa virar invariante.
- Veredito: refuta o `prop:` e **confirma** o desenho; vira regra (§5a). `checked` segue igual: presença, nunca `"false"` (`06:31`).

### A11. `<textarea>`: não existe atributo `value`; o valor é filho de texto
- Citação: `06-forms-inputs.md:88` — "Não suporta `value` attribute. Use child text + `prop:value`:".
- Semântica: para pré-preencher em SSR, o conteúdo vai entre as tags.
- Consequência: o gerador emite o valor como **texto filho escapado por `Text`**, nunca atributo. A spec atual já permite (`children = ["text"]`, sem `value` em `allowed`); falta a invariante nomeada e o primitivo.
- Veredito: confirma a spec atual; pede `value_as_text_child` (§5b).

### A12. `<select>`: o estado mora no `option`, não no `select`
- Citação: `06-forms-inputs.md:110` — "        prop:value=move || value.get().to_string()".
- Semântica: o Leptos sincroniza o `select` por property; no HTML não há `value` de `select`.
- Consequência: no nosso vocabulário o valor atual vira `selected` no `option` (já permitido) e `option` já exige `value`. O primitivo `Select` calcula `selected` a partir do dado da request — determinístico.
- Veredito: confirma `spec/elements.toml` (`option`: `required = ["value"]`, `allowed` com `selected`).

### A13. `NodeRef` e `bind:`: o canal para ler estado do DOM
- Citação: `13-js-interop.md:50` — "view! { <input node_ref=input_ref/> }"; `:60` — "`NodeRef::get()` retorna `Option<leptos::HtmlElement<T>>`, que implementa"; `06-forms-inputs.md:43` — "    <input type="text" bind:value=(name, set_name)/>".
- Semântica: `node_ref` abre referência ao DOM; `bind:` instala listener e escreve na property.
- Consequência: `node_ref` é atributo fora de qualquer allowlist e a leitura só acontece em `Effect` (que não roda no SSR); `bind:` é `on*` disfarçado. Ler valor de campo é trabalho do handler, no servidor.
- Veredito: refuta D3 · D4.

### A14. Listeners `on:evento` (incluindo `:target` tipado)
- Citação: `02-view-syntax.md:31` — "4. Event listeners usam `on:{evento}`".
- Semântica: handler no markup; no SSR o atributo **não é emitido** (vira registro de hidratação).
- Consequência: confirma a Tabela 2.0 — o HTML fica limpo e o comportamento desaparece sem JS. Ou seja, "sem `on*`" não garante "funciona sem JS"; isso exige `<form>`/`<a>`/`<details>` nativos (A36).
- Veredito: refuta D3 e **reforça** que progressive enhancement é escolha de elemento, não ausência de atributo.

### A15. `class:` condicional e `class=(nome, condição)`
- Citação: `02-view-syntax.md:47` — "class:red=move || count.get() % 2 == 1"; `:50` — "class=("button-20", move || count.get() % 2 == 1)".
- Semântica: classes ligadas por condição; nomes não-parsáveis por tupla.
- Consequência: a **forma** é portável — condicional em posição de atributo com tokens fechados (`Class::of`), sem string livre. O predicado reativo não é: a condição é avaliada uma vez sobre dado da request.
- Veredito: refina (porte parcial, como em `18-aprendizados.md` §topcoat adição 3).

### A16. Texto é literal Rust entre aspas
- Citação: `02-view-syntax.md:28` — "1. **Text nodes** devem ser strings Rust entre aspas: `"texto"`".
- Semântica: o caminho natural do corpus é o literal; i18n não é assunto do framework.
- Consequência: divergência deliberada — nosso texto visível é `Msg` (`02` §6). Como o literal é o menor esforço, o advisory N3 de "texto visível literal" não é polimento: é o único mecanismo que impede a view monolíngue.
- Veredito: refuta (divergência) · confirma `05-acessibilidade` §4.

### A17. `if`/`match`/`Option` como view — e por que `into_any` não nos pertence
- Citação: `05-control-flow.md:7` — "1. Expressões `if`, `match` retornam valores"; `:8` — "2. `Option<T>` e `Result<T, impl Error>` implementam `IntoView`"; `:247` — "Ou use os enums `Either`, `EitherOf3`, `EitherOf4`."
- Semântica: controle de fluxo é o da linguagem; ramos divergentes precisam ser unificados por `into_any`/`Either`.
- Consequência: portamos `if`/`match` e `Option<Markup>`; `Result` não (erro é antes do render, no handler). `Either` é dívida de árvore tipada: com `Markup` único, o problema não existe.
- Veredito: confirma P8/`10-ergonomia` §1; refuta `Either` (eliminado por construção).

### A18. "Para ser reativo, o valor deve ser uma função" — e `<Show/>` existe por isso
- Citação: `05-control-flow.md:10` — "4. Para ser reativo, o valor deve ser uma **função** (`move ||`)"; `:59` — "`<Show/>` memoiza a condição: renderiza o conteúdo **uma vez** e só troca".
- Semântica: reatividade = closure sobre signal; `<Show/>` é um `if` com memoização para não re-renderizar.
- Consequência: é a linha divisória — **nenhuma closure atravessa a API de html-auy**. Se algo muda na tela, muda por nova request (ou por asset lendo `data-*`). Sem re-render, `Show` não tem função: `@if` cobre tudo.
- Veredito: refuta · irrelevante (`Show`). Citar nos ADRs.

### A19. Listas estáticas e `collect_view()`
- Citação: `05-control-flow.md:90` — "{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}".
- Semântica: acumular views num container heterogêneo.
- Consequência: portamos como `for` escrevendo no buffer do `Markup` (uma alocação por bloco, `09` §1). `Vec<View>` não entra: coleção heterogênea é o que força `into_any` (A17).
- Veredito: confirma D7 · `09` §1.

### A20. `<For key>`: chave estável, "não use índice!"
- Citação: `05-control-flow.md:132` — "- `key`: função `&T -> K` onde K é única e estável (não use índice!)".
- Semântica: a chave é a identidade do item para o diff.
- Consequência: sem diff, a lição transfere para **`id` gerado em laço**: id vindo do índice muda de alvo de `label for`/`aria-describedby` quando a lista reordena. Regra: id de item deriva da chave do dado. `[INFERENCE]`
- Veredito: refina; vira regra de geração (§5d).

### A21. Children é valor, e `ChildrenFragment` permite re-embrulhar
- Citação: `07-parent-child.md:133` — "| `Children` | `Box<dyn FnOnce() -> AnyView>` |"; `:142` — "pub fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {".
- Semântica: children são opacos (`Fn`), chamados uma vez (`FnOnce`) ou como `Vec<View>` no fragment.
- Consequência: portamos children como valor **já renderizado** (`Markup`/`impl Render`), sem boxing nem closure; `List` emite o `<li>` (o análogo do `WrapsChildren`). "Consumido uma vez" vira restrição real: o mesmo conteúdo em dois pontos exige duas props.
- Veredito: confirma/refina (`18-aprendizados.md` §leptos itens 4 e 6).

### A22. Slots são structs tipadas com `#[slot]`
- Citação: `07-parent-child.md:181-182` — "#[slot] / struct Then { children: ChildrenFn }"; `:207` — "    <Then slot:then_slot>"Show this"</Then>".
- Semântica: slot nomeado é tipo próprio, com seus campos.
- Consequência: portamos o **fato** (`Picture { source, img, caption }`), não a mecânica (`slot:` é açúcar de macro sobre children). Nunca slot por string nem mapa de nomes.
- Veredito: confirma D7 · `18-aprendizados.md` §leptos item 3.

### A23. Render props e `clone:`
- Citação: `07-parent-child.md:159` — "    render_prop: F,"; `:246` — "        <Inner clone:name>".
- Semântica: uma prop pode produzir view; `clone:` resolve uso múltiplo de valor movido.
- Consequência: render prop é closure (A6), cai pela mesma regra. `clone:` resolve um problema que `Markup` owned recria — mas deve ficar explícito na assinatura, não escondido na macro (`10-ergonomia` §1: quem escreve view não usa `move`/clone).
- Veredito: refuta render prop; `clone:` irrelevante (resolvido por assinatura).

### A24. Context API: estado ambiente identificado por tipo
- Citação: `07-parent-child.md:66` — "    provide_context(set_toggled);"; `:101` — "- Context é identificado por **tipo**: só pode haver um valor de cada tipo".
- Semântica: o pai publica, qualquer descendente consome sem prop; a chave é o tipo.
- Consequência: (a) a dependência some da assinatura — e `use_context` seria a quinta construção em `10-ergonomia` §1; (b) colisão silenciosa de tipo (a doc recomenda newtype, `:95`); (c) ordem de execução passa a importar. Nosso `Ctx` é `&mut` explícito, consumido por `shell()` (`06` §9).
- Veredito: refuta.

### A25. Stores: acesso reativo por campo
- Citação: `10-global-state.md:104` — "let data = Store::new(Data { rows: vec![/* ... */] });"; `:64` — "    provide_context(Store::new(GlobalState::default()));".
- Semântica: `#[derive(Store)]` gera acesso reativo por campo, evitando signals aninhados.
- Consequência: resolve granularidade que só existe com reatividade; traz macro de derivação, `StoreFieldIterator` e modelo de mutação. Ganho zero: nosso estado global é o dado da request.
- Veredito: refuta.

### A26. A URL é o melhor lugar para estado global
- Citação: `10-global-state.md:9-10` — "A URL é o melhor lugar para estado global: acessível de qualquer componente, / persiste entre reloads, é compartilhável."
- Semântica: query string como transporte de estado de UI, não signal global.
- Consequência: confirma `<search>` + `form method=get` e paginação por query já no vocabulário; filtro/ordenação de listagem não precisam de estado de cliente.
- Veredito: confirma P1 · `03-vocabulario` (landmark `search`).

### A27. Signals são índices de arena; ler um descartado dá `panic`
- Citação: `21-appendix-lifecycle.md:43-44` — "Signals são **índices** em uma arena (estrutura de dados global), / não ponteiros com reference counting. Por isso são `Copy`."; `:69` — "Se você tentar **ler** um signal descartado → panic (use `.try_get()`)."; `04-reactivity.md:119` — "**Não roda no servidor**".
- Semântica: o valor reativo é um `usize` em estado global por runtime, com dono = effect que o criou; `Effect` não existe no SSR.
- Consequência: é o runtime que P5 proíbe, com falha em `panic` de produção (não de compilação), e qualquer coisa dependente de `Effect` é invisível ao HTML.
- Veredito: refuta P5 · P7. Prova mais forte do corpus contra o modelo reativo.

### A28. `Resource` separa "o que dispara" de "o que busca" — e executa de novo no cliente
- Citação: `08-async.md:40-41` — "Signals no **source** são trackeados; signals no **fetcher** não são (importante / para hidratação correta)."; `16-server-functions.md:194` — "| Suspend (Client) | — | Executa de novo para hidratação | Executa quando necessário |".
- Semântica: o fetcher pode rodar duas vezes (servidor e hidratação) e o resultado é serializado.
- Consequência: (a) busca duplicada + resultado no documento; (b) `<Suspense>` cria fallback → **dois HTMLs possíveis para a mesma URL**, ambíguos para `Cache-Control`/`ETag`; (c) `Transition` "**mantém o conteúdo antigo**" (`08:110`) — o documento pode exibir dado de outra request.
- Veredito: refuta. Carregar no handler, renderizar depois; a view nunca busca.

### A29. Streaming fora de ordem é o **padrão** do framework
- Citação: `15-ssr.md:63` — "### Out-of-Order Streaming (padrão)"; `:65-66` — "Envia HTML shell imediatamente com fallbacks, e "streama" fragments / de `<Suspense/>` conforme resolvem."; `:54` — "<Route path=path!("/post/:id") view=BlogPost ssr=SsrMode::Async/>".
- Semântica: o corpo é escrito em pedaços, na ordem em que os resources resolvem; `SsrMode` escolhe a política **por rota**.
- Consequência: sem corpo fechado não há `sha256` do corpo, `Content-Length` nem `ETag` forte — mata `09` §2 e o teste "dois renders = mesmos bytes" (`12-testes` §7).
- Veredito: refuta D14 · `09` §2.

### A30. Os dois bugs de hidratação são regras de marcação — e um gate que faltava
- Citação: `15-ssr.md:117` — "**HTML inválido**: `<div>` dentro de `<p>` é ilegal — browser fecha o `<p>`"; `:125` — "**`<table>` sem `<tbody>`**: browser insere `<tbody>` automaticamente."; `:128` — "// ✅ Sempre inclua <tbody>".
- Semântica: o navegador corrige o HTML ao parsear; com hidratação a correção vira erro, sem hidratação vira **bytes diferentes do que emitimos**.
- Consequência: `<div>` em `<p>` nós já recusamos (`p` → `phrasing`) e `tbody` explícito já está na spec (`invariants = ["tbody_explicit"]`). Falta elevar o teste de árvore (`12-testes` §6, `html5ever`) a invariante de determinismo — é o papel que a hidratação cumpre no Leptos.
- Veredito: confirma P6 · D8; reforça o gate de árvore.

### A31. Ilhas: os bytes exatos que entram no documento
- Citação: `18-islands.md:126-128` — "<leptos-island data-component="Counter_12345" data-props='{"initial":0}'> / <button>Click Me: 0</button> / </leptos-island>"; `:51` — "| Pode usar código server-only (em modo islands) | Props serializáveis (JSON) |".
- Semântica: o conteúdo é envolvido por um elemento customizado que carrega (a) identificador de componente com **hash de build** e (b) as props **serializadas em JSON** num atributo.
- Consequência: quatro violações de uma vez — estado serializado no documento (P1), identificador de build no HTML (`ETag` muda por build sem mudar dado), atributo cujo valor é JSON (exigiria `JsonLd` **dentro** de contexto de atributo, que não temos por decisão) e elemento fora do vocabulário.
- Veredito: refuta P1 · D14 · D4. Contraexemplo definitivo.

### A32. `HydrationScripts` e o piso de 24 KB de WASM
- Citação: `18-islands.md:26` — "<HydrationScripts options islands=true/>"; `:55-57` — "- App "Hello World" SSR: ~274kb / - App "Hello World" islands: ~24kb (sem interatividade) / - App com um island: ~166kb"; `19-deployment.md:145` — "Sempre sirva WASM comprimido (gzip/brotli). WASM comprime para <50%."
- Semântica: o shell ganha um componente que emite o bootstrap; o payload é o binário.
- Consequência: 24 KB é o **piso** sem interatividade, 166 KB com uma ilha — contra o orçamento de JS ≤ 150 KB e os ~2.800 LOC de JS vanilla do consumidor (`07-js-e-assets` §4). Custo estrutural, não de configuração.
- Veredito: confirma D13 · orçamento de `09` §4.

### A33. `leptos_meta`: o `<head>` é montado **durante** o render
- Citação: `12-metadata.md:3` — "Use o pacote `leptos_meta` para injetar elementos no `<head>`."; `:15` — "<Title formatter=|text| format!("{text} — My Site")/>"; `:40-41` — "`<Script/>` (capital S) → insere no `<head>`. / `<script>` (minúsculo) → insere no `<body>`."
- Semântica: componentes declaram metadados; presença e ordem dependem de quais rodaram.
- Consequência: (a) título por closure (A6); (b) ordem do `<head>` dependente de execução, contra a ordem fixa do shell e o prefixo de 14 KB; (c) até a posição do script é decidida por **nome de componente** — ambiguidade que resolvemos por posição estrutural.
- Veredito: refuta D11. O `<head>` é do shell, uma vez, antes do miolo.

### A34. Estilo "sem opinião", JS dono do DOM: dois avisos do mesmo tipo
- Citação: `11-styling.md:3` — "Leptos não tem opinião sobre CSS. Você pode usar qualquer abordagem."; `:80` — "## Styled (runtime scoped CSS)"; `13-js-interop.md:79-80` — "manipulam o DOM diretamente — tanto a lib quanto o Leptos assumem ser / a fonte da verdade para o estado da UI."
- Semântica: CSS por injeção em runtime é aceitável; e o framework reivindica ownership do DOM, criando disputa com libs.
- Consequência: `<style>` em runtime quebra `style-src 'self'` e nomes de classe fora do vocabulário quebram `Class`/`Token`. Nosso JS é dono de **nada**: lê `data-*` e chama endpoints; o HTML é a fonte da verdade.
- Veredito: refuta D3 · D12 · confirma D13.

### A35. Server functions: "açúcar sintático para APIs públicas"
- Citação: `16-server-functions.md:78-80` — "Server Functions são **açúcar sintático para APIs públicas**. / O corpo da função nunca é exposto, mas o endpoint é publicamente / acessível."; `:50` — "- Input encoding: `serde_qs` (default, compatível com `<form>`)"; `:52` — "- Endpoints: URLs hasheadas para evitar colisão".
- Semântica: cada `#[server]` é endpoint HTTP real, com input form-encoded e URL hasheada.
- Consequência: (a) o vocabulário não ganha "chamada de servidor": a ação é `<form method=post action=LUrl>`, e CSRF/autz é do consumidor (`08-seguranca` §7); (b) endpoint com hash de build no `action` repete o defeito de A31 — muda o `ETag` do HTML sem mudar o dado; (c) a nota de segurança só existe porque transporte e domínio estão co-localizados. Nós não co-localizamos.
- Veredito: refuta (co-localização) · confirma `08-seguranca` §7.

### A36. Progressive enhancement: os cinco princípios são o nosso programa
- Citação: `17-progressive-enhancement.md:9-13` — "1. **SSR** — sem SSR, sem conteúdo sem JS/WASM / 2. **Elementos HTML nativos** — `<a>`, `<form>`, `<details>` funcionam sem JS / 3. **Estado via URL** … / 5. **Formulários** — `<form>` é a base da progressive enhancement".
- Semântica: receita mínima para "funciona sem JS" — e a única do corpus.
- Consequência: (1)–(5) já estão no desenho. O capítulo acrescenta a prova de que **nada disso vem do framework**: vem do elemento escolhido — o que justifica `03-vocabulario` §3.
- Veredito: confirma P4 · `05-acessibilidade` (3.2.1/3.2.2) · `07` §10.

### A37. `<ActionForm/>` com e sem JS; validação de cliente; nomes indexados
- Citação: `17-progressive-enhancement.md:51-53` — "| Submit | Fetch assíncrono | POST com reload | / | Redirect (via `leptos_axum::redirect`) | Navegação client-side | Redirect HTTP |"; `:67` — "<ActionForm on:submit:capture=on_submit action=add_todo>"; `:87` — "        <input type="text" name="hefty_arg[first_name]" value="leptos"/>".
- Semântica: sem JS o sucesso é POST + redirect; validação pré-submit é listener em fase de captura; nomes de campo podem ser gramática (`a[b][c]`) para `serde_qs`.
- Consequência: (a) nosso pós-POST tem de ser `303 See Other` + próximo GET (302 é re-POSTado e cacheável) — `Page` já carrega `status` (D11), falta a regra; (b) validação é constraint validation nativa + `aria-invalid` no re-render; (c) **lacuna**: nosso `name` é string livre, então a gramática vira convenção invisível — precisa de `FieldName` ou proibição.
- Veredito: confirma D11 · refuta (b) · refina/lacuna (c).

### A38. Testar comparando `inner_html()`, e o formatter próprio
- Citação: `14-testing.md:55` — "        test_wrapper.inner_html(),"; `:5` — "Extraia a lógica dos componentes para structs testáveis:"; `:98` — "O mínimo de lógica dentro dos componentes facilita testar."; `01-getting-started.md:75` — "### `leptosfmt` (formatador do `view!`)"; `:87` — "RUSTFLAGS="--cfg erase_components" trunk serve".
- Semântica: o teste de componente do ecossistema é "monte e compare o HTML"; a lógica sai do componente; a família macro-based precisa de formatador e de alívio de monomorfização.
- Consequência: é a nossa pirâmide (`12-testes` §2) sem WASM nem browser — o mesmo teste com um passo menos; `cq fmt` (D6) é padrão de mercado, não invenção; `erase_components` é a mesma pressão que justifica "bloco > ~200 linhas dividir" (`09` §5).
- Veredito: confirma `12-testes` §2 · P8 · D6.

### A39. `<A>` emite `aria-current` — e a ergonomia de rota empurra para script inline
- Citação: `09-routing.md:161` — "// Adiciona aria-current="page" se for a rota ativa"; `:164` — "`<A/>` resolve `href` relativo ao path do route aninhado."; `:191` — "    oninput="this.form.requestSubmit()"".
- Semântica: o link resolve relativo e marca a rota ativa; para busca instantânea, a doc recomenda `oninput` inline.
- Consequência: (a) `aria-current` sai do primitivo de navegação a partir da rota da request — nunca de estado de cliente (senão varia o atributo e o `ETag`); (b) a linha 191 é a evidência de que a ergonomia do framework **empurra** para `unsafe-inline`: CSP sem inline só se sustenta com forms nativos + `Vary: HX-Request`.
- Veredito: confirma D3 · `07-js-e-assets` §5.

## 3. Confirmações do desenho

1. **Componente = função que roda uma vez** (`03-components.md:27`) é exatamente `fn view(p: Dados) -> Markup`.
2. **Props são argumentos tipados** (`03:28`), struct é detalhe de macro — sustenta "a assinatura é a API" e a proibição de builder fluente.
3. **`optional`/`default` no parâmetro** (`03:47`, `:67`) mapeiam em `attr?: Option<T>` e em "nunca atributo vazio" (`09` §4).
4. **Children como valor, consumido uma vez** (`07:133`) é o nosso `impl Render`; `WrapsChildren` (`:142`) é o nosso `List` que emite `<li>`.
5. **Slots nomeados e tipados** (`07:181-182`): `Table::new(caption, head, body)`, `Picture{source,img,caption}` — nunca mapa de nomes.
6. **Controle de fluxo é Rust** (`05:7`) e `Option` como view (`:8`): `@if`/`for`/`match` corretos; `Either` (`:247`) é dívida do modelo de árvore.
7. **Texto literal é o caminho natural** (`02:28`): por isso o advisory N3 de literal (`05-acessibilidade` §4) é infraestrutura.
8. **`value` attribute = valor inicial** (`06:27`): emitir só atributos é o modelo de página cacheável.
9. **`<textarea>` sem atributo `value`** (`06:88`): a spec atual (`children = ["text"]`) está certa; falta o primitivo e a invariante.
10. **O navegador corrige HTML** (`15:117`, `:125`): `<p>`/`div` recusado e `tbody` explícito são determinismo; o teste de árvore (`12-testes` §6) é o gate correspondente.
11. **Progressive enhancement vem do HTML nativo** (`17:9-13`) e sem JS o único sucesso é POST + redirect (`17:51-53`).
12. **Snapshot de HTML é teste de framework** (`14:55`); "lógica fora do componente" (`14:5`) é o nosso `09` §5/§6.
13. **`aria-current` do link ativo** (`09:161`) é papel do primitivo de navegação a partir da rota da request (`06` §1, `ShellCtx.nav`).

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata · arquivo) |
|---|---|---|
| `02` §8 lista o que não existe, sem mencionar closure como prop | prop pode ser `Fn() -> T` (`03:107`, `:122`), tornando a saída dependente de código no render | Em `docs/design/02-tipos-e-contextos.md` §8: `\| Closure/callback como prop \| a saída deixaria de depender só do dado (Leptos 0.8, 03-components.md:107) \|` |
| `docs/design/04` não tem regra sobre correção do parser | navegador fecha `<p>` e insere `<tbody>` (`15:117`, `:125`) | Em `docs/design/04-validacao.md` §6: "A árvore parseada por `html5ever` deve ser **idêntica** à árvore emitida; correção do parser (ex.: `<tbody>` inserido, `<p>` fechado antes de `<div>`) é falha, não tolerância." |
| `docs/design/09` §2 fixa `ETag` = `sha256` do corpo, mas não proíbe identificador de build | `data-component="Counter_12345"` (`18:126`) muda por build | Em `docs/design/09-desempenho-e-cache.md` §2: "Nenhum identificador de build, componente ou sessão entra no HTML; o único nome versionado é o do asset, derivado do **hash do conteúdo**." |
| `docs/design/09` §4 proíbe atributo vazio, mas não trata estado embutido | props em `data-props='{"initial":0}'` (`18:126`, `:51`) | Em `docs/design/09` §2: "Estado nunca é serializado no documento: nenhum atributo carrega JSON de props, sessão ou cache." |
| `docs/design/07` §3 lista `data-*` escalares sem proibir JSON | `data-props` exige escape de JSON em contexto de atributo | Em `docs/design/07-js-e-assets.md` §3: "`data-*` transporta escalar declarado (enum), nunca JSON de estado; o valor é validado pelo vocabulário do consumidor." |
| `docs/design/03` §3 não fixa o estado inicial de campo | `value` = inicial (`06:27`); `<textarea>` por texto filho (`:88`); `select` por `selected` (`:110`) | Em `docs/design/03-vocabulario-de-elementos.md` §3, três linhas: `\| input \| value/checked são estado inicial; property não existe \|` · `\| textarea \| valor é filho de texto (`Text`), nunca atributo \|` · `\| select \| estado atual é `selected` no `option` correspondente \|` |
| `spec/elements.toml` não nomeia a invariante de `textarea` | mesmo conteúdo | Em `[element.textarea]`: `invariants = ["label_or_labelledby_in_block", "value_as_text_child"]` |
| `docs/design/06` §1 não liga `form method=post` a redirect | sem JS o sucesso é redirect HTTP (`17:51-53`) | Em `docs/design/06-shell-e-tema.md` §1, nota em `Page`: "`form method=post` responde `303 See Other`; o resultado aparece no GET seguinte." |
| `spec/elements.toml` deixa `input.name` como string livre | nomes indexados `a[b][c]` (`17:87`) | Em `docs/design/02` §7: `\| nome de campo \| FieldName \| segmentos declarados; `[`/`]` só por `FieldName::nested` \|` |

## 5. Adições prontas para colar

**(a) Formulário: onde o estado inicial vive** (para `docs/design/03` §3)

| Controle | Estado inicial em SSR | Proibido |
|---|---|---|
| `input[type=text\|email\|…]` | `value="…"` (escapado por `AttrVal`) | property, `bind:`, `prop:value` |
| `input[type=checkbox\|radio]` | `checked` (presença; falso = omite) | `checked="false"` |
| `textarea` | texto filho (`Text`) | atributo `value` |
| `select` | `selected` no `option` cujo `value` casa | atributo `value` no `select` |
| qualquer | — | JSON de estado em `data-*` |

**(b) `spec/elements.toml` — duas linhas de invariante**

```toml
[element.textarea]
invariants = ["label_or_labelledby_in_block", "value_as_text_child"]

[element.table]
invariants = ["caption_required", "tbody_explicit", "no_text_children", "parsed_tree_equals_emitted"]
```

**(c) Checklist de determinismo (para `docs/design/09` §2)**

1. Dois renders do mesmo dado → bytes idênticos (`==`), incluindo ordem de atributos.
2. Rebuild sem mudança de fonte → `ETag` idêntico.
3. Nenhum identificador de build/componente/sessão no HTML.
4. Nenhum estado serializado (`data-props`, JSON em atributo, marcador de hidratação).
5. Árvore parseada por `html5ever` == árvore emitida (nenhuma correção do navegador).

**(d) Regra de `id` gerado em laço (para `docs/design/03` §4)**

> `id` de item de lista deriva da **chave do dado** (o mesmo valor que serviria de `key`), nunca do
> índice: `label for` e `aria-describedby` precisam apontar para o mesmo alvo em qualquer ordem e em
> qualquer render (`05-control-flow.md:132` — "não use índice!").

**(e) Proibições explícitas de atributo (cabeçalho de `spec/elements.toml`)**

```toml
# Atributos nunca permitidos: style, on*, srcdoc, @*, hx-on*, x-on:*,
# data-props, data-component, node_ref, prop:*, bind:*  (Leptos 0.8 — 18-islands.md:126)
```

**(f) Tabela para `docs/design/14-alternativas.md` §"o que se perde"**

| Perda assumida ao não usar Leptos | Ganho medido |
|---|---|
| reatividade de grão fino, atualização sem reload | HTML fechado: `ETag`/304, `Content-Length`, cache por página |
| hidratação e ilhas (24 KB de piso; 166 KB com uma ilha) | zero WASM: JS ≈ 2.800 LOC declarados por `Asset` |
| streaming fora de ordem (TTFB menor) | corpo atômico: `sha256` calculável, snapshot determinístico |
| `leptos_meta` (head por componente) | `<head>` fixo e ordenado uma vez pelo shell |

## 6. Armadilhas de complexidade

| Armadilha | Por que não entra | Custo se entrar |
|---|---|---|
| **Hidratação** (`hydrate`, `HydrationScripts`) | exige WASM e markup idêntico dos dois lados (`SKILL.md:33`) | `<script>` de bootstrap em toda página, `connect-src` para o `.wasm`, e um modo de falha (mismatch) que só existe no browser |
| **Ilhas** (`#[island]`, `data-props`) | estado serializado + hash de build no documento (`18:126`) | `ETag` instável por build; escape de JSON em atributo; elemento fora do vocabulário |
| **Streaming OOO / `Suspense`** | corpo em pedaços, fora de ordem (`15:63-66`) | perde `sha256`, `Content-Length`, `ETag` forte e o teste de bytes; fallback vira segundo HTML para a mesma URL |
| **Signals/Effects/Memos** | arena por request, `panic` ao ler descartado (`21:43`, `:69`), `Effect` fora do SSR (`04:119`) | runtime em toda view; falha em produção, não em compilação |
| **Context/Store** | estado ambiente por tipo (`07:101`), invisível na assinatura | quinta construção em `10-ergonomia` §1; ordem de execução passa a importar |
| **`Resource`/`Action`/`Transition`** | busca dupla (`16:194`), conteúdo antigo na recarga (`08:110`) | view deixa de ser função pura do dado; teste deixa de ser determinístico |
| **Builder syntax / `into_any` / `Either`** | existem para árvores de tipos divergentes | obriga um tipo de view genérico — o oposto de `Markup` único |
| **Server functions (`#[server]`)** | co-localiza transporte e domínio; endpoint público hasheado (`16:52`, `:78-80`) | framework web (HTTP, sessão, authz) entra pela porta dos fundos — não-objetivo permanente (`00` §3) |
| **`leptos_meta`** | `<head>` montado durante o render (`12:3`) | ordem do `<head>` imprevisível → `ETag` instável e prefixo de 14 KB não garantido |
| **CSS em runtime (`styled`, `<Style>{include_str!}`)** | `<style>` inline | quebra `style-src 'self'` e o hash do HTML |
| **Macro de props (`#[component]`, `#[prop(...)]`)** | macro de derivação sobre a assinatura | segundo dialeto ao lado de `html!`; `#[prop(into)]` reabre `From` infalível |

## 7. Lacunas do corpus

1. **Estado pós-POST de campo.** O corpus assume property no cliente; não diz como o valor volta ao
   campo sem JS. Decisão nossa (§5a), sem respaldo textual.
2. **`id` determinístico em laço.** `<For key>` resolve diff no cliente, não `label for` estável entre
   renders. A regra da §5d é `[INFERENCE]`.
3. **i18n.** Nenhum capítulo trata tradução; texto é literal (`02:28`). Todo o modelo de `Msg`, locale
   e paridade de catálogo é invenção nossa (`02` §6).
4. **Cache HTTP.** Nada sobre `Cache-Control`, `ETag`, `304`, `Vary`; `19` só fala de compressão de
   WASM. Toda a §2 de `docs/design/09` é decisão nossa.
5. **Gramática de `name`.** `serde_qs` aparece como encoding (`16:50`, `17:75`) sem que o vocabulário de
   marcação defina gramática; precisamos de `FieldName` ou de proibir colchetes.
6. **Ação que não é formulário.** Menu, popover, diálogo e gaveta dependem de JS tanto aqui quanto em
   nós; nada no corpus ajuda a fechar `inert`/popover sem asset.
7. **Custo de render.** Nenhum número de µs/alocação; só tamanho de WASM (`18:55-57`). As metas de
   `docs/design/09` §1 não são calibráveis por este conjunto.
8. **Multi-tenant e tema.** Nada. `data-component`/`data-props` são o único uso de `data-*` e não servem
   de modelo para `data-a11y-*`/`data-section`.
9. **CSRF e autenticação em form nativo.** O corpus delega (`16:80`) e assume cookie/`ResponseOptions`
   do servidor; não há modelo de campo oculto de token. `[INFERENCE]` nosso `Form` precisará de um campo
   tipado de token — a decidir.
