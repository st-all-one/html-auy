# `mdn-web-api-docs` — guia v2.0 (base MDN, 2026-07-21)

Este conjunto (24 arquivos, 4.681 linhas) **não documenta Web APIs**. Documenta como se *escrevem
páginas de referência no MDN*: front matter YAML, macros KumaScript, ordem de badges, capitalização de
títulos, taxonomia de `page-type`. Foi produzido lendo o repositório `mdn/` (`/api/` e `/media/`), não
as especificações — portanto é um corpus **indutivo sobre prosa**, e o objeto de estudo é a forma da
página, nunca a semântica do elemento ou da interface. O que ele pode dar à crate é pequeno e
específico: uma disciplina de exemplo (sintaxe vs. código executável), um punhado de anti-patterns
editoriais que mapeiam para invariantes nossas (fonte única de verdade, sem acoplamento por string,
cross-link em vez de duplicata) e um catálogo de dependências que serve para decidir **o que a crate
não vai documentar**. O restante é ruído: 20 documentos sobre um sistema editorial que a crate não tem
(uma crate não tem sidebars, `page-type`, `browser-compat` nem `spec-urls`).

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `index.md` | 49 | Índice dos 20 documentos + metodologia ("1228+ entries", 25 lacunas) | Metadados; define a expectativa errada (parece Web API semantics) |
| `SKILL.md` | 483 | 20 seções de regras de redação MDN: front matter, anatomia, badges, macros, templates, anti-patterns, checklist | Único artefato com valor operacional direto: checklists e a distinção sintaxe/exemplo |
| `VERSION` | 4 | `MDN Web API Documentation Guide v2.0`, `Date: 2026-07-21` | Fixa a data-base; nenhum conteúdo técnico |
| `01-padrao-moderno-web-api.md` | 206 | Filosofia, estrutura de pastas, front matter, macros, relação slug↔pasta | Modelo de organização **editorial**; zero informação sobre elementos ou atributos |
| `02-semantica-arquitetura.md` | 182 | Princípios semânticos, hierarquia de 4 níveis, interdependências, "seções críticas" | Recomenda a seção `Security requirements` que o doc 16 proíbe |
| `03-boas-praticas-armadilhas.md` | 223 | 8 boas práticas + 8 anti-patterns com páginas reais citadas (WebGPU, media/) | Anti-patterns são de prosa e duplicação; §1.6 conflita com 16 |
| `04-template-modelo-base.md` | 321 | 5 templates completos (overview, guide, interface, método, propriedade) + ordem de elementos | Contém a seção proibida; ordem de elementos é convenção de página |
| `05-mapeamento-interdependencias.md` | 179 | APIs de fundação, cadeias (Shared Storage→Fenced Frame), matriz de compatibilidade | Mapa de **produto** (ad tech, mídia); serve só para dizer o que não cobrimos |
| `06-glossario-macros-tipos.md` | 179 | Catálogo de macros, 12 `page-type`, convenções de pasta e slug | Fonte do inventário de macros; `Request/request/` contradiz o doc 10 |
| `07-integracao-media-api.md` | 179 | Cobertura media/api, lacunas (WebCodecs, PiP, Screen Capture), novos guias | Plano editorial do MDN; nada transpõe |
| `08-front-matter-sidebar.md` | 123 | Dois mecanismos de sidebar: YAML (`mediasidebar`) vs macro (`DefaultAPISidebar`) | Lição aproveitável: um único mecanismo por seção, nunca dois |
| `09-template-eventos.md` | 164 | Template de página de evento, em português | Estrutura de evento; cabeçalhos PT violam a regra do doc 17 |
| `10-template-construtores.md` | 191 | Template de construtor; `new`, `options`, exceções, `js-nolint` | Distinção "sintaxe não executável vs exemplo executável" é o achado útil |
| `11-macros-adicionais.md` | 128 | Macros não documentadas, variantes de caixa, 4º valor de `AvailableInWorkers` | Inverte a ordem obrigatória de badges do `SKILL.md` |
| `12-front-matter-status.md` | 105 | Campo `status:` (array vs string) e sua relação com badges | Declara como correto duplicar estado em YAML **e** corpo |
| `13-convencoes-guias.md` | 121 | Convenções de subdiretório de guia; guias são 3,7% das APIs | Regra de tamanho (≈300 linhas → extrair guia) é aproveitável |
| `14-integracao-media-api-v2.md` | 102 | Correção/expansão da tabela do doc 07 | Só editorial; reforça "cross-link, não duplicar" |
| `15-mapeamento-api-grupos.md` | 224 | Grupos ausentes (Privacy Sandbox, PWA, Houdini, SVG, WebTransport) + matriz final | Mapa de dependências de produto; irrelevante para marcação |
| `16-seguranca-boas-praticas.md` | 124 | Segurança como **badge**, não seção; checklist de requisitos | Refuta explicitamente 02/03/04 — é o documento correto do próprio conjunto |
| `17-convencoes-titulos.md` | 108 | Sentence case, exceções, tabela de correção de títulos | A crate não escreve títulos em inglês: `Msg` é localizado |
| `18-embedlivesample-convencoes.md` | 210 | Mecânica do `{{EmbedLiveSample}}`: ID = slug do cabeçalho, case-sensitive | Acoplamento por string entre dois pontos distantes — exemplo a **evitar** |
| `19-spec-urls-anti-pattern.md` | 147 | Anti-pattern de múltiplos `spec-urls` (14 URLs na Performance API) | Regra "1 spec primária por entidade, sem versões antigas" transpõe |
| `20-page-types-complementares.md` | 169 | `page-type` adicionais, `landing-page` vs `listing-page`, validação | Taxonomia divergente de 06/`SKILL.md` (`web-api-method` vs `web-api-instance-method`) |
| `99-exemplo-completo.md` | 760 | Exemplo completo: 7 páginas da API fictícia `MediaProcessor` + checklist de conformidade | Único artefato operacional do conjunto — e viola o próprio checklist |

Nenhum arquivo do diretório foi omitido. Nenhum contém: tabela de atributos HTML, semântica de
elemento, regra de validação de formulário, regra de foco, requisito de acessibilidade de API, ou
qualquer linha de Rust.

## 2. Achados

### A1. O corpus é sobre redação de páginas MDN, não sobre Web APIs
- Citação: `SKILL.md:4` — "Especialista em documentação das páginas MDN Web API: front matter YAML,"
- Semântica: o `description` do próprio skill declara o escopo: front matter, anatomia de página, capitalização, macros, convenções de código, mapeamento de diretórios.
- Consequência: o conjunto não pode fundamentar nenhuma decisão da crate sobre elementos, atributos, escaping ou acessibilidade. Só pode fundamentar decisões sobre **como documentar** a crate.
- Veredito: **refuta** a expectativa de que `_RUST/mdn-web-api-docs` fosse fonte de semântica de API.

### A2. A instrução de papel no topo do skill é inequívoca
- Citação: `SKILL.md:21` — "You are a documentation specialist for MDN Web API pages. Follow these rules strictly."
- Semântica: o skill se instala como especialista em *páginas MDN Web API* — o gênero textual, não a tecnologia.
- Consequência: qualquer leitor futuro que procure "como `<dialog>` se comporta" fecha este diretório imediatamente; o custo de reabri-lo é uma perda de tempo recorrente.
- Veredito: **irrelevante** para a crate (mas é a razão de este documento existir).

### A3. Escopo declarado em metadado, não em prosa
- Citação: `SKILL.md:13` — "scope: documentation, web-api, mdn, front-matter, yaml, macros, templates"
- Semântica: `scope` lista seis temas, dos quais quatro são de formatação (`front-matter`, `yaml`, `macros`, `templates`).
- Consequência: serve como teste objetivo. Qualquer afirmação deste conjunto que não caia em um desses seis itens não tem base no corpus.
- Veredito: confirma A1 · irrelevante.

### A4. A base de evidência é o repositório, não a especificação
- Citação: `index.md:3` — "Este diretório contém uma análise aprofundada dos padrões modernos de documentação de Web APIs, baseada na revisão completa dos diretórios `/api/` (1228+ interfaces) e `/media/` (guias de mídia) do repositório MDN."
- Semântica: o método é indutivo sobre 1228 páginas do MDN, e o critério de verdade é "o que o repositório faz", não "o que a especificação diz".
- Consequência: tudo aqui é *convenção observada*, com validade estatística e sem validade normativa. O próprio corpus admite inconsistências no repositório (ver A15, A16).
- Veredito: refina — a crate trata convenção observada como advisory, nunca como spec.

### A5. Zero cobertura semântica: teste por busca de termos
- Citação: `index.md:37` — "Revisão de **1228+ entries** no diretório `/api/`"
- Semântica: apesar de afirmar revisão de 1228 entradas, buscas por `showModal`, `showPopover`, `popover`, `inert`, `reportValidity`, `ValidityState`, `requestSubmit`, `formAssociated`, `adoptedCallback`, `Intl.` retornam **zero** ocorrências em todo o diretório. `HTMLElement` só aparece como exemplo de macro de link (`SKILL.md:137`), e `{{HTMLElement("video")}}` como exemplo de sintaxe (`01-padrao-moderno-web-api.md:158`).
- Consequência: os elementos e interfaces que a crate precisa (`dialog`, `popover`, validação, `inert`, formatação localizada) **não têm uma linha** neste conjunto. A fonte é `html-moderno` (§7).
- Veredito: **refuta** em definitivo a hipótese de utilidade semântica.

### A6. A prioridade "conceito antes de referência" é de página, não de API
- Citação: `01-padrao-moderno-web-api.md:11` — "O padrão moderno prioriza esta ordem: **Conceito → Tutorial → Referência**. Primeiro explica-se o 'porquê' e o 'o quê', depois o 'como', e por fim os detalhes técnicos."
- Semântica: a ordem é a do documento (overview → guia → referência), um arranjo de navegação do leitor.
- Consequência: análogo legítimo na crate: docstring de primitivo começa pelo problema, depois a assinatura. Análogo ilegítimo: criar "páginas" — a crate não tem páginas de documentação geradas. [INFERENCE]
- Veredito: refina P9 (nada de açúcar que esconde preço) no plano da documentação interna.

### A7. O acoplamento por string do `EmbedLiveSample` é o anti-exemplo central do conjunto
- Citação: `18-embedlivesample-convencoes.md:24` — "O ID usado por `{{EmbedLiveSample}}` corresponde ao **slug do cabeçalho** da seção — a versão slugified do texto."
- Semântica: um macro no meio do corpo referencia, por texto, um cabeçalho que está em outro ponto do arquivo; a ligação só existe se o slug bater caractere a caractere.
- Consequência: é exatamente o defeito que `label for`/`aria-labelledby`/`popovertarget` têm em HTML e que a crate resolve validando a existência do `id` no bloco em tempo de compilação (D8). Confirma a decisão de **não** aceitar referência textual não verificada.
- Veredito: confirma D8 · confirma P3 (prevenção acima de detecção).

### A8. O próprio doc 18 não consegue cumprir sua regra de slug
- Citação: `18-embedlivesample-convencoes.md:28` — "1. O texto do cabeçalho é convertido para lowercase"
- Semântica: a regra diz lowercase, mas a tabela da mesma página (`:38`) e todos os exemplos usam maiúscula inicial: `{{EmbedLiveSample("Simple_request")}}` (`:58`), com o comentário "**ID**: `Simple_request` (slug do cabeçalho `### Simple request`)" (`:61`).
- Consequência: a regra é irrealizável como escrita — o corpus não consegue descrever seu próprio mecanismo em 200 linhas. Prova empírica de que acoplamento por string invisível gera ambiguidade insolúvel.
- Veredito: refuta a confiabilidade do conjunto como referência normativa; confirma D8.

### A9. Case-sensitivity é admitida como requisito de vigilância humana
- Citação: `18-embedlivesample-convencoes.md:130` — "4. **Consistência de capitalização**: O ID no `EmbedLiveSample` deve corresponder exatamente ao slug do cabeçalho (case-sensitive)"
- Semântica: a correção depende de o autor digitar igual; não há verificação automática no fluxo descrito (`SKILL.md:378` só tem checkbox manual).
- Consequência: nenhum invariante da crate pode depender de "o autor lembrar". Reforça `trybuild` para erro de tipo e N2/N3 para o que a macro não vê (D8).
- Veredito: confirma P3.

### A10. O exemplo canônico do MDN exige JavaScript e iframe — modelo inaplicável
- Citação: `99-exemplo-completo.md:126` — "const canvas = document.getElementById('output');"
- Semântica: todos os 7 exemplos do documento operam via `getElementById`/`querySelector` (`99:126`, `99:226`, `99:613`), `captureStream()` (`99:136`) e `postMessage`, e cada bloco termina em `{{EmbedLiveSample}}` — que renderiza um iframe executando HTML+CSS+JS.
- Consequência: nossos exemplos de primitivo em `docs/` são Rust; demo interativa, se existir, é asset declarado (`ctx.require(Asset)`, D13) e nunca iframe inline — CSP sem inline proíbe.
- Veredito: **refuta** o modelo de exemplo; confirma P5 (sem runtime) e D13.

### A11. Duplicação de estado é recomendada como prática correta
- Citação: `12-front-matter-status.md:94` — "O campo `status:` no front matter **não** substitui os badges inline — ambos devem ser usados:"
- Semântica: o mesmo fato (experimental/deprecated) deve ser escrito duas vezes, em dois lugares, e o `SKILL.md:374` transforma a divergência entre os dois em anti-pattern.
- Consequência: antítese de fonte única (D5). Na crate, "experimental" nem existe como estado: o elemento está no vocabulário ou não está (`spec/elements.toml` allowlist fechada).
- Veredito: **refuta** — e é a melhor justificativa escrita para D5 que este corpus oferece.

### A12. O corpus se contradiz sobre a seção `Security requirements`
- Citação: `02-semantica-arquitetura.md:108` — "### \"Security requirements\"" (listada como "seção crítica", com propósito e conteúdo)
- Semântica: os docs 02, 03 (`:84` "Documentar Segurança Proativamente", com bloco `## Security requirements`) e 04 (`:54`, no template do overview) exigem a seção; o doc 16 (`:5`) diz que ela "não é usada no repositório real" e a classifica como anti-pattern (`:70`).
- Consequência: instruções contraditórias sem marcador de qual venceu. Quem lesse só 01–07 implementaria o que o doc 16 proíbe.
- Veredito: refuta a consistência interna do conjunto.

### A13. A página que se diz conforme viola a regra que invoca
- Citação: `99-exemplo-completo.md:98` — "## Security requirements"
- Semântica: o exemplo canônico contém a seção proibida, enquanto o checklist final afirma o contrário: `99:739` — "| `{{securecontext_header}}` (não seção \"Security requirements\") | 16 | Overview + interface pages |".
- Consequência: o documento que deveria ser o padrão-ouro não é verificável por leitura; não pode ser usado como referência de forma.
- Veredito: refuta — o doc 99 não serve como template.

### A14. Ordem de badges: duas regras incompatíveis
- Citação: `SKILL.md:91` — "### Badge order (MANDATORY)" (sidebar → contexto → status, `{{sidebar}}{{securecontext_header}}{{AvailableInWorkers(...)}}{{status_badge}}`)
- Semântica: `11-macros-adicionais.md:127` dá outra ordem: "3. **Múltiplos badges**: Ordem padrão: `securecontext_header` → `DefaultAPISidebar` → `AvailableInWorkers`" — sidebar depois do badge de contexto.
- Consequência: o corpus não tem autoridade única nem sobre formatação. Análogo na crate: ordem de emissão de atributos é única e vem do arquivo de spec (`spec/elements.toml`, comentário de cabeçalho: "A ordem dos atributos neste arquivo é a ordem de emissão na saída").
- Veredito: confirma D14 (determinismo) por contraste.

### A15. Taxonomia de `page-type` divergente entre documentos
- Citação: `20-page-types-complementares.md:13` — "| `web-api-method` | Subpágina de método (`api/NomeInterface/nome_metodo/`) |"
- Semântica: o doc 20 usa `web-api-method`/`web-api-property`; `SKILL.md:52` e `06-glossario-macros-tipos.md` usam `web-api-instance-method`/`web-api-instance-property` e acrescentam `web-api-static-method`. O doc 20 ainda propõe `svg-element`, `css-function`, `css-property` que nenhum outro documento reconhece.
- Consequência: a lista de tipos cresce por documento, sem fechamento. É o padrão exato do que a crate proíbe: vocabulário aberto que admite novas variantes por conveniência.
- Veredito: refuta · confirma P7 (erro de marcação é erro de compilação) e D4 (sem escape hatch).

### A16. Slug↔pasta: "espelho exato" com regra ambígua, contra-exemplo no próprio conjunto
- Citação: `01-padrao-moderno-web-api.md:179` — "Regra de conversão: `slug = path` com `_` mapeando para `/`, PascalCase para PascalCase, lowercase para lowercase."
- Semântica: se `_` vira `/`, então `Web/API/Fetch_API/Using_Fetch` → `api/Fetch/API/Using/Fetch`; a regra não fecha. O próprio doc 13 lista `using_element-scoped/` (`13:51`) — hífen no meio de um esquema de underscore.
- Consequência: nenhum mapeamento derivado de string deve ser usado como fonte de identidade. Nossa identidade é o `id` tipado/gerado, não um nome normalizado.
- Veredito: refuta a robustez da regra; confirma D8.

### A17. Construtores: diretório e slug se contradizem entre docs 06 e 10
- Citação: `06-glossario-macros-tipos.md:114` — "| Construtor | PascalCase (como interface) | `Request/request/` |"
- Semântica: a coluna "formato" diz PascalCase, o exemplo mostra `request` em lowercase; o doc 10 (`:5`) e o `SKILL.md:56` (`web-api-constructor` → `Web/API/Interface/Interface`) dizem que o slug repete o nome da interface — e a tabela de slugs do próprio doc 06 (`:116`–`:122`) nem lista o caso do construtor.
- Consequência: exemplo inconsistente com a regra que ele ilustra. Reforça A16: convenção textual sem verificador apodrece no mesmo documento.
- Veredito: refuta.

### A18. Cabeçalhos em português dentro de um guia que manda usar inglês sentence case
- Citação: `09-template-eventos.md:51` — "## Sintaxe" (e `:60` "## Tipo de Evento", `:100` "## Especificações", `:108` "## Veja também")
- Semântica: `17-convencoes-titulos.md:9` manda "Sentence case" em inglês; o doc 09 escreve os cabeçalhos em português, e o doc 99 mistura os dois idiomas na mesma página (`99:580` "## Type", `99:588` "## Cancelável").
- Consequência: em um sistema com texto tipado e localizado isso seria impossível — o cabeçalho é dado, não literal. Na crate, texto visível é `Msg` resolvido por locale (`docs/02-tipos-e-contextos.md` §6); nenhum lint de capitalização sobre string é necessário nem desejável.
- Veredito: irrelevante para a crate · refuta a coerência do conjunto.

### A19. `js-nolint` vs `js`: a distinção sintaxe/exemplo é o achado mais portável
- Citação: `SKILL.md:193` — "- `` ```js-nolint `` for syntax blocks (not executable)"
- Semântica: bloco de **sintaxe** documenta a forma (todas as sobrecargas, sem execução); bloco `js` documenta uso executável; `js example-bad` marca o que não se deve escrever.
- Consequência: análogo direto no nosso `trybuild`: um exemplo de primitivo que não compila (ou que compila com advisory) precisa caber em uma destas categorias declaradas, nunca em "exemplo genérico". Proposta em §5.
- Veredito: **refina** D7/D8 — adiciona vocabulário de exemplo.

### A20. "Todo exemplo com tratamento de erro" como regra rígida
- Citação: `SKILL.md:196` — "- **ALL examples must include error handling** (`try/catch` or `.catch()`)"
- Semântica: toda amostra de código precisa mostrar o caminho de erro, não só o caminho felizes.
- Consequência: na crate isso já é estrutural — `Result` obrigatório no primitivo e `Page` como único construtor de resposta (D11). A regra vira requisito de *exemplo de doc*: exemplo que ignora o `Err` é exemplo incompleto.
- Veredito: confirma D11 · refina a política de exemplos.

### A21. Guias são raros: a documentação vive no índice
- Citação: `13-convencoes-guias.md:5` — "Subdiretórios de guia (`api/nome_api/using_algo/`) são **opcionais e raros** — apenas ~45 das 1228+ APIs (~3.7%) os possuem."
- Semântica: fragmentar documentação em subpáginas é exceção, não regra; o padrão é um único `index.md` por API.
- Consequência: argumento quantitativo contra fragmentar nossa documentação: um doc por primitivo, sem "guia de uso do `Field`" separado do doc do `Field`. Também vale para `docs/18-aprendizados/`: um arquivo por corpus é o limite.
- Veredito: confirma P1/P9 no plano da documentação.

### A22. Regra de tamanho explícita para extrair conteúdo
- Citação: `03-boas-praticas-armadilhas.md:27` — "**Errado**: Overview com tutorial de 200 linhas embutido. Veja WebGPU API (638 linhas) como contra-exemplo"
- Semântica: acima de ~300 linhas (`13:75`), ou com 3+ casos de uso distintos, o conteúdo deve sair do overview.
- Consequência: número acionável para nós: doc de primitivo acima de ~200 linhas é sinal de que o primitivo faz duas coisas — o problema é de **desenho da API**, não de documentação. [INFERENCE]
- Veredito: refina P9 — tamanho de doc como sintoma de acoplamento.

### A23. Anti-pattern de `spec-urls` → regra de "uma spec primária"
- Citação: `19-spec-urls-anti-pattern.md:13` — "### Exemplo Excessivo — Performance API (14 spec-urls)"
- Semântica: listar 14 URLs mistura TR atual, rascunho do GitHub e versão de 2018; a regra aceitável é 1 spec, no máximo 2–3 disjuntas, nunca versões antigas nem specs tangenciais (`:86`–`:96`).
- Consequência: transponível a `spec/elements.toml`: cada elemento declara **uma** referência primária; nada de campo `spec_urls` como lista livre. Proposta em §5.
- Veredito: **refina** D5 (a spec é dado versionado) — com regra de cardinalidade.

### A24. Dois mecanismos de sidebar, e a proibição de misturá-los
- Citação: `08-front-matter-sidebar.md:51` — "Nenhuma página em `/api/` usa `sidebar:` no front matter. Em vez disso, usam macros KumaScript:" (e `SKILL.md:351` — "NEVER mix both mechanisms in the same page")
- Semântica: duas fontes de configuração para a mesma navegação, separadas por seção, com proibição de coexistência.
- Consequência: a crate já tem esse invariante para tema (D12: tema é dado, CSS derivado, servido como arquivo). A lição generaliza: um mecanismo por preocupação, declarado, sem fallback silencioso.
- Veredito: confirma D12.

### A25. Cross-link em vez de duplicata (media/ ↔ api/)
- Citação: `14-integracao-media-api-v2.md:89` — "| Referência de API (interfaces, métodos, propriedades, eventos, construtores) | `/api/` | `Request`, `Response`, `fetch()` |"
- Semântica: o conjunto dedica dois documentos (07 e 14) a mapear sobreposição entre as seções `/media/` e `/api/` e a recomendar links bidirecionais (`14:99`–`:100`) em vez de cópia.
- Consequência: aplica-se à nossa documentação: `docs/18-aprendizados/<corpus>.md` detalha, `docs/18-aprendizados/README.md` resume e linka — já é o arranjo adotado; o corpus recomenda não repetir o conteúdo nos dois.
- Veredito: confirma P1 no plano editorial.

### A26. Tradução: o slug não muda, o rótulo muda
- Citação: `06-glossario-macros-tipos.md:150` — "Nota: o slug NÃO muda em traduções — apenas o title."
- Semântica: identidade (o caminho/URL) e rótulo humano (o título) são coisas distintas; traduzir o rótulo nunca reescreve a identidade, e macros "NÃO devem ser traduzidas" (`:154`).
- Consequência: é a separação que a crate já faz entre `Msg` (texto localizado) e o token/`id` (identidade estável). A regra inversa — traduzir a identidade — é a que quebra links e determinismo.
- Veredito: **confirma** D14 e `docs/02` §6 — com o argumento de i18n que faltava.

### A27. `EmbedGHLiveSample`: exemplo como asset endereçável, não blob inline
- Citação: `11-macros-adicionais.md:28` — "| `{{EmbedLiveSample}}` | Bloco de código inline na página | Autocontido, sem dependência externa |"
- Semântica: quando o exemplo precisa de vários arquivos (HTML+CSS+JS), ele deixa de ser texto na página e vira caminho em repositório — `{{EmbedGHLiveSample('dom-examples/webgl-examples/tutorial/sample8/index.html', 670, 510)}}` (`:14`).
- Consequência: é a mesma fronteira da crate: exemplo no doc = Rust inline; demo com arquivos = `Asset` declarado com hash (`docs/07`). O corpus mostra que a fronteira é inevitável e que mantê-la explícita evita um sistema de exemplos dentro do outro.
- Veredito: **confirma** D13.

### A28. Asset inline (base64) é anti-pattern declarado
- Citação: `03-boas-praticas-armadilhas.md:170` — "**Problema**: Ícones base64 (~50KB+) poluem a documentação."
- Semântica: o anti-pattern #6 é embutir base64 no CSS; a correção recomendada é "SVG inline ou links para assets externos".
- Consequência: justifica a regra de asset da crate (arquivo próprio, nome com hash de conteúdo, `immutable`). Nada de `data:` URI gerado por conveniência no CSS derivado.
- Veredito: **confirma** D12/D13 — com evidência empírica de por que inline degrada.

### A29. Nenhum corpus cobre `Intl`; o único uso vizinho é cliente-side
- Citação: `lit/13-custom-directives.md:85` — "return new Intl.DateTimeFormat('pt-BR', format).format(new Date());"
- Semântica: formatação localizada só aparece como `Intl` no browser (lit) e como nota de paridade de `Number` (topcoat `10-servicos.md:185`); `mdn-web-api-docs` não menciona `Intl` uma única vez.
- Consequência: a crate formata em Rust (`Timestamp`, `Count`, `Money` — `docs/02` §7); falta **fonte** para as regras de locale (ordem de campos, separadores, plural). Precisa de decisão própria, não de corpus.
- Veredito: lacuna — ver §7.

## 3. Confirmações do desenho

1. **D8 — validação de referência em tempo de compilação.** O par `EmbedLiveSample`/slug de cabeçalho (`18:24`, `18:130`) é a prova empírica de que referência textual entre dois pontos distantes, sem verificador, apodrece — e o doc 18 não consegue nem descrever o mecanismo sem se contradizer (A7, A8, A9).
2. **D5 — a spec é dado versionado, com fonte única.** `12:94` recomenda escrever o mesmo estado duas vezes; `SKILL.md:374` pune a divergência entre as duas. A crate não tem esse buraco: `spec/elements.toml` é a única fonte e `experimental` não é um estado representável (A11).
3. **D14 — determinismo de bytes.** O corpus, no melhor documento que tem, admite ordem de badges contraditória (`SKILL.md:91` vs `11:127`, A14); a crate resolve com ordem derivada do arquivo de spec.
4. **P5/D13 — sem runtime, JS como asset declarado.** O modelo de exemplo do MDN é iframe + JS obrigatório (`99:126`, `99:136`, A10); funciona lá porque o MDN é uma aplicação JS. Não funciona conosco por CSP e por "tudo funciona sem JavaScript".
5. **P3 — prevenção acima de detecção.** Todos os controles de qualidade do conjunto são checkboxes humanos (`SKILL.md:378`–`:398`). Nenhum é mecânico. Isso confirma que checklist manual não é validação.
6. **D4 — vocabulário fechado.** A taxonomia de `page-type` ganha tipos novos a cada documento (A15) porque nada a fecha; nosso allowlist fecha e qualquer nome fora dele é erro de compilação.
7. **P1/P9 — documentação enxuta e não fragmentada.** 3,7% das APIs têm subguia (`13:5`) e a regra de extração é explícita em ~300 linhas (`03:27`): doc de primitivo deve caber em uma tela de leitura.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata a inserir e onde) |
|---|---|---|
| `docs/07-js-e-assets.md` (D13) diz que comportamento é arquivo JS declarado | `99:126` exemplifica com `document.getElementById` + `{{EmbedLiveSample}}` (iframe executando JS inline) | Em `docs/07-js-e-assets.md`, após a tabela de assets: "Exemplo de documentação de primitivo **nunca** assume `document.querySelector` nem iframe executável; demo interativa, quando existir, é `Asset` declarado e carregada da mesma origem." |
| `docs/03` (vocabulário) e `spec/elements.toml` não têm noção de "tipo de página" | `06:114`/`20:13` oferecem 12+ `page-type` mutuamente inconsistentes | Em `docs/03-vocabulario-de-elementos.md`, na abertura: "Não existe taxonomia de páginas de documentação: um elemento tem **um** doc (o próprio verbete da spec) e nenhum metadado de 'tipo de página'." |
| `docs/02` §6 exige `Msg` para todo texto visível, resolvido por locale | `17:9` normaliza capitalização de título em **inglês** (`Concepts and Usage` → `Concepts and usage`) | Em `docs/02-tipos-e-contextos.md` §6, ao fim do bloco de i18n: "Capitalização de título é dado do catálogo de tradução, por locale; nenhum lint compara strings visíveis com regras de sentence case em inglês." |
| `spec/elements.toml` declara ordem de atributos = ordem de emissão (D14) | `SKILL.md:91` e `11:127` dão duas ordens de badge incompatíveis (A14) | No cabeçalho de `spec/elements.toml`, junto ao comentário de ordem: "Ordem de emissão é a ordem deste arquivo e ponto final; não há regra de ordenação derivada de 'tipo de badge' ou de qualquer heurística externa." |
| `docs/05-acessibilidade.md` exige nome acessível e `id` gerado por primitivo | `18:24`–`18:130` liga macro a cabeçalho por slug textual com case-sensitive | Em `docs/04-validacao.md` §4 (N2 — o que a macro valida), acrescentar item: "toda referência por `id` (`for`, `aria-*`, `popovertarget`, `commandfor`) aponta para um `id` existente no bloco; não existe referência por texto normalizado." |
| `docs/13-decisoes.md` D5 (spec versionada por campo) não diz cardinalidade de referências | `19:13` critica 14 `spec-urls` e prescreve 1, no máximo 2–3 disjuntas | Em `docs/03-vocabulario-de-elementos.md`, na tabela de campos do verbete: "`spec` é **uma** URL canônica por elemento — nunca lista, nunca versão datada de rascunho, nunca spec tangencial." |
| `docs/12-testes.md` fala de `trybuild` para erro de tipo | `SKILL.md:193`/`:196` exigem blocos de sintaxe não executáveis e tratamento de erro em todo exemplo | Em `docs/12-testes.md`, na seção de `trybuild`: "Cada exemplo em `docs/` é de um destes tipos: `syntax` (não compila por desenho), `ok` (compila, `trybuild` verifica) ou `bad` (erro esperado, `trybuild` verifica a mensagem). Exemplo sem tipo declarado é doc incompleta." |
| `docs/18-aprendizados/README.md` já corrige o corpus em 3 linhas | este documento tem 300+ linhas | Nenhuma: o resumo linka este arquivo; a duplicação é intencional na medida em que o resumo não repete as citações (conforme `14:99`–`:100`, cross-link em vez de cópia). |

## 5. Adições prontas para colar

**5.1 Vocabulário de exemplo (para `docs/12-testes.md` ou `docs/10-ergonomia.md`):**

| tipo de exemplo | compila? | verificado por | uso |
|---|---|---|---|
| `syntax` | não (por desenho) | inspeção em revisão | mostra assinatura/sobrecargas, como `js-nolint` no MDN (`SKILL.md:193`) |
| `ok` | sim | `trybuild` (`pass`) | uso canônico de um primitivo |
| `bad` | não | `trybuild` (`compile_fail` + mensagem) | erro de marcação com a mensagem exata (`docs/02` §9) |
| `advisory` | sim | lint N3 com supressão declarada | caso em que a acessibilidade avisa e o autor aceita (D10) |

**5.2 Checklist de revisão de doc de primitivo (derivado de `SKILL.md:378`, filtrado pelo nosso escopo):**

```
- [ ] primeira frase diz o problema que o primitivo resolve, não a assinatura (01:11)
- [ ] toda referência por id no exemplo existe no bloco (18:24 → D8)
- [ ] exemplo declara seu tipo: syntax | ok | bad | advisory (§5.1)
- [ ] nenhum exemplo usa document.*, event handler inline ou iframe executável (99:126 → D3/D13)
- [ ] texto visível é Msg; nenhum literal em inglês ou português no exemplo (docs/02 §6)
- [ ] o exemplo cabe em ~200 linhas; se não, o primitivo faz duas coisas (03:27)
- [ ] nenhum campo de estado duplicado: nada de "experimental" fora da presença no vocabulário (12:94 → D5)
```

**5.3 Regra de cardinalidade de spec (para o comentário de esquema de `spec/elements.toml`, sem novo campo):**

```toml
# Referência normativa, se um dia existir campo para isso: UMA URL canônica por elemento
# (nunca lista, nunca versão datada de rascunho, nunca spec tangencial — A23).
# Hoje o vocabulário não guarda URL: a fonte é a spec do HTML, uma só.
```

## 6. Armadilhas de complexidade

1. **Importar a taxonomia de `page-type`.** Doze tipos e crescendo, com duas grafias concorrentes (A15). Custo se entrar: um enum novo no `html-spec` gerado, uma coluna nova em `spec/elements.toml`, e uma migração a cada variante que aparecer — para expressar algo que a crate não produz (páginas).
2. **Qualquer forma de macro/template textual no corpo do markup.** `{{DOMxRef}}`, `{{EmbedLiveSample}}`, `{{SubPagesWithSummaries}}` existem porque o MDN é um sistema de conteúdo com renderização em duas fases. Trazer isso é trazer um motor de template dentro de um projeto cujo princípio é "o HTML é o artefato" (P1) e "sem runtime" (P5). Custo: um parser a mais, um passo a mais, e a porta de entrada para string não escapada (D2).
3. **Live samples (iframe + JS).** `99` inteiro depende disso. Custo: CSP com `frame-src`/`script-src` inline, um segundo pipeline de render, e a quebra de "tudo funciona sem JavaScript".
4. **Front matter como fonte de verdade.** `title`/`slug`/`page-type` triplicam a mesma identidade (A16, A17) e o `status` duplica estado (A11). Custo: um verificador de consistência entre três representações da mesma coisa — exatamente o que D5 elimina.
5. **Badges e ordem de badges.** Dois documentos, duas ordens (A14). Custo: uma tabela de prioridade de atributos no validador, divergindo da ordem de emissão determinística (D14).
6. **Regra de "todo exemplo tem try/catch".** Vem de JavaScript (A20). Em Rust, o análogo é `?`/`Result` — copiar a forma literal produz exemplos com `catch` cerimonial. Custo: ruído em toda doc e uma falsa sensação de tratamento de erro.

## 7. Lacunas do corpus

O corpus não cobre **nenhuma** das APIs que a crate precisa. Para cada uma, onde a informação existe de fato:

| API / recurso | Cobertura em `mdn-web-api-docs` | Onde está a informação |
|---|---|---|
| **`<dialog>`** (`showModal`, `show`, `close`, top layer, foco, `returnValue`, `closedby`) | **zero** (`showModal` não ocorre) | `html-moderno/03_MODERN_PATTERNS.md:59`–`:85` (métodos, eventos `close`/`cancel`, atributos `open`/`closedby`/`returnValue`, foco e inércia do resto do documento) e `04_ACCESSIBILITY.md:164`–`:166`; `07_INTERDEPENDENCIES.md:115`–`:117` mapeia `close`/`cancel`/`beforetoggle`. Falta a semântica de top layer e o comportamento de `Esc` por valor de `closedby` — **fonte externa**: HTML Living Standard §the dialog element. |
| **Popover** (`popover`, `popovertarget`, `popovertargetaction`, light dismiss, `:popover-open`, `::backdrop`) | **zero** (`popover` não ocorre) | `html-moderno/03_MODERN_PATTERNS.md:88`–`:112` (três valores, três ações, CSS) e `07_INTERDEPENDENCIES.md:43`, `:117`. Falta a interação entre `popover="auto"` e `<dialog>` modal, e a ancoragem (`anchor` + `anchor()`) só é mencionada de passagem — **fonte externa**: HTML Living Standard §popover. |
| **Constraint validation** (`required`, `pattern`, `minlength`, `min`, `step`, `ValidityState`, mensagens, `reportValidity`) | **zero** (`reportValidity`, `ValidityState`, `setCustomValidity` não ocorrem) | `html-moderno/11_CONSTRAINT_VALIDATION.md:35`–`:53` (métodos e os 10 membros de `ValidityState`), `:123` (elementos que suportam `setCustomValidity`), `03_MODERN_PATTERNS.md:202`–`:211`. Falta **localização das mensagens nativas** e a política de quando confiar no browser — decisão da crate (mensagem própria, `aria-describedby`, `:user-invalid`), fora de qualquer corpus. |
| **`inert`** | **zero** (`inert` não ocorre) | `html-moderno/04_ACCESSIBILITY.md:171`, `09_BOILERPLATE.md:340`, `18_GLOBAL_ATTRIBUTES.md` (via `SKILL.md:83`); semântica de foco e de árvore de acessibilidade: **fonte externa** (HTML Living Standard §inert). |
| **`Intl` / formatação localizada** | **zero** (nenhuma ocorrência de `Intl`) | Nenhum corpus cobre. O único uso vizinho é cliente-side (`lit/13-custom-directives.md:85` — `new Intl.DateTimeFormat('pt-BR', …)`) e uma nota de paridade (`topcoat/10-servicos.md:185` — "`Number` segue `Intl.NumberFormat`"). A crate formata em Rust (`docs/02` §7: `Timestamp`, `Count`, `Money`): a tabela de locale (ordem de campos, separador de milhar, plural, moeda) precisa de **fonte externa** — CLDR/UTR#35 — e de decisão registrada em `docs/13-decisoes.md`. |
| **Invoker Commands** (`command`, `commandfor`) | **zero** | `html-moderno/03_MODERN_PATTERNS.md:84`–`:85`, marcado como experimental; `03_MODERN_PATTERNS.md:110` lista o CSS relacionado. Não pode ser caminho obrigatório (registrado como armadilha na seção `html-moderno` do resumo). |
| **`<details name>` (acordeão nativo), `<search>`, `inputmode`, `autocomplete`** | **zero** | `html-moderno/03_MODERN_PATTERNS.md:114`–`:121` (`details name`), `11_CONSTRAINT_VALIDATION.md`, e a tabela do resumo em `docs/18-aprendizados/README.md` (seção `html-moderno`). |

Conclusão operacional: **este corpus não deve ser reaberto** por nenhuma pergunta sobre comportamento de
elemento ou de API. Ele só responde a perguntas de forma documental (como nomear um guia, quantas
páginas uma API deve ter, como não duplicar conteúdo) — e, mesmo aí, com contradições internas que
exigem escolher um documento como vencedor (o doc 16 vence o 02/03/04 sobre segurança; o `SKILL.md`
vence o 11 sobre ordem de badges; o doc 10 vence o 06 sobre construtores).
