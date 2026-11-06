# topcoat — 0.11.0

Conjunto de 23 arquivos markdown (+ `SKILL.md` + `VERSION`) que documenta o **Topcoat 0.11.0**, framework
Rust full-stack experimental: `view!` server-rendered, `#[component]` async, roteamento por módulo,
runtime de sinais no browser (`$()`, `@event`, `:bind`), `#[shard]`/`#[procedure]`, streaming
(`suspense`/`live!`/`emit!`/`connected`), cookies/sessão, mail, Fluent, assets com hash, Tailwind, fontes, ícones, htmx/Alpine/Datastar/SSE/WS/Tower. É o corpus mais próximo do nosso quadrante — macro
de HTML em Rust, HTML real vindo do servidor, sem WASM — e por isso é a melhor fonte de **sintaxe de
macro** e o melhor catálogo de **custos que não queremos pagar**. Serve aqui como contraste API por
API com `html!`: o que copiar (gramática de atributo, chamada de componente, `id` por call site),
o que recusar (runtime, spread, escotilhas de escape) e o que ele faz *pior* que a nossa spec
(atributo não validado, `alt` como string, `ETag` de asset inexistente, SRI inexistente).

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-index.md` | 185 | exemplo canônico, modelo mental em 7 itens, fluxo da request, mapa | fixa o vocabulário do corpus; `00:114` ("Segurança não é herdada por composição") é a frase que a nossa fronteira de shell evita |
| `01-quickstart.md` | 226 | CLI, presets, `topcoat dev`, setup manual, features, `topcoat fmt` | `01:176` explica por que `cq fmt` tem de existir; features por macro nos dizem o que **não** ter |
| `02-roteamento.md` | 344 | `module_router!`, `segment!`, `module_param!`, paths, layers, `href!`, erros, rewrite, servir | `href!`/`path_param!` mostram o custo de roteador embutido — nós recebemos `LUrl` de fora (D7) |
| `03-contexto.md` | 226 | `Cx` keyed por tipo, `with`, `#[memoize]`, helpers de request, armadilhas | contexto por tipo + newtype confirma nosso `Ctx`; `memoize` sem igualdade é armadilha a não copiar |
| `04-request-response.md` | 238 | extratores, body limit, respostas, redirects, rewrites, SSE/WS | status/headers como nós de view (contraste com `Page`); `Body` cru sem limite |
| `05-views.md` | 321 | **gramática de `view!`**, atributos, `class!`, `attributes!`, identidade, metadados, escaping, a11y | arquivo mais importante do corpus: é a especificação da sintaxe que estamos desenhando |
| `06-componentes.md` | 250 | `#[component]`, `#[default]`, `#[into]`, `Props`, `Child`, concorrência, keys | define a ergonomia de primitivo que `Field`/`Dialog` devem mirar |
| `07-streaming.md` | 230 | `suspense`, `error_boundary`, `live!`/`emit!`, `connected` | tudo fora do nosso escopo; ensina que status não pode ser decidido depois do primeiro byte |
| `08-runtime.md` | 356 | sinais, `$()`, `raw!`, `#[record]`, `#[procedure]`, `#[shard]`, prefetch, WS | catálogo do que D3/D13 proíbem; `shard` é o "componente que busca dados" que P9 exclui |
| `09-cookies-sessoes.md` | 309 | jar, prefixos, assinados/privados, `cookie_store`, sessão com hash | confirma "guarde só o hash" e que defaults seguros precisam de helper explícito |
| `10-servicos.md` | 209 | `mail!` com `html:` de `view!`, Fluent (`messages!`/`t!`), `Number`/`DateTime` | mail com vocabulário restrito e `Msg` tipado (nós já temos); `t!` sem validação é o que evitamos |
| `11-assets-estilo.md` | 307 | `asset!`, bundle por perfil, CDN, Tailwind, fontes, ícones, UI vendorizada | melhor fonte sobre assets; revela as lacunas (SRI, ETag, leitura por request, download sem checksum) |
| `12-integracoes.md` | 348 | htmx, Alpine AJAX, Datastar, SSE, WS, multipart, sitemap, Tower, CORS, fs | `12:78` ("Fragmento é endpoint") valida nosso `Vary: HX-Request` + revalidação |
| `13-seguranca.md` | 256 | modelo de confiança, matriz de escaping, `OriginPolicy`, riscos rankeados S1–S10 | arquivo mais útil de todos: é uma lista de defeitos de composição que o nosso desenho evita por tipo |
| `14-performance.md` | 186 | pipeline de render, barreira de TTFB, alocação, assets, compressão, medição | confirma "literal fundido em compile-time" e "view estática promovida"; `14:150` admite não haver benchmark |
| `15-testing.md` | 217 | `Router::handle`, `CxTestBuilder`, o que testar, shards, mail, CI | modelo para o nosso harness de conformidade; `.single()` panics = nosso `Markup → Response` não existe |
| `16-exemplos.md` | 184 | 28 examples + `demos/admin` (6k linhas) e o que **não** cobrem | `16:177` lista e2e/auth/CSRF como inexistentes — não copiar como arquitetura de referência |
| `17-api-reference.md` | 229 | features, macros, atributos de handler, `RouterBuilder`, tipos, defaults | tabela de defaults (`17:196-202`) é o retrato dos "padrões inseguros por omissão" |
| `18-cheatsheet.md` | 241 | referência de bolso: router, views, runtime, cookies, segurança, limites | `18:206` ("CSRF token: não existe") é o resumo da postura de segurança do framework |
| `19-melhores-praticas.md` | 186 | arquitetura, routing, views, contexto, runtime, segurança, anti-padrões | `19:133` ("se a doc e o código divergirem, o código vence") é um aviso de governança para nós |
| `20-faq-troubleshooting.md` | 275 | sintoma → causa → correção, 60+ entradas | mapeia exatamente os panics por erro de marcação que queremos transformar em erro de compilação |
| `21-padroes-avancados.md` | 288 | deploy, nginx/Caddy, containers, multi-tenant, jobs, observabilidade, edge, roadmap | `21:279-286` é a lista de ausências declaradas — fronteira que não devemos atravessar |
| `SKILL.md` | 111 | metadados + 11 "core rules" condensadas | `SKILL.md:18` ("Experimental. Expect breaking changes") fixa o custo de adotar plataforma |
| `VERSION` | 1 | `0.11.0` | versão de referência de todo este documento |

## 2. Achados

### A1. A gramática é HTML real, com escape por posição sintática
- Citação: `05-views.md:42` — "Elementos usam o nome real. Void (`area base br col embed hr img input link meta" (`05:44` — "`<!DOCTYPE html>` só nessa grafia; `<br/>` renderiza `/>` literal").
- Semântica: o macro aceita HTML como está — void sem fechamento, doctype em grafia única, `<br/>` inválido de propósito.
- Consequência: `html!` deve manter o vocabulário com a mesma forma; `void` na spec é o que decide "sem filhos", e doctype é responsabilidade do shell (D11).
- Veredito: confirma P1/D5.

### A2. Texto só existe entre aspas; `(expr)` é expressão
- Citação: `05-views.md:48` — "Texto **só** entre aspas; `("texto")` é expressão `&str`."
- Semântica: nada de texto nu; a interpolação é sempre uma expressão Rust.
- Consequência: nosso `Msg` (D2/02 §6) ocupa o lugar do literal; a gramática fica com **um** caminho de texto visível, o que o lint de literal (N3) pode exigir sem ambiguidade.
- Veredito: confirma P3/D1.

### A3. Nome de tag e de atributo podem ser dinâmicos — e "validados, não escapados"
- Citação: `05-views.md:58` — "A expressão é avaliada **uma vez** e reutilizada. Nome inválido → panic; nomes" (`05:59` — "dinâmicos são **validados**, não escapados.")
- Semântica: `<(tag) (attr)="v">` existe; nome inválido é panic de servidor, nunca erro de compilação.
- Consequência: nosso D4 mantém `(key)=(value)` fora da gramática — não por XSS (eles validam), mas porque panic em request é exatamente o "erro só aparece em produção" que P3 elimina; `13:62` confirma ("Nome inválido derivado de input → panic → 500").
- Veredito: refuta (nós recusamos); reforça D4.

### A4. Controle de fluxo: `if`/`for`/`match`/`let` com regras explícitas de vírgula
- Citação: `05-views.md:82` — "`match`: corpo de bloco com vários nós não leva vírgula; corpo de nó único leva" (`05:84` — "`let … else` não é suportado")
- Semântica: vírgula é contextual; `let` exige inicializador e `;`; `break`/`continue` recusados em posição de nó.
- Consequência: se aceitarmos `@if`, a regra de vírgula precisa ser **uma** e documentada em `03`/`10`; `let` dentro do bloco nunca entra (P9: nada de açúcar que esconde preço).
- Veredito: refina D6 (gramática mínima, sem variantes).

### A5. `if`/`for` em posição de **atributo** emitem atributos, não nós
- Citação: `05-views.md:86` — "`if`/`for`/`match` em atributos emitem **atributos**, não nós:"
- Semântica: a mesma palavra-chave muda de significado conforme a posição; o exemplo emite `aria-current` + `class` condicionalmente.
- Consequência: é a melhoria de sintaxe mais valiosa do corpus para nós: substitui `@if` aninhado em volta de um elemento por um bloco dentro da lista de atributos — e, com allowlist, não há como injetar atributo novo.
- Veredito: refina (adoção proposta em §5, item 1).

### A6. A tabela de formas de atributo é o contrato inteiro
- Citação: `05-views.md:101` — "| `name="lit"` | sempre presente; literal escapado em compile-time |" (`05:102` — "`name=(bool)` | `true` presente vazio; `false` omite").
- Semântica: seis formas — literal, `bool`, `Option<T>`, expressão, nome dinâmico (recusado por nós), spread (recusado por nós).
- Consequência: nossa forma `attr?: Option<T>` (02 §4) é a linha 103; falta-nos a linha 102 declarada como **regra de tipo** (`disabled` é presença, não `AttrVal`).
- Veredito: confirma 02 §4; refina (§5, item 2).

### A7. `disabled="false"` desabilita: literal em atributo booleano é armadilha de API
- Citação: `05-views.md:110` — "`disabled="false"` **desabilita** — literal está presente. Use `disabled=(flag)`."
- Semântica: presença é o valor; a string `"false"` é só presença.
- Consequência: para nós o literal deve ser **erro de compilação** — `disabled` não é `AttrVal`, é booleano de presença; `20:70` repete a armadilha porque o tipo permite.
- Veredito: refina spec (`spec/elements.toml`) e N2.

### A8. Duplicata estática é erro; ordem é indefinida quando vem de `HashMap`
- Citação: `05-views.md:113` — "Atributo estático duplicado → erro de compilação." (`05:114` — "`Attributes` é `HashMap`: ordem não especificada e chaves case-sensitive")
- Semântica: eles fecham a duplicata estática, mas o spread reabre ordem e caixa.
- Consequência: `class` e `CLASS` coexistentes violam P6 (bytes determinísticos) e o nosso teste `render(x) == render(x)`; confirma D4 e proíbe qualquer spread.
- Veredito: confirma D4/D14.

### A9. `class!` com condições, literais fundidos e omissão de lista vazia
- Citação: `05-views.md:130` — "Entradas ausentes (`None`, `false`) são puladas; lista vazia omite o atributo." (`05:131` — "Literais consecutivos são fundidos e escapados em compile-time")
- Semântica: classe é lista de tokens com condicionais; vazio não emite `class=""`.
- Consequência: nosso `Class::of(&[Token])` precisa da regra de omissão explícita, senão viola `09 §4` ("Nenhum atributo emitido com valor vazio"); a fusão em compile-time é o que já queremos em `09 §1`.
- Veredito: confirma P6; refina 02 §5.

### A10. `attributes!` tem um parâmetro `cx =>` não documentado
- Citação: `05-views.md:153` — "Aceita `cx =>` (não documentado), que exige `&Cx`." (`05:154` — "Valores ausentes viram `Absent`, mas a chave pode permanecer")
- Semântica: comportamento existe no código, não na doc; atributo ausente deixa chave morta.
- Consequência: argumento direto a favor de D5 (spec como dado versionado): superfície documentada por código é superfície que ninguém revisa. Nada disso entra em `html!`.
- Veredito: irrelevante para a API; confirma D5.

### A11. Chamada de componente: nomeados primeiro, filhos depois, sem vírgula
- Citação: `05-views.md:176` — "Nomeados com `nome: valor`; filhos depois, sem vírgulas." (`06:93` — "Passar `child:` explícito **e** filhos inline: o inline vence (substitui)")
- Semântica: filhos são posicionais e delimitados por `<…>`; nomeado depois de filho é erro.
- Consequência: é a ergonomia que `Field`, `Dialog` e `Table` precisam — e "inline vence" é regra de desempate que devemos copiar textualmente no `fmt`/diagnóstico.
- Veredito: confirma 10 §1; adota (§5, item 4).

### A12. Props avaliam em *hoist*, antes de qualquer corpo rodar
- Citação: `06-componentes.md:63` — "Props são avaliadas na fase *hoist*, em ordem de fonte, **antes** de qualquer" (`06:62` — "`#[into]` aplica `Into` ao argumento")
- Semântica: `#[default]`, `#[default("info")]`, `#[into]`; avaliação de argumentos é separada da renderização.
- Consequência: nós não temos `hoist` porque não há concorrência; mas `#[default]`/`#[into]` são ergonomia real. `#[into]` continua proibido (From infalível, ver `README.md`), `#[default]` vira `Option<T>` obrigatório.
- Veredito: refina 03 §7 (parâmetros opcionais `Option`, nunca default mágico).

### A13. `cx` é injetado **pelo nome** do parâmetro
- Citação: `06-componentes.md:39` — "Props são os parâmetros nomeados; `cx: &Cx` é detectado **pelo nome** e injetado."
- Semântica: renomear o parâmetro quebra a injeção, sem erro de tipo.
- Consequência: nosso `Ctx` é passado explicitamente (`fn list(ctx: &mut Ctx, …)`); a "injeção por nome" é o tipo de magia que P9 recusa e que `20:83` lista como FAQ ("`cx` não existe no componente").
- Veredito: refuta; reforça P9.

### A14. `#[derive(Props)]` gera builder typestate
- Citação: `06-componentes.md:84` — "`#[derive(Props)]` gera o builder typestate: campos sem default precisam ser"
- Semântica: campos obrigatórios precisam ser setados antes de `build()`; setter consome `self`.
- Consequência: alternativa a macro de atributo para primitivos com >4 parâmetros; nossa decisão (02 §7) é função com parâmetros obrigatórios primeiro — mais simples e sem trait extra. Fica registrado como opção se `Field` estourar.
- Veredito: refina (backlog, não v1).

### A15. `Child` é sempre encaixotado; `Slot` é alias
- Citação: `06-componentes.md:91` — "`Child` é **sempre encaixotado** (`Box::pin`), inclusive `Child::default()`."
- Semântica: composição de filhos custa alocação por child; `Slot<'a>` é o mesmo tipo em layout.
- Consequência: nós temos `impl Render` e `Markup` já renderizado (sem box) — superior em `09 §1`; manter `Markup` como parâmetro de filho único evita o custo que `14:61` documenta.
- Veredito: refuta para nosso caso; confirma P1.

### A16. Irmãos renderizam concorrentes; ordem de execução é indefinida
- Citação: `06-componentes.md:163` — "A ordem de execução dos corpos é **indefinida**: não dependa de outro componente" (`06:165` — "O primeiro byte espera o **primeiro conteúdo de todos** os irmãos")
- Semântica: paralelismo com ordenação de saída, e uma barreira de primeiro byte (repetida em `14:34`).
- Consequência: é a fonte de duas classes de bug (efeito entre irmãos, TTFB) e exatamente o que P5/P9 eliminam: sem `async`, sem ordem indefinida, `html!` é determinístico por construção.
- Veredito: refuta; confirma P5/P6.

### A17. Identidade é derivada do call site; em laço exige `#[key]`, senão panic
- Citação: `06-componentes.md:182` — "Cada call site tem identidade estável derivada de arquivo/linha/coluna/ordinal." (`06:192` — "Sem key, consumir identidade (sinais, `element_id`, regiões live) entra em")
- Semântica: determinismo de `id` por posição lexical; laço sem chave é erro **de runtime**.
- Consequência: podemos ter o melhor dos dois: `id` derivado do call site **e** erro de compilação quando o call site está dentro de `for` (N2 vê o bloco inteiro, D8).
- Veredito: refina (adoção com guarda de compilação, §5 item 3).

### A18. `element_id(cx)` produz `tc-<identidade>` para `label for`/`id`
- Citação: `06-componentes.md:195` — "`element_id(cx)` gera `tc-<identidade>`; estável entre renders do mesmo"
- Semântica: o par `label`/`input` sai do mesmo cálculo, sem id manual.
- Consequência: nosso `Field::text(...)` gera `label for` + `id` internamente (05 §3); o que falta é o nome do id ser **previsível e documentado** para `aria-describedby` e testes de snapshot.
- Veredito: confirma D10; refina 05 §3.

### A19. Status e headers podem ser nós de view — e somem ao renderizar para `String`
- Citação: `04-request-response.md:114` — "Primeiro status vence; para cada header, a primeira declaração fornece todos os" (`05:219` — "Renderizar para string descarta metadados; use `render_response`/`.single()`")
- Semântica: status viaja dentro do corpo da view, com regra de precedência e um modo silencioso de perda.
- Consequência: é a armadilha que D11 elimina: em `html-auy` status só existe em `Page`, e `Markup` não tem conversão para resposta; nenhum caminho "renderiza para string" existe.
- Veredito: refuta o mecanismo; confirma D11.

### A20. Escaping é por **posição**, com três tabelas de três caracteres
- Citação: `05-views.md:226` — "| texto | `&`, `<`, `>` | `HtmlContext::Text` |" (`05:227` — "| valor de atributo | `&`, `"` | `AttributeValue` |")
- Semântica: texto escapa 3, atributo escapa 2, comentário escapa 3 (e `<` passa); nome de tag/atributo não escapa, valida e dá panic (`05:229`).
- Consequência: nossa tabela (02 §3) é superconjunto estrito (5 caracteres em texto e atributo). Nada a corrigir; cabe registrar por que excedemos — e que comentário HTML não existe para nós.
- Veredito: confirma D1/D2 (contexto é tipo, não tabela de posição).

### A21. Quatro escotilhas de markup confiável
- Citação: `05-views.md:234` — "Escotilhas de confiança: `Unescaped::new_unchecked`," (`13:74` — "**Não existe `raw!` para HTML.** `raw!` é JavaScript de runtime.")
- Semântica: `Unescaped`, `ViewHandle::unescaped_unchecked`, `push_*_unescaped`, `IconData::unescaped_unchecked` são funções **safe**; o compilador não impede XSS.
- Consequência: quatro portas é quatro vezes o problema de D2; `html-auy` não tem nenhuma, e a necessidade legítima (AST de texto rico) é resolvida nó a nó no vocabulário.
- Veredito: refuta; confirma D2.

### A22. Acessibilidade é explicitamente "o HTML é seu"
- Citação: `05-views.md:281` — "Topcoat não gerencia `<head>` nem a11y por você; o HTML é seu."
- Semântica: a doc dá **conselhos** (label com `element_id`, `aria-live` em região, mover foco após navegação, `lang`/`dir` no `<html>`) e `alt="Ferris"` como string livre.
- Consequência: confirma P4 por contraste: eles listam como disciplina o que nós tornamos ausência de API (`img` sem `Alt` não compila, `nav` sem `Label` não compila).
- Veredito: confirma P4/D10; é a maior lacuna do corpus.

### A23. `suspense`: o fallback nunca aparece se o filho está pronto
- Citação: `07-streaming.md:40` — "O filho é pollado primeiro; se já estiver pronto, o fallback **nunca** aparece." (`07:45` — "Views dentro do suspense são isoladas; status/headers já enviados não mudam")
- Semântica: `Stream` (default) mantém região e envia frames; `Wait` espera e não cria marcadores.
- Consequência: fora do escopo (exige swap de DOM), mas o aviso "status já enviado não muda" é a razão pela qual `Page` recebe `status` **antes** do corpo (06 §1) e pelo qual marcadores em comentário não existem para nós (P1).
- Veredito: irrelevante como API; reforça P1/D11.

### A24. `error_boundary` devolve 200 se o fallback não declarar status
- Citação: `07-streaming.md:85` — "Declare `(StatusCode::…)` no fallback; sem isso a resposta é 200." (`07:86` — "Só substitui conteúdo se o filho for live/streaming")
- Semântica: erro tratado em composição rebaixa silenciosamente o código HTTP.
- Consequência: em `html-auy` `Page::new(ctx, main, status)` **exige** status; o teste espelhado é `06 §9` ("tema com valor inválido é ignorado" mas status nunca é default). Adicionar fixture: página de erro tem status ≠ 200.
- Veredito: confirma D11; gera item de teste.

### A25. `emit!` substitui; o corpo precisa emitir ao menos uma vez
- Citação: `07-streaming.md:110` — "Cada `emit!` **substitui** o conteúdo atual da região no browser (não acrescenta)." (`07:119` — "Depois do streaming começar, status/headers não mudam; cookies tardios panicam")
- Semântica: streaming exige placeholder, token de tipo `EmitToken` e disciplina de "emitir o estado completo".
- Consequência: prova empírica de que streaming contamina o tipo de retorno, os cookies e o status — três coisas que `Markup` puro não tem. Mantém-se fora (P1/P5).
- Veredito: refuta; confirma D11/D13.

### A26. `connected(cx)` abre WebSocket e o corpo `live!` reinicia no reconnect
- Citação: `07-streaming.md:168` — "Durante o HTTP, `connected(cx)` devolve `false` e **pede** que o browser abra um" (`07:173` — "O corpo `live!` **reinicia** no reconnect")
- Semântica: um WS por aba multiplexa regiões, 64 renders por conexão, 429 no excedente.
- Consequência: custo de servidor invisível no markup; nosso contrato "tudo funciona sem JavaScript" (07 §1) exclui o mecanismo inteiro.
- Veredito: irrelevante para a crate; útil como argumento de escopo.

### A27. `$()` é um vocabulário fechado com paridade Rust↔JS testada
- Citação: `08-runtime.md:98` — "<p>$(format_like(count.get()))</p>   // ❌ fora do vocabulário" (`08:135` — "Paridade Rust↔JS é checada por `crates/topcoat-runtime/coherence`")
- Semântica: expressão única avaliada nos dois lados, com harness V8 e limitações conhecidas (floats não finitos, `Option<()>`, `return` em `if`).
- Consequência: é o custo máximo de D3: para ter interatividade, o framework mantém **dois interpretadores** e um corpus de coerência. Nosso `cq` não pode herdar nada disso.
- Veredito: refuta; confirma D3.

### A28. Sinais não têm `set` no servidor e "nunca confie no valor restaurado"
- Citação: `08-runtime.md:83` — "**No servidor não há `set`**: mutação só existe no vocabulário de expressões; se" (`08:90` — "roda silenciosamente. **Nunca confie no valor restaurado.**")
- Semântica: estado do cliente é input; desserialização falha e o `init` roda sem avisar.
- Consequência: confirma 02 §1 ("um `String` que chega de fora não vira HTML") e o modelo de ameaça de 08 §1: estado do cliente não é dado.
- Veredito: confirma P2/D1; irrelevante como API.

### A29. `:attr` faz bind e `value` estático é só default do usuário
- Citação: `08-runtime.md:189` — "`:atributo=$(expr)` faz bind reativo; emite `data-topcoat-bind:<attr>`." (`08:190` — "`value`/`checked`/`selected` estáticos são **defaults**")
- Semântica: atributo estático perde para o que o usuário digitou; para forçar valor usa-se `Expr::dynamic()`.
- Consequência: no nosso desenho não há diferença entre "default" e "valor": o HTML é o artefato e o servidor decide. É uma classe de bug que nunca existirá; o `data-*` inventado (`08:200` "`data-topcoat-on:<evento>`") confirma a regra de `03 §6` (só `data-*` declarados).
- Veredito: refuta; confirma 03 §6.

### A30. `#[record]` envia **todos** os campos ao browser, inclusive privados
- Citação: `08-runtime.md:222` — "**Todos os campos são enviados ao browser, inclusive privados.** Não coloque"
- Semântica: segredo em struct capturada vaza por serialização; campos reservados (`__proto__`, `toJSON`, `then`) são rejeitados.
- Consequência: lista de proibições que a nossa crate não precisa ter porque não serializa estado (P1). Vira item de checklist do consumidor (nada de segredo em `data-*`).
- Veredito: irrelevante para a crate; confirma P1.

### A31. `#[procedure]` tem path instável e exige autorização própria
- Citação: `08-runtime.md:259` — "Endpoint é POST JSON; path gerado por arquivo+linha+coluna+nome — **instável**." (`08:264` — "**Valide e autorize dentro da procedure.** Guards de página não rodam.")
- Semântica: endpoint derivado de posição no arquivo; mover a função quebra contrato; autorização não é herdada.
- Consequência: "rota = posição no código" é incompatível com contrato de cache/URL; e "guard não herda" é o oposto do nosso shell único (D11). Nada entra.
- Veredito: refuta; confirma D11/D8.

### A32. `#[shard]` renderiza inline, troca HTML in-place e tem as mesmas isenções
- Citação: `08-runtime.md:289` — "Render inicial **inline** na página (sem request extra)." (`08:296` — "**Guards de página/layout não rodam.**")
- Semântica: região re-renderiza no servidor quando sinal muda, preservando foco/scroll; path gerado muda ao mover a função (`08:298`).
- Consequência: é o "componente que busca dados" que P9 recusa; e a preservação de foco documentada em `05:284` mostra que a11y pós-swap é problema do framework, não do markup.
- Veredito: refuta; confirma P9.

### A33. `asset!` é constante tipada com hash no nome
- Citação: `11-assets-estilo.md:12` — "const FERRIS: Asset = asset!("./ferris.png");" (`11:23` — "Renderiza `/_topcoat/assets/ferris-<16hex>.png`")
- Semântica: fontes de asset: relativo ao arquivo, manifesto, absoluto, remoto (`11:31-33`); opções `rename`, `checksum`, `content_type` (`11:47-49`).
- Consequência: é o desenho de `ctx.require(Asset)` + `Src::asset(Asset)` (07 §2), com um extra que vale copiar: `content_type` explícito no construtor evita `mime_guess` divergindo de `Content-Type` servido.
- Veredito: confirma D13; refina 07 §2 (§5, item 9).

### A34. O bundle é descoberto varrendo o binário, por perfil, e ausência é panic
- Citação: `11-assets-estilo.md:53` — "`asset!` grava uma declaração no binário; o bundler **varre o binário**." (`11:68` — "**Cada perfil tem seu bundle.**"; `11:87` — "Asset não presente no catálogo → panic no render")
- Semântica: link-time discovery via seção do binário; bundle de outro perfil serve 404/pânica.
- Consequência: referência indireta para o nosso gate de assets (06 §9: "Asset declarado e não existente → erro no `cq check`"); a diferença é diagnóstico em build, não panic em request.
- Veredito: refina 06 §9.

### A35. Assets servidos: cache imutável de 1 ano, sem ETag, sem Range, lendo o arquivo inteiro
- Citação: `11-assets-estilo.md:120` — "Sem ETag/304, sem `Range`, sem variantes pré-comprimidas." (`11:121` — "Cada request lê o arquivo inteiro em memória (`tokio::fs::read`).")
- Semântica: o asset system só compensa atrás de CDN; sem ele, cada request aloca o arquivo.
- Consequência: nossa decisão "sem CDN, tudo vendorizado" (07 §8) continua válida porque o servidor é o do consumidor; mas o `ETag`/304 e o pré-comprimido com hash **precisam** existir no nosso pipeline, e o corpus mostra que não é de graça.
- Veredito: refina 07 §8 e 09 §3.

### A36. SRI não é gerado: o hash no nome não é `integrity`
- Citação: `11-assets-estilo.md:126` — "**SRI não é gerado**: o hash no nome não serve como `integrity`." (`13:221` — "SRI para scripts de CDN: adicione manualmente (`integrity`).")
- Semântica: escopo do hash (nome de arquivo) e escopo do SRI (conteúdo verificado pelo browser) são diferentes; sobra para o chamador.
- Consequência: nossa regra de `integrity` obrigatório para asset enviado pelo usuário (07 §8) deve ser **gerada pelo pipeline**, não escrita à mão: `Asset::integrity()` calculado no build e exigido pelo tipo quando a origem não é própria.
- Veredito: confirma 07 §8; refina (tipo novo, §5 item 10).

### A37. Tailwind: binário baixado sem verificação de checksum e scanner de classe literal
- Citação: `11-assets-estilo.md:158` — "O binário Tailwind é baixado pela rede (versão fixa; checksum não é verificado" (`11:154` — "O scanner detecta **classes literais**")
- Semântica: build depende de rede e de nomes literais; `format!` produz classe que não existe no CSS final.
- Consequência: dois argumentos para o nosso modelo (07 §8: "Sem CDN. Sem `latest`. Tudo vendorizado") e para `Class` ser enum fechado: token montado dinamicamente não existe em `html-auy` por construção.
- Veredito: refuta; confirma 02 §5/D12.

### A38. Fontes: `@latest` por default, 18 faces sem `weight`, `crossorigin="true"` inválido
- Citação: `11-assets-estilo.md:191` — "Sem `host: Asset`, usa jsDelivr `@latest` — conteúdo mutável numa URL imutável." (`11:195` — "`crossorigin="true"` emitido pelo `preload_link` é um valor inválido de CORS")
- Semântica: defaults inseguros e um bug de atributo aceito pela API; sem `weight` são 18 `preload`.
- Consequência: nosso `@font-face` sai do pipeline local; a lição direta é o item já aceito em `README.md` ("`crossorigin="anonymous"`") e o limite "≤ 1 família, ≤ 2 pesos" (web-performance).
- Veredito: refuta; confirma 07 §8.

### A39. `IconData::unescaped_unchecked` desliga escaping **e** validação
- Citação: `11-assets-estilo.md:230` — "`IconData::unescaped_unchecked` desliga escaping/validação: só com SVG"
- Semântica: para suportar SVG arbitrário, abriu-se a quinta escotilha.
- Consequência: nosso `Icon` é primitivo com SVG do vocabulário (05 §3) e `aria-hidden` fixo; nenhum caminho aceita SVG de fora — o que também elimina a necessidade de sanitizador.
- Veredito: refuta; confirma D2/P4.

### A40. `mail!` reaproveita `view!`, mas texto puro derivado do HTML é best-effort
- Citação: `10-servicos.md:81` — "Texto puro derivado do HTML é best-effort, não sanitizador." (`10:78` — "`Receipt` contém `Message-ID`: significa que o transporte aceitou, **não** que")
- Semântica: um macro, dois perfis de conteúdo; `send` é `await` no handler, sem fila/retry (`10:80`).
- Consequência: confirma o item de `README.md` (perfil de elementos próprio + `text:` explícito). Custo de tipo: `Markup` de e-mail precisa de vocabulário restrito, não do mesmo tipo sem restrição.
- Veredito: confirma; refina 01 §6 (variante `EmailMarkup` no futuro, nunca `Raw`).

### A41. i18n: locale em layout não alcança a página; `t!` não valida chaves
- Citação: `10-servicos.md:127` — "**Criar o locale num layout não alcança a página**; use uma `#[layer]`." (`10:165` — "`t!` não valida chaves/args em compile-time")
- Semântica: locale vive no contexto da request e precisa de layer; mensagens são resolvidas em runtime por chave string.
- Consequência: confirma `Msg` como enum gerado com paridade obrigatória (02 §6) e locale no `ShellCtx` (06 §1) — dois erros que o tipo impede.
- Veredito: confirma D1/02 §6.

### A42. CSP estrita é incompatível com o runtime: `unsafe-inline` + `unsafe-eval`
- Citação: `13-seguranca.md:196` — "Consequência: `script-src` precisa de `'unsafe-inline'` e `'unsafe-eval'` nas" (`13:198` — "streaming nessas rotas ([INFERENCE] não há caminho documentado para nonce).")
- Semântica: streaming injeta script inline e expressões viram `new Function`.
- Consequência: é o custo que D3 evita; nosso `script-src 'self'` sem nonce (08 §6) só é possível porque não há inline **nem** eval. Também explica por que `hx-on`/`x-on` estão fora da spec (03 §5).
- Veredito: confirma D3; refuta qualquer runtime.

### A43. `OriginPolicy` cobre métodos unsafe; GET fica descoberto e `exempt_paths` ignora método
- Citação: `13-seguranca.md:86` — "`GET`/`HEAD`/`OPTIONS` — **GET que muda estado é vulnerável**;" (`13:88` — "`exempt_paths` é independente de método: eximir um path de WS também exime POST.")
- Semântica: proteção por origem em vez de token; exceção é grossa.
- Consequência: valida a regra de 05 §2 ("não existe `on*`; mudança só por submit") e a de `03`: mutação só por método unsafe. Para nós, o ganho é estrutural: sem `on*` e sem fetch no markup, não há mutação por GET a inventar.
- Veredito: confirma P4/03 §5.

### A44. Shards/procedures são rotas e **não herdam** guards de página ou layout
- Citação: `13-seguranca.md:5` — a linha diz que "shards/procedures não herdam guards, não há token" (`00:114` — "**Segurança não é herdada por composição.**")
- Semântica: o risco S1 do ranqueamento (`13:228`) é "shard/procedure sem auth — severidade alta".
- Consequência: é o argumento decisivo contra qualquer API de "componente que busca dados": cada superfície nova é uma nova rota a autorizar. `html-auy` não cria rotas (P5), então a classe de bug não existe.
- Veredito: confirma P5; refuta shards.

### A45. Cookies: jar puro sem atributos, selagem no streaming, `Key::generate()` por processo
- Citação: `09-cookies-sessoes.md:49` — "O jar puro **não** aplica `Secure`/`HttpOnly`/`SameSite`/`Path`. Centralize:" (`09:42` — "o jar é selado: `add`/`remove`/`commit` **panicam**"; `09:145` — "`Key::generate()` a cada arranque invalida todos os cookies entre restarts")
- Semântica: defaults inseguros por omissão; streaming sela o header; chave efêmera desloga todo mundo.
- Consequência: nada disso é da crate, mas o padrão "default seguro + override explícito" é o mesmo de `Alt::decorative()` e `Label::sr_only()` (05 §6): o inseguro nunca é o default.
- Veredito: confirma P4 (por analogia); irrelevante como API.

### A46. Sessão: `token_hash` não valida nada e `rotate` precisa apagar o revogado
- Citação: `09-cookies-sessoes.md:236` — "`token_hash` **não valida** nada: só devolve o hash do token apresentado." (`09:240` — "`rotate` devolve `{ revoked, session }`; sem apagar `revoked`, o token antigo")
- Semântica: o framework entrega o primitivo e deixa a validação (existência + expiração) para o app.
- Consequência: registra a fronteira de 08 §7 ("Autenticação, CSRF, rate limit — do consumidor") — coerente com o nosso não-objetivo de framework.
- Veredito: irrelevante para a crate; confirma 08 §7.

### A47. O pipeline de render: literais fundidos e view estática promovida
- Citação: `14-performance.md:11` — "Uma view totalmente estática vira `ViewHandle::unescaped_unchecked("…")` — sem" (`14:15` — "Números até `u64`/`isize` usam buffer fixo (sem heap)"); `14:19` — "Buffer é thread-local"
- Semântica: escape em compile-time, buffer por thread, `size_hint` pré-calculado.
- Consequência: é exatamente a meta de `09 §1` (uma alocação por bloco, sem buffer intermediário); o número `14:61` ("`Child` | um `Box::pin` por child") mostra onde eles pagam e nós não.
- Veredito: confirma 09 §1/P6.

### A48. Testes: `Router::handle` + `CxTestBuilder`, e `.single()` panica em view live
- Citação: `15-testing.md:3` — "Topcoat não traz um test client dedicado, mas expõe tudo o que é preciso:" (`15:56` — "views com `live!`/`suspense` são live; `.single()` panic.")
- Semântica: render puro é síncrono e barato; streaming só por HTTP; envelope de shard é interno.
- Consequência: o nosso harness (12 §2-§3) é mais forte (golden de bytes por elemento); o que vale copiar é a proibição de teste fraco em `15:139` ("Não escreva testes que só verificam que "compila"").
- Veredito: confirma 12; refina §3 (adicionar fixture de status em erro).

### A49. `cargo fmt` não formata macro: o formatter é produto separado
- Citação: `01-quickstart.md:176` — "`cargo fmt` **não** formata o interior de macros. Formate os corpos com:" (`15:201` — "`topcoat fmt --check` é obrigatório se o projeto usa macros formatadas.")
- Semântica: formatter próprio, modo `--stdin`/`--check` para editor e CI, `--macros` por lista.
- Consequência: valida D6 e a existência de `cq fmt`; o detalhe operacional a copiar é a ordem (rustfmt roda antes, ambos precisam passar) e o modo `--stdin` para `rust-analyzer` (10 §9).
- Veredito: confirma D6; refina 10 §3.

### A50. Panics são o mecanismo de erro para erro de marcação/programação
- Citação: `02-roteamento.md:88` — "Ler `path_param::<T>` de um nome não declarado **entra em panic** (500)." (`02:168` — "Duplicata de path+método faz panic no `build()`")
- Semântica: panic em build ou no primeiro render; `02:344` resume: "trate como erro de programação, não de usuário".
- Consequência: é a diferença de grau que define P7: nós movemos esse mesmo conjunto de erros para `cargo check`, com span e `trybuild`; e o `20-faq` inteiro é a prova de que panic é diagnóstico ruim.
- Veredito: refuta; confirma P7/D8.

### A51. O roadmap aberto diz o que não é problema de HTML
- Citação: `21-padroes-avancados.md:279` — "autenticação/RBAC/MFA pronta;" (`21:281` — "OpenAPI;"; `21:283` — "export estático/pré-render;"; `SKILL.md:19` — "jobs, or OpenAPI.")
- Semântica: o framework reconhece que auth, validação, jobs, OpenAPI, export estático, imagem e Markdown estão fora.
- Consequência: confirma a lista de não-objetivos permanentes (00 §3) e o corte de escopo de 01 §1: nada de framework, nada de rota, nada de estado.
- Veredito: confirma 00 §3/D0.

### A52. Governança: "se a doc e o código divergirem, o código vence"
- Citação: `19-melhores-praticas.md:133` — "Se a doc e o código divergirem, o código vence; registre a divergência."
- Semântica: a doc é derivada do código, sem teste de conformidade documental.
- Consequência: nossa D5 é o inverso (spec é dado, código é gerado, `cq spec` valida) e 12 §6 exige conformidade spec↔render. O corpus é a evidência de que documentação não versionada como dado apodrece.
- Veredito: refuta; confirma D5.

## 3. Confirmações do desenho

- **Contexto de escape é o tipo, não a posição sintática** — `13:27` admite que o escape de `view!` "protege contra quebrar a sintaxe do documento. Ele não valida semântica", com tabela de lacunas para URL/CSS/JS/`srcdoc`/nome de atributo. Confirma D1 e P2.
- **Atributo não validado é o buraco de todos os frameworks** — `05:45` aceita `hx-get`, `xmlns:xlink`, qualquer nome; A22/`05:281` deixa `alt` como string. Confirma o quadrante de 14 §1 e D4.
- **`Markup` como único renderizável é superior ao handle mutável deles** — `15:56` e `05:219` mostram que `String` derivada de view perde metadados; nós não temos essa conversão. Confirma P1/D11.
- **Determinismo de bytes como contrato** — `05:114` (HashMap sem ordem) e `11:120` (sem ETag) mostram o custo de não ter ordem canônica. Confirma P6/D14.
- **Assets declarados por tipo com hash de conteúdo** — `11:12` + `11:23`. Confirma D13/07 §2.
- **`id` derivado do call site** — `06:182`/`06:195`. Confirma a adição 4 de `README.md`, com a correção de laço (§4 abaixo).
- **Formatação própria é obrigatória para macro** — `01:176` + `15:201`. Confirma D6/P8.
- **Acessibilidade tem de ser emitida, não documentada** — `05:281` lista conselhos; `05 §3` da nossa spec lista primitivos. Confirma P4/D10.
- **Zero `unsafe` é viável em framework de HTML** — `13:4` ("nenhum `unsafe`"). Confirma D9.
- **`<script>` inline + `new Function` é o preço do runtime** — `13:196`. Confirma D3 e a CSP de 08 §6.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata → arquivo de html-auy) |
|---|---|---|
| texto escapa `& < > " '` (02 §3) | texto escapa `& < >`; atributo escapa `& "` (`05:226-227`) | acrescentar em `docs/02-tipos-e-contextos.md` §3: "Escapamos também `"` e `'` em texto e `\r\n\t` em atributo: o custo é zero e a tabela deixa de depender de o valor nunca ser reinterpretado." |
| `Class::of(&[Token])` sem regra para lista vazia (02 §5) | "lista vazia omite o atributo" (`05:130`) | acrescentar em `docs/02-tipos-e-contextos.md` §5: "Lista vazia não emite `class`; `class=""` é impossível por construção." |
| gramática de `html!` sem atributo condicional (10 §1) | `if cond { aria-current="page" class="active" }` dentro da lista (`05:86`) | acrescentar em `docs/10-ergonomia.md` §1: "`@if cond { … }` existe **somente** na lista de atributos de um elemento e emite apenas atributos validados pela spec; em posição de nó, use `@if` com elementos." |
| `id` derivado do call site permitido em qualquer bloco | identidade exige `#[key]` e dá panic em laço (`06:192`) | acrescentar em `docs/04-validacao.md` §4 (N2): "`id` derivado do call site dentro de `for` é erro de compilação; use `id:` explícito derivado do dado — nunca um `id` que se repete na página." |
| status e cabeçalhos só em `Page` (D11) | status como nó de view, com perda silenciosa ao renderizar para string (`05:219`) | acrescentar em `docs/06-shell-e-tema.md` §1: "`Markup` não tem conversão para resposta e não carrega status: a única forma de publicar HTML é `Page::new(ctx, main, status)`." |
| nada dito sobre spread de atributos (02 §8) | spread é `HashMap`, ordem instável, `class`/`CLASS` coexistem (`05:114`) | acrescentar em `docs/02-tipos-e-contextos.md` §8: "Não existe spread de atributos: atributo é sempre nomeado no bloco e validado contra a spec (D4); nenhuma ordem é decidida em runtime." |
| `let` não existe na gramática, sem regra escrita | `let` dentro do bloco, `let … else` não suportado (`05:84`) | acrescentar em `docs/10-ergonomia.md` §1: "Cálculo acontece antes do `html!`; `let` dentro do bloco não existe (P9), e nada de `let … else`." |
| atributo booleano tratado como `AttrVal` | `disabled="false"` desabilita (`05:110`, `20:70`) | acrescentar em `docs/03-vocabulario-de-elementos.md` §5: "Atributo booleano é presença: `disabled` recebe `true`/`false` do tipo, e literal (`disabled="false"`) é erro de compilação." |
| `integrity` só para asset enviado pelo usuário (07 §8) | SRI nunca é gerado; o hash do nome não serve (`11:126`) | acrescentar em `docs/07-js-e-assets.md` §8: "Todo `<script>`/`<link>` cuja origem não seja a própria build emite `integrity` calculado no build; `Asset::integrity()` é obrigatório no tipo, não opcional no chamador." |
| `Asset` não declara `content_type` nem `checksum` (07 §2) | `content_type` sobrescreve `mime_guess`; `checksum` falha o bundle (`11:48-49`) | acrescentar em `docs/07-js-e-assets.md` §2: "`Asset` carrega `content_type` explícito e `checksum` do conteúdo; `Src::asset` só aceita `Asset` já verificado." |

## 5. Adições prontas para colar

**Dez melhorias de sintaxe/API para o nosso macro, cada uma com a citação que a inspira.**

1. **`@if` na lista de atributos** (nunca em posição de nó) — inspira `05-views.md:86`: "`if`/`for`/`match` em atributos emitem **atributos**, não nós". Reduz aninhamento de elemento e mantém a allowlist: cada atributo do bloco é validado normalmente.
2. **Booleano por presença com literal proibido** — inspira `05-views.md:110` e `20:70`. Em `spec/elements.toml`: `disabled`, `required`, `readonly`, `multiple`, `checked`, `open`, `inert`, `novalidate`, `autofocus` passam a `kind = "presence"`.
3. **`id` do call site com guarda de compilação** — inspira `06-componentes.md:182` ("Cada call site tem identidade estável derivada de arquivo/linha/coluna/ordinal"). Diferença: laço é erro de compilação, não panic (`06:192`).
4. **Chamada de primitivo com nomeados + filhos sem vírgula** — inspira `05-views.md:176`: "Nomeados com `nome: valor`; filhos depois, sem vírgulas". Aplicar em `Dialog`, `Field`, `Table`, `Picture`.
5. **`Class` com condicionais e omissão de vazio** — inspira `05-views.md:130`: "Entradas ausentes (`None`, `false`) são puladas; lista vazia omite o atributo". Com tokens fechados, o merge continua determinístico.
6. **Estado de link como fonte única** — inspira `02-roteamento.md:265`: "Bom para `aria-current` e `class!`". Em `LUrl`: `fn state(&self, ctx: &Ctx) -> LinkState`, e `Link` primitivo deriva `aria-current="page"` + token ativo do mesmo cálculo.
7. **Nome de atributo nunca dinâmico, por decisão gramatical** — inspira `05-views.md:105` (`(key)=(value)`) e `13:62` ("Nome inválido derivado de input → panic → 500"). Nossa gramática simplesmente não tem produção para isso; o erro é de sintaxe, com dica.
8. **`@if` de bloco não cria escopo; cálculo antes do macro** — inspira `05-views.md:84` ("`let … else` não é suportado"). Regra: `html!` não contém `let`; `let` antes, uma vez (P9).
9. **`Asset` com `content_type` + `checksum` no construtor** — inspira `11-assets-estilo.md:48-49`: "`checksum`: falha o bundle se o arquivo remoto mudar" / "`content_type`: sobrescreve o `mime_guess`".
10. **`Asset::integrity()` obrigatório em origem externa** — inspira `11-assets-estilo.md:126`: "**SRI não é gerado**: o hash no nome não serve como `integrity`". O tipo passa a carregar `sha384` e o shell emite `integrity` + `crossorigin="anonymous"`.

**Tabela: prática do topcoat → entra? → por quê**

| Prática (com citação) | Entra? | Por quê |
|---|---|---|
| `@if` em lista de atributos (`05:86`) | **sim** | economiza aninhamento sem abrir allowlist |
| booleano por presença, literal proibido (`05:110`) | **sim** | elimina `disabled="false"` por tipo |
| `class!` com condicionais e vazio omitido (`05:130`) | **sim** | determinismo + nenhum `class=""` |
| nomeados + filhos sem vírgula (`05:176`) | **sim** | ergonomia de primitivo, sem custo de tipo |
| `id` por call site (`06:182`) | **sim, com guarda** | laço precisa ser erro de compilação, não panic |
| `element_id` para `label for` (`06:195`) | **sim** | já é o que `Field` faz; falta documentar o formato |
| erro de atributo desconhecido listando válidos (nossa `03` §7) | **sim** | já existe; o corpus confirma que é a melhor mensagem |
| `href!` com params por nome e `is_current` (`02:265`) | **tipo sim, macro não** | `LUrl` + `LinkState` bastam; roteador não é nosso |
| `asset!` como constante com hash (`11:12`) | **sim** | é o nosso `Asset`/`Src::asset` |
| `content_type`/`checksum` no asset (`11:48-49`) | **sim** | fecha divergência `mime_guess` ↔ `Content-Type` |
| bundle por perfil descoberto no binário (`11:53-68`) | **não** | nosso registro é a spec + `cq check`; sem varredura de binário |
| `suspense`/`live!`/`emit!` (`07:40-110`) | **não** | exige swap de DOM; mata `ETag` e "sem JS" |
| `$()`/`@event`/`:bind` (`08:98`) | **não** | D3: CSP sem eval, estado não mora no HTML |
| `#[shard]`/`#[procedure]` (`08:289-264`) | **não** | P5/P9; rota nova = autorização nova |
| `#[record]` (`08:222`) | **não** | serializa estado (e segredos) no cliente |
| spread `(attrs)` / `Attributes: HashMap` (`05:114`) | **não** | ordem instável, `class`/`CLASS`, allowlist furada |
| nome de tag/atributo dinâmico (`05:58`) | **não** | panic em request; P3/P7 |
| `Unescaped`/`*_unescaped` (`05:234`) | **não** | D2 |
| status/headers como nós (`04:114`) | **não** | D11: `Page` é o único caminho |
| prefetch é render completo (`08:325`) | **não** | carga de servidor invisível no markup |
| formatter próprio com `--stdin --check` (`01:176`) | **sim** | já é `cq fmt`; copiar o modo de editor |

## 6. Armadilhas de complexidade

- **Runtime Rust↔JS com paridade testada** (`08:135`): dois interpretadores, corpus de coerência em V8 e limitações conhecidas. Custo: um segundo produto para manter; entra em conflito direto com D3 e com a CSP de 08 §6.
- **Streaming com regiões e substituição de DOM** (`07:110`): exige placeholder, `EmitToken`, script inline e status decidido antes do primeiro byte. Custo: `sha256` do corpo deixa de existir → sem `ETag` (P6/D14 morrem juntos).
- **Concorrência com barreira de primeiro byte** (`06:165`): ordem de execução indefinida produz bugs não reproduzíveis e obriga `suspense` em todo I/O. Custo: diagnóstico e teste; `html!` síncrono não tem essa classe.
- **Spread de atributos** (`05:114`): `HashMap` sem ordem, `class`/`CLASS`, atributo fora da allowlist por construção. Custo: P6 e D4 caem; um único `(attrs)` reabre todos os typos.
- **Escotilhas de markup confiável** (`05:234`): quatro caminhos `*_unescaped`, todos `safe`. Custo: auditoria permanente e `grep`-gate; D2 elimina a categoria.
- **Path de endpoint derivado de linha/coluna** (`08:259`, `08:298`): mover uma função muda URLs públicas. Custo: cache, testes e links quebrados sem diff de contrato.
- **Descoberta link-time via varredura do binário** (`11:53`): registro fora do código versionado, bundle por perfil. Custo: build não reprodutível entre perfis; binário e assets viram par acoplado.
- **Download de ferramenta no `build.rs` sem checksum** (`11:158`): build depende de rede e de conteúdo mutável. Custo: CI offline quebra e a cadeia de suprimentos fica aberta.
- **`#[record]` como transporte de estado** (`08:222`): todos os campos, inclusive privados, vão ao cliente. Custo: vazamento por refactor inocente de struct.
- **Formato de envelope de shard interno** (`15:162` — "Formato exato do envelope (args/signals/identity) é interno"): teste de contrato acoplado a detalhe interno. Custo: manutenção a cada versão.

## 7. Lacunas do corpus

- **Acessibilidade por construção**: o corpus só dá conselhos (`05:281`) e aceita `alt` como string; não há `img` sem `alt` como erro, nem `th scope`, nem nome acessível de `nav`. A crate decide sozinha (05 §2-§3).
- **Validação de atributo intrínseco**: `05:45-47` aceita nomes arbitrários e tags customizadas; nada no corpus sugere allowlist por elemento. O desenho inteiro de `spec/elements.toml` é nosso.
- **Escape de JSON-LD/URL**: `13:30` trata como lacuna ("correção: allowlist"); não há tipo `JsonLd` nem `Href`. Nossa D1 não tem precedente no corpus.
- **`srcset`/`sizes` tipados**: `05-views.md` mostra atributos como valores comuns; não há ordenação canônica nem par obrigatório. Nosso `SrcSet`+`Sizes` (02 §2) não é confirmado nem refutado.
- **`ETag`/304 de HTML e de asset**: `09 §2` da crate exige; `11:120` mostra que eles não têm nem para asset. Nenhuma pista de implementação no corpus.
- **Pré-compressão com hash**: a crate exige `.br`/`.gz` de build (09 §3); o corpus comprime por resposta (`14:102`), o oposto. Decidimos sozinhos.
- **Allowlist de origem externa no lint**: 07 §8/§9 exige; o corpus resolve por CDN configurado, sem gate. Sem precedente.
- **Paridade de tradução como gate**: `10:165` empurra para `topcoat l10n check` (ferramenta), não para o tipo. Nosso `Msg` com 4 locales é decisão própria.
- **Formato de erro com `span` + regra + ajuda**: `20-faq` mostra que eles diagnosticam por panic; nosso padrão de mensagem (10 §2) não vem do corpus.
- **Custo de compilação por tamanho de bloco**: `09 §5` (bloco > ~200 linhas deve ser dividido) não tem equivalente; o corpus não mede macro.
