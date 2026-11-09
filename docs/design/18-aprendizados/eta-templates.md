# `_RUST/eta-templates` — Eta 4.6.0

Eta é um motor de template JavaScript leve (EJS-like), ESM-only, que roda em Node, Deno e browser, e
compila cada template (`<%= %>`, `<%~ %>`, `<% %>`) para uma **função JS** criada em runtime. Este
conjunto cobre 17 arquivos e 2.400 linhas: sintaxe, partials, helpers, layouts/blocks, custom tags,
o tipo `config` completo, a API pública, segurança, Deno/browser, cheatsheet, FAQ, padrões avançados e
integrações.

Ele importa aqui por um motivo específico: **é a família de solução que a crate decidiu não ser**
(`minijinja/tera/handlebars` em [`14-alternativas.md`](../14-alternativas.md)). Todo recurso do Eta é
uma facilidade que a crate recusa deliberadamente; ler o conjunto serve para (a) saber exatamente o que
se perde, (b) verificar se alguma dessas perdas é maior do que o documentado, e (c) extrair os poucos
pontos onde o Eta acerta e a crate ainda não tem resposta escrita.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-index.md` | 100 | Exemplo canônico, navegação, "modelo mental" em 5 itens (`new Function`, `it`, três tags, helpers globais, async explícito) | achados A1, A6, A31; o modelo mental é o oposto exato de P5/D3 |
| `01-quickstart.md` | 156 | Instalação, estrutura `templates/`, `new Eta`, primeiro render sync e async, `loadTemplate` com `@`, Deno, browser, tabela de opções | achados A31, A38; confirma que mesmo o caminho "simples" exige runtime + filesystem |
| `02-sintaxe.md` | 153 | As três tags, comentários, `if`/`for`, `it`, whitespace (`-`/`_`), delimitadores custom, prefixos do parser, armadilhas | achados A4, A5, A7, A8, A10, A11; a frase da linha 152 é a melhor evidência externa de D1 |
| `03-partials.md` | 123 | `include`/`includeAsync`, mesclagem de dados, `resolvePath` (relativo vs `views`), `@` programático, escolha da tag de saída | achados A12–A14, A21; "contexto mesclado" é o item que mais claramente se quer perder |
| `04-helpers.md` | 80 | `output()`, `capture()`, `captureAsync()`, tabela retorno/emite/async | achados A14–A16; `capture` tem equivalente direto (`let`) |
| `05-layouts-blocks.md` | 183 | `layout()` (um pai, encadeável), `it.body`, `block(nome, fn)`, fallback, `blockAsync`, exemplo completo com saída | achados A17–A21; é o "shell + slots" do Eta, com ordem implícita e fallback silencioso |
| `06-custom-tags.md` | 93 | `customTags` (prefixo → handler), assinatura, conteúdo como string estática, retorno **sem escape**, restrições de prefixo, i18n | achados A22–A24; `customTags` é a porta de injeção que `Msg` fecha |
| `07-configuracao.md` | 227 | Tipo `config` completo (`autoEscape`, `autoFilter`, `autoTrim`, `cache`, `customTags`, `debug`, `escapeFunction`, `filterFunction`, `outputFunctionName`, `functionHeader`, `parse`, `plugins`, `rmWhitespace`, `tags`, `useWith`, `varName`) | achados A9, A25–A28; 18 knobs = 18 formatos de saída possíveis, contra D14 |
| `08-api.md` | 251 | `render`/`renderAsync`/`renderString*`/`loadTemplate`, `configure`/`withConfig`, `compile`/`compileToString`/`parse`, classes de erro, tipos, browser, `readFile`/`resolvePath` sobrescrevíveis | achados A13, A29–A31; mapa completo da superfície que a crate não tem |
| `09-seguranca.md` | 100 | "templates são código", ausência de sandbox, tabela de XSS por vetor, escape contextual ausente, `filterFunction` não é sanitização, path traversal, checklist | achados A2–A4, A25, A37; a fonte mais citável contra si mesma |
| `10-deno-browser.md` | 114 | JSR/Deno (`--allow-read`), Node ESM, `eta/core` no browser, matriz de compatibilidade, **CSP: `new Function` → `'unsafe-eval'`** | achados A2, A38; custo decisivo e mensurável |
| `11-cheatsheet.md` | 208 | Cheatsheet de tags, partials, layout, blocks, capture, custom tags, whitespace, API, erros, opções, tabela de helpers | conferência: nenhum achado exclusivo, só confirma A16, A18 e A28 |
| `12-faq-troubleshooting.md` | 170 | 20 perguntas, tabela sintoma→causa→correção, 8 armadilhas, debug | achados A20, A31, A33; a lista de armadilhas é o melhor teste de "o que não repetir" |
| `13-padroes-avancados.md` | 219 | Organização de diretórios, composição em camadas, i18n, tabela de performance, integração HTTP, tipagem TS, pré-compilação, testes | achados A32, A35, A36; a admissão sobre `unknown` no TS é a evidência mais forte de P3 |
| `14-integracoes.md` | 150 | Express (`app.engine` recriado com `readFile` + `renderString`), Fastify (`@fastify/view`), Deno, bundlers, lista de integrações **não** suportadas | achado A34; "integração oficial" é concatenação de string |
| `SKILL.md` | 72 | Front matter, "Core Rules" (singleton, `views` absoluto, `cache` em prod, `<%=` sempre que dinâmico), "Core Patterns", mapa de arquivos | achado A38; as regras do próprio autor confirmam as armadilhas |
| `VERSION` | 1 | `4.6.0` | fixa a versão de referência (`jsr:@bgub/eta`, `npm:eta`) |

## 2. Achados

### A1. Template é código executável criado em runtime
- Citação: `00-index.md:79` — "**Template = função JS.** `eta.render` compila a string para uma função e a"
- Semântica: o texto de template é compilado para uma `Function` e executada; a linha 80 conclui que templates "**nunca** devem vir de usuários".
- Consequência: toda a família de recursos abaixo (interpolação, `if`, loops, helpers, plugins) existe porque há um interpretador; removê-lo remove a categoria inteira de risco.
- Veredito: **refuta** P5/D3 por construção — não é um ajuste de configuração, é a arquitetura.

### A2. CSP: compilação por `new Function` exige `'unsafe-eval'`
- Citação: `10-deno-browser.md:78` — "- Cuidado com CSP: templates compilam para `new Function` — pode exigir"
- Semântica: renderização no browser (e, em rigor, qualquer `renderString` em qualquer runtime) depende de avaliação dinâmica de string.
- Consequência: `script-src 'self'` sem `unsafe-eval` fica incompatível; a crate promete exatamente o contrário (`docs/design/08-seguranca.md:83`).
- Veredito: **confirma** D3/D13 com custo medido; é o argumento mais curto e mais forte contra qualquer adoção futura.

### A3. Não há sandbox, e não está no roadmap
- Citação: `09-seguranca.md:25` — "Eta **não** faz sandbox da execução de templates, e isso não é objetivo do"
- Semântica: a única defesa para template não confiável é trocar de motor (Mustache) ou isolar o processo.
- Consequência: "template vindo do banco" (conteúdo editável por admin) nunca é resolvido por configuração; a crate também não deve oferecer esse caminho ([`00-principios.md:88`](../00-principios.md)).
- Veredito: **confirma** o não-objetivo "HTML/CSS vindo de banco".

### A4. O escape do `<%=` é HTML, não contextual — admitido pelo corpus
- Citação: `02-sintaxe.md:152` — "- Assumir que `<%=` escapa URLs/atributos JS — escape é **HTML**, não contextual."
- Semântica: a mesma frase reaparece em `09-seguranca.md:40`: "| Contexto não-HTML (JS, URL, CSS) | Escape **contextual** separado; `<%=` só cobre HTML |".
- Consequência: `href`, `src`, `srcset`, JSON-LD e classe CSS ficam todos por conta do chamador, em cada call site, sem tipo.
- Veredito: **confirma** D1; é a lacuna que a crate fecha com `Href`/`Src`/`SrcSet`/`JsonLd`/`Class`.

### A5. O conjunto de caracteres escapados é fixo e único
- Citação: `02-sintaxe.md:20` — "O valor é automaticamente XML-escaped. `&`, `<`, `>`, `\"`, `'` viram entidades,"
- Semântica: existe **uma** função de escape (substituível por `escapeFunction`, ver A27) aplicada a texto e a valor de atributo indistintamente.
- Consequência: `\r`, `\n` e `\t` em valor de atributo não são tratados (a crate os converte em `&#13;`/`&#10;`/`&#9;`, [`02-tipos-e-contextos.md:58`](../02-tipos-e-contextos.md)); inserir HTML já escapado em texto escaparia de novo se fosse `String`.
- Veredito: **refina** a tabela de `docs/design/02` §3: confirma que a separação `Text`/`AttrVal` é necessária, e não cosmética.

### A6. `<%~ %>` é a porta de escape, obrigatória para partials
- Citação: `00-index.md:85` — "`<%~ expr %>` → interpola **sem escape** (HTML cru, use para partials/blocks)"
- Semântica: compor uma página exige, em cada composição, escolher a tag crua; a alternativa (`<%=`) produz `&lt;div&gt;` (ver `12-faq-troubleshooting.md:16`).
- Consequência: a "porta" não é exceção, é o caminho normal de composição — é o que `Raw`/`PreEscaped` são em outros motores.
- Veredito: **refuta** D2 como opção de configuração; a crate só pode eliminar a porta se a composição for tipada (`Markup` é o único renderizável).

### A7. `<% %>` executa JS e não emite; comentário é comentário JS
- Citação: `02-sintaxe.md:38` — "Não emite nada. Serve para declarar variáveis, abrir blocos `if`/`for`, etc."
- Semântica: `02-sintaxe.md:42` acrescenta "- Comentários são apenas comentários JS de bloco:".
- Consequência: o corpo do template é um programa; variáveis, efeitos colaterais e `console.log` (`11-cheatsheet.md`, seção Logging) cabem dentro da view.
- Veredito: **refuta**; a crate aceita `@if`/`for` (controle de fluxo) mas não expressão arbitrária, e `cq check` proíbe texto literal.

### A8. Whitespace tem quatro marcadores e três configurações globais
- Citação: `02-sintaxe.md:111` — "| `_` no **início** da tag | Remove **todo** whitespace antes da tag |"
- Semântica: as linhas 112–114 completam o conjunto (`_`/`-` no início e no fim), e `02-sintaxe.md:145` avisa que os prefixos `parse.*` "**não** aceitam `-` nem `_`".
- Consequência: o mesmo template produz bytes diferentes conforme marcação local + configuração global; nenhum `ETag` estável sobrevive a isso sem disciplina extra.
- Veredito: **refuta**; `cq fmt` tem regras fixas e idempotência testada ([`10-ergonomia.md:61`](../10-ergonomia.md)).

### A9. `autoTrim` aceita três valores por lado; `rmWhitespace` aplica regex
- Citação: `07-configuracao.md:126` — "No padrão `[false, \"nl\"]`, o texto após a tag sofre trim de 1 newline."
- Semântica: `07-configuracao.md:123` define `false`/`"nl"`/`"slurp"` e o par `[trimConfig, trimConfig]`; `07-configuracao.md:134` define `rmWhitespace` como `.replace(/^\s+|\s+$/gm, "")`.
- Consequência: dois projetos com a mesma view e configs diferentes emitem HTML diferente — inclusive dentro de `<pre>`.
- Veredito: **refuta** D14 (um formato, sem knobs) e confirma a adição "`cq fmt` sem configuração" registrada em [`README.md`](README.md).

### A10. Delimitadores são configuráveis por instância
- Citação: `02-sintaxe.md:131` — "Hi {{= it.name }}"
- Semântica: `new Eta({ tags: ["{{", "}}"] })` troca a sintaxe; `parse.interpolate`/`raw`/`exec` mudam os prefixos internos.
- Consequência: a sintaxe não é um contrato do motor, é dado de configuração — ferramenta de lint/formatter precisa de configuração por projeto.
- Veredito: **irrelevante como recurso**, relevante como aviso: `html!` colide com nada porque Rust não tem `<%`.

### A11. `varName`, `useWith` e `functionHeader` controlam o escopo dos dados
- Citação: `02-sintaxe.md:90` — "new Eta({ useWith: true })     // \"Hi <%= name %>\"  (evite: colisões/performance)"
- Semântica: `02-sintaxe.md:89` renomeia `it`; `02-sintaxe.md:91` extrai campos com `functionHeader: "const name=it.name"`.
- Consequência: existem três modelos de escopo simultâneos (objeto, `with`, destructuring textual), e o terceiro é código injetado no topo da função.
- Veredito: **refuta**; parâmetros de função Rust dão o mesmo resultado sem escopo implícito nem string de código.

### A12. `include` mescla o contexto do pai com o objeto extra
- Citação: `03-partials.md:19` — "Os dados passados no 2º argumento são **mesclados** com o `it` da template atual"
- Semântica: o partial vê tudo do pai **mais** as chaves novas, sem declaração e sem checagem; chave desconhecida vira `undefined` renderizado como `"undefined"` ou silêncio.
- Consequência: o contrato de um partial é invisível no call site; renomear um campo do pai quebra partials distantes sem erro de compilação.
- Veredito: **refuta**; props explícitas de primitivo são a substituição, e a mesclagem implícita é perda desejável.

### A13. Resolução de nome é relativa ao pai, com `@` para templates programáticas
- Citação: `03-partials.md:62` — "- O caminho final **precisa ficar dentro de `views`** — tentativas de sair do"
- Semântica: `"./x"` é relativo ao arquivo atual, `"/x"` é relativo a `views`, `@x` lê do store em memória (`08-api.md:42`: "Eta não tentará resolver em disco e buscará no cache").
- Consequência: o grafo de composição só existe em runtime; não há verificação estática de que o partial existe (o erro é `EtaFileResolutionError` na request).
- Veredito: **refuta** "template em runtime"; na crate o grafo é o grafo de chamadas Rust, checado por `cargo check`.

### A14. `include()` devolve string; `capture()` existe para não re-renderizar
- Citação: `03-partials.md:90` — "`include()` é apenas uma chamada de render. Para reutilizar o mesmo fragmento"
- Semântica: `04-helpers.md:53` fecha a tabela — "| Custo | renderiza 1x | renderiza a cada uso |".
- Consequência: reuso na mesma template exige um helper extra (`capture`) que produz string, alimentando o mesmo problema de escape cru.
- Veredito: **confirma** a adição "calcular uma vez": `let card = NewsCard::new(n).render();` antes do `html!` tem custo e semântica idênticos, com tipo.

### A15. `output()` concatena sem escapar
- Citação: `04-helpers.md:27` — "> Diferença: `output()` **não escapa** o valor. Se `item` vier do usuário,"
- Semântica: é uma segunda porta de injeção, independente de `<%~`, disponível em qualquer ponto do corpo.
- Consequência: `output("<li>" + item + "</li>")` é o exemplo canônico da doc (`04-helpers.md:20`), isto é, concatenação de string com dado não escapado.
- Veredito: **refuta**; na crate, escrita crua só existe dentro de `html-core`, em `Render::render_to`.

### A16. `capture()` retorna string e não emite; `captureAsync` é a variante async
- Citação: `04-helpers.md:32` — "Executa um bloco de template e **retorna** o resultado como string, em vez de"
- Semântica: `04-helpers.md:60` mostra `captureAsync` com `await` no corpo; `04-helpers.md:72-73` definem `string` e `Promise<string>`.
- Consequência: o fragmento capturado é `String`, logo reinseri-lo com `<%=` escapa o HTML do próprio fragmento (o exemplo usa `<%= greeting %>`, `04:40`).
- Veredito: **parcialmente equivalente**; `Markup` capturado em `let` é estritamente melhor, e a variante async não tem análogo nem necessidade.

### A17. Layout é um só pai, e o corpo chega como `it.body`
- Citação: `05-layouts-blocks.md:8` — "Uma template pode ter **um** layout pai (e layouts podem ter seus próprios"
- Semântica: `05-layouts-blocks.md:25` mostra o layout renderizando `"  <%~ it.body %>"`; `05:15` diz que o conteúdo "estará disponível como `it.body`".
- Consequência: `it.body` é uma chave de dados como outra qualquer — pode ser sobrescrita pelo objeto passado a `layout("./base", { body: … })`, e o shell não sabe se recebeu o corpo certo.
- Veredito: **refuta**; `Page::new(ctx, main, status)` recebe `Markup`, nunca `String` ([`13-decisoes.md:114`](../13-decisoes.md)).

### A18. Block com fallback silencioso
- Citação: `05-layouts-blocks.md:83` — "| Filha não define | renderiza o fallback (se houver) ou nada |"
- Semântica: `05:93` repete a regra ("Fallback só é usado quando a filha não define aquele block"); não há erro para block não preenchido nem para block ignorado.
- Consequência: um `<link rel=stylesheet>` que a página esqueceu de declarar simplesmente não aparece — falha invisível, descoberta em produção.
- Veredito: **refuta**; slots devem ser exaustivos (`enum Slot` fechado, preenchimento obrigatório ou `Option` explícita).

### A19. `block()` fora de layout renderiza inline
- Citação: `05-layouts-blocks.md:92` — "- **Sem layout ativo**, `block()` renderiza inline (útil para componentes reusáveis)."
- Semântica: o mesmo nome de função tem dois comportamentos opostos (capturar vs emitir) conforme estado ambiente invisível.
- Consequência: mover um trecho entre "filha" e "componente" muda a semântica sem mudar o código aparente.
- Veredito: **refuta**; na crate, componente é `fn -> Markup` e composição é chamada de função — dois nomes, duas semânticas.

### A20. `layout()` precisa vir antes do conteúdo
- Citação: `05-layouts-blocks.md:178` — "- `layout()` deve ser chamado **antes** do conteúdo para que a filha seja"
- Semântica: a ordem dos statements no corpo é o que define a captura do `it.body`; a doc lista isso como armadilha (`12-faq-troubleshooting.md:146`, item 3).
- Consequência: erro de ordem não é erro de parse; produz página sem corpo ou corpo duplicado.
- Veredito: **refuta** ordem implícita; `Page` recebe o miolo já pronto — a ordem é a da linguagem.

### A21. `blockAsync`/`includeAsync` só existem em render async
- Citação: `03-partials.md:28` — "> `includeAsync` só funciona dentro de uma renderização async"
- Semântica: `05-layouts-blocks.md:102` mostra `blockAsync("data", async () => { … })` e `05:110` o consumo com `await blockAsync("data")`.
- Consequência: cada helper tem duas versões, e usar a errada falha em runtime (`12-faq-troubleshooting.md:131`: "| `await` inválido / erro de sintaxe | `await` em render síncrono | usar `renderAsync` |").
- Veredito: **refuta**; views síncronas eliminam a família inteira (`*Async` × `include`/`capture`/`block`/`render`).

### A22. Custom tag recebe string estática e devolve HTML sem escape
- Citação: `06-custom-tags.md:57` — "- O conteúdo da tag é passado ao handler como **string estática**, não"
- Semântica: `06:62` completa: "- O retorno do handler é concatenado **diretamente** ao output — **sem / auto-escape**"; `12-faq-troubleshooting.md:150` registra a armadilha ("o handler recebe **string estática**, não expressão avaliada").
- Consequência: `<%* greeting %>` não é um lookup tipado — é uma chave textual cujo resultado entra cru, sem i18n verificada por compilação.
- Veredito: **refuta**; `enum Msg` gerado do catálogo ([`02-tipos-e-contextos.md:89`](../02-tipos-e-contextos.md)) dá chave verificada + paridade de locales + escape.

### A23. Prefixos custom não podem colidir com `=`/`~`/`-`/`_`
- Citação: `06-custom-tags.md:67` — "Prefixos custom **não podem** conflitar com:"
- Semântica: a gramática precisa reservar símbolos para trim e para os três modos nativos; adicionar um prefixo é editar um namespace global de uma letra.
- Consequência: qualquer extensão de sintaxe consome símbolos que a formatação também usa — acoplamento entre features independentes.
- Veredito: **irrelevante como recurso**; registra o custo de gramática extensível, que D6/D9 evitam.

### A24. A alternativa recomendada para i18n volta ao `<%=`
- Citação: `06-custom-tags.md:86` — "eta.renderString(\"<p><%= it.t('greeting') %></p>\", {"
- Semântica: `06-custom-tags.md:92` declara a vantagem — "Vantagem: participa do auto-escape (`<%=`)." — e reserva `customTags` para sintaxe própria.
- Consequência: mesmo o caso de uso que motivou custom tags é melhor servido por função no contexto — e essa função não valida a chave.
- Veredito: **refina** P3: a crate deve dar o passo seguinte (chave como variante de enum), não só a função.

### A25. `filterFunction` roda antes do escape e também no raw
- Citação: `09-seguranca.md:64` — "Filtros globais **não** substituem o escape. Mesmo com `autoFilter: true`, a"
- Semântica: `07-configuracao.md:187` completa ("o resultado ainda passa pelo escape quando a tag é `<%=`"), e `09:69` avisa: "Atenção: `autoFilter` também é aplicado ao `<%~` (raw), mas **sem** escape / posterior."
- Consequência: a mesma função global transforma dado de texto e HTML confiável, com semânticas de segurança diferentes — configuração que parece sanitização e não é.
- Veredito: **refuta**; transformação de dado pertence ao domínio (antes do `Markup`), não ao emissor.

### A26. `escapeFunction` é substituível por instância
- Citação: `07-configuracao.md:94` — "| `escapeFunction` | `XMLEscape` | Substitui a função de escape (ex.: custom entities). |"
- Semântica: o emissor aceita que o conjunto de entidades mude por configuração.
- Consequência: dois tenants com funções diferentes emitem bytes diferentes para o mesmo texto — `ETag` deixa de ser função do dado.
- Veredito: **refuta** D14; `Text`/`AttrVal` têm **um** escape, testado por property test.

### A27. Plugins interceptam AST e string de função gerada
- Citação: `07-configuracao.md:54` — "    processAST?: (ast: AstObject[], env?: EtaConfig) => AstObject[]"
- Semântica: `07:52-55` (e `07:209-212`) definem três hooks: `processFnString`, `processAST`, `processTemplate`; `07:222-226` mostra o exemplo de prepend de banner.
- Consequência: a saída final depende de código arbitrário que o formatter não vê; transformação estrutural pós-parse é o oposto de D6 (um parser, três passes sobre o mesmo AST).
- Veredito: **refuta**; qualquer transformação entra no parser/validador, versionada e testada.

### A28. `outputFunctionName` renomeia a função de saída
- Citação: `07-configuracao.md:204` — "> Ao trocar o nome, `output` deixa de existir. Use um identificador JS válido."
- Semântica: o nome do escritor de output é configuração de instância — e trocá-lo remove o original.
- Consequência: ferramenta de lint que reconhece `output(` precisa saber a config; exemplos de terceiros quebram.
- Veredito: **irrelevante**; na crate o escritor é `String` interno, sem nome exposto.

### A29. A superfície de compilação é pública
- Citação: `08-api.md:117` — "| `compile(str, options?)` | `TemplateFunction` | Compila string → função reutilizável |"
- Semântica: `08:118-120` expõem `compileToString`, `compileBody(ast)` e `parse(str)`; `08:123` mostra `fn.call(eta, { name: "Ben" })`.
- Consequência: o AST é API pública instável, e a string de template atravessa a fronteira do módulo em quatro formas.
- Veredito: **refuta**; o AST da crate é interno justamente para poder mudar (D6/D15).

### A30. Erros são de runtime, com hierarquia própria
- Citação: `08-api.md:146` — "| `EtaParseError` | Sintaxe inválida / tag não fechada |"
- Semântica: `08:147` define `EtaRuntimeError` ("Erro ao avaliar o template (com `debug`)"); a hierarquia inclui `EtaFileResolutionError` e `EtaNameResolutionError`.
- Consequência: tag não fechada e nome de template errado só aparecem quando aquela rota é exercitada — `debug: true` só melhora a mensagem.
- Veredito: **confirma** P7; `trybuild` cobre o que aqui é `EtaParseError`, e o custo de mensagem é pago no `cargo check`.

### A31. Async é explícito e não contagia a assinatura
- Citação: `00-index.md:88` — "5. **Async é explícito:** `renderAsync`/`renderStringAsync` + `await includeAsync()`"
- Semântica: cada helper tem variante sync e async; `01-quickstart.md:72` mostra `const res = await eta.renderAsync("simple", { name: "Ben" })`.
- Consequência: `includeAsync` esquecido produz `[object Promise]` no HTML (`12-faq-troubleshooting.md:44`), e `await` em render sync é erro de sintaxe/execução (`08-api.md:247`).
- Veredito: **refuta**; views síncronas tornam o modo de falha impossível — dado remoto é resolvido antes da view (A21).

### A32. Tipagem TS é decorativa: `it` é `unknown`
- Citação: `13-padroes-avancados.md:187` — "> Interpolações em templates são `unknown` para o TS — valide/normalize antes de"
- Semântica: `13:173` mostra `} satisfies HomeData)` — o `satisfies` valida o objeto passado, não o uso dentro do template; `it.titel` compilaria e renderizaria vazio.
- Consequência: toda a promessa de "props tipadas" morre na fronteira do arquivo `.eta`; o erro é sempre em request.
- Veredito: **confirma** P3/D1 pela boca do corpus: sem macro/compilador sobre a view, tipo não protege marcação.

### A33. Cache só vale para templates de arquivo; hot-reload é armadilha
- Citação: `12-faq-troubleshooting.md:153` — "8. **Assumir hot-reload**: com `cache: true`, mudanças em `.eta` não aparecem;"
- Semântica: `12:59` explica que "O cache só se aplica quando `name` ou `filename` é passado. Templates via / `renderString` normalmente não são cacheadas."
- Consequência: o ciclo de desenvolvimento depende de `cache: false` + restart, e o de produção de `cache: true`; dois regimes com comportamento diferente.
- Veredito: **confirma** a perda declarada ("hot-reload de template") e a substitui por `cargo check` incremental + `.snap` ([`15-migracao-blade.md:90`](../15-migracao-blade.md)).

### A34. A "integração oficial" com Express renderiza string lida do disco
- Citação: `14-integracoes.md:59` — "      const fileContent = eta.readFile(path)"
- Semântica: `14:60` segue com `eta.renderString(fileContent, opts)`; a doc registra que `app.engine()` foi removido no v3+ (`14:9`) e que as demais integrações "**não** são oficialmente suportadas" (`14:108`).
- Consequência: mesmo o caminho recomendado abre mão de `views`, cache e da validação de caminho (`resolvePath`) — o engine recebe `path` do framework.
- Veredito: **refuta** a ideia de "engine plugável"; na crate a integração é `IntoResponse` sobre um tipo, não um callback de string.

### A35. Os testes recomendados são de string, com escaping como caso
- Citação: `13-padroes-avancados.md:202` — "- Teste escaping: passe `<script>` e garanta que a saída com `<%=` está escapada."
- Semântica: `13:208-211` mostram `assertEquals(eta.renderString("Hi <%= it.name %>", { name: "<b>Ben</b>" }), "Hi &lt;b&gt;Ben&lt;/b&gt;")`.
- Consequência: o teste vive no arquivo de teste, ao lado do template — não há artefato revisável de HTML, só expectativa de string.
- Veredito: **confirma** a pirâmide de [`12-testes.md`](../12-testes.md): golden `.snap` é o artefato; o mesmo caso vira property test sobre o alfabeto hostil.

### A36. Performance recomendada = pré-computar fora do laço
- Citação: `13-padroes-avancados.md:92` — "<% /* BOM: pré-computa no JS */ %>"
- Semântica: `13:82` também recomenda "| `capture()` em vez de re-`include` | renderiza 1x |"; `13:79-84` listam `cache`, `debug: false`, evitar `useWith`.
- Consequência: a otimização é manual e local — nada impede `it.format(i)` dentro de `forEach` (o caso "RUIM" de `13:89`).
- Veredito: **confirma** a regra "calcular uma vez" (`let` antes do `html!`) e a irrelevância de knobs de runtime.

### A37. Path traversal é contido, mas nomes de template continuam perigosos
- Citação: `09-seguranca.md:78` — "- Não monte nomes de template com input do usuário."
- Semântica: `09:75` descreve a defesa (`dirIsChild` + `EtaFileResolutionError`), e `09:87-88` trata template vinda de banco como "código privilegiado".
- Consequência: a defesa é de caminho, não de origem: o conteúdo continua sendo código; a única mitigação real é revisão humana.
- Veredito: **refuta** D4 de outra direção: na crate nem o **nome** do elemento vem de dado ([`08-seguranca.md`](../08-seguranca.md) §2), então não há superfície a validar em runtime.

### A38. ESM-only e instância única por módulo
- Citação: `SKILL.md:23` — "- Instantiate `Eta` **once** per module (singleton); never `new Eta()` per request."
- Semântica: `01-quickstart.md:18` fixa "Eta v4 é **ESM-only**. Consumo via `require()` (CJS) não é suportado"; `10:61` repete para Node.
- Consequência: o motor carrega estado mutável (config, cache, store de `@`) que precisa ser compartilhado — mais uma peça global por processo.
- Veredito: **irrelevante**; a crate não tem estado: `html!` é código gerado, `Page` é valor por request.

### Tabela T1 — facilidade do Eta → equivalente em html-auy → o que se perde

| # | Facilidade do Eta | Equivalente na crate | O que se perde |
|---|---|---|---|
| 1 | `<%= %>` com escape HTML (`02:20`) | `Text`/`Msg` com escape por tipo | nada; ganha-se escape contextual |
| 2 | `<%~ %>` cru (`00:85`) | inexistente (D2) | injetar HTML pronto; resolvido por render nó-a-nó ([`13:34`](../13-decisoes.md)) |
| 3 | `<% %>` JS arbitrário (`02:38`) | `@if`/`@else`/`for` + Rust | expressão inline não tipada |
| 4 | Comentário `<% /* */ %>` (`02:42`) | `//` de Rust dentro do bloco | nada |
| 5 | Marcadores `-`/`_` de trim (`02:111`) | nenhum; `cq fmt` fixo ([`10:56`](../10-ergonomia.md)) | controle fino de whitespace |
| 6 | `autoTrim`/`rmWhitespace` (`07:126`, `07:134`) | um único modo de emissão | configurar whitespace por projeto |
| 7 | `tags: ["{{","}}"]` (`02:131`) | nenhum (`html!` não colide) | escolher delimitador |
| 8 | `parse.exec/interpolate/raw` (`02:145`) | nenhum | sintaxe própria por projeto |
| 9 | `varName`/`useWith` (`02:89`, `02:90`) | parâmetros da `fn` | escopo implícito (desejável perder) |
| 10 | `functionHeader` (`02:91`) | `let` no topo da `fn` | nada |
| 11 | `include()` com contexto mesclado (`03:19`) | primitivo com props explícitas | mesclagem implícita (desejável perder) |
| 12 | `resolvePath` relativo + `@` (`03:62`, `08:42`) | chamada de função Rust | template por nome em runtime |
| 13 | `renderString`/`loadTemplate` (`08:26`, `08:28`) | nenhum (view é `fn`) | template vinda de dado — a porta `Raw` por outro caminho |
| 14 | `layout()` pai único + `it.body` (`05:8`, `05:25`) | `Page::new(ctx, main, status)` ([`13:114`](../13-decisoes.md)) | trocar layout por nome; corpo como string |
| 15 | `block("x", fn)` + fallback (`05:83`) | slot tipado e exaustivo no shell | fallback silencioso |
| 16 | `block()` sem layout emite inline (`05:92`) | `fn -> Markup` | nada (dois nomes, duas semânticas) |
| 17 | `blockAsync` (`05:102`) | views síncronas | bloco com I/O |
| 18 | `capture()` (`04:32`) | `let` antes do `html!` | nada |
| 19 | `captureAsync()` (`04:60`) | nenhum | fragmento assíncrono |
| 20 | `output()` cru (`04:27`) | `Render::render_to` interno | escrita crua pública |
| 21 | `includeAsync`/`renderAsync` (`00:88`) | nenhum | I/O dentro da view |
| 22 | `customTags` (`06:14`, `06:62`) | `enum Msg` gerado ([`02:89`](../02-tipos-e-contextos.md)) | tag de domínio sem escape |
| 23 | `escapeFunction` (`07:94`) | escape fixo por tipo | entidades customizadas |
| 24 | `filterFunction`/`autoFilter` (`07:187`) | transformação em Rust antes do `Markup` | filtro global implícito |
| 25 | `plugins`/`processAST` (`07:54`) | macro é o único transformador ([`10:56`](../10-ergonomia.md)) | transformação pós-parse |
| 26 | `outputFunctionName` (`07:204`) | `String` interno | renomear a função de saída |
| 27 | `compile`/`parse` públicos (`08:117`, `08:120`) | AST interno, não público | ferramenta de terceiros sobre o AST |
| 28 | `EtaParseError`/`EtaRuntimeError` (`08:146`, `08:147`) | `compile_error!` + `trybuild` ([`04:15`](../04-validacao.md)) | erro só em request |
| 29 | `cache: true` + ausência de hot-reload (`12:59`, `12:153`) | `cargo check` incremental + `.snap` ([`15:90`](../15-migracao-blade.md)) | hot-reload de template |
| 30 | Express/Fastify como engine (`14:59`, `14:98`) | `Page` + `IntoResponse` | engine de terceiros plugável |
| 31 | `eta/core` no browser (`10:65`) | nenhum (SSR) | render no cliente |

## 3. Confirmações do desenho

1. **Escape por contexto é tipo, não posição sintática (D1/P2).** `02-sintaxe.md:152` e `09-seguranca.md:40`
   são a admissão explícita de que `<%=` cobre só HTML; URL, JS e CSS ficam sem tipo.
2. **O bypass de escape é o defeito central (D2/P1).** `<%~ %>` não é exceção de depuração: é o mecanismo
   normal de composição (`00-index.md:85`), e `output()` é uma segunda porta (`04-helpers.md:27`).
3. **Determinismo exige um formato único (D14/P6).** `autoTrim` (`07:126`), `rmWhitespace` (`07:134`),
   `escapeFunction` (`07:94`), `filterFunction` (`07:187`), `plugins` (`07:54`) e `tags` (`02:131`) são
   seis caminhos independentes para o mesmo template emitir bytes diferentes.
4. **Erro de marcação tem de ser erro de compilação (P7).** `EtaParseError`/`EtaRuntimeError`
   (`08-api.md:146-147`) só disparam na request; `debug: true` melhora a mensagem, não antecipa o erro.
5. **Tipagem de props sem macro é decorativa (P3).** `13-padroes-avancados.md:187`: interpolações são
   `unknown`; `satisfies` valida o objeto, não o uso.
6. **Sem runtime significa sem interpretador, não só "sem unsafe" (P5).** `new Function`
   (`10-deno-browser.md:78`) é o mecanismo; a CSP pagaria `'unsafe-eval'`.
7. **`Page` como único construtor de resposta (D11).** O Eta chega ao mesmo problema por outro caminho:
   `it.body` é chave de dados (`05:15`, `05:25`) e pode ser sobrescrita.
8. **Slots devem ser exaustivos.** O fallback silencioso de `05-layouts-blocks.md:83` é exatamente o modo
   de falha que `Alt::decorative()` explícito evita ([`05-acessibilidade.md:111`](../05-acessibilidade.md)).
9. **"Calcular uma vez" é regra, não otimização.** `13-padroes-avancados.md:82` e `13:92` recomendam
   pré-computar; a crate pode simplesmente exigir o `let` antes do `html!`.
10. **Golden `.snap` é o artefato.** Os testes recomendados (`13:202`) comparam strings no arquivo de
    teste; `docs/design/12` promove o snapshot a artefato revisável e proíbe aceite em massa
    ([`12-testes.md:22`](../12-testes.md)).
11. **Nada de template por string em ponto de entrada nenhum.** `renderString`/`loadTemplate`
    (`08-api.md:26`, `08:28`) são precisamente o que D2 proíbe, com outro nome.
12. **Views síncronas.** `00-index.md:88` e `03-partials.md:28` mostram que async contamina helpers e
    modos de falha; a crate resolve dado remoto antes da view.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata e arquivo) |
|---|---|---|
| `14-alternativas.md` trata a família Eta/EJS como "zero tipagem, erro em request" | `13-padroes-avancados.md:187` diz que interpolações são `unknown`, mas `13:173` sugere `satisfies` como se tipasse | Inserir em `docs/design/14-alternativas.md` §3 ("minijinja / tera / handlebars"): "**Tipagem é decorativa por construção:** `satisfies` valida o objeto passado, não o uso dentro do template — `it.titel` compila e renderiza vazio (`_RUST/eta-templates/13-padroes-avancados.md:187`)." |
| Nenhum doc lista "template por string" como categoria de risco nomeada | `08-api.md:26` e `08:28` mostram quatro entradas que aceitam string de template | Acrescentar em `docs/design/13-decisoes.md`, dentro de D2, o parágrafo: "**Consequência operacional.** Nenhum ponto de entrada da crate aceita template por string (`renderString`, `loadTemplate`, `compile`): o análogo dessas APIs é `unsafe` em outro ecossistema — `_RUST/eta-templates/08-api.md:26`." |
| `docs/design/09-desempenho-e-cache.md` §2 exige ordem canônica, sem listar o que a quebra | `07-configuracao.md:126`, `07:134`, `07:94`, `07:187` mostram seis knobs que mudam bytes | Inserir em `docs/design/09-desempenho-e-cache.md` §2, após a lista de ordem canônica: "Nenhuma opção de configuração altera a emissão: não existem equivalentes a `autoTrim`, `rmWhitespace`, `escapeFunction`, `filterFunction`, `plugins` ou `tags` (`_RUST/eta-templates/07-configuracao.md:52`)." |
| `docs/design/06-shell-e-tema.md` §1 descreve `slots` como `Vec<Markup>` ordenado | `05-layouts-blocks.md:83` mostra fallback silencioso e `05:92` block que emite inline sem layout | Inserir em `docs/design/06-shell-e-tema.md` §1, na linha de `slots`: "Slot não preenchido é ausência **declarada** (`Option`), nunca fallback silencioso; não existe `it.body` — o miolo chega como `Markup` (`_RUST/eta-templates/05-layouts-blocks.md:83`)." |
| `docs/design/04-validacao.md` §2 justifica N3/N4 pela limitação de `proc_macro::Diagnostic` | `08-api.md:146-147` mostra o custo real de erro em request | Acrescentar ao fim de §2 de `docs/design/04-validacao.md`: "O contraste é o motor de template em runtime: lá, tag não fechada é `EtaParseError` na request (`_RUST/eta-templates/08-api.md:146`); aqui, é `compile_error!` com span, coberto por `trybuild`." |
| `docs/design/07-js-e-assets.md` §5 promete CSP sem `unsafe-eval` | `10-deno-browser.md:78` mostra o motor compilando com `new Function` | Acrescentar à tabela de `docs/design/07-js-e-assets.md` §5, na coluna "Proibido": "`new Function`/compilação de template em runtime — é o que força `'unsafe-eval'` em qualquer motor EJS-like (`_RUST/eta-templates/10-deno-browser.md:78`)." |
| `docs/design/02-tipos-e-contextos.md` §3 afirma escape único por contexto | `02-sintaxe.md:20` escapa os mesmos 5 caracteres em texto e atributo | Nenhuma mudança de texto; reforçar com o comentário da tabela de §3: "texto e atributo têm escapes **distintos** justamente porque o motor JS não distingue (`_RUST/eta-templates/02-sintaxe.md:20`)." |
| `docs/design/01-arquitetura.md` §4 proíbe `app()`/container na view | `09-seguranca.md:87` admite template como "código privilegiado" | Reforço opcional em `docs/design/01-arquitetura.md` §4, lista "O módulo **não pode**": "ler arquivo, compilar template ou resolver nome de template em tempo de execução — não existe caminho (`_RUST/eta-templates/09-seguranca.md:78`)." |
| `docs/design/12-testes.md` §2 não menciona teste de "template não pode vir de dado" | `13-padroes-avancados.md:202` testa apenas escaping de string | Acrescentar a `docs/design/12-testes.md` §5 (`trybuild`): "e um caso que comprova a ausência de API de template por string (`renderString`/`loadTemplate` não existem) — paridade com `_RUST/eta-templates/08-api.md:28`." |
| `docs/design/10-ergonomia.md` §3 fixa o formato do `cq fmt` sem justificar contra config | `02-sintaxe.md:145` mostra prefixos reservados e `-`/`_` proibidos nos modos | Acrescentar a `docs/design/10-ergonomia.md` §3: "Não há marcadores de whitespace (`-`/`_`) nem `autoTrim`: o formato é o formato, e é isso que torna `fmt` idempotente (`_RUST/eta-templates/02-sintaxe.md:111`)." |

## 5. Adições prontas para colar

**5.1 — Decisão D16 (nova), para `docs/design/13-decisoes.md`:**

```markdown
## D16 — Nenhum ponto de entrada aceita template por string

**Contexto.** Motores EJS-like expõem `renderString`/`loadTemplate`/`compile`
(`_RUST/eta-templates/08-api.md:26-28`); é por essas portas que "template vinda do banco" entra e o
escape por tipo deixa de valer.
**Decisão.** A crate não tem API que aceite marcação como `&str` em runtime. View é `fn`; composição é
chamada de função; `Markup` só nasce no macro, em `Render` ou em `html-core`.
**Consequência.** Conteúdo dinâmico é sempre dado tipado; a auditoria de "quem pode escrever HTML" fica
reduzida ao repositório.
```

**5.2 — Linha para a tabela de `docs/design/02-tipos-e-contextos.md` §8 ("O que não existe"):**

```markdown
| template em runtime (`render`/`renderString`/`loadTemplate`) | é o `Raw` por outra porta (D16) |
```

**5.3 — Bloco para `docs/design/06-shell-e-tema.md` §1 (contrato de slots):**

```markdown
Slots são **exaustivos**: o shell recebe `main: Markup` e cada região opcional é `Option<Markup>`.
Slot não preenchido é ausência declarada; não existe fallback silencioso nem preenchimento por nome de
string. Não existe `it.body`: o miolo é valor, não chave de dados.
```

**5.4 — Item de checklist para `docs/design/12-testes.md` §5:**

```markdown
- [ ] A crate não expõe construtor de `Markup` a partir de `&str`/`String` (paridade com a ausência de
      `renderString`/`loadTemplate` de motores em runtime).
```

**5.5 — Regra para `docs/design/09-desempenho-e-cache.md` §2:**

```markdown
Nenhuma opção de configuração afeta a emissão. Se um dia existir uma, ela é **major** (D15), porque
muda bytes e, portanto, `ETag` de páginas já cacheadas.
```

**5.6 — Linha para a tabela de `docs/design/07-js-e-assets.md` §5:**

```markdown
| `new Function` / template compilada em runtime | força `'unsafe-eval'`; nenhum ganho de HTML |
```

## 6. Armadilhas de complexidade

1. **Compilar template em runtime.** Exige interpretador, cache de compilação e política de origem;
   custo: `'unsafe-eval'` na CSP e perda de `ETag` estável.
2. **`autoTrim`/`rmWhitespace`/marcadores `-`/`_`.** Quatro regras de whitespace × três valores = matriz
   de saída; custo: o formatter precisa reproduzir todas, e `fmt(fmt(x)) == fmt(x)` deixa de valer.
3. **`plugins`/`processAST`.** Um hook pós-parse transforma o AST que a validação já aprovou; custo:
   a validação vira "vale para o plugin padrão" e o contrato de bytes deixa de existir.
4. **`customTags`.** Prefixo de uma letra com handler arbitrário; custo: gramática extensível,
   colisão com trim/prefixos nativos e saída não escapada por definição.
5. **`escapeFunction`/`filterFunction` configuráveis.** Dois hooks globais sobre dado e HTML; custo:
   property tests de escape deixam de cobrir a produção, e `ETag` passa a depender de config.
6. **Variantes `*Async`.** Cinco pares sync/async (`render`, `renderString`, `include`, `capture`,
   `block`) mais o flag `{ async }` de `loadTemplate`; custo: metade dos modos de falha vem de escolher
   a variante errada (A21, A31).
7. **`layout()`/`block()` por nome de string.** Ordem implícita, fallback silencioso, dois significados
   para `block()`; custo: erros que o compilador não vê e a página não mostra.
8. **`it` como objeto dinâmico.** Acesso por chave textual em template; custo: renomear campo quebra em
   runtime, e nenhum renome é seguro.
9. **`useWith`/`functionHeader` como código injetado.** Escopo implícito e string de JS no topo da
   função; custo: análise estática do template fica impossível.
10. **`@` + store de templates em memória.** Registry mutável por processo; custo: estado global,
    mais um cache a invalidar, e a porta para "template vinda de fora" aberta.

## 7. Lacunas do corpus

1. **Nada sobre acessibilidade.** O conjunto não menciona `alt`, `label`, `scope` ou ARIA; é um motor de
   string e delega tudo. A crate não pode extrair nada aqui além da confirmação de que a camada precisa
   existir em outro lugar.
2. **Nada sobre determinismo de bytes.** Não há menção a `ETag`, hash de saída ou comparação binária; a
   única noção de estabilidade é "mesmo dado → mesma string", não verificada.
3. **Nada sobre custo de compilação.** `cache: true` é descrito como "compila cada template 1x"
   (`13:78`), sem números; a crate precisa medir `cargo check` por conta própria.
4. **Nada sobre formatação de HTML.** Não há `fmt`; o corpus não trata indentação canônica nem
   idempotência — decisão de `cq fmt` fica sem referência externa.
5. **Nada sobre streaming.** `render` devolve string inteira; não há `render_to`/buffer; a crate decide
   sozinha o contrato de `Render::render_to`.
6. **Nada sobre múltiplos locales.** i18n é `it.t()` ou custom tag; não há catálogo, paridade de locales
   nem locale no contexto — a crate decide com `Msg`/`Ltr`.
7. **Nada sobre composição tipada de fragmento.** `capture` devolve `string`; não há noção de fragmento
   com contrato (props/children) — o desenho de primitivo é inteiramente nosso.
8. **Nada sobre auditoria de saída.** Não há lint de HTML gerado; a crate decide as regras de N4 sozinha
   (`docs/design/04` §6).
9. **Nada sobre o que o Eta considera "erro recuperável".** `EtaRuntimeError` aparece só com `debug`
   (`08:147`); não há política de fail-safe — a crate adota "dado inválido é ignorado" (`docs/design/04` §7).
10. **Nada sobre integração com banco/editor.** A doc trata template como arquivo de repositório e não
    entra no caso "conteúdo editável" além de dizer "trate como código privilegiado" (`09:87`).
