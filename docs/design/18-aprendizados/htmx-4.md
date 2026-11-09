# `htmx-4` — versão 4 (12 arquivos, 2.380 linhas, `VERSION` = `4`)

O conjunto é **um guia de aplicação**, não a especificação do htmx: ensina a construir um "Task Manager"
em Rust/Axum + Askama + SQLite usando htmx 4, com layout de 3 colunas e respostas multi-target. Lido
aqui porque é a única descrição de htmx 4 disponível no corpus — e porque o nosso `docs/design/07-js-e-assets.md`
§7 já promete um contrato que ninguém escreveu. O documento abaixo **é** esse contrato: o que o corpus
ensina, o que ele contradiz, e o que a crate precisa decidir para que htmx entre sem quebrar
determinismo, `ETag`, CSP ou acessibilidade. Duas leituras transversais importantes antes de tudo:
o conjunto **não contém a palavra `Vary` em nenhum arquivo**, e contém **uma única** menção de
acessibilidade em 2.380 linhas (`03-three-column-layout.md:71-72`, um `focus()`). Tudo o que este
projeto exige de a11y e de cache é, portanto, adição nossa — não confirmação deles.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `VERSION` | 1 | literal `4` | fixa a versão-alvo; não há semver menor para citar |
| `SKILL.md` | 76 | regras centrais + file map + 2 padrões (layout 3 colunas, form) | é o resumo normativo do conjunto; contém `hx-oob`, atributo que não existe (achado A31) |
| `01-introduction.md` | 67 | filosofia HATEOAS + **tabela v2→v4** + "zero estado no cliente" | fonte primária das mudanças v2→v4 (engine, herança, erros, timeout) |
| `02-setup.md` | 153 | download do JS, `htmx-config`, CORS, **tabela de headers de requisição** | única listagem de `HX-Request/-Request-Type/-Target/-Source/-Boosted/-Current-URL` |
| `03-three-column-layout.md` | 234 | grid de 3 colunas, `hx-trigger="load"` ×6, `:inherited`, primeiro `focus()` pós-swap | mostra o padrão lazy que **quebra sem JS** (A21) e o único cuidado de foco do corpus |
| `04-task-crud.md` | 318 | CRUD completo, `hx-on:*` em 5 lugares, `hx-confirm`, `hx-swap="none"` | melhor fonte do ciclo real create/read/update/archive e das trocas `none` |
| `05-multi-target.md` | 177 | `<hx-partial>`, `swapEmpty:true`, `hx-swap-oob`, `hx-select-oob`, **ordem OOB** | define multi-target: elemento inventado + ordem main→OOB (A2–A6) |
| `06-forms.md` | 258 | login, validação 422, **`hx-status:NNN`**, `hx-validate`, `hx-delete` + `hx-include` | fonte do formato `hx-status` e do conflito `hx-push-url` em POST (A22) |
| `07-error-handling.md` | 244 | "toda resposta é trocada (exceto 204/304)", `hx-status` granular, `HX-Retarget/Reswap/Reselect`, eventos de erro, `noSwap` | fonte da semântica de erro e do único mecanismo de "não trocar" |
| `08-optimizations.md` | 241 | `htmx-request`/`hx-indicator`/`hx-disable`, morph, polling, `revealed`, view transitions, `hx-boost`, debounce, `hx-sync`, ETag | a fonte mais densa em riscos: morph, polling sem pausa, `keyup[filter]` (A17, A24–A27) |
| `09-rust-axum-patterns.md` | 349 | handlers, helpers de resposta (`HX-Redirect/-Refresh/-Push-Url`), `HX-Target` como roteador, paginação, `Render::{full,partial}` | única fonte dos headers **de resposta** e do roteamento por `HX-Target` (A12) |
| `10-migration-from-v2.md` | 263 | 12 passos, renomeações, tabela de eventos, `hx-config="timeout"`, modo de compatibilidade | fonte das renomeações e da ordem obrigatória `hx-disable`→`hx-ignore` (A14, A23) |

## 2. Achados

### A1. O corpus é guia de aplicação, não contrato — e assume spec que não existe
- Citação: `01-introduction.md:5` — "htmx 4 é uma biblioteca JavaScript que estende HTML com atributos `hx-*` para fazer requisições AJAX, acionar eventos, e atualizar o DOM — tudo sem escrever JavaScript."
- Semântica: htmx é descrito por comportamento observável (DOM muda, requisição sai), nunca por sintaxe de atributo. Não há uma única página de referência de atributo; a lista de headers em `02-setup.md:123-130` é a única tabela formal do conjunto.
- Consequência: nada aqui vale como fonte de verdade para o **nome ou a gramática** de um atributo. Toda a nossa allowlist precisa vir de decisão própria, e o corpus serve só como evidência de uso.
- Veredito: refina — obriga `docs/design/07` §7 a ser normativo, não descritivo.

### A2. `<hx-partial>` é um elemento novo, fora de qualquer vocabulário
- Citação: `05-multi-target.md:12` — "O elemento `<hx-partial>` permite que a resposta do servidor especifique **vários destinos** em um único response."
- Semântica: multi-target seria feito por um *elemento* (`<hx-partial hx-target="#task-tree">…</hx-partial>`, `05:25`, `05:80-84`, `09:89`), não por atributo em elemento existente.
- Consequência: emitir `<hx-partial>` exige um nome de elemento que `spec/elements.toml` não tem — ou seja, um caminho equivalente a `Raw`, exatamente o que D2 elimina. O crate **não consegue** produzir a resposta multi-target do corpus.
- Veredito: refuta o mecanismo. Multi-target só pode ser OOB (`hx-swap-oob`), que usa elemento do vocabulário.

### A3. Resposta só com `<hx-partial>` não troca o target principal
- Citação: `05-multi-target.md:90` — "Se a resposta contém **apenas** `<hx-partial>` (sem conteúdo principal), o htmx 4 **não** realiza o swap principal. […] O target original do `hx-target` é ignorado."
- Semântica: o servidor decide, pelo *corpo*, se o alvo principal é tocado. Para forçar a troca vazia existe `hx-swap="innerHTML swapEmpty:true"` (`05:92-97`).
- Consequência: "tocar ou preservar o alvo principal" é uma decisão de render, não de string de swap. Precisa ser um enum (`MainSwap::Replace` / `MainSwap::Keep`) no tipo de resposta, serializado sem `swapEmpty`.
- Veredito: refina `Render::{Full, Partial}` — a variante `Partial` precisa carregar essa escolha.

### A4. Ordem de aplicação: principal primeiro, OOB depois, em ordem de documento
- Citação: `05-multi-target.md:126` — "No htmx 2, OOB swaps aconteciam **antes** do conteúdo principal. No htmx 4, o conteúdo principal é trocado **primeiro**, depois os OOB (em ordem do documento)."
- Semântica: ordem fixa e determinística (repetida em `10-migration-from-v2.md:201`: "o conteúdo principal vai primeiro").
- Consequência: (a) o alvo principal não pode depender de nada que venha por OOB — a UI fica inconsistente por um tick; (b) anúncio por `aria-live` acontece antes das regiões OOB, então regiões OOB que também anunciam produzem ordem de fala previsível e testável.
- Veredito: confirma determinismo (P6/D14) e vira asserção de e2e.

### A5. `hx-swap-oob` depende de `id` estável no DOM
- Citação: `05-multi-target.md:105` — "Alternativa para quando os elementos têm `id` fixo no DOM" (uso em `05:111` e `05:118`: `<div id="sprint-info" hx-swap-oob="innerHTML">`).
- Semântica: OOB endereça por `id`; `hx-swap-oob="true"` substitui o elemento, `hx-swap-oob="innerHTML"` substitui o conteúdo.
- Consequência: cada região OOB precisa de `id` que venha do shell/primitivo, determinístico e único — e `id` duplicado na página já é regra do N4. O `id` deixa de ser detalhe do dev e vira contrato de tipo (`RegionId`).
- Veredito: confirma N4 e a necessidade de ids gerados; refina `docs/design/03`.

### A6. `hx-select-oob` extrai do próprio corpo já renderizado
- Citação: `05-multi-target.md:136` — `hx-select-oob="#sprint-info, #task-detail">` seguido de `05:141`: "Isso extrai `#sprint-info` e `#task-detail` da resposta e faz swap OOB, mesmo que eles não tenham `hx-swap-oob` no HTML."
- Semântica: o cliente pode fatiar uma resposta maior; o servidor responde **uma** coisa e o cliente pega pedaços.
- Consequência: é o único mecanismo do corpus que permite **o mesmo corpo** para navegador e para htmx — logo o único que dispensa `Vary` e mantém `ETag` único. Deve ser o caminho preferido para multi-região.
- Veredito: confirma e melhora `docs/design/07` §7 item 3 (é uma saída parcial do problema de `Vary`).

### A7. Toda resposta troca, menos 204 e 304
- Citação: `07-error-handling.md:5` — "No htmx 2, respostas `4xx` e `5xx` eram ignoradas (não trocavam o DOM). No **htmx 4, toda resposta é trocada** (exceto `204` e `304`)."
- Semântica: o status deixa de significar "não mexa no DOM"; o corpo passa a comandar. `01-introduction.md:44` e `06-forms.md:65` repetem a mudança como a de maior impacto da versão.
- Consequência: sem contrato, um `500` num poll de sidebar apaga a sidebar (`07:9`: "ele vai substituir o target"). O crate precisa de matriz explícita status→swap, e de um jeito de pedir "não troque" que não seja `noSwap` global.
- Veredito: refina `docs/design/07` §7 — item novo obrigatório (ver §5, 7.8).

### A8. `hx-status:NNN` é uma DSL de string livre com 6 campos
- Citação: `07-error-handling.md:114-120` — "O formato `hx-status:XXX` aceita: - `swap:` — estilo de swap - `target:` — seletor CSS […] - `select:` […] - `push:` […] - `replace:` […] - `transition:` — usar View Transition".
- Semântica: um atributo por status (`hx-status:200`, `:422`, `:500`, `:503`, e a forma curinga `hx-status:5xx` em `07:49`), com valor multi-campo separado por espaço.
- Consequência: `target:#form-errors` é seletor CSS legado em texto; `transition:` é animação por decisão de markup. Nenhum dos dois cabe no vocabulário. Versão tipada deve aceitar só `swap` + `target`, ambos enums, e o wildcard `5xx`.
- Veredito: refina — adotar o **conceito**, recusar a gramática.

### A9. `HX-Retarget` com fragmento que repete o `id` cria `id` duplicado
- Citação: `07-error-handling.md:36` — `[("HX-Retarget", "#form-errors")]`, com corpo em `07:31`: `r#"<div id="form-errors" class="alert alert-error">…"#`.
- Semântica: o mesmo `id` é o alvo do swap (no `form`) e a raiz do fragmento devolvido; com swap `innerHTML` no alvo, o resultado no DOM tem `#form-errors` dentro de `#form-errors`.
- Consequência: viola "`id` único global" (N4) e quebra `aria-describedby`/`aria-labelledby` que apontem para ele. Regra necessária: fragmento devolvido para retarget **não repete** o `id` do alvo.
- Veredito: conflito real — a crate deve emitir o fragmento sem `id` (ou com swap `outerHTML`). Ver §4.

### A10. `HX-Reswap` e `HX-Reselect` dão ao servidor o controle do swap
- Citação: `07-error-handling.md:158-161` — `[("HX-Reselect", "#task-content")],` / `Html(full_page_html),  // Só #task-content será extraído`.
- Semântica: a resposta pode sobrescrever swap (`HX-Reswap`, `07:144-148`) e recortar o próprio corpo (`HX-Reselect`).
- Consequência: `HX-Reselect` permite responder **página inteira** e o cliente usar um pedaço — de novo o caminho sem `Vary`. Mas é string de seletor: precisa de tipo (`TargetId`), não de `&str`.
- Veredito: confirma A6 por outro caminho; adotar como `RespondWith::Reselect(TargetId)`.

### A11. Os três headers de controle de navegação são `HX-Redirect`, `HX-Refresh`, `HX-Push-Url`
- Citação: `09-rust-axum-patterns.md:52` `[("HX-Redirect", path)]`, `09:60` `[("HX-Refresh", "true")]`, `09:69` `("HX-Push-Url", url)`.
- Semântica: redirect (carga completa), refresh (recarrega a página atual) e push de URL são comandos de resposta, cada um com corpo vazio (`Html("")`).
- Consequência: são `LUrl`, não `&str` — e `HX-Refresh` deve ser raro o suficiente para virar advisory (perde scroll, foco e estado de formulário). O crate precisa de um tipo de resposta que carregue esses comandos junto do status.
- Veredito: confirma D11 (`Page` carrega status/cabeçalhos) e exige extensão dele.

### A12. `HX-Target` usado como roteador faz a resposta variar por header
- Citação: `09-rust-axum-patterns.md:116` — "Use o header `HX-Target` para comportamentos condicionais", com `09:127-131` decidindo o handler por `match target { "task-tree" => …, "task-detail" => … }`.
- Semântica: a URL é a mesma e o corpo muda conforme o alvo declarado pelo cliente.
- Consequência: qualquer cache intermediário (CDN, proxy, até o cache do navegador) serve o fragmento errado. Exige `Vary: HX-Target` **além** de `Vary: HX-Request`, ou — preferível — proibir o roteamento por `HX-Target` e usar uma URL por região.
- Veredito: conflito — `docs/design/07` §7 fala só de `Vary: HX-Request`; a matriz completa está em §5 (7.4).

### A13. `HX-Request` aparece como header de resposta — e `HX-Request-Type` desaparece
- Citação: `09-rust-axum-patterns.md:46` — `(status, [("HX-Request", "true")], Html(body)).into_response()`.
- Semântica: `02-setup.md:125` define `HX-Request` como header **enviado pelo cliente** ("Identificar que é uma requisição htmx"); `09:138-142` o lê como entrada. O helper o devolve como saída.
- Consequência: dois papéis para o mesmo nome. Contrato necessário: headers `HX-*` de requisição são somente leitura; resposta só usa o conjunto fechado de comandos. `HX-Request-Type` (`02:126`: "`partial` para swaps, `full` para body-level") tem uma única menção em todo o corpus e nenhum uso — não é base para decisão.
- Veredito: refuta o helper; confirma que a lista de headers de resposta precisa ser fechada e tipada.

### A14. `HX-Trigger` nunca é nomeado, mas o mecanismo é usado
- Citação: `SKILL.md:38` — `<aside id="sidebar" hx-get="/sidebar" hx-trigger="load, taskChanged from:body" hx-swap="innerHTML">`.
- Semântica: `taskChanged` é evento customizado escutado em `body`; alguém precisa dispará-lo — é exatamente o que o header `HX-Trigger` faz. O conjunto não menciona `HX-Trigger` em nenhum dos 12 arquivos (grep).
- Consequência: o padrão "uma ação atualiza N regiões por evento" fica sem origem declarada. Nós **não** precisamos dele: OOB (`hx-swap-oob`) resolve o mesmo caso e mantém a resposta determinística e verificável por `ETag`; evento indireto esconde o que foi trocado.
- Veredito: lacuna + decisão: recusar `HX-Trigger` na v1, registrar o motivo.

### A15. Timeout: 60 s padrão, 30 s recomendado, `0` restaurável
- Citação: `01-introduction.md:45` — "| Timeout padrão | 0 (infinito) | 60000ms |"; `02-setup.md:61` — "| `defaultTimeout` | `30000` | 30s é suficiente para consultas SQL no SQLite |"; `10-migration-from-v2.md:251` — "// Restaura timeout infinito" com `htmx.config.defaultTimeout = 0`.
- Semântica: o conjunto não decide um número; oferece três. `10:156` mostra override por elemento (`hx-config="timeout:120000"`).
- Consequência: timeout é política de servidor (o handler já tem deadline); deixar o cliente escolher cria duas políticas. Fixar um valor, proibir `0` e proibir override por elemento.
- Veredito: refina — número é nosso, não deles.

### A16. Configuração por `<meta name="htmx-config">` **ou** por `<script>` inline
- Citação: `02-setup.md:52-54` — `<meta name="htmx-config" content=' {"defaultTimeout":30000,"defaultSwap":"innerHTML","history":true,"transitions":true} '>`; e `10-migration-from-v2.md:243-253`, que configura por `<script>` inline: `htmx.config.implicitInheritance = true;`.
- Semântica: dois canais, um declarativo e um imperativo; o `<script>` precisa vir **antes** do script do htmx (`10:253`).
- Consequência: o canal `<script>` é inline — impossível sob `script-src 'self'` sem `nonce` (que `docs/design/07` §5 proíbe). Só `<meta>` sobrevive, e ele precisa estar no `<head>` antes do `<script src>` → ordem de `<head>` passa a ser contrato do shell.
- Veredito: confirma CSP e restringe: `<meta>` + JSON apenas. A forma multi-linha "key:value" de `02:41-46` não é JSON e não deve ser adotada.

### A17. `hx-trigger` com expressão de filtro exige avaliação dinâmica
- Citação: `08-optimizations.md:182` — `hx-trigger="input delay:500ms, keyup[key=='Enter']"`, explicado em `08:188`: "`keyup[key=='Enter']` — também dispara ao pressionar Enter".
- Semântica: `[expr]` é expressão avaliada pelo cliente sobre o evento.
- Consequência: se for avaliada por `eval`/`new Function`, colide de frente com `docs/design/08` §6 ("Sem `unsafe-eval`: nenhum caminho no vocabulário exige avaliação dinâmica") e com Trusted Types. Sob a nossa política, é proibido; Enter resolve-se com `<form>` + submit nativo.
- Veredito: conflito direto — proibido. Ver §4.

### A18. Estado de carregamento é classe CSS + `hx-indicator` + desabilitar o gatilho
- Citação: `08-optimizations.md:5` — "O htmx 4 mantém o sistema de classes CSS `htmx-request` e `htmx-indicator`"; e `08:37`, `hx-disable="this"` para "desabilitar elementos durante a requisição" (`08:31`).
- Semântica: (a) `.htmx-request` é adicionada ao elemento que pediu; (b) `hx-indicator` aponta o elemento que ganha a classe; (c) `hx-disable="this"` marca o gatilho como desabilitado.
- Consequência: (a) e (b) são CSS — cabem no bundle com hash, sem inline. (c) **remove o foco** do botão que acabou de ser ativado se ele estiver em foco (2.4.7/2.4.3), e é a armadilha clássica de "botão que sumiu do tab order". O correto é `aria-busy` na região + `hx-sync="this:drop"`.
- Veredito: confirma a classe, refuta `hx-disable`.

### A19. `hx-disable` mudou de significado entre v2 e v4, com ordem de renomeação obrigatória
- Citação: `10-migration-from-v2.md:27-28` — "| `hx-disable` | `hx-ignore` | Renomear **primeiro** |" e "| `hx-disabled-elt` | `hx-disable` | Renomear **depois** |".
- Semântica: na v2 `hx-disable` significa "não processe esta subárvore"; na v4 significa "desabilite este elemento durante a requisição". O mesmo nome, semântica oposta, e a ordem de renomeação existe para não colidir.
- Consequência: template copiado de v2 "desabilita processamento" e passa a "desabilitar botão" silenciosamente. Dois atributos para a mesma palavra é risco puro: o vocabulário não deve expor nenhum dos dois.
- Veredito: refuta ambos; registra a colisão como motivo.

### A20. Herança implícita removida — `:inherited` explícito, e um flag para voltar atrás
- Citação: `03-three-column-layout.md:206` — "No htmx 4, atributos não herdam automaticamente. Use `:inherited`"; e `10-migration-from-v2.md:89`, `htmx.config.implicitInheritance = true;`.
- Semântica: herança de `hx-target`/`hx-swap` de um container para os filhos só acontece se declarada; o flag restaura o comportamento v2.
- Consequência: `:inherited` é estado ambiente explícito no markup — aceitável, porque é visível e local. O flag compatível é o oposto: muda o significado de todo markup existente por configuração. Proibido.
- Veredito: confirma P6/P9 (nada de estado ambiente implícito) e D3 (nenhum caminho de configuração abre porta).

### A21. Lazy loading por `hx-trigger="load"` deixa a página vazia sem JS
- Citação: `03-three-column-layout.md:202` — "Isso significa que o layout inicial é leve e cada seção carrega de forma independente", sobre seis regiões com `hx-trigger="load"` (`03:13-15`, `03:23-25`, `03:33-39`, `03:194-199`).
- Semântica: o HTML inicial das regiões é vazio ou um spinner; o conteúdo só existe depois da requisição.
- Consequência: contradiz o requisito "tudo funciona sem JavaScript". A primeira pintura precisa do conteúdo real; `load` só pode servir a região **secundária** e sempre com `<noscript>` com o mesmo conteúdo (ou um link para a rota completa).
- Veredito: conflito com o requisito do projeto — regra nova obrigatória (7.11).

### A22. `hx-push-url` em formulário de login (POST) junto com `HX-Redirect`
- Citação: `06-forms.md:11-14` — `<form method="POST" action="/login" hx-post="/login" hx-target="body" hx-swap="innerHTML" hx-push-url="true">`, com a resposta em `06:47` sendo `("HX-Redirect", …)`.
- Semântica: o POST empurra sua própria URL para o histórico; logo depois a resposta manda recarregar.
- Consequência: o histórico guarda uma entrada de POST; "voltar" reenvia credenciais (e, em qualquer app real, o token CSRF já consumido). Regra: `hx-push-url` só em GET, e nunca junto de `HX-Redirect`.
- Veredito: conflito real. Correção em §4.

### A23. Histórico: sem `localStorage`, então fragmento tem de ser reproduzível pela URL
- Citação: `10-migration-from-v2.md:207` — "O htmx 4 não usa mais localStorage para cache de histórico."; escapatória em `10:216`, `htmx.config.history = 'reload';`.
- Semântica: voltar/avançar refaz a requisição (ou usa o cache HTTP) em vez de restaurar um snapshot na memória do cliente.
- Consequência: qualquer fragmento com valor não derivável da URL (token, "agora", id aleatório, ordem de sorteio) produz histórico inconsistente e `ETag` diferente a cada visita. Reforça o determinismo que já é P6 — agora com uma segunda razão.
- Veredito: confirma P6 por motivo novo.

### A24. Morph swaps: um segundo algoritmo de DOM, com dois atributos de escape
- Citação: `08-optimizations.md:44` — "Para preservar estado de elementos durante swaps (como foco, scroll, vídeos), use morph swaps", com `hx-swap="innerMorph"`/`outerMorph` (`08:51`, `08:58`), `hx-preserve` (`08:69`) e `hx-morph-skip` (`08:79`).
- Semântica: o cliente reconcilia nós em vez de substituir; `hx-preserve` congela subárvore, `hx-morph-skip` exclui subárvore do morph.
- Consequência: é reativação por outra porta — o DOM deixa de ser projeção do último render, e o servidor perde a capacidade de garantir o que está na tela. Custa um algoritmo a mais no bundle (o argumento que `docs/design/07` §4 usa para tudo). Foco preservado é bom; mas isso se resolve com foco explícito pós-swap, não com reconcile.
- Veredito: recusar. `innerHTML`/`outerHTML` apenas.

### A25. Polling sem controle de pausa
- Citação: `08-optimizations.md:89-93` — `<div id="task-tree" hx-get="…" hx-trigger="every 30s" hx-swap="innerMorph">`; e o padrão auto-substituinte em `08:98-108`.
- Semântica: atualização periódica automática, sem interface de parada; "Para parar, retorne um div sem trigger" (`08:108`) — ou seja, a parada é decisão do servidor.
- Consequência: WCAG 2.2.2 (Pausar, Parar, Ocultar) exige controle para conteúdo que se atualiza sozinho; `docs/design/05` já mapeia 2.2.2 a "**tipo**: `Carousel` exige controle de pausa". Polling entra na mesma família e precisa do mesmo controle — e `hx-sync` para não empilhar requisições.
- Veredito: lacuna de a11y do corpus; regra nova.

### A26. `hx-sync` é o antídoto para disparo duplo
- Citação: `08-optimizations.md:208-210` — "| `drop` | Ignora novo evento se requisição ativa | / | `replace` | Cancela requisição ativa, inicia nova | / | `queue` | Enfileira (com `first`/`last`/`all`) |".
- Semântica: três políticas de concorrência por escopo (`this`, `closest div` — `08:194`, `08:199`).
- Consequência: com `hx-trigger="change"` e `blur` no mesmo campo (o corpus usa os dois: `04:168`, `04:208`), duas requisições disputam o mesmo alvo e a resposta mais lenta vence. `drop` é o default correto para escrita; `queue` nunca.
- Veredito: adotar como enum fechado (`Sync::{Drop, Replace}`), sem `queue`.

### A27. `hx-boost` + `hx-select` = mesma resposta para os dois clientes
- Citação: `08-optimizations.md:173` — "`hx-boost` faz com que o link use AJAX. `hx-select="#layout-grid"` extrai só o grid, mantendo scripts e estilos."
- Semântica: o link aponta para a rota normal; o servidor responde a página inteira; o cliente recorta a região que quer.
- Consequência: é o **único** padrão do corpus em que navegador e htmx recebem bytes idênticos — logo `ETag` único, cache honesto e `Vary` desnecessário. Deve ser o modo padrão de navegação, com resposta parcial por URL dedicada (não por header).
- Veredito: confirma `Render::Full` e reduz a necessidade de `Partial`.

### A28. `hx-trigger="change"` provoca mudança de contexto por entrada (WCAG 3.2.2)
- Citação: `06-forms.md:217` — "| `change` | Input, select, checkbox — dispara ao perder foco com valor alterado |", usado em `04:168-169` (`change` + `hx-swap="none"`) e `04:180-182` (`change` + troca de `#task-tree`).
- Semântica: alterar um `select` envia POST e troca região, sem confirmação.
- Consequência: `docs/design/05` afirma que 3.2.1/3.2.2 são garantidos **estruturalmente** ("não existe `on*`; mudança só por submit"). Aceitar `change` como gatilho de swap derruba essa garantia. O `hx-swap="none"` mitiga o caso de texto, mas não o de `select` que troca outra região.
- Veredito: conflito — gatilho de escrita permitido só em `submit`; `change` só com `swap: none` e feedback por região de status.

### A29. `hx-on:*` inline é usado em 8 lugares, contra a própria regra do conjunto
- Citação: `SKILL.md:31` — "Avoid `hx-on` inline with strict CSP; use delegated listeners." — enquanto `04-task-crud.md:47` traz `hx-on:htmx:after:swap="document.getElementById('task-modal')?.close()"`.
- Semântica: o conjunto prega uma regra e a viola em todo exemplo não trivial (também `03:71`, `04:14`, `04:170`, `04:210`, `04:281`, `06:73`, `06:194`, `06:207`).
- Consequência: nenhum desses exemplos é utilizável por nós. Cada um tem substituto no servidor: abrir modal = `<dialog>` nativo, fechar = `<form method="dialog">`, "salvo" = região `role="status"` trocada por OOB, limpar detalhe = `<hx-partial>`→OOB vazio.
- Veredito: refuta; reforça `docs/design/07` §7 item 6 e a lista de proibições (§5).

### A30. `hx-confirm`/`hx-prompt` são `window.confirm`/`window.prompt`
- Citação: `04-task-crud.md:280` — `hx-confirm="Tem certeza que deseja arquivar esta tarefa?"`; e `01-introduction.md:48` — "| Atributos removidos | `hx-vars`, `hx-params`, `hx-prompt` | Usar alternativas |".
- Semântica: diálogo nativo do navegador, texto do servidor, sem controle de foco, sem `Esc` garantido, sem estilo, sem tradução por `Msg`.
- Consequência: o projeto tem primitivo `Dialog` (com `aria-labelledby`, foco inicial, botão de fechar) justamente para isso. `hx-confirm` seria uma segunda forma de confirmar, pior. Proibido.
- Veredito: refuta; `Dialog` já cobre (D10).

### A31. `hx-oob` não existe como atributo — nem no próprio conjunto
- Citação: `SKILL.md:18` — "`hx-get`/`hx-post`/`hx-target`/`hx-swap`/`hx-trigger`/`hx-oob` patterns"; o uso real em `05:111` é `hx-swap-oob="true"`.
- Semântica: o resumo lista um nome que nenhum outro arquivo usa.
- Consequência: se o vocabulário fosse gerado a partir do `SKILL.md`, nasceria com um atributo morto. Evidência de que a allowlist precisa ser conferida contra uso, não contra resumo.
- Veredito: irrelevante tecnicamente, útil como aviso de método.

### A32. `hx-validate` duplica validação que já é nossa
- Citação: `06-forms.md:226` — "No htmx 4, o atributo `hx-validate` controla a validação nativa do browser", com `06:230`, `hx-validate="true"`.
- Semântica: liga/desliga constraint validation por formulário.
- Consequência: nossa validação de cliente já vem de `required`/`pattern`/`minlength` emitidos pelos primitivos, e `docs/design/18` (achado do `html-moderno`) **proíbe `novalidate`**. Um atributo que desliga validação é a porta de trás. Proibido.
- Veredito: refuta.

### A33. `hx-delete` não envia o formulário; `hx-include="closest form"` é seletor em texto
- Citação: `06-forms.md:245` — "No htmx 4, `hx-delete` **não** inclui dados do formulário por padrão (assim como `hx-get`)."; solução em `06:253`, `hx-include="closest form"`.
- Semântica: métodos sem corpo precisam de inclusão explícita de campos.
- Consequência: `hx-include` recebe seletor CSS — `closest form` é sintaxe de htmx, não CSS. Precisa de enum (`Include::ClosestForm`), e a lista de campos enviados deve continuar sendo a do `<form>` (nunca campos arbitrários escolhidos no cliente).
- Veredito: refina — tipo, não string.

### A34. ETag manual + `304` sem swap + `Vary` ausente = região que nunca preenche
- Citação: `08-optimizations.md:225-236` — `if headers.get("if-none-match")… == Some(&etag) { return StatusCode::NOT_MODIFIED.into_response(); }`, com `ETag` gerado de `get_mtime` (`08:223`).
- Semântica: o servidor compara o `If-None-Match` à mão e devolve `304` sem corpo; e, por A7, `304` **não troca** o DOM.
- Consequência: (a) o cliente só envia `If-None-Match` se tiver recebido o `ETag` antes — primeira carga com `304` deixa a região vazia para sempre; (b) `ETag` de `mtime` não é o `sha256` dos bytes (contraria `docs/design/09` §2) e muda sem que o HTML mude; (c) sem `Vary: HX-Request`, 304/200 de fragmento e de página se contaminam no cache.
- Veredito: conflito — Adotar `ETag` = `sha256` canônico; `304` só como revalidação de um `200` anterior idêntico.

## 3. Confirmações do desenho

1. **HTML-over-the-wire é o nosso modelo** (`01:23`: "O servidor envia **HTML completo** (não JSON)") — nenhuma das 2.380 linhas pede que o cliente renderize; confirma D11 e o "HTML é o artefato" (P1).
2. **Zero estado no cliente** (`01:63`: "**Zero estado no cliente** (toda lógica está no servidor)") — mesmo que o corpus, na prática, use JS inline para feedback; o princípio confirma D13.
3. **Resposta parcial é fragmento do mesmo vocabulário** (`SKILL.md:25`: "Server returns **HTML fragments**, not JSON") — logo o fragmento passa pelo mesmo `Markup`/macro, e a garantia de escape vale para os dois caminhos.
4. **`hx-swap-oob` precisa de `id`** (A5) confirma N4 (id único global) como pré-requisito funcional, não só de a11y.
5. **Ordem main→OOB é determinística** (A4) — confirma P6/D14 e permite asserção de e2e sobre ordem.
6. **`hx-select-oob`/`HX-Reselect` permitem corpo único** (A6, A10) — confirma `Render::Full` como caminho principal e reduz `Vary` a um caso específico.
7. **`:inherited` explícito** (A20) valida P9: nenhum estado ambiente invisível no markup.
8. **Classe `htmx-request` em vez de `style`** (A18) mantém `style-src 'self'` intacto — confirma `docs/design/07` §5.
9. **O corpus nunca escreve `Vary`** — confirma que `docs/design/07` §7 item 3 é adição nossa e que ela é indispensável (A12, A34).
10. **O corpus quase não trata a11y** (1 menção em 2.380 linhas) — confirma que foco/anúncio pós-swap não vêm de graça do htmx e precisam ser contrato (7.6).

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata + arquivo) |
|---|---|---|
| `docs/design/07` §7: "O vocabulário **não** inclui htmx" e nada sobre como um `hx-*` é emitido | todo uso é `hx-*` em elemento comum (`03:13`, `04:44`) | inserir em `docs/design/03-vocabulario-de-elementos.md` §7: "Perfil `htmx` (opcional): `spec/htmx.toml` declara os atributos `hx-*` permitidos, com valor tipado; nenhum `hx-*` fora dessa tabela compila." |
| `docs/design/03` §5 proíbe `hx-on*` sem dizer o que substitui | `hx-on:*` em 8 exemplos (A29) | acrescentar em `docs/design/07` §7.10: "`hx-on` não é substituído por nada: cada uso do corpus tem equivalente nativo (`<dialog>`, `<form method=\"dialog\">`, região `role=\"status\"` por OOB)." |
| `docs/design/07` §7 item 3: "`Vary: HX-Request` **obrigatório**" | resposta varia por `HX-Target` (`09:116-131`) | trocar por: "`Vary: HX-Request` em toda resposta que difere entre parcial e completa; `Vary: HX-Request, HX-Target` quando o handler lê `HX-Target`; resposta única (`hx-select`/`hx-select-oob`) dispensa `Vary`." |
| `docs/design/07` §7 item 4: "`aria-live=\"polite\"` e `aria-busy` durante a troca" | nenhuma menção; único `focus()` em `03:71` | manter e completar: "`aria-busy=\"true\"` na região durante a requisição (removido pelo swap) e foco movido para o título da região em `htmx:after:settle`; `aria-live=\"polite\"` só na região de status." |
| `docs/design/05`: "3.2.1/3.2.2 — não existe `on*`; mudança só por submit" | `hx-trigger="change"` troca região (`04:180-182`) | adicionar a `docs/design/05` §2, linha 3.2.2: "gatilho htmx de escrita só em `submit`; `change`/`blur` apenas com `swap: none` e feedback em região `role=\"status\"`." |
| `docs/design/08` §6: "Sem `unsafe-eval`: nenhum caminho no vocabulário exige avaliação dinâmica" | `hx-trigger="keyup[key=='Enter']"` (`08:182`) | acrescentar a `docs/design/08` §6: "filtros `[expr]` de `hx-trigger` e valores `js:` são proibidos: exigem avaliação dinâmica; Enter vem de `<form>` + submit." |
| `docs/design/09` §2: `ETag` = `sha256` dos bytes finais | `ETag` de `mtime` e `304` manual (`08:223-229`) | acrescentar a `docs/design/09` §2: "`304` só após um `200` com o mesmo `ETag` canônico; fragmento nunca responde `304` na primeira carga." |
| `docs/design/06` §1: `Page` é o único construtor de resposta HTML | fragmentos crus em `Html(format!(…))` (`05:79`, `09:87`) | acrescentar a `docs/design/06` §1: "resposta parcial é `Partial` (mesmo `Markup`, mesmo macro), nunca `String`; `Page` e `Partial` são os dois únicos `IntoResponse`." |
| `docs/design/12` §9: "`axe` **depois** do swap" | nada equivalente | manter; detalhar em `docs/design/12` §9 com as asserções de §5 (checklist) — hoje é uma linha, precisa ser gate nomeado. |
| `docs/design/07` §5: "`<script type=\"application/ld+json\">`" permitido | config só por `<script>` (`10:243-253`) | acrescentar a `docs/design/07` §7.12: "configuração exclusivamente por `<meta name=\"htmx-config\">` em JSON, no `<head>`, antes do `<script src>`; `htmx.config.*` por script inline é proibido." |

## 5. Adições prontas para colar

### 5.1 Bloco `7.1`–`7.12` para `docs/design/07-js-e-assets.md`

```markdown
## 7. htmx (opcional, e como adotar)

htmx **não** faz parte do núcleo. É um perfil: sem `data-hx-offer`, nenhum byte de htmx é emitido e
nenhum `hx-*` compila. As doze regras abaixo são o contrato completo; qualquer uso fora delas é bug.

**7.1 Declaração.** O perfil é ligado por `data-hx-offer` no elemento raiz da região interativa. O shell
só emite `<script src>` de `Asset::Htmx` (vendorizado, hash de conteúdo, `defer`) quando esse marcador
existe; sem ele, zero JS de htmx e nenhum `hx-*` no HTML.

**7.2 Atributos.** Allowlist fechada em `spec/htmx.toml`, com valor tipado (ver 5.2 deste documento).
Atributo `hx-*` fora da tabela **não compila**. Proibidos por decisão: `hx-on`, `hx-on:*`,
`hx-vals`, `hx-vars`, `hx-params`, `hx-prompt`, `hx-ext`, `hx-confirm`, `hx-disable`,
`hx-disabled-elt`, `hx-ignore`, `hx-validate`, `hx-preserve`, `hx-morph-skip`.

**7.3 Escrita.** O verbo vem do tipo (`Hx::post(LUrl)`, `Hx::get(LUrl)`), nunca de string: URL é `LUrl`
(com locale), não `&str`. GET/HEAD só leem; POST/PUT/PATCH/DELETE exigem token CSRF do primitivo `Form`.

**7.4 Resposta e `Vary`.** Três formas, nesta ordem de preferência:
(a) **corpo único** — mesma resposta para navegador e htmx (`hx-boost` + `hx-select`, ou `HX-Reselect`);
`Vary` desnecessário, `ETag` único;
(b) **parcial por URL dedicada** — rota própria, `Vary: HX-Request`;
(c) **parcial na mesma URL** — `Vary: HX-Request`; e `Vary: HX-Request, HX-Target` se o handler lê
`HX-Target`. Nunca rotear comportamento por `HX-Current-URL` nem por `HX-Source`.

**7.5 Cache.** `ETag` = `sha256` dos bytes do fragmento (P6/D14). `304` só em revalidação de um `200`
idêntico já entregue; `304` não troca o DOM. `Cache-Control: private, no-cache` para parcial
autenticado; `public, no-cache` para parcial público. Fragmento nunca é `immutable`.

**7.6 Swap, foco e anúncio.** Default `innerHTML`; `outerHTML` quando o próprio nó é o item. Ordem
garantida: alvo principal primeiro, OOB depois, em ordem de documento. Em `htmx:after:settle`, o asset
move o foco para o primeiro heading da região trocada quando o gatilho foi teclado, e a região de
status (`role="status"`, `aria-live="polite"`) recebe uma frase emitida pelo servidor. Sem `hx-on`:
quem escreve a frase é o Rust.

**7.7 Carregamento.** `aria-busy="true"` na região enquanto a requisição corre (o swap o remove).
Indicador é `.htmx-request`/`.htmx-indicator` no CSS do bundle — nunca `style`. O gatilho não é
desabilitado (`hx-disable` proibido): disparo duplo é contido por `hx-sync="this:drop"`.

**7.8 Erros.** Padrão do htmx 4: 4xx e 5xx **trocam**; 204 e 304 não. Matriz obrigatória por região:
`422` → região de erro do formulário (`HX-Retarget` com tipo, fragmento **sem** repetir o `id` do
alvo); `404` → mensagem no próprio alvo; `5xx` → `swap: none` + OOB para a região de status
(nunca apagar a região de conteúdo); `204` → nenhum swap. `noSwap` global é proibido: ele tira do
servidor a decisão.

**7.9 Multi-target.** Só OOB (`hx-swap-oob`), endereçado por `RegionId` gerado pelo shell.
`<hx-partial>` é proibido: é elemento fora de `spec/elements.toml` e exigiria o caminho de `Raw`.
Resposta só-OOB não toca o alvo principal (equivalente a `MainSwap::Keep`).

**7.10 Proibições.** `hx-on`/`hx-on:*`; `hx-vals='js:…'` e qualquer filtro `js:`; filtros `[expr]` em
`hx-trigger`; `hx-confirm`/`hx-prompt`; `innerMorph`/`outerMorph` e `hx-preserve`/`hx-morph-skip`;
`hx-trigger="every Ns"` sem controle de pausa visível; `transition:`/`transitions:true`;
`implicitInheritance`; `noSwap`; `defaultTimeout = 0`.

**7.11 Sem JavaScript.** A primeira pintura contém o conteúdo real de todas as regiões de conteúdo.
`hx-trigger="load"` só é permitido em região cuja ausência de JS tenha `<noscript>` com o mesmo
conteúdo (ou link para a rota completa). Nenhuma rota existe apenas como parcial: a URL sempre
responde completa quando não há `HX-Request`.

**7.12 Configuração.** Exclusivamente `<meta name="htmx-config">` com **JSON**, no `<head>`, antes do
`<script src>`. Valores fixos: `defaultTimeout: 10000`, `defaultSwap: "innerHTML"`,
`history: true`, `transitions: false`. `htmx.config.*` por script inline é proibido (CSP).
```

### 5.2 Tabela "atributo htmx → permitido? → por quê"

| atributo / header | permitido? | por quê |
|---|---|---|
| `hx-get` `hx-post` `hx-put` `hx-patch` `hx-delete` | **sim** (`Hx::verbo(LUrl)`) | verbo+URL tipados; a URL nunca é `&str` |
| `hx-target` | **sim** (`RegionId`) | id de região gerado pelo shell; sem seletor livre |
| `hx-swap` | **sim** (`Swap::{Inner, Outer, None}`) | morph/`transition:`/`swapEmpty` excluídos |
| `hx-trigger` | **parcial** (`Trigger::{Submit, Click, Changed, Revealed, Delay, Every(Pausable)}`) | sem filtros `[expr]`; `every` exige controle de pausa |
| `hx-select` `hx-select-oob` | **sim** (`RegionId`/`Vec<RegionId>`) | corpo único → sem `Vary` (7.4a) |
| `hx-swap-oob` | **sim, só na resposta** (`Oob` emitido por tipo) | elemento do vocabulário; nunca escrito à mão |
| `hx-push-url` `hx-replace-url` | **sim, só em GET** | POST no histórico reenvia corpo ao voltar (A22) |
| `hx-boost` | **sim** | enhancement de link/form; funciona sem JS |
| `hx-indicator` | **sim** (`RegionId`) | alvo da classe `htmx-request` |
| `hx-sync` | **sim** (`Sync::{Drop, Replace}`) | `queue` nunca: resposta fora de ordem |
| `hx-include` | **parcial** (`Include::ClosestForm`) | campos do `<form>`, nunca seleção arbitrária |
| `hx-status:NNN` / `hx-status:5xx` | **sim, restrito** (`swap` + `target`) | `push/replace/select/transition` fora |
| `hx-disable` | **não** | remove o foco do gatilho (2.4.7) e colide com o nome da v2 |
| `hx-disabled-elt` | **não** | renomeado na v4; armadilha de cópia |
| `hx-ignore` | **não** | desligar processamento em markup gerado é sintoma, não solução |
| `hx-on` / `hx-on:*` | **não** | JS inline; CSP `script-src 'self'` |
| `hx-vals` / `hx-vals="js:…"` | **não** | `js:` exige avaliação dinâmica |
| `hx-vars` / `hx-params` / `hx-prompt` | **não** | removidos na v4 |
| `hx-confirm` | **não** | `window.confirm`; o primitivo `Dialog` é acessível |
| `hx-ext` | **não** | removido; extensão é `Asset` declarado |
| `hx-validate` | **não** | duplica (e permite desligar) a validação nativa que já emitimos |
| `hx-preserve` / `hx-morph-skip` | **não** | só fazem sentido com morph, que é reativação |
| `hx-encoding` | **não** | multipart só pelo primitivo de upload |
| `hx-oob` | **não existe** | nome citado em `SKILL.md:18`; o real é `hx-swap-oob` |
| `HX-Request` (req.) | **leitura** | único sinal de "quero parcial"; dispara `Vary` |
| `HX-Request-Type` | **leitura opcional** | uma só menção no corpus; não é base de decisão |
| `HX-Target` | **leitura proibida como roteador** | faria a resposta variar por header (A12) |
| `HX-Source` `HX-Current-URL` `HX-Boosted` | **leitura proibida** | influenciáveis pelo cliente; nunca em decisão de conteúdo |
| `HX-Retarget` `HX-Reswap` `HX-Reselect` | **sim** (`Retarget(RegionId)`, `Reswap(Swap)`, `Reselect(RegionId)`) | comando de resposta tipado |
| `HX-Redirect` `HX-Refresh` `HX-Push-Url` | **sim** (`LUrl`) | `HX-Refresh` gera advisory (perde foco/scroll) |
| `HX-Trigger` | **não na v1** | OOB já expressa o mesmo e é verificável por `ETag` (A14) |

### 5.3 Checklist e2e/axe (gate `htmx`, ≥ 8 asserções)

1. **Sem JS**: com `javaScriptEnabled: false`, cada página entrega o conteúdo real de todas as regiões de conteúdo (comparar com a árvore textual do render com JS) — nenhuma região vazia.
2. **`Vary`**: toda resposta parcial tem `Vary` contendo `HX-Request`; com `HX-Target` lido pelo handler, contém também `HX-Target`.
3. **`ETag`/`304`**: `GET` do parcial → `200` + `ETag`; repetir com `If-None-Match` → `304` e **DOM inalterado**; bytes do `200` são idênticos entre duas execuções (determinismo).
4. **Swap em erro**: `422` troca só a região de erro; `500` **não** apaga a região de conteúdo e publica uma mensagem na região de status; `204` não altera o DOM.
5. **Ordem main→OOB**: instrumentar `htmx:after:settle` e verificar que o alvo principal é trocado antes de cada OOB, na ordem do documento.
6. **Foco pós-swap**: acionar o gatilho por teclado e verificar `document.activeElement` dentro da região trocada (primeiro heading), com `:focus-visible` visível.
7. **`aria-busy`**: durante a requisição a região tem `aria-busy="true"`; depois do swap, não tem.
8. **Anúncio**: a região `role="status"` (`aria-live="polite"`) recebe exatamente uma mensagem por ação — e o texto vem do render, não de JS que monta frase.
9. **`axe` depois do swap**: `axe.run()` com 0 violações A/AA **em cada estado** (antes, durante e depois de cada swap), não só na carga inicial.
10. **Nada de inline após o swap**: nenhum nó inserido tem atributo `style`, `<script>` sem `src` ou `hx-on*` (varredura do DOM após 3 swaps encadeados).
11. **Sem requisição duplicada**: alterar um campo com `change` e imediatamente com `blur` produz **uma** requisição (`hx-sync="this:drop"`).
12. **Histórico**: navegar, voltar e avançar não gera requisição POST nem região vazia; o `ETag` da região restaurada é o mesmo.
13. **Sem JS obsoleto**: nenhuma chamada de rede a domínio externo (allowlist) e nenhum `eval`/`new Function` no console (CSP `script-src 'self'` sem `unsafe-eval`).
14. **Polling**: toda região com `hx-trigger="every …"` tem controle visível de pausa, e pausar interrompe as requisições (contagem de requests para de crescer).

## 6. Armadilhas de complexidade

1. **`<hx-partial>`** — o que é: elemento novo para multi-target. Por que não entra: nome fora de `spec/elements.toml`; para emiti-lo seria preciso um caminho equivalente a `Raw`, e o `MainSwap` viraria string (`swapEmpty:true`). Custo se entrar: um elemento fantasma no vocabulário, uma exceção no validador e a reabertura de D2 por um nome novo.
2. **Morph (`innerMorph`/`outerMorph` + `hx-preserve`/`hx-morph-skip`)** — o que é: reconciliação de DOM no cliente para preservar foco/scroll/mídia. Por que não entra: o DOM deixa de ser projeção do último render; são ~1 algoritmo e 2 atributos a mais no bundle, e o estado preservado pode divergir do servidor. Custo se entrar: perda da garantia "o que está na tela é o que o Rust emitiu", mais superfície de bug de a11y (nó antigo mantido com `aria-*` velho).
3. **`hx-status:` como DSL de string** — o que é: 6 campos em texto (`swap:`, `target:`, `select:`, `push:`, `replace:`, `transition:`). Por que não entra: `target:` é seletor CSS e `transition:` é decisão de animação em markup. Custo se entrar: um parser de string no consumidor e uma allowlist que aceita "quase qualquer coisa" — o oposto de D4.
4. **`hx-trigger` com filtros `[expr]`** — o que é: expressão avaliada no cliente (`keyup[key=='Enter']`). Por que não entra: exige avaliação dinâmica, colidindo com `script-src 'self'`/Trusted Types. Custo se entrar: `unsafe-eval` na CSP ou um interpretador de expressões — em troca de uma tecla que o `<form>` já entrega.
5. **`hx-on:*`** — o que é: handler inline por evento. Por que não entra: é `on*` com outro nome. Custo se entrar: `unsafe-inline` na CSP, `ETag` variável por markup e o fim de "tudo funciona sem JS" como afirmação testável.
6. **`HX-Trigger` + eventos de corpo** — o que é: broadcast de evento nomeado para várias regiões. Por que não entra na v1: a origem do evento fica invisível na resposta; OOB mostra exatamente o que mudou, na ordem, e continua cacheável. Custo se entrar: um grafo implícito de listeners que nenhum teste de bytes enxerga.
7. **`hx-disable`/`hx-ignore`** — o que é: dois atributos, um com nome reciclado entre versões. Por que não entra: colisão semântica entre v2 e v4 e perda de foco do gatilho. Custo se entrar: bug intermitente de teclado que só aparece com rede lenta.
8. **`transitions`/`transition:`** — o que é: View Transitions por configuração ou por swap. Por que não entra: animação passa a ser decisão de markup, com custo de a11y (`prefers-reduced-motion`) e de teste. Custo se entrar: mais uma matriz (swap × motion) para o gate verificar.

## 7. Lacunas do corpus

1. **Nenhum tratamento de `Vary`** — a palavra não aparece. O corpus não tem modelo de cache intermediário; a matriz da §7.4 é inteiramente nossa.
2. **`HX-Trigger`/`HX-Trigger-After-Swap`/`HX-Trigger-After-Settle` ausentes** — o mecanismo é usado (`taskChanged from:body`) mas nunca nomeado; a semântica de "quando" disparar não está em lugar nenhum.
3. **Acessibilidade pós-swap: uma linha.** Nenhum `aria-live`, `aria-busy`, `role="status"`, gestão de foco além de um `focus()` solto, nenhuma menção a anúncio, ordem de leitura ou `prefers-reduced-motion` de view transition.
4. **Semântica de `204` além de "não troca"** — não se diz se dispara eventos, se conta como sucesso em `after:request`. O crate precisa decidir (`204` = sucesso sem swap, sem evento de conteúdo).
5. **`HX-Request` para requisições não-htmx** (ex.: `fetch` de terceiro) e o comportamento de `HX-Request: false` não são definidos.
6. **Sem matriz de métodos e corpo**: `hx-get`/`hx-delete` não incluem formulário (A33), mas o corpus não diz o que acontece com `hx-put`/`hx-patch` nem com `enctype` multipart.
7. **Sem limite de concorrência e de taxa**: `hx-sync` cobre o single-element, não o "N regiões em `load` ao mesmo tempo" (o corpus dispara 6 — `03:194-199`).
8. **Sem contrato de erro de transporte**: `htmx:error`/`htmx:response:error` são citados (`07:212-221`) mas não há política de retry, backoff ou estado "offline". O crate precisa decidir que **não** há retry automático (retry silencioso em POST não é idempotente).
9. **Sem acessibilidade de `aria-current`/URL** quando `hx-push-url` muda a URL: a navegação não anuncia mudança de contexto, e nada no corpus trata disso.
10. **Sem política de tamanho**: nenhum limite de payload por fragmento, nenhuma relação entre fragmento e orçamento de bytes (`docs/design/09` §4) — a crate precisa declarar que o orçamento por página inclui os fragmentos.
