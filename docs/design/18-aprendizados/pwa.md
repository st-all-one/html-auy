# `_RUST/pwa` — guia v2.0 (manifest + service worker, base MDN pós-2023)

Este conjunto é um guia de referência sobre **Progressive Web App** compilado a partir da MDN, com 17
arquivos: 13 capítulos (essenciais, manifest, service worker, offline/background, instalação,
boas práticas, template mínimo, dois tutoriais, localização do manifest, `scope_extensions`,
`serviceworker` de pagamento e "nuances"), um `EXAMPLE.md` de 893 linhas com uma implementação
completa, além de `index.md`, `SKILL.md` e `VERSION`. Declara base "Web App Manifest (W3C Candidate
Recommendation)", "Service Worker (W3C Candidate Recommendation)" e implementação em "Chromium 125+,
Safari 17.4+, Firefox 126+ (partial)" (`VERSION:1-5`).

Serve aqui por um motivo estreito: o que o PWA exige **recaia sobre o HTML gerado** (quais `<link>` e
`<meta>` o `<head>` precisa emitir, em que ordem, com que valores) e sobre o **pipeline de assets**
(nomes com hash, `Cache-Control`, `ETag`, `service worker` interceptando o que o crate emitiu). O
resto do conjunto — push, background sync, badging — é território do consumidor e está registrado
aqui como fronteira, não como requisito. O critério é o do projeto: se o aprendizado exige runtime,
JavaScript obrigatório ou API que aceite string crua, ele entra como **armadilha**.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `01-pwa-essentials.md` | 80 | definição, espectro site→PWA→nativo, dois requisitos, arquitetura bipartida main/SW, ciclo de vida, ordem de progressive enhancement | fixa a fronteira: manifest + HTTPS é tudo que a crate pode influenciar; SW é arquivo do consumidor |
| `02-web-app-manifest.md` | 270 | referência completa do manifest: essenciais, display, icons, screenshots, cores, shortcuts, share_target, file_handlers, protocol_handlers, display_override, launch_handler, orientation, `*_localized` | **fonte primária** dos membros que o crate precisa tipar e das regras de URL (`start_url`, `scope`, `id`) |
| `03-service-worker-patterns.md` | 189 | estrutura do `sw.js`, install/activate/fetch, 5 estratégias, tabela de trade-off, versionamento por byte, registro, debugging | define qual estratégia é compatível com asset hasheado (`immutable`) e qual destrói `ETag` de HTML |
| `04-offline-and-background.md` | 197 | hierarchy: Background Sync, Periodic Sync, Background Fetch, Push; limites (~5min, 30s idle), permissões | nada a emitir no HTML; delimita o que nunca entra no crate |
| `05-installation-and-integration.md` | 165 | critérios de instalação no Chromium, `beforeinstallprompt`, pós-instalação, app stores, Web Share, share target em SW | critérios duplicam `02`; `beforeinstallprompt` é o único ponto que exige JS de consumidor |
| `06-best-practices.md` | 140 | matriz cross-browser, performance, deep links, acessibilidade, aparência nativa, "o que evitar", checklist, segurança | matriz de suporte = tabela de "não emita esperando que funcione em todo lugar"; §Segurança dá `img-src`/`use-credentials` |
| `07-pwa-minimal-template.md` | 203 | app mínimo end-to-end: `index.html`, `manifest.json`, `sw.js`, `app.js`, `style.css` | é a **forma canônica do `<head>`** que o shell deve conseguir emitir; contém a armadilha de precachear `manifest.json` |
| `08-cycletracker-tutorial.md` | 302 | tutorial iniciante (localStorage, cache-only, manifest com múltiplos `sizes`) | única fonte que mostra `sizes` com vários valores (`"72x72 96x96"`) e SVG com `sizes` |
| `09-js13kgames-tutorial.md` | 278 | tutorial intermediário (fetch + render, 8 ícones, cache-first com fallback, push) | reforça "8 tamanhos de ícone" e `innerHTML` de template string — o anti-padrão que a crate existe para eliminar |
| `10-howto-localize-manifest.md` | 161 | membros `*_localized`, formato string/objeto (`value`/`dir`/`lang`), ordem de resolução, `icons_localized` autocontida | permite **um** manifest com N variantes de texto (sem gerar manifest por request); e exige check de completude dos arrays localizados |
| `11-manifest-scope-extensions.md` | 120 | `scope_extensions` + `.well-known/web-app-origin-association`, experimental | multi-domínio do portal: registro de fronteira; hoje não justifica nada no crate |
| `12-manifest-serviceworker.md` | 68 | membro `serviceworker` = Payment Handler JIT, `use_cache`↔`updateViaCache`, "não é o SW geral" | evita erro de modelagem (achar que o manifest registra o SW do app); e documenta a única alavanca de cache do SW |
| `13-pwa-nuances.md` | 217 | 10 armadilhas + checklist: SW≠PWA, cache evictável, fallthrough, promoção sem manifest, prompt Android, fallback chain, scope, update detalhado, streams, POST | **a lista de decisões de cache do consumidor**; quase todas viram regra de pipeline |
| `EXAMPLE.md` | 893 | implementação "ModerApp" com todos os membros, `sw.js` multi-estratégia, `app.js` com feature detection, CSS, `offline.html`, `.well-known`, mapa de 40 práticas | referência de forma completa; contém 2 violações do vocabulário (`style=` e `onclick=`) que precisam ser corrigidas ao copiar |
| `index.md` | 29 | índice + três pilares (HTTPS, manifest, SW) + "SW não é obrigatório para instalação" | confirma que o crate só é dono do pilar 2 |
| `SKILL.md` | 97 | resumo operacional: 9 "core rules", manifest+SW de exemplo, mapa de arquivos, ordem de leitura | as 9 regras são a versão citável e curta; usadas aqui como veredito de cada achado |
| `VERSION` | 5 | "PWA Guide v2.0", base W3C CR, Chromium 125+/Safari 17.4+/Firefox 126+ parcial | fixa a versão lida; qualquer regra abaixo vale para esse recorte |

## 2. Achados

### 2.1 Instalação e `<head>`

#### A1. Só dois requisitos, e nenhum deles é o service worker
- Citação: `01-pwa-essentials.md:17` — "1. **Web App Manifest** — informações mínimas para instalação"; `SKILL.md:24` — "Installability requires **Manifest + HTTPS**; a Service Worker is optional for install but needed for offline/background."
- Semântica: instalar não exige SW; exige manifest válido + origem segura. Offline é capacidade separada.
- Consequência: o crate pode entregar instalação sem saber nada de SW. `<link rel=manifest>` é `<head>`, SW é arquivo do consumidor. Isso mantém PWA fora do vocabulário.
- Veredito: confirma P5 (zero JS embutido) e D3; refina o escopo do shell (emite 1 link a mais).

#### A2. O manifest é referenciado em **todas** as páginas, com MIME próprio
- Citação: `02-web-app-manifest.md:10` — "Cada página do PWA **deve** referenciar o manifest. Tipo MIME: `application/manifest+json`."
- Semântica: escopo de instalação cobre a origem inteira, mas o reconhecimento é por documento; página sem o link não é "parte do PWA".
- Consequência: é obrigação do **shell**, não da view — se cada view pudesse esquecer, 222 views esqueceriam. Reforça D11 (`Page::new` como único caminho).
- Veredito: confirma D11; refina o contrato do shell.

#### A3. `crossorigin="use-credentials"` quando o manifest fica atrás de auth
- Citação: `02-web-app-manifest.md:7` — "`<link rel="manifest" href="/app.webmanifest" crossorigin="use-credentials" />`"; `06-best-practices.md:137` — "`crossorigin=\"use-credentials\"` se manifest requer auth".
- Semântica: o manifest é um fetch separado, com modo CORS; sob sessão, precisa de credenciais.
- Consequência: `spec/elements.toml` já permite `crossorigin` em `link` (`elements.toml:106`), mas o valor aceito não é tipado — sem enum, `crossorigin="use-credential"` compila e o manifest falha silenciosamente (o app deixa de ser instalável sem erro visível).
- Veredito: refina D4 — `crossorigin` deve ser `CrossOrigin::{Anonymous, UseCredentials}` no gerador.

#### A4. O conjunto mínimo instalável é pequeno e enumerável
- Citação: `05-installation-and-integration.md:30` — "`prefer_related_applications: false`"; `SKILL.md:30` — "Manifest must include `name`/`short_name`, 192+512 icons, `start_url`, `display`, `prefer_related_applications:false`."
- Semântica: cinco campos decidem se o Chromium oferece instalação: nome, ícones 192+512, `start_url`, `display`, e um booleano que precisa ser `false` (ou ausente).
- Consequência: dá para modelar o manifest como **struct com campos obrigatórios** e um só opcional perigoso; `prefer_related_applications` não deveria ser `bool` exposto — é constante `false` (omitir o campo é mais seguro que confiar no chamador).
- Veredito: confirma P7 (erro de marcação = erro de compilação) aplicado a dado, não a tag.

#### A5. `id` define identidade; mudar cria um app diferente
- Citação: `02-web-app-manifest.md:34` — "| `id` | String (URL) | Opcional | ID único do app. Fallback: `start_url`. Mesma origem que `start_url` |"; `02-web-app-manifest.md:268` — "| `id` mudou = app diferente | Pode instalar lado a lado com versão anterior |"
- Semântica: sem `id`, a identidade é o `start_url` resolvido; qualquer mudança de `id` (ou de `start_url` sem `id`) é outro produto instalado.
- Consequência: `start_url` e `id` passam a ser **contrato de URL**, na mesma prateleira de `LUrl`: não podem conter locale nem parâmetro que varie por request, senão cada request "cria" um app. Isso colide com `LUrl` que injeta locale.
- Veredito: refuta o uso direto de `LUrl` para `start_url`; exige tipo próprio (`ManifestUrl`/`OriginPath`).

#### A6. `start_url` é same-origin, relativo ao manifest e sem fingerprint
- Citação: `02-web-app-manifest.md:269` — "| `start_url` relativo | Resolvido contra URL do manifest; usar caminho consistente |"; `06-best-practices.md:135` — "`start_url` **não** deve conter fingerprinting (`?user=123`)".
- Semântica: a resolução é contra a URL do **manifest**, não do documento — logo a localização do manifest altera o significado do `start_url`.
- Consequência: se o pipeline renomear o manifest com hash em subdiretório diferente, `start_url: "."` muda de destino. Regra de pipeline: manifest com URL estável, na raiz do `scope`.
- Veredito: refina §8 de `docs/design/07-js-e-assets.md` (hash para CSS/JS/fontes, **não** para o manifest).

#### A7. `scope` exige barra final
- Citação: `02-web-app-manifest.md:270` — "| `scope` sem trailing `/` | Prefix match pode capturar paths indesejados |"; `02-web-app-manifest.md:36` — "| `scope` | String (URL) | Opcional | Path das páginas do app. Sempre terminar com `/` |"
- Semântica: o match é por prefixo de string; `/noticias` casa com `/noticias-antigas`.
- Consequência: `scope` não pode ser `AttrVal` livre; precisa de tipo que **acrescente** a barra (normalização na construção, não no lint). Mesma classe de bug que `Href` sem `LUrl`.
- Veredito: confirma P2 (contexto é tipo) e D1.

#### A8. `display` tem cadeia de fallback fixa
- Citação: `13-pwa-nuances.md:105-108` — "O `display` member segue uma cadeia de fallback:" … "fullscreen → standalone → minimal-ui → browser".
- Semântica: o valor pedido degrada por uma ordem conhecida quando o ambiente não suporta.
- Consequência: `display` é enum fechado de 4 valores + `display_override`. Enum, não string: o custo é uma linha no TOML e elimina `"standlone"`.
- Veredito: confirma D4 (allowlist) aplicada a **valor**, não só a nome de atributo.

#### A9. `display_override` redefine a cadeia e habilita `window-controls-overlay`
- Citação: `13-pwa-nuances.md:122` — "window-controls-overlay → standalone → fullscreen → browser"; `02-web-app-manifest.md:198` — "Substitui a cadeia de fallback do `display`:".
- Semântica: uma lista ordenada de preferências; WCO vem primeiro no exemplo, `standalone` como rede de segurança.
- Consequência: WCO só rende algo com CSS (`env(titlebar-area-*)`, região arrastável) — que é exatamente o que o crate proíbe inline. Se o consumidor quiser WCO, precisa de tokens/regras no `/theme.css`, não de `style=`.
- Veredito: irrelevante para o crate hoje; registrado como decisão do consumidor (§5c).

#### A10. `theme_color`/`background_color` são par, e o `<meta>` pode vencê-los
- Citação: `02-web-app-manifest.md:93-95` — "`theme_color`: toolbar, status bar, task switcher. Fully opaque recomendado." … "`background_color`: splash screen + fundo durante carregamento. Deve combinar com CSS `background-color`." … "`theme_color` pode ser sobrescrito por `<meta name="theme-color" content="...">` (suporta `prefers-color-scheme`)"
- Semântica: o manifest define a cor de espera (splash) e a cor de chrome; o `<meta>` tem precedência e aceita `media`.
- Consequência: três consumidores da mesma cor (manifest, `<meta>`, CSS). O crate já tem uma fonte única de cor — tokens do tema (D12). `background_color` **tem** que ser derivado do token de fundo, senão o splash pisca cor errada antes do CSS.
- Veredito: confirma D12; refina: a cor do manifest é dado do tema, não configuração separada.

#### A11. Dois `theme-color` com `media` em vez de JS
- Citação: `EXAMPLE.md:39-40` — "`<meta name="theme-color" content="#3367d6" media="(prefers-color-scheme: light)" />`" e `… dark" />`
- Semântica: a alternância claro/escuro é resolvida pelo navegador via media query, sem script.
- Consequência: `spec/elements.toml` já permite `media` em `meta` (`elements.toml:97`); o que falta é a regra de **duas** ocorrências canônicas com ordem fixa (light antes de dark) para o `ETag` não oscilar.
- Veredito: confirma P6 (determinismo) com uma cláusula nova no shell.

#### A12. `icons` com `purpose` e a área segura de 80%
- Citação: `02-web-app-manifest.md:66-67` — "`purpose`: `any` (default), `maskable` (adaptável), `monochrome`" … "`maskable`: conteúdo importante dentro de 80% do diâmetro central"
- Semântica: `maskable` é um recorte do SO, não um ícone diferente por capricho; a margem é a garantia de não cortar o símbolo.
- Consequência: a crate não desenha ícone, mas **exige** que o registro de assets declare `purpose`; ícone maskable sem `purpose` é o defeito clássico (logo cortado no Android). Isso é um campo tipado no asset, não um atributo HTML.
- Veredito: confirma D13 (asset declarado por tipo) estendido com metadado de ícone.

#### A13. `sizes` aceita lista e `any`; o array de ícones é conteúdo, não atributo
- Citação: `08-cycletracker-tutorial.md:181` — "{ \"src\": \"icons/circle.svg\", \"sizes\": \"72x72 96x96\", \"purpose\": \"maskable\" }"; `08-cycletracker-tutorial.md:183` — "{ \"src\": \"icons/wheel.svg\", \"sizes\": \"512x512\" }"
- Semântica: um arquivo pode declarar vários tamanhos (SVG escala; PNG não), e `sizes` é string com gramática própria dentro do JSON.
- Consequência: `sizes` no manifest **não** é o `sizes` de `<link>`/`sizes` de imagem responsiva. Precisa de tipo próprio (`IconSizes`) que valide `NxN` ou lista separada por espaço, e que recuse lista para PNG.
- Veredito: refina `Sizes` de `docs/design/02-tipos-e-contextos.md` (gramática `(condition) length` não serve aqui).

#### A14. `screenshots` exige `label` e distingue `form_factor`
- Citação: `02-web-app-manifest.md:80-81` — "`label`: obrigatório para acessibilidade" … "`form_factor`: `narrow` (mobile) ou `wide` (desktop)"
- Semântica: screenshot entra no install prompt do Android; sem `label` não há descrição acessível.
- Consequência: é o único membro do manifest com requisito de acessibilidade **declarado**. Para o crate a consequência é de processo: a demonstração de a11y do PWA não é só HTML — inclui metadado de imagem.
- Veredito: confirma P3/P4 fora do markup; refina `Alt` para o mundo JSON (mesma ideia, outro tipo).

#### A15. `description` + `screenshots` mudam o prompt no Android
- Citação: `13-pwa-nuances.md:83` — "No Chromium Android, se o manifest incluir `description` e `screenshots`, o install prompt nativo mostra:"
- Semântica: campos "de marketing" têm efeito funcional no diálogo de instalação — e só nessa plataforma.
- Consequência: `description` deve vir do mesmo `Msg`/SEO que o `<meta name=description>`, não de um campo duplicado; senão o app instalado e a página contam histórias diferentes.
- Veredito: refina `ShellCtx.meta` (`docs/design/06-shell-e-tema.md:40`): um valor, dois destinos.

#### A16. `shortcuts` são URLs dentro do `scope`
- Citação: `02-web-app-manifest.md:122-123` — "- `name` e `url` são obrigatórios" … "- `url` deve estar dentro do `scope`"
- Semântica: atalho é navegação interna, com as mesmas regras de URL do app.
- Consequência: `url` de shortcut deve ser `LUrl` (rota real do roteador) — não string. Se o roteador mudar, o atalho quebra sem erro de compilação hoje.
- Veredito: confirma D1; refina: `LUrl` serializa em JSON, não só em `href`.

#### A17. `share_target` tem dois modos incompatíveis de tratamento
- Citação: `02-web-app-manifest.md:128` — "### share_target (Receber Compartilhamento)"; `05-installation-and-integration.md:122` — "**Files (POST):** processar no service worker:"
- Semântica: GET entrega query string para a página; POST/multipart entrega `request.formData()` para o SW.
- Consequência: GET cria uma rota pública com parâmetros — logo exige sanitização e eco seguro (o crate resolve o eco; não resolve a semântica). POST exige `fetch` no SW, fora do crate.
- Veredito: irrelevante para o crate; relevante para o consumidor decidir rota.

#### A18. `file_handlers` exige `launchQueue` no cliente
- Citação: `05-installation-and-integration.md:146` — "if (\"launchQueue\" in window) {" (bloco de consumo de `launchParams.files`)
- Semântica: associar tipo de arquivo ao app só vira experiência com código de consumidor lendo os handles.
- Consequência: nada no HTML; confirma a fronteira "comportamento é arquivo JS declarado" (D3/D13).
- Veredito: irrelevante.

#### A19. `protocol_handlers` usa `%s` na URL
- Citação: `02-web-app-manifest.md:188` — "{ \"protocol\": \"mailto\", \"url\": \"/handle-mail?to=%s\" }"
- Semântica: o placeholder é substituído pelo navegador com o valor do protocolo; a URL resultante é entrada controlada por terceiros.
- Consequência: se algum dia entrar, `url` é `{ LUrl, template }` com validação de esquema e do placeholder — nunca `AttrVal`. Portal municipal não precisa.
- Veredito: irrelevante; registra a armadilha de URL templada.

#### A20. `launch_handler` decide o que acontece ao reabrir
- Citação: `02-web-app-manifest.md:210` — "\"launch_handler\": { \"client_mode\": \"focus-existing\" }"
- Semântica: `focus-existing` reusa a janela aberta em vez de criar outra; experimental.
- Consequência: para um portal, focar a aba existente é bom padrão e não exige nada do HTML — um default no builder de manifest.
- Veredito: refina §5a (default proposto), não vira regra.

#### A21. `scope_extensions` é experimental e exige opt-in da outra origem
- Citação: `11-manifest-scope-extensions.md:3` — "**Status:** Experimental"; `11-manifest-scope-extensions.md:43` — "Crie o arquivo `/.well-known/web-app-origin-association` no servidor da origem estendida:"
- Semântica: só um domínio que publica o `.well-known` aceita ser absorvido por outro app.
- Consequência: subdomínios do mesmo operador (ex.: `help.`/`api.`) precisariam de arquivo em cada host; para a crate, é zero markup e 100% infraestrutura.
- Veredito: refuta qualquer suporte hoje; registra em §5c.

#### A22. O membro `serviceworker` do manifest **não** é o SW do app
- Citação: `12-manifest-serviceworker.md:8` — "O membro `serviceworker` no manifest especifica um service worker que é **instalado e registrado Just-In-Time (JIT)** para suportar o **Web-based Payment Handler API**."; `12-manifest-serviceworker.md:45` — "| Status | Standard | Experimental + Non-standard |"
- Semântica: dois service workers com propósitos distintos; o do manifest é registrado pelo navegador para pagamentos.
- Consequência: se o builder de manifest tivesse um campo `serviceworker`, induziria exatamente a confusão que o documento desmente. Campo **não existe** no nosso schema, com comentário explicando por quê.
- Veredito: confirma D4 (nada de campo genérico); fecha uma porta.

#### A23. Localização usa `*_localized` e cai por tag mais genérica
- Citação: `10-howto-localize-manifest.md:155` — "3. Fallback para o valor não-localizado → Usa `name`, `icons`, etc."; `02-web-app-manifest.md:258` — "Membros localizáveis: `name`, `short_name`, `description`, `icons`, e dentro de `shortcuts`."
- Semântica: `fr-CA` → `fr` → não localizado; o navegador escolhe, o servidor não.
- Consequência: o manifest não precisa de geração por locale *para o nome*; pode ser um só, com `_localized`. Isso simplifica o pipeline: **um** manifest estável, com `Msg` resolvido no build.
- Veredito: refina a suposição de "manifest por locale"; agora é "um manifest, N variantes de texto".

#### A24. Ícones localizados são autocontidos
- Citação: `10-howto-localize-manifest.md:157` — "Se o ícone localizado existe mas tem um único tamanho, o browser **não** busca no `icons` global para completar. A array de ícones localizada é autocontida."
- Semântica: `icons_localized[x]` substitui, não completa; se faltar 512px lá, o app perde instalabilidade naquele locale.
- Consequência: um invariante de validação — para cada locale em `icons_localized`, exigir 192 e 512. É o tipo de regra que o crate coloca em `cq check`, não no doc.
- Veredito: adição nova (check de consistência), não existente hoje.

### 2.2 Service worker e ciclo de vida

#### A25. O escopo do SW é o diretório do arquivo
- Citação: `13-pwa-nuances.md:131` — "O escopo do service worker é determinado pelo diretório onde o `sw.js` está"; `13-pwa-nuances.md:139-142` — "O escopo pode ser expandido via `Service-Worker-Allowed` header HTTP:" … "Service-Worker-Allowed: /"; `SKILL.md:29` — "Serve SW from the scope root; `Service-Worker-Allowed` for wider scopes."
- Semântica: `/assets/js/sw.a1b2.js` controla só `/assets/js/` — inútil. SW tem que morar na raiz.
- Consequência: esse é o argumento definitivo contra "hash no nome do SW sem pensar": se o pipeline separa assets por tipo em diretórios, o SW quebra silenciosamente. Regra: `sw.js` na raiz, ou header dedicado.
- Veredito: refina §8 de `docs/design/07-js-e-assets.md` com exceção explícita.

#### A26. Atualização do SW só acontece por diferença de bytes no script
- Citação: `03-service-worker-patterns.md:164` — "A única maneira de forçar atualização do service worker é alterar o arquivo `sw.js` (mudança de byte)."
- Semântica: o gatilho de update é o arquivo do SW; o cache name dentro dele é convenção sua, não mecanismo.
- Consequência: o `sw.js` **não pode** receber hash no nome (URL nova = registro novo, não update) nem ser servido `immutable`. Se o pipeline hasheia tudo, o SW fica de fora com justificativa registrada.
- Veredito: correção direta à regra "tudo tem hash" do `docs/design/07-js-e-assets.md:137`.

#### A27. O update passa por `waiting`, e `skipWaiting()` é decisão, não default
- Citação: `SKILL.md:28` — "Version caches; activate with `skipWaiting()` + `clients.claim()` only when safe (avoid broken sessions)."; `13-pwa-nuances.md:217` — "- [ ] skipWaiting ⏤ use em dev, avalie em prod"
- Semântica: `skipWaiting` faz o SW novo assumir páginas abertas; com assets hasheados, a aba antiga pede chunks que já não existem no cache novo.
- Consequência: o custo de `skipWaiting` é **exatamente** o modelo de asset hasheado: HTML antigo em memória + cache podado = 404. Regra de pipeline: manter o cache da versão anterior por uma geração se `skipWaiting` estiver em uso. [INFERENCE]
- Veredito: refuta `skipWaiting()` incondicional do `EXAMPLE.md:221`.

#### A28. `addAll` é atômico: um 404 derruba a instalação inteira
- Citação: `03-service-worker-patterns.md:28` — "Se `addAll` falhar para qualquer recurso, a instalação inteira falha"
- Semântica: precache é tudo-ou-nada; um asset renomeado no deploy impede o SW de instalar.
- Consequência: precache deve conter só o que é **estável e indispensável** (shell, CSS, bundle, offline), e o pipeline precisa de um gate que compare a lista de precache com o registro de assets — exatamente o `cq check` de assets que já existe (`docs/design/06-shell-e-tema.md:160`).
- Veredito: confirma o gate de assets; adiciona "lista de precache" como consumidor do registro.

#### A29. Precachear o `manifest.json` cria manifest congelado
- Citação: `07-pwa-minimal-template.md:66` — "const PRECACHE = [\".\", \"./index.html\", \"./style.css\", \"./app.js\", \"./manifest.json\"];"
- Semântica: o template cacheia o próprio manifest no install.
- Consequência: o manifest é buscado pelo navegador por URL, não pelo SW — então o precache quase nunca é usado, e quando for (via `caches.match`) serve versão velha. É peso morto com risco. [INFERENCE]
- Veredito: refuta `07:66` e `EXAMPLE.md:206-214` (que também precacheia `/manifest.json`).

#### A30. Fallback de navegação é obrigatório
- Citação: `SKILL.md:31` — "Always provide an offline fallback; never leave users on a dead page."; `EXAMPLE.md:340` — "const offline = await caches.match(\"/offline.html\");"
- Semântica: requisição `mode === "navigate"` que falha deve cair numa página do próprio app.
- Consequência: `offline.html` **tem** que ser uma página do vocabulário (com shell, `lang`, `<main>`), não HTML solto. Ou seja: uma rota real do consumidor, renderizada pela crate, precacheada.
- Veredito: refina D11 — a página offline é `Page`, não arquivo estático escrito à mão.

#### A31. As cinco estratégias têm custo declarado
- Citação: `03-service-worker-patterns.md:156-158` — "| Cache first | ✅ | ❌ | Assets estáticos |" / "| Stale-while-revalidate | ✅ | ⚠️ Médio | Avatares, listas |" / "| Network first | ✅ (fallback) | ✅ | Dados semi-dinâmicos |"
- Semântica: cache-first maximiza offline e congela frescor; network-first preserva frescor e depende da rede.
- Consequência: mapeamento direto para o pipeline: asset com hash → cache-first (seguro por construção, porque o nome muda quando o conteúdo muda); HTML → network-first; `/theme.css` → network-only ou stale-while-revalidate com revalidação.
- Veredito: confirma que hash de conteúdo é o que **autoriza** cache-first; sem hash, cache-first é bug.

#### A32. Cache pode ser despejado pelo navegador
- Citação: `13-pwa-nuances.md:17` — "## 2. Cache Pode Ser Limpo pelo Browser"; `13-pwa-nuances.md:31` — "O fallback `fetch(request)` na estratégia **cache first** não é redundante — ele salva o app se o cache foi limpo entre versões."
- Semântica: Cache API não é storage durável; quota, pressão de disco e limpeza do usuário apagam.
- Consequência: toda estratégia precisa de caminho de rede; e a crate não pode assumir que "está no cache" significa "está disponível". Nenhum caminho do crate pode depender de cache do cliente para correção.
- Veredito: confirma P1 (o artefato é o HTML; o cache é do consumidor).

#### A33. Não chamar `respondWith` é fallthrough para a rede
- Citação: `13-pwa-nuances.md:49` — "No `fetch` event do SW, se você **não** chamar `event.respondWith()`, o browser trata a requisição normalmente (vai para a rede)."
- Semântica: "network only" não é código, é ausência de código.
- Consequência: o roteamento do SW deve interceptar por allowlist (destination/tipo), não por denylist. Para o crate, é o mesmo princípio do vocabulário fechado.
- Veredito: irrelevante para o markup; orienta o exemplo do consumidor.

#### A34. POST nunca é cacheado
- Citação: `03-service-worker-patterns.md:60` — "**Nunca** intercepte POST requests (semântica de mutação)"; `13-pwa-nuances.md:202` — "event.respondWith(cacheFirst(event.request));" (exemplo rotulado ERRADO)
- Semântica: cache de POST corrompe semântica; o filtro é `request.method !== "GET"` no topo.
- Consequência: formulário do portal (que existe em toda página pública) nunca pode ser servido por cache. Isso valida a escolha de HTML `no-cache` + network-first.
- Veredito: confirma `docs/design/09-desempenho-e-cache.md:29` na parte de formulário.

#### A35. Response é stream; sem `clone()` o corpo é consumido
- Citação: `13-pwa-nuances.md:34` — "cache.put(request, response.clone());"; `03-service-worker-patterns.md:61` — "Responses são streams: use `.clone()` antes de `cache.put()`"
- Semântica: um corpo só pode ser lido uma vez; cachear e devolver exige duas leituras.
- Consequência: erro do consumidor, mas o sintoma (resposta vazia) é indistinguível de bug da crate. Vale um item no checklist de integração.
- Veredito: irrelevante para o crate; registro de diagnóstico.

#### A36. `use_cache` / `updateViaCache` é a única alavanca de HTTP cache no SW
- Citação: `12-manifest-serviceworker.md:29` — "| `use_cache` | Boolean | Não | Como usar HTTP cache durante atualizações |"; `03-service-worker-patterns.md:170` — "Browser detecta diferença no byte do `sw.js`, instala novo SW em background, ativa quando páginas antigas fecharem."
- Semântica: o navegador pode servir o próprio `sw.js` do HTTP cache, e `updateViaCache` decide o que é revalidado.
- Consequência: `Cache-Control` no `sw.js` importa duas vezes — é o que garante que "mudança de byte" seja vista. Regra: `no-cache` (revalidar sempre), nunca `immutable`.
- Veredito: refina a tabela de cache de `docs/design/09-desempenho-e-cache.md`.

### 2.3 Nuances que recaem sobre o produto

#### A37. "SW instalado" ≠ "PWA instalado"
- Citação: `13-pwa-nuances.md:5` — "o service worker é instalado (evento `install`) assim que o usuário **visita o site**, independentemente de instalar o PWA no dispositivo."
- Semântica: visita ≠ instalação; o SW roda para todos os visitantes e o cache offline passa a existir sem consentimento.
- Consequência: um SW cache-first mal configurado congela conteúdo para **todo** visitante, inclusive quem nunca instalou. Isso eleva o custo de um erro de cache: não é "só para quem instalou".
- Veredito: reforça A31 — HTML nunca cache-first.

#### A38. `beforeinstallprompt` é Chromium, one-shot e pode nunca disparar
- Citação: `05-installation-and-integration.md:67` — "**Não suportado no iOS** — fallback necessário"; `05-installation-and-integration.md:66` — "- `event.prompt()` só pode ser chamado **uma vez** por evento"
- Semântica: o botão de instalação é opcional e nunca é a via principal; o prompt nativo do navegador continua sendo o caminho.
- Consequência: se o portal tiver botão "Instalar app", ele precisa existir no HTML sem JS (visível ou escondido?). O crate não tem como esconder por JS sem estado. Solução: `<button hidden>` + JS do consumidor revelando — mas `hidden` remove do DOM acessível, o que é correto para um botão indisponível.
- Veredito: confirma P5 (JS não obrigatório); refina: botão de instalação nasce `hidden` no markup.

#### A39. Instalação é isolada por navegador
- Citação: `05-installation-and-integration.md:80-81` — "Cada navegador mantém instalação isolada" … "Mesmo PWA em dois browsers = duas instâncias independentes"
- Semântica: não existe "app instalado" no servidor; existem N instalações por usuário.
- Consequência: nenhuma decisão de servidor pode depender de "está instalado"; `ETag`/304 e cache HTTP continuam valendo igual. Mata qualquer ideia de "modo app" server-side.
- Veredito: confirma a fronteira do crate (não conhece cliente).

#### A40. O `EXAMPLE.md` viola o vocabulário duas vezes
- Citação: `EXAMPLE.md:805` — "<main style=\"text-align:center;padding:4rem 1rem;\">"; `EXAMPLE.md:809` — "<button onclick=\"window.location.reload()\">Try again</button>"
- Semântica: a página offline de referência usa `style=` inline e handler inline — justamente o que `docs/design/03-vocabulario-de-elementos.md:114-119` proíbe (CSP sem inline).
- Consequência: copiar esse arquivo como "boa prática" reintroduz o defeito. A versão correta é `Page` com `Tone`/`Class` e um asset `Shared` que expõe "recarregar" por `data-*`.
- Veredito: refuta o exemplo; registra a correção em §4.

## 3. Confirmações do desenho

1. **Manifest é `<head>`, SW é arquivo** — `01:17-18` e `SKILL:24` põem instalação em manifest+HTTPS; nada exige que o crate conheça SW. O shell emite um `<link>` e para.
2. **`Page::new` como único caminho (D11)** — `02:10` exige o link em **toda** página. Só um shell obrigatório garante isso em 222 views; convenção não garante.
3. **Vocabulário fechado (D4) aplica-se a valores** — `13:105-108` (display), `02:270` (scope com barra), `02:66` (purpose) mostram três enums que hoje seriam strings num manifest escrito à mão.
4. **Hash de conteúdo autoriza `immutable`; ausência de hash proíbe cache-first** — `03:156` + `03:164`: cache-first só é correto quando o nome muda com o conteúdo, e o SW é a exceção onde o nome **não** pode mudar.
5. **Cor é token, não configuração (D12)** — `02:94` exige que `background_color` combine com o CSS; a única forma de garantir isso é derivar do mesmo token.
6. **Acessibilidade não termina no HTML** — `02:80` exige `label` em `screenshots`; `docs/design/05-acessibilidade.md` não cobre metadado de manifest. Lacuna real, não conflito.
7. **Determinismo de bytes (P6/D14) sobrevive ao PWA** — todas as variações são por media query (`EXAMPLE:39-40`), não por script; nenhum membro do manifest exige valor por request.
8. **Sem JS obrigatório** — `06:42` ("HTML semântico (`<button>`, `<form>`, `<nav>`) — já suporta todos os inputs nativamente") e a ordem de enhancement em `01:64-68` põem HTML antes de tudo.
9. **A crate não decide cache** — `13:17-34` mostra que o cache do cliente é volátil e evictável; decisão de `Cache-Control` continua no consumidor, com as regras de §5b.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata + arquivo) |
|---|---|---|
| `docs/design/07-js-e-assets.md:137`: "css (tailwind + daisyUI) ─┐ … hash sha256 do conteúdo ─▶ nome.a1b2c3.css/.js" para **todo** asset | `03:164`: SW atualiza por byte-diff; `13:131`: escopo do SW é o diretório do script | Inserir em `docs/design/07-js-e-assets.md` §8: **"Exceção ao hash: `/sw.js` e `/manifest.webmanifest` têm URL estável e não recebem hash — o primeiro porque a atualização do service worker é detectada por diferença de bytes na mesma URL, o segundo porque `start_url` relativo é resolvido contra a URL do manifest."** |
| `docs/design/09-desempenho-e-cache.md:29`: "`Cache-Control: public, no-cache` para HTML com formulário em área própria; HTML público pode ser cacheável (decisão do consumidor)." | `13:5`: SW roda para todo visitante, instalado ou não; `03:156`: cache-first só para estático | Acrescentar em `docs/design/09-desempenho-e-cache.md` §2: **"Com service worker ativo, `Cache-Control` deixa de ser a política efetiva: o SW responde antes. Regra de compatibilidade: HTML nunca é cache-first no SW; asset com hash é cache-first e `immutable`; `/theme.css` e `/manifest.webmanifest` são `no-cache` + `ETag`; `/sw.js` é `no-cache`, nunca `immutable`."** |
| `docs/design/06-shell-e-tema.md:104`: "`304` por revalidação, não por `immutable` (o tema muda sem deploy)." | `EXAMPLE:39-40` mostra `<meta name=theme-color media=…>` casando com o tema | Acrescentar em `docs/design/06-shell-e-tema.md` §4: **"O mesmo token de tema alimenta `theme_color`/`background_color` do manifest e os dois `<meta name=\"theme-color\">` (light/dark, nesta ordem); não existe cor de PWA configurada separadamente."** |
| `spec/elements.toml:106` permite `crossorigin` em `link` como texto | `06:137`: precisa ser exatamente `use-credentials` sob auth | Em `spec/elements.toml`, comentar a entrada: **"`crossorigin` é enum `anonymous|use-credentials`; o gerador emite o valor como keyword, nunca string livre."** |
| `spec/elements.toml:96-97` permite `meta` com `media`, sem regra de multiplicidade | `EXAMPLE:39-40` usa duas ocorrências com `prefers-color-scheme` | Em `docs/design/06-shell-e-tema.md` §3, adicionar: **"`theme-color` é emitido no máximo duas vezes (light e dark, nessa ordem canônica), logo após `<meta name=\"viewport\">`."** |
| `docs/design/03-vocabulario-de-elementos.md:114-119` proíbe `style`/`on*` | `EXAMPLE:805,809` usa ambos na página offline | Nenhuma mudança na regra; corrigir a cópia do exemplo. Registrar em `docs/design/17-faq.md`: **"A página offline é uma `Page` normal; `EXAMPLE.md` do corpus a escreve com `style=`/`onclick=` e não deve ser copiado."** |
| Nenhum documento menciona PWA | `01:17-18`, `02:10`, `SKILL:24` | Criar `docs/design/19-pwa.md` com §5a/§5b deste documento e uma linha no índice do `README.md`. |

## 5. Adições prontas para colar

### 5a. Itens de `<head>` que o shell deve emitir (ordem canônica)

| # | Saída | Condição | Tipo | Observação |
|---|---|---|---|---|
| 1 | `<meta charset="utf-8">` | sempre | fixo | já é base do shell |
| 2 | `<meta name="viewport" content="width=device-width, initial-scale=1">` | sempre | fixo | — |
| 3 | `<meta name="theme-color" content="…" media="(prefers-color-scheme: light)">` | `ShellCtx.pwa.is_some()` | cor derivada do token `surface` | `EXAMPLE:39` |
| 4 | `<meta name="theme-color" content="…" media="(prefers-color-scheme: dark)">` | idem | token do tema escuro | `EXAMPLE:40`; ordem fixa |
| 5 | `<link rel="manifest" href="/manifest.webmanifest">` | idem | URL estável (sem hash) | `02:10`; `crossorigin=use-credentials` só sob auth (`06:137`) |
| 6 | `<link rel="icon" href="/icons/icon-<hash>.svg" sizes="any" type="image/svg+xml">` | idem | `Src::asset` + `IconSizes::Any` | `08:181` |
| 7 | `<link rel="icon" href="/icons/icon-192-<hash>.png" sizes="192x192" type="image/png">` | idem | idem | `02:60` |
| 8 | `<link rel="apple-touch-icon" href="/icons/apple-touch-<hash>.png">` | idem | idem | `EXAMPLE:47` (corpus não fixa 180×180 — ver §7) |
| 9 | `<title>` … `<script src=… defer>` … | sempre | existentes | assets já determinísticos |

```toml
# proposta: bloco de configuração do consumidor (dado, não API)
[pwa]
manifest_path   = "/manifest.webmanifest"   # URL estável, na raiz do scope
start_url       = "/"                        # sem locale, sem query
scope           = "/"                        # normalizado com barra final
display         = "standalone"               # enum: fullscreen|standalone|minimal-ui|browser
launch_handler  = "focus-existing"           # default
```

### 5b. Regras de cache compatíveis com service worker

| Caminho | `Cache-Control` | Estratégia no SW | Por quê |
|---|---|---|---|
| `/sw.js` | `public, no-cache` | — (não interceptar) | update por byte-diff (`03:164`) |
| `/manifest.webmanifest` | `public, no-cache` + `ETag` | — (não precachear) | cachear congela instalação (`07:66`) |
| HTML de página | `public, no-cache` + `ETag` | **network-first** com fallback `/offline` | `13:5`, `03:158` |
| `nome.<hash>.css/.js/.woff2` | `public, max-age=31536000, immutable` | **cache-first** | nome muda com conteúdo (`03:156`) |
| `/theme.css` | `public, no-cache` + `ETag` | network-only ou SWR | muda sem deploy (`06:104`) |
| `/offline` (rota) | `public, no-cache` + `ETag` | precache no `install` | `SKILL:31` |
| ícones/screenshots com hash | `immutable` | cache-first | são estáticos por build |
| `/api/**` e POST | `no-store` | network-only (sem `respondWith`) | `03:60`, `13:49` |

Regra de precache: **só** shell + CSS + bundle do shell + `/offline` + ícones do manifest. `addAll` é
atômico (`03:28`), e a lista é comparada ao registro de assets no `cq check`.

### 5c. O que fica **fora** da crate (fronteira escrita)

1. O arquivo `sw.js` e sua lógica de cache — consumidor.
2. O registro do SW (`navigator.serviceWorker.register('/sw.js')`) — asset `Shared` do consumidor; o
   crate só emite `<script src defer>`.
3. Push/VAPID, Background Sync, Periodic Sync, Background Fetch, Badging — `04:1-197` inteiro.
4. `share_target` POST e `file_handlers` (`launchQueue`) — código de consumidor (`05:150-160`).
5. `scope_extensions` e o `.well-known/web-app-origin-association` — infraestrutura (`11:43`).
6. Geração de ícones (192/512/maskable/`monochrome`), screenshots e seus `label`.
7. `window-controls-overlay`: CSS de barra de título; exigiria tokens novos, não markup.
8. Escolha de `Cache-Control` — o crate documenta a compatibilidade (§5b), não a decide.
9. Validação de manifest em CI (`npx pwa-validator`) e testes em dispositivo real (`SKILL:33`).

## 6. Armadilhas de complexidade

1. **Gerar o manifest por request** — parece natural (nome do tenant, locale do request). Custo: manifest
   deixa de ser artefato estável, `id`/`start_url` passam a variar e o navegador cria/recria o app. Fica
   de fora: manifest é gerado no build a partir de tokens + `Msg`.
2. **Manifest a partir do banco** — abriria o primeiro caminho em que uma string de dado vira estrutura
   de app instalado. Custo: some a garantia de `Href`/`Src`; cada campo precisaria de validação em
   runtime. Fica de fora: manifest é config versionada.
3. **Componente de botão "Instalar app"** — exigiria estado de JS e `hidden` dinâmico; no crate viraria
   markup condicionado a dado de cliente. Custo: um primitivo que não funciona sem JS. Fica de fora.
4. **Precaching automático das rotas** — o crate conhece a tabela de rotas e poderia emitir a lista.
   Custo: acopla ciclo de release de conteúdo ao de código; um asset renomeado derruba a instalação
   inteira (`03:28`). Fica de fora: lista é do consumidor, validada contra o registro.
5. **Registrar o SW pelo shell** — uma linha de `<script>` inline resolveria. Custo: viola D3 e a CSP;
   reintroduz `unsafe-inline`. Fica de fora: asset declarado.
6. **Suporte a `scope_extensions`/multi-domínio** — exige `.well-known` por host e modelo de "um app,
   N origens". Custo alto, status experimental (`11:3`). Fica de fora.
7. **Membros experimentais no schema (`note_taking`, `serviceworker`, `tabbed`)** — cada um é um campo a
   manter e documentar, com risco de modelagem errada (`12:8`). Fica de fora até haver caso real.
8. **Tema reagindo a `display-mode`** — `@media (display-mode: standalone)` no CSS é legítimo
   (`EXAMPLE:785`), mas no crate significaria gerar variantes de `/theme.css` por modo. Custo: mais um
   eixo no `ETag`. Fica de fora: o consumidor escreve a regra estática.

## 7. Lacunas do corpus

1. **`apple-touch-icon` sem tamanho** — `EXAMPLE:47` usa `icon-192.png` e não menciona 180×180 nem
   `apple-mobile-web-app-*`. A crate precisa decidir se emite um ícone dedicado e qual tamanho. [INFERENCE: iOS ignora `sizes` do manifest e usa o `apple-touch-icon`]
2. **`mobile-web-app-capable` vs `apple-mobile-web-app-capable`** — não aparecem em nenhum arquivo; a
   crate decide não emitir (não são necessários com manifest+HTTPS).
3. **Cache do próprio manifest** — o corpus diz quando o SW revalida (`12:29`), mas não diz como o
   navegador revalida o manifest. Decisão da crate: `no-cache` + `ETag` por simetria com `/theme.css`.
   [INFERENCE]
4. **Manifest por locale** — `10` mostra `*_localized`, mas nada diz sobre `start_url`/`scope` por
   locale. Decisão: um manifest único, `start_url` neutro, locale resolvido pelo roteador.
5. **Ordem do `<head>` com os itens de PWA** — nenhum arquivo trata determinismo de ordem; a crate
   fixa a ordem em §5a e testa por golden.
6. **`hidden` e acessibilidade** — `05:34` mostra `<button id="install" hidden>`, sem discutir que
   `hidden` remove o elemento da árvore acessível. Aceitável (é o comportamento desejado), mas a crate
   precisa afirmar isso em `docs/design/05-acessibilidade.md`.
7. **Ícone `monochrome`/badge e `Src`** — `04:143` usa `/badge.png` para notificação; a crate não tem
   categoria de asset para "ícone de notificação". Decisão: asset comum com `purpose` declarado.
8. **`start_url` e analytics** — `06:135` proíbe fingerprinting, mas não diz se parâmetros de campanha
   são aceitos. Decisão da crate: `start_url` sem query; campanha vai em `Msg`/`shortcuts`.
9. **Contrato de `/offline` no roteador** — o corpus trata `/offline.html` como arquivo estático; aqui
   é rota renderizada pela crate (`SKILL:31` + D11), o que exige que o shell renderize sem banco.
