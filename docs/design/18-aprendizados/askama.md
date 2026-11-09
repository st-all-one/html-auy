# _RUST/askama — Askama 0.16.0

Corpus de 12 documentos (3.811 linhas) sobre o motor de templates Jinja-like em compile-time para Rust.
Cobre: criação por `#[derive(Template)]`, enums, `Values` em runtime, debug por `print`, `askama.toml`,
sintaxe completa (herança, macros com `caller()`, filtros, whitespace), filtros built-in e custom,
integração web (Axum/Actix/Warp/Rocket/Poem) e FAQ de erros reais. Serve aqui como **fonte de
contraexemplo calibrado**: o askama é a alternativa "template de arquivo" madura, é o que o consumidor
usa hoje em dia via Blade/Jinja no CMS, e é citado nominalmente em
[`14-alternativas.md`](../14-alternativas.md) como plano de adoção caso o time de layout deixe de ser
Rust. Cada achado abaixo diz o que **copiar** (disciplina de determinismo, dump de AST, `SIZE_HINT`) e o
que **rejeitar** (porta de escape, escaping por extensão, whitespace configurável, runtime values).

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `01-introduction.md` | 168 | pitch, features default (`config`,`derive`,`std`,`urlencode`), primeiro template, `render`/`render_into`/`write_into`, aviso "render é 100%-200% mais rápido que `to_string`" | confirma que `Display` é o caminho lento; `Markup` não implementa `Display` (02:12) por decisão, não por acaso |
| `02-template-creation.md` | 344 | tabela completa de `#[template(...)]` (12 sub-atributos), mapa extensão→escape, `print`, `block`/`blocks`, `in_doc`, `whitespace` | é a superfície de configuração que html-auy **não** deve ter: 12 knobs por derive |
| `03-template-enums.md` | 173 | enum com template único (`match self`), template por variante, atributos herdados, regras do `match` | mostra que "uma view = um struct" escala mal para variantes; html-auy resolve por função + `Msg` |
| `04-runtime-values.md` | 166 | trait `Values`, `HashMap<&str, Box<dyn Any>>`, `get_value`, `ValueMissing`/`ValueType`, custom filters lendo env | tipagem que se desfaz em runtime: contexto por string volta a errar em request |
| `05-debugging.md` | 170 | `print` em stderr, AST e codegen, erros de compilação comuns, `opt-level=3` para `askama_derive`, `-Z threads` | modelo de diagnóstico a imitar (dump do AST), mas o custo de compilação é o preço do macro pesado |
| `06-configuration.md` | 241 | `askama.toml`: `dirs` com glob, 3 modos de whitespace, sintaxes customizadas, escapers por extensão | prova de que escape por extensão é ambíguo; tabela de precedência é boa referência negativa |
| `07-template-syntax.md` | 932 | referência de sintaxe: variáveis, borrow, filtros, whitespace, herança, escaping (5 chars), controle, expressões, macros, `caller`, raw | **fonte principal**; 20 citações abaixo |
| `08-filters.md` | 621 | 28 filtros built-in, tabela feature→filtro, filtros custom com `#[askama::filter_fn]`, `Safe`/`MaybeSafe`/`HtmlSafe` | `|safe`, `json|safe`, `escape` e o bug de doc do `escape` |
| `09-integration-web.md` | 354 | `render()` agnóstico, erro customizado por framework, `askama_web`, exemplo Axum completo, `into_box`/`into_io_error` | `Markup` não implementa `IntoResponse` (D11) é o oposto: aqui o template *é* a resposta |
| `10-advanced-patterns.md` | 341 | performance (`to_string` 100-200% pior), `FastWritable`/`NO_VALUES`, PGO 15%, estrutura de projeto, XSS/SQLi/path traversal, armadilhas | "Evitando Armadilhas" é uma lista de bugs que html-auy elimina por tipo |
| `11-faq-troubleshooting.md` | 230 | MSRV 1.88, diferenças vs Jinja, async, `let` vs `set`, 9 erros reais com causa/solução, tabela "Dicas Rápidas" | **fonte dos erros observados**; 10 citações abaixo |
| `SKILL.md` | 71 | resumo de 1 página com "Core Rules" e mapa de arquivos | útil só como índice; contém uma afirmação errada ("implement `askama::filters::Filter`") |
| `VERSION` | 1 | `0.16.0` | fixa a versão de todas as afirmações acima |

## 2. Achados

### A1. A lista de caracteres escapados contradiz o próprio exemplo da mesma página
- Citação: `07-template-syntax.md:435` — "Caracteres escapados: `<`, `>`, `&`, `"`, `'`"
- Semântica: declara **cinco** caracteres; vinte linhas depois o exemplo normativo mostra `"// my <html>…"` saindo como `&#x2f;&#x2f; my &lt;html&gt;…` (`07-template-syntax.md:450`): a barra `/` **também** é escapada.
- Consequência: o corpus não permite saber, por leitura, o conjunto real de saída. Para html-auy a resposta é a tabela de [`02-tipos-e-contextos.md:57`](../02-tipos-e-contextos.md), normativa em cinco caracteres — escapar `/` é herança de defesa antiga contra `</script>` e não tem função em HTML5 com `Content-Type` correto.
- Veredito: **confirma D1/P2** e **refuta** a doc como fonte de verdade sobre bytes; golden de migração vem do código, não deste texto.

### A2. O escape é escolhido pela **extensão do arquivo**, não pelo contexto
- Citação: `02-template-creation.md:49` — "A extensão do arquivo define o escape mode:" … `:51` — "`.md`, `.yml`, `.txt`, `.none`, sem extensão → Text escaping (nenhum)"
- Semântica: renomear `foo.html` para `foo.txt` (ou usar `ext = "txt"` em `source`) desliga todo o escape sem tocar em nenhum valor. O mesmo `{{ var }}` é seguro ou inerte conforme uma string de nome de arquivo.
- Consequência: é o defeito estrutural que html-auy existe para eliminar (D1: "contexto de escape é o tipo"). `Text`/`AttrVal`/`Href` não degradam por configuração; `Markup` não é construível de fora (02:11-12).
- Veredito: **confirma D1 e P2**; entra em [`14-alternativas.md:44`](../14-alternativas.md) como evidência dura, não como opinião.

### A3. Não existe escape por contexto: atributo e texto recebem os mesmos bytes
- Citação: `08-filters.md:118` — "Útil para templates com `escape = \"none\"` onde você quer escapar apenas algumas expressões."
- Semântica: o único escaper embutido é o HTML-escaper de texto; `alt="{{ x }}"` e `<p>{{ x }}</p>` produzem os mesmos bytes. Nada escapa `\r`, `\n`, `\t` em valor de atributo; nada distingue URL de texto, `srcset` de `src`.
- Consequência: a tabela de `02-tipos-e-contextos.md:55-61` (texto / atributo / JSON-LD / URL / class) é **estritamente mais forte**. `AttrVal` escapa `&#13; &#10; &#9;`, coisa que o corpus não faz em lugar nenhum.
- Veredito: **confirma D1**; é a justificativa mais concreta para a existência dos ~15 tipos.

### A4. `|safe` é a porta, e a doc a promove como solução padrão de dois problemas diferentes
- Citação: `11-faq-troubleshooting.md:215` — "| HTML aparecendo como texto | Use `{{ var \| safe }}` |" e `:220` — "| Double escaping | Use `{{ var \| safe }}` ou implemente `HtmlSafe` |"
- Semântica: a mesma construção resolve "não escapou" e "escapou duas vezes". Não há tipo que carregue a garantia; a garantia é a **ausência** de uma chamada.
- Consequência: html-auy recusa a porta (D2, 02 §8) e substitui por renderização nó-a-nó. O corpus mostra o preço de tê-la: o FAQ a recomenda quatro vezes em 230 linhas.
- Veredito: **confirma D2**; refina [`14-alternativas.md:46-47`](../14-alternativas.md) ("aceite o `|safe` com um grep-gate") — grep não vê o que deveria ser `JsonLd`.

### A5. `|safe` e o escaper por extensão não interagem por regra declarada
- Citação: `07-template-syntax.md:461` — "| Desligar escaping em uma expressão | `{{ var \| safe }}` |" (na mesma tabela de `:460` — "| Desligar escaping no template | `#[template(escape = \"none\")]` |")
- Semântica: `|safe` marca o valor como `HtmlSafe` e o escaper não é aplicado; em `ext = "txt"` o resultado é idêntico ao sem `|safe` porque não havia escaper. É **no-op silencioso** onde não há escaping e **bypass** onde há — inclusive dentro de `{% filter %}` e de `caller()`.
- Consequência: `[INFERENCE]` uma revisão que procura `|safe` para autorizar HTML pronto precisa saber a extensão do template para decidir; a propriedade não é local ao ponto de inserção. É exatamente o que `Markup`+`JsonLd` eliminam.
- Veredito: **refuta** a suficiência de auditoria textual proposta em 14:47.

### A6. Dois namespaces de escaper: registro por extensão vs seleção por nome no filtro
- Citação: `06-configuration.md:150` — "path = `::tex_escape::Tex`" com `:151` — "extensions = [\"tex\"]"; e `07-template-syntax.md:463` — "| Forçar escaping específico | `{{ var \| escape(\"html\") }}` |"
- Semântica: escapers são **registrados** por extensão (`[[escaper]] extensions = [...]`) e **invocados** por um nome textual. O corpus nunca diz se `escape("tex")` procura o nome da extensão, o nome do tipo ou um terceiro registro.
- Consequência: um erro de digitação em `escape("tex")` não tem como ser detectado em compilação sem tabela fechada. html-auy não deve ter esse eixo: o escaper é o **tipo** do valor e o único seletor é a posição no vocabulário.
- Veredito: **lacuna do corpus** e **refutação** de qualquer API de escaper nomeado no macro.

### A7. Escaper customizado é um trait sobre `fmt::Write` genérico, decidindo por `char`
- Citação: `06-configuration.md:190` — "fn write_escaped<W: std::fmt::Write>(&self, mut w: W, s: &str) -> std::fmt::Result"
- Semântica: o escaper recebe writer e string crua e mapeia caractere a caractere (`'&' => w.write_str(r"\&")`, `:196`). Não há contrato de "escapei tudo"; é responsabilidade do implementador e do teste dele.
- Consequência: `[INFERENCE]` um escaper customizado mal escrito desliga o escape globalmente para uma extensão e nenhum teste do crate o pega. html-auy já tem a versão segura: escape na construção do valor, coberto por property test sobre alfabeto hostil ([`08-seguranca.md:84`](../08-seguranca.md)).
- Veredito: **confirma P2** ("não existe função que escape na hora").

### A8. `|json` + `|safe` dentro de `<script>` é rotulado "Bom"
- Citação: `08-filters.md:478` — "Bom:  `<script>var data = {{data \| json \| safe}};</script>`"
- Semântica: o corpus ensina a injetar JSON serializado com `|safe` em script inline e classifica como boa prática; `:481` marca como "Ruim" só a variante entre aspas.
- Consequência: com `|safe` não há escape — um título contendo `</script>` encerra o script e o resto vira markup, o vetor que [`07-js-e-assets.md:22`](../07-js-e-assets.md) fecha exigindo `JsonLd` com escape fixo de `< > & '` e proibindo `<script>` inline. O corpus **ensina o bug** que a crate existe para prevenir.
- Veredito: **refuta frontalmente** o corpus; **confirma P1/D3** e a linha `14:22` ("Script/estado no HTML: ❌").

### A9. Quatro mecanismos concorrentes de "isto já está seguro", incluindo `{% raw %}`
- Citação: `08-filters.md:579` — "Tipos que implementam `HtmlSafe` não são escapados automaticamente" com `:602-608` — `MaybeSafe::NeedsEscaping` / `MaybeSafe::Safe`; e `07-template-syntax.md:872` — "Conteúdo dentro de `{% raw %}...{% endraw %}` não é processado:"
- Semântica: (a) `|safe` no template, (b) `impl HtmlSafe for T` no Rust, (c) retorno `Safe<T>`/`MaybeSafe<T>` de filtro, (d) bloco `raw` que suspende delimitadores. Nenhum é verificado quanto ao conteúdo; `raw` é `PreEscaped` disfarçado de bloco.
- Consequência: um tipo marcado `HtmlSafe` ontem pode passar a conter input de usuário amanhã e todos os call sites continuam compilando. `Markup` de construtor privado (02:11) não tem esse modo de falha. `raw` quebraria D2/D4 na primeira vez que alguém o usasse.
- Veredito: **confirma D2** por exaustão de alternativas; documentar as quatro como ausentes em 02 §8.

### A10. `escape = "none"` no template é o caminho documentado para recursão — e a recursão não tem guarda
- Citação: `07-template-syntax.md:927` — `"#, ext = "html", escape = "none")]` com `:924` — "   {{ item.render()? }}"; e `10-advanced-patterns.md:307` — "❌ PERIGO: se a expressão resultar em `self`, causa stack overflow"
- Semântica: para renderizar a si mesmo (árvore) a doc desliga o escaping **do template inteiro**, inclusive do `name` que vem do dado; e uma expressão que devolve `self` se chama recursivamente sem limite detectado em compilação.
- Consequência: o escape passa a depender de lembrar `|safe` em cada expressão, e um ciclo vira stack overflow. Em html-auy, recursão é `Render` de `Markup` — o filho é valor já escapado, não programa.
- Veredito: **confirma P1/P2**; confirma que "o artefato é valor" remove a classe inteira de bug.

### A11. `{% extends %}` deve ser a primeira tag, e o conteúdo de topo é descartado em silêncio
- Citação: `07-template-syntax.md:410` — "- `{% extends %}` deve ser o **primeiro** tag no template filho" e `:411` — "- O conteúdo de nível superior do template filho é ignorado"
- Semântica: texto antes do `extends` invalida; texto **depois** (fora de bloco) equivale a apagá-lo, sem erro.
- Consequência: é o pior modo de falha para quem promete "o que se escreve é o que sai" (P1). html-auy deve **errar** para conteúdo descartado; o precedente interno do próprio corpus está em A34 (`03:173`, literal fora do `when` é erro).
- Veredito: **refuta** o comportamento; **confirma P1/P7** e a necessidade de "nenhum texto é droppado" como invariante testada.

### A12. `{{ super() }}` é a única forma de compor bloco, e não aceita variáveis
- Citação: `07-template-syntax.md:412` — "- `{{ super() }}` renderiza o conteúdo do bloco pai"; `11-faq-troubleshooting.md:23` — "- Askama não tem `{{ super() }}` com variáveis arbitrárias — precisa do `block`"
- Semântica: composição pai/filho é chamada sem parâmetros, resolvida sintaticamente; composição parametrizada exige bloco novo ou macro.
- Consequência: um slot que precisa receber dado (shell com `main` + `aside`) vira bloco + captura. html-auy resolve por parâmetro de função (`fn shell(ctx, main: Markup, …)`), que é D7/D11.
- Veredito: **confirma D7** ("as funções são o contrato") e refuta herança como mecanismo de parametrização.

### A13. `extends` não aceita controle de whitespace, e a exceção não é derivável
- Citação: `07-template-syntax.md:413` — "- `{% extends %}` **não** aceita whitespace control (`{%- extends \"...\" %}` é inválido)"
- Semântica: a mesma família de operadores (`-`, `+`, `~`) que funciona em todo bloco é proibida em um bloco específico, por razão sintática (repetido em `10-advanced-patterns.md:316`).
- Consequência: o autor precisa memorizar uma exceção. Em html-auy não há operador de whitespace: `cq fmt` normaliza (`10-ergonomia.md:3`) e os bytes saem do AST.
- Veredito: **confirma P8** (um parser, três passes) e **refuta** operadores inline (ver A29-A31).

### A14. Blocos não podem viver dentro de `if`/`for`, e `block`/`blocks` multiplicam tipos por arquivo
- Citação: `07-template-syntax.md:414` — "- Blocks só podem ser definidos no nível superior ou dentro de outros blocks" e `:415` — "- Blocks **não** podem estar dentro de `if`/`else` ou `for`"; `02-template-creation.md:168` — `#[template(path = "pagina.html", block = "sidebar")]`
- Semântica: a posição do bloco no arquivo é parte do contrato de herança — condicionar bloco a dado é impossível. `blocks = [...]` (`02:201`) gera N sub-templates (`template.as_title().render()`), cada um com o contexto inteiro do struct.
- Consequência: variação estrutural só existe **dentro** do bloco, e a API cresce O(N) por arquivo. html-auy usa função/`Render` por primitivo e snapshot por página.
- Veredito: **refuta** o modelo; **útil na migração**: cada partial do Blade vira função, não sub-template.

### A15. "Renderização vazia" é o sintoma de herança mal configurada, sem diagnóstico
- Citação: `11-faq-troubleshooting.md:194` — "### Renderização retorna string vazia" … `:196` — "**Causa:** Pode ser que o template base não tenha blocks definidos, ou o template filho não esteja extendendo corretamente."
- Semântica: o erro mais citado do FAQ é uma página em branco causada por composição de arquivos: nenhum erro, nenhum aviso, saída vazia.
- Consequência: é o argumento mais forte do corpus a favor de "composição é chamada de função tipada". Um shell em html-auy não pode "não ter blocks": `Page::new(ctx, main, status)` exige `main` (D11).
- Veredito: **confirma D11** com evidência de campo.

### A16. `caller()` é uma variável implícita injetada no corpo da macro
- Citação: `07-template-syntax.md:797` — "Permite que a macro receba um \"corpo\":" com o par `{% call centered() %}` / `{{ caller() }}` em `:799-807`
- Semântica: o corpo do call site é compilado como closure e ligado ao nome `caller` **dentro** da expansão; não é parâmetro declarado na assinatura.
- Consequência: `[INFERENCE]` o nome é reservado e invisível na assinatura, então `macro f()` e `macro f(g)` parecem idênticas a quem lê. Em html-auy o análogo é `children: impl Render` — declarado, tipado, verificável.
- Veredito: **confirma D7**; refina 14:44 ("macros com `caller()`") como perda concreta e pequena.

### A17. `caller(x)` passa argumentos do **interno** do laço para o corpo do chamador
- Citação: `07-template-syntax.md:815` — `{% for user in users %}` … `:816` — `{{ caller(user) }}` e `:820` — `{% call(user) dump_users(list_of_users) %}`
- Semântica: a macro decide quantas e quais variáveis expõe ao corpo; o chamador declara os nomes e os usa no corpo. É um protocolo textual, sem tipo.
- Consequência: é a forma "render prop" do Jinja, e é o que 14:45 chama de "slot nomeado exigindo captura de string". html-auy entrega o mesmo com `fn lista<T>(itens: &[T], linha: impl Fn(&T) -> Markup)`.
- Veredito: **refina** o veredito de 14 (perda real, custo de migração por view).

### A18. `caller` é valor de primeira classe, salvo com `set` e testado com `is defined`
- Citação: `07-template-syntax.md:859` — "{% set outer_caller = caller %}" e `:862` — "{{ outer_caller() }}"; e `:835` — "{% if caller is defined %}"
- Semântica: `caller` atravessa `set`, e a macro testa sua existência para ter corpo opcional (`:828-848`).
- Consequência: existência de parâmetro verificada em **runtime do template**, não em compilação — a macro tem duas assinaturas efetivas. Em html-auy, `children: Option<impl Render>` é decidido no tipo.
- Veredito: **refuta** `is defined` como mecanismo de contrato; **confirma P3** ("prevenção acima de detecção").

### A19. Cadeia de filtros e `{% filter %}` compartilham a semântica, na ordem esquerda→direita
- Citação: `07-template-syntax.md:178` — "{% filter lower|capitalize %}" e `:182` — "Aqui, `lower` é aplicado primeiro, depois `capitalize`."
- Semântica: filtros são funções unárias compostas por pipe; o bloco aplica a cadeia ao corpo **já renderizado** (portanto a uma `String`, não a um valor tipado).
- Consequência: `{% filter %}` é o único ponto onde html-auy poderia ser tentado a criar "pipeline de markup"; aplicado a `Markup` já escapado, qualquer filtro de string reabre A3. Recomendação: filtros só sobre dados, nunca sobre `Markup`.
- Veredito: **refuta** filtros sobre o artefato; **confirma P1**.

### A20. Filtro custom tem assinatura obrigatória, e `#[askama::filter_fn]` virou requisito
- Citação: `08-filters.md:503` — "// #[askama::filter_fn] é OBRIGATÓRIO a partir da v0.15", com a forma `value: T, _env: &dyn Values` em `:505-508`
- Semântica: dois parâmetros fixos (valor, ambiente) e retorno `askama::Result<T> where T: Display`; o ambiente de runtime entra em **todo** filtro, mesmo quando não usado.
- Consequência: `[INFERENCE]` o `env: &dyn Values` permite que o filtro dependa de estado da request sem mudar a assinatura — e é o que torna a saída não-determinística por construção, contra P6/D14.
- Veredito: **refuta** env implícito; em html-auy o contexto é parâmetro explícito do primitivo.

### A21. Argumentos opcionais de filtro usam `#[optional(None)]` sobre `Option<&str>`
- Citação: `07-template-syntax.md:232` — "Em filtros customizados com `#[askama::filter_fn]`, argumentos opcionais usam `#[optional]`" e `:238` — "#[optional(None)] opt: Option<&str>,"
- Semântica: a opcionalidade e o default são declarados no **atributo**, não no tipo; dois mecanismos (`Option` vs literal default) para a mesma ideia (`08:552-553`).
- Consequência: o call site aceita `myfilter(n = 4)` e `myfilter(4)` (`07:229`), com mistura posicional-antes-de-nomeado (`07:783`). html-auy não tem filtros: tem campos nomeados na assinatura, e `Option<T>` significa "não emite" (02 §4).
- Veredito: **irrelevante** para o desenho; **útil** como evidência de que "opcional" precisa de um só mecanismo.

### A22. O corpus recomenda bounds `impl Display` porque referência quebra filtro concreto
- Citação: `08-filters.md:559` — "Use trait bounds (`impl Display`, `impl ToString`) em vez de tipos concretos para evitar problemas com referências"
- Semântica: `fn f(v: &str)` falha quando o pipe entrega `&&str`/`String`; `impl Display` absorve as camadas de `&`. O tipo real na fronteira é "algo que faz Display", sem garantia de contexto.
- Consequência: é a assinatura que html-auy **evita** nos primitivos (`Alt`, `Href`, `Src` são concretos justamente para não absorver `String` arbitrária — D1/D4). `impl Display` é o oposto do contrato de tipo.
- Veredito: **refuta** `impl Display` como parâmetro de API de vocabulário; aceitável só em utilitário interno.

### A23. Bug de documentação no filtro `escape`: o exemplo mostra escape duplo
- Citação: `08-filters.md:114` — `{{ "Escape <>&" | e }}` e `:115` — "{# Output: Escape &amp;lt;&amp;gt;&amp;amp; #}"
- Semântica: a saída documentada é o resultado de escapar **duas vezes** (`&` → `&amp;` → `&amp;amp;`), inconsistente com o filtro descrito na mesma página.
- Consequência: quem implementar conformidade a partir do corpus produz golden errado; prova que "sem escape duplo" não é verificável por inspeção. html-auy declara a propriedade como norma (`02:64` — "Nunca há escape duplo") e a torna impossível por tipo.
- Veredito: **refuta** a confiabilidade do corpus em exemplos de saída; reforça **P2**.

### A24. v0.16 removeu `{% let %}` sem valor; `decl`/`declare` substitui
- Citação: `07-template-syntax.md:121` — "A partir da v0.16, `let` sem valor não é mais permitido — use `decl`/`declare`." (repetido em `11-faq-troubleshooting.md:118`)
- Semântica: declaração sem inicializador ganhou keyword própria, com duas grafias (`decl` e `declare`, `07:12`); a atribuição condicional posterior usa `let` dentro de `if`/`else`.
- Consequência: o padrão `decl x; if … let x = …` é a "variável possivelmente não inicializada" que Rust proíbe — o template esconde a checagem. `[INFERENCE]` só compila porque cada ramo é expandido em bloco.
- Veredito: **irrelevante** para html-auy (não há variáveis de template); **confirma** que estado mutável no template gera gramática própria.

### A25. Regras de empréstimo: a **forma sintática** da inicialização decide se o valor vai por referência
- Citação: `07-template-syntax.md:123` — "### Regras de Empréstimo (Borrow)" e `:125` — "Em alguns casos, o valor da inicialização é colocado atrás de uma referência:" — tabela em `:127-131` (composta/variável/filtro/`?` → não; acesso a campo `x.y` → sim)
- Semântica: o codegen insere `&` conforme a forma da expressão, não conforme o tipo. O autor precisa memorizar cinco linhas de tabela para prever se `{% let a = b %}` move ou empresta.
- Consequência: é o custo de fingir Rust sem Rust. html-auy mantém empréstimo no Rust normal, fora do vocabulário (`10-ergonomia.md:1` — "Não precisa de: `&`, lifetimes…").
- Veredito: **refuta** a replicação; é o achado que mais pesa contra "DSL de template" para o consumidor.

### A26. `mut` no template gera atribuição composta começando por keyword
- Citação: `07-template-syntax.md:139` — "{%- let mut counter = 0 -%}" e `:141` — "{%- mut counter += i -%}"
- Semântica: além de `{% let mut x = … %}`, existe `{% mut x += … %}` — statement que começa por `mut`. Todos os compostos do Rust são aceitos (`:149`).
- Consequência: duas gramáticas para a mesma operação (`let mut` declara, `mut` atribui) e uma keyword Rust reinterpretada. Em html-auy, acumular estado é `fold` na função Rust, com o tipo do acumulador visível.
- Veredito: **refuta**; item de checklist "nada de estado mutável no markup".

### A27. `{% let x %} … {% endlet %}` captura o corpo renderizado como string
- Citação: `07-template-syntax.md:99` — "Atribuição com bloco de conteúdo:" e `:101-103` — `{% let x %}` / `{{ crate::some_function() }} = {{ a * b }}` / `{% endlet %}`
- Semântica: o bloco é renderizado para uma string e **atribuído** a uma variável — captura de markup com tipo `String`/`Display`, sem contexto.
- Consequência: é o mecanismo que permite "montar HTML e inserir depois"; a inserção posterior exige `|safe` (A4). html-auy tem a versão tipada: `let x: Markup = html! { … }`, e inserir `Markup` é sempre seguro.
- Veredito: **confirma** que captura é necessária; **refuta** que ela deva ser string.

### A28. `is defined` não vê campos de campos — limitação declarada, não diagnosticada
- Citação: `07-template-syntax.md:562` — "Só funciona para variáveis do template ou campos do struct — não para campos de campos (`x.y is defined` não funciona)."
- Semântica: a checagem de existência opera no namespace do contexto, não no tipo; `x.y` cai em erro genérico em vez de responder `false`.
- Consequência: `[INFERENCE]` "existe ou não" e "compila ou não" se confundem — o mesmo construto serve para condicional de dado e para introspecção de estrutura. html-auy remove a distinção: `Option<T>` cobre opcionalidade, e campo opcional não existe no vocabulário (02 §4).
- Veredito: **refuta** `is defined` como API; **confirma** `Option` como único mecanismo de "pode faltar".

### A29. Três modos globais de whitespace, cada um com um operador de inversão próprio
- Citação: `06-configuration.md:43` — "| `\"preserve\"` (padrão) | Preserva todo whitespace | `-` suprime |", `:44` — "| `\"suppress\"` | Suprime antes/depois de blocks | `+` preserva |", `:45` — "| `\"minimize\"` | Suprime tudo exceto 1 caractere | `~` minimiza |"
- Semântica: o significado de `-`/`+`/`~` depende de um modo global configurável por `askama.toml` **e** por derive (`:83`). O mesmo template produz bytes diferentes em dois projetos.
- Consequência: viola P6/D14 de forma pior do que "mesmo dado → mesmos bytes": o mesmo **código** produz bytes diferentes conforme configuração. html-auy não deve ter modo; `cq fmt` normaliza o fonte e o codegen emite ordem canônica.
- Veredito: **refuta** P6 na presença de modo configurável; registra a decisão de não ter opção.

### A30. Precedência declarada: inline > derive > config, com desempate entre operadores
- Citação: `06-configuration.md:82` — "1. **Operador inline** (`-`, `+`, `~`) — maior prioridade" … `:84` — "3. **Configuração** (`askama.toml`)" e `:86` — "Entre operadores inline conflitantes:" (`:87`: suppress > minimize > preserve)
- Semântica: há **duas** ordens — uma de escopo (inline vence derive vence config) e uma de força entre operadores no mesmo sítio.
- Consequência: para prever um byte o autor precisa das duas tabelas mais o modo. `[INFERENCE]` é impossível revisar isso em 222 views. html-auy tem uma regra: "ordem de atributos é a do TOML" (03 §7).
- Veredito: **confirma P9** ("nada de açúcar que esconde preço") por contraexemplo.

### A31. `~` minimiza preservando um newline — e o mesmo glifo concatena strings
- Citação: `06-configuration.md:70` — "Suprime todo whitespace exceto um caractere (se houver um newline, mantém o newline):"; `07-template-syntax.md:668` — "Use `~` como operador de concatenação:" e `:678` — "O `~` deve ser cercado por espaços para não ser confundido com operador de whitespace."
- Semântica: a minimização não colapsa em espaço — newline tem prioridade, então o resultado depende do whitespace original. E `~` significa duas coisas, desambiguadas por espaço à volta.
- Consequência: `[INFERENCE]` o operador não é obviamente idempotente, e o formatador precisa de regra especial eterna para os espaços. html-auy não tem operador de concatenação: interpolação de `Text`/`Markup` já é concatenação ordenada.
- Veredito: **refuta** os dois usos de `~`; item de "armadilhas" (seção 6).

### A32. Enum com template por variante herda 5 atributos e não herda 2
- Citação: `03-template-enums.md:52` — "Quando você usa `#[template]` na variante, os seguintes atributos são **herdados** do enum:" (`:54-58`: config, escape, ext, syntax, whitespace) e `:60` — "Os seguintes **NÃO são herdados**" (`:62-63`: block, print)
- Semântica: a herança de configuração entre enum e variante é lista fechada e assimétrica, sem regra derivável ("por que `print` não herda?").
- Consequência: uma variante pode acabar com escaping diferente do resto do enum por omissão — matriz de estados que D5 (spec como dado) existe para evitar. html-auy não tem derive por variante: a view é `match` em Rust devolvendo `Markup`.
- Veredito: **refuta** como API; confirma que "configuração por item" multiplica estados.

### A33. `match self` no template, com `Self::` opcional e `endwhen` opcional
- Citação: `03-template-enums.md:8` — "Você pode usar `{% match self %}` para lidar com cada variante dentro de um único template:"; `:31` — "`Self::` é opcional no padrão — `Square(side)` também funciona."; `:144` — "5. Pode-se usar `{% endwhen %}` opcionalmente para fechar um bloco `when`"
- Semântica: o template despacha sobre `self`, o prefixo de caminho é opcional e o fechamento de `when` também: dois spellings para o padrão e dois para fechar.
- Consequência: `[INFERENCE]` quatro combinações de fechamento para testar, e dois spellings por padrão em mensagens de erro. Em html-auy, `match` é Rust — uma grafia, diagnóstico do rustc.
- Veredito: **irrelevante** para a crate; **confirma** que html-auy acerta em não ter gramática de padrões.

### A34. Regras do `match`: exaustivo, `else` por último, e literal fora do `when` é **erro**
- Citação: `03-template-enums.md:140` — "1. **Deve ser exaustivo** — todos os casos possíveis devem ser cobertos" … `:142` — "3. `{% else %}` deve vir **por último**" … `:173` — "Match blocks **não podem** conter conteúdo literal fora dos `{% when %}`"
- Semântica: exaustividade é checada (bom), `{% else %}` é açúcar de `{% when _ %}`, e texto órfão é **rejeitado** — ao contrário do caso de A11, onde é descartado.
- Consequência: é a única regra do corpus que rejeita texto órfão, e é a regra que html-auy deve **generalizar** para todo o vocabulário: nenhum texto é droppado, em lugar nenhum.
- Veredito: **confirma P7 e P8**; citar como precedente interno do próprio corpus.

### A35. `Values` é um `Any` indexado por string, com `ValueMissing`/`ValueType` em runtime
- Citação: `04-runtime-values.md:15` — "- `HashMap<&str, Box<dyn Any>>`", `:66` — "| `askama::Error::ValueMissing` | A chave não existe nos valores |" e `:67` — "| `askama::Error::ValueType` | O tipo do valor não corresponde ao esperado |"
- Semântica: a tipagem que o derive garante para campos do struct **desaparece** para runtime values; dois erros novos (chave ausente, tipo errado) passam a existir só em execução, e `get_value` aparece em filtros (`:91`).
- Consequência: é a "porta dos fundos" que html-auy resolve por parâmetro de função e `ctx.require` (D13) ou por locale no `Ctx`. Nada de `Any` no caminho de render.
- Veredito: **refuta**; confirma P2/P3 e a decisão de `Values` não existir.

### A36. `?` no template vira `Error::Custom`, e não funciona com `Option`
- Citação: `07-template-syntax.md:662` — "Se for `Err`, o template falha com `askama::Error::Custom`." e `:664` — "Não funciona com `Option` — apenas `Result`."
- Semântica: `{{ some_result? }}` aborta o render inteiro com um erro que carrega a mensagem original; `Option` exige `if let`.
- Consequência: `[INFERENCE]` o render pode falhar por causa de um dado, contra a política de fail-safe de [`04-validacao.md:91`](../04-validacao.md) ("Valor inválido vindo de dado: ignorado, a camada anterior vence"). Em html-auy, dado inválido não entra no tipo — não há `?` no markup.
- Veredito: **refuta**; **confirma** a política de fail-safe do consumidor como decisão explícita.

### A37. `print = "ast"|"code"|"all"` despeja AST e codegen em stderr, com `SIZE_HINT` no código gerado
- Citação: `05-debugging.md:26` — "A saída é impressa em **stderr** durante a compilação."; `02-template-creation.md:337` — "| Debug da geração | `print = \"ast\"` / `\"code\"` / `\"all\"` |"; `:155` — "    const SIZE_HINT: usize = 11usize;"
- Semântica: o AST aparece como `[Lit("", "Hello,", " "), Expr(WS(false, false), Var("name")), Lit("", "!", "\n")]` (`05:32`) — estrutura visível, inclusive o estado de whitespace por nó. O codegen declara um tamanho esperado e o usa para pré-dimensionar (`05:59`).
- Consequência: é o modelo de diagnóstico que html-auy deve copiar — **um** dump do AST serve para validar, formatar e explicar — e `SIZE_HINT` é a técnica concreta para "uma alocação por bloco" de [`09-desempenho-e-cache.md:5`](../09-desempenho-e-cache.md).
- Veredito: **confirma P8 e 09 §1**; adotar `cq ast` com a mesma forma de saída e `const SIZE_HINT` por `Render`.

### A38. O custo de compilação é tratado com `opt-level = 3` no crate de derive, e o PGO em "até 15%"
- Citação: `05-debugging.md:157` — "[profile.dev.package.askama_derive]" / `:158` — "opt-level = 3" / `:167` — "rustflags = [\"-Z\", \"threads=16\"]"; `10-advanced-patterns.md:83` — "PGO pode melhorar a performance do Askama em até **15%**:"
- Semântica: a mitigação oficial para macro lenta é compilar o **próprio** derive otimizado em dev e paralelizar em nightly; o único número de ponta a ponta é o PGO, sem metodologia declarada.
- Consequência: `[INFERENCE]` com 222 views, se o macro de html-auy custar como o do askama, o `cargo check` do dia a dia fica dominado pelo derive — risco direto para `M1`, e argumento por parser pequeno sem `syn` (D9). O 15% não é auditável e não deve orientar decisão.
- Veredito: **confirma D9**; alimenta os critérios de [`16-roadmap.md`](../16-roadmap.md).

### A39. `to_string()` custa 100%-200% mais que `render()`
- Citação: `01-introduction.md:161` — "Prefira `.render()` a `.to_string()`. O `to_string()` usa `fmt::Display` com dispatch dinâmico, enquanto `.render()` usa código monomorfizado — em média 100%-200% mais rápido." (repetido em `10-advanced-patterns.md:14`)
- Semântica: a diferença é dispatch de `fmt::Write` genérico vs. monomorfizado; o corpus dá o número, não um benchmark reproduzível. Os três métodos (`render`, `render_into`, `write_into`) estão em `01:150-158`.
- Consequência: html-auy acerta ao (a) não implementar `Display` em `Markup` (02:12) e (b) expor `render_to(&mut String)` (02 §1). O número justifica a assinatura sem `Display` como decisão de performance, não só de segurança.
- Veredito: **confirma** `Render`/`render_to` e a ausência de `Display`.

### A40. Os erros reais do FAQ: `E0434`, overflow aritmético e duas mensagens para a mesma causa
- Citação: `11-faq-troubleshooting.md:204` — "### \"error[E0434]: can't capture dynamic environment in a fn item\"" com `:207` — "Askama só aceita expressões que podem ser avaliadas em tempo de compilação."; `:133` — "### \"Error: attempt to subtract with overflow\""; `:140` — "### \"Error: `X` is not a member of `Y`\"" (e `05-debugging.md:80` — "No field `x` on type `Y`")
- Semântica: (a) o codegen emite `fn` items e erros do rustc chegam crus ao autor; (b) aritmética no template faz panic em debug e wrap em release, conforme perfil; (c) campo inexistente produz duas mensagens diferentes, uma do askama e uma do rustc.
- Consequência: (a) exige que o macro intercepte e reescreva com `= ajuda:` (`10-ergonomia.md:2`); (b) o mesmo template se comporta diferente em dev e produção; (c) uma causa deve ter uma mensagem, garantida por `trybuild`. html-auy não tem aritmética nem resolução dinâmica de campo no markup.
- Veredito: **refuta** os três padrões; **confirma P7** e a exigência de `trybuild` por mensagem.

### A41. O erro de macro é resolvido com `askama = $crate::__askama`
- Citação: `11-faq-troubleshooting.md:219` — "| Erro em macro | Adicione `askama = $crate::__askama` ao template |" e `02-template-creation.md:33` — "| `askama` | Path para o módulo askama (para macros/libs) | `askama = $crate::__askama` |"
- Semântica: `#[derive]` reexportado por um crate intermediário não encontra o crate `askama` no escopo do usuário; a correção é um parâmetro de path injetado por `$crate`.
- Consequência: html-auy é interno e vendorizado (D0), então o path é fixo e o knob é desnecessário: `$crate::__html_auy` resolve no próprio workspace. Não copiar o atributo. `[INFERENCE]` o mesmo vale para campos chamados como keywords Rust ou com prefixo reservado (`10:328-329`), que existem só porque o namespace do struct é compartilhado com o codegen.
- Veredito: **irrelevante para a crate** (D0 elimina o caso); **útil** como aviso se primitivos forem reexportados.

### A42. `{% include %}` só aceita string literal, e a busca tem duas raízes
- Citação: `07-template-syntax.md:586` — "O caminho deve ser uma **string literal** (conhecida em tempo de compilação)." e `:587` — "Askama procura primeiro relativo ao template atual, depois no diretório base."
- Semântica: path resolvido em compilação, com duas raízes de busca; caminho dinâmico é impossível — e por isso não há path traversal (`10:178-179`).
- Consequência: duas raízes significam que mover um arquivo muda qual include vence, sem erro se ambos existirem. html-auy elimina include: composição é chamada de função, e o resolvedor do Rust diagnostica o path errado.
- Veredito: **confirma** o argumento de segurança de `10:178`; **refuta** a resolução implícita.

### A43. Integração web: `render()` devolve `Result<String, Error>` e o template **é** a resposta
- Citação: `09-integration-web.md:3` — "Askama é agnóstico de framework. `Template::render()` retorna `Result<String, askama::Error>`, e você decide como converter isso em uma resposta HTTP."; `:348` — "template.render().map_err(|e| e.into_box())?"; `:51-58` — template de erro renderizado dentro do `IntoResponse` de erro
- Semântica: o handler devolve `Html(template.render()?)` (`:34`); o erro de render vira 500 no próprio `IntoResponse`, que renderiza **outro** template — que também pode falhar (`if let Ok(body) = … else …`).
- Consequência: é o oposto de D11 (`Page::new(ctx, main, status)` é o único construtor; `Markup` não implementa `IntoResponse`). O corpus mostra o custo do padrão que D11 fecha — e `into_io_error()` apaga a distinção entre falha de formatação e de I/O.
- Veredito: **confirma D11** por contraexemplo explícito; **refuta** conversão para `io::Error` (`Render -> Markup` não falha).

### A44. `render()` é síncrono, dado async é preparado antes, e a MSRV é 1.88
- Citação: `11-faq-troubleshooting.md:38` — "Não diretamente. O `render()` é síncrono. Para dados async, prepare os dados antes de criar o struct:" e `:13` — "| 0.16 | 1.88 |"
- Semântica: o template nunca faz I/O; a coleta é do handler (`:42-45` — `fetch_users().await` e depois `let template = UserListTemplate { users }`). Stable é requisito permanente com piso declarado por minor.
- Consequência: é a mesma fronteira de P5 ("nenhuma leitura de arquivo em tempo de request") e justifica `Render` síncrono mesmo com streaming no consumidor: o buffer é `String`, a resposta é bytes prontos. Para html-auy, 1.88 é o piso de referência do ecossistema atual.
- Veredito: **confirma P5** e o desenho síncrono; **confirma** a política de MSRV de [`11-estabilidade.md`](../11-estabilidade.md) (falta o número lá — ver seção 4).

## 3. Confirmações do desenho

1. **P2/D1 — contexto de escape é o tipo.** `07:435` + `02:49-51` provam que a alternativa real é "um escaper, escolhido por extensão". `02-tipos-e-contextos.md:55-61` é estritamente mais forte; nada no corpus chega perto de `AttrVal` escapando `\r\n\t`.
2. **D2 — não existe porta de escape.** O corpus tem quatro (`|safe` em `07:461`, `HtmlSafe` em `08:579`, `Safe`/`MaybeSafe` em `08:594-608`, `raw` em `07:872`) e recomenda a primeira quatro vezes no FAQ (`11:215`, `11:220`).
3. **P1 — o HTML é o artefato.** `05:32` mostra o AST com estado de whitespace por nó e `02:155` que a saída é escrita por `write_str` num writer: não há árvore intermediária. Confirma que macro→bytes é viável em Rust.
4. **D7 — o macro é a API, as funções são o contrato.** `caller()` (`07:797`), `caller(x)` (`07:816`), `super()` (`07:412`) e `blocks` (`02:201`) são quatro substitutos textuais do que uma assinatura tipada resolve.
5. **P7 — erro de marcação é erro de compilação.** `03:140` (match exaustivo) e `03:173` (nada de literal fora do `when`) são os casos em que o corpus **rejeita**; `07:411` é o caso em que ignora. A crate fica do lado dos dois primeiros.
6. **P8 — um parser, três passes.** `print = "ast"` (`05:26`) expõe o mesmo AST que alimentaria `fmt` e codegen; `askama.toml` com `dirs`/`syntax`/`escaper` mostra o custo de três fontes lendo o mesmo arquivo.
7. **P5 — sem runtime.** `11:38`: `render()` é síncrono, dado é preparado antes. Nada no corpus consulta I/O no template.
8. **P6/D14 — determinismo.** Não confirmado: `06:43-45` e `06:82-87` mostram o oposto (bytes dependem de config). A crate está mais correta que a referência.
9. **D9 — dependências mínimas.** O único sinal é o custo de compilação (`05:157-158`), que é o argumento a favor de parser próprio sem `syn`.
10. **D11 — `Page` é o único construtor.** `09:34` e `09:51-58` mostram o padrão que D11 elimina.
11. **09 §1 — uma alocação por bloco.** `02:155` (`SIZE_HINT`) dá a técnica exata; copiar o conceito, não o nome.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata + arquivo) |
|---|---|---|
| `02-tipos-e-contextos.md:57` — texto escapa `& < > " '` | `07:435` declara os mesmos 5, mas `07:450` escapa `/` → `&#x2f;` | Em `docs/design/02-tipos-e-contextos.md` §3, após a tabela: "Escapar `/` (como fazem Jinja e o askama) é desnecessário em HTML5 servido com `Content-Type: text/html; charset=utf-8`; a crate **não** escapa `/` e o golden correspondente fixa isso." |
| `docs/design/07-js-e-assets.md:5` proíbe `<script>` inline e exige `JsonLd` | `08-filters.md:478` classifica `<script>var data = {{data \| json \| safe}};</script>` como **Bom** | Em `docs/design/07-js-e-assets.md` §6: "Nenhum caminho produz JSON dentro de `<script>` que não seja `JsonLd::graph`, que escapa `< > & '`. A construção `{{ data \| json \| safe }}` é rejeitada em revisão por definição: `safe` não existe no vocabulário." |
| `docs/design/14-alternativas.md:46-47` — "aceite o `\|safe` com um grep-gate" | `07:461` + `08:579` + `08:602-608`: três portas, e `\|safe` é no-op em `ext = "txt"` (A5) | Substituir a frase por: "Aceitar `\|safe` com grep-gate é insuficiente: em template sem escaping o mesmo `\|safe` é inerte, e `HtmlSafe`/`Safe`/`MaybeSafe` não aparecem no template. Se o askama for adotado, o gate é E2E por página (axe + `html-lint`), não textual." |
| `docs/design/02-tipos-e-contextos.md` não fala de whitespace | `06:43-45` + `06:82-87`: 3 modos, 6 operadores, 2 ordens de precedência | Em `docs/design/10-ergonomia.md` §3: "Não há supressão de whitespace configurável nem operador inline. `cq fmt` normaliza o fonte (4 espaços, um elemento por linha) e o codegen emite a partir do AST; a única forma de mudar whitespace é mudar o AST." |
| `docs/design/04-validacao.md:83` — macro valida o local; lint valida a página | `07:562` (`x.y is defined` não funciona) e `11:140`/`05:80` (duas mensagens para a mesma causa) | Em `docs/design/04-validacao.md` §4, item 5: "Toda condição que a macro não consegue decidir é **erro**, nunca `false`: não existe `is defined` parcial. Uma causa, uma mensagem (`trybuild`)." |
| `docs/design/11-estabilidade.md` não fixa MSRV numérica | `11:13` — Askama 0.16 exige Rust **1.88** | Em `docs/design/11-estabilidade.md` §1: "MSRV do html-auy: 1.88 (mesmo piso do askama 0.16, que é o motor em uso no consumidor); CI em 1.88, stable e beta." |
| `docs/design/09-desempenho-e-cache.md:5` — uma alocação por bloco, sem mecanismo | `02:155` — `const SIZE_HINT: usize = 11usize;` no código gerado | Em `docs/design/09-desempenho-e-cache.md` §1: "Cada `Render` expõe `const SIZE_HINT: usize` somado em compilação; `render()` faz `String::with_capacity(SIZE_HINT)` e `render_to` não realoca quando a capacidade disponível é suficiente." |
| `docs/design/14-alternativas.md:8` marca askama como "HTML final = artefato ✅" | `07:411` (conteúdo de topo ignorado) e `11:194` (render vazio sem erro) | Ajustar a linha para `⚠️` e nota de rodapé: "O artefato é determinístico, mas a **composição** pode descartar conteúdo sem erro (`07:411`); por isso a coluna 'erro de marcação em compilação' não cobre composição." |

## 5. Adições prontas para colar

**5.1 — Gate de vocabulário proibido (novo bloco em `docs/design/04-validacao.md`, §5, como N3)**

```toml
# cq-vocabulario.toml — padrões proibidos no código de view (N3, severidade error)
[[proibido]]
padrao     = "\\|\\s*safe\\b"
motivo     = "porta de escape: use o valor tipado do contexto (D2)"
substituto = "Text/AttrVal/Href/JsonLd/Markup"

[[proibido]]
padrao   = "escape\\s*=\\s*\"none\""
motivo   = "escape por template: o contexto é o tipo (D1)"

[[proibido]]
padrao   = "impl\\s+.*HtmlSafe"
motivo   = "trust declarado sobre o tipo: use Markup como tipo de retorno"

[[proibido]]
padrao   = "\\b(style|srcdoc|onclick|hx-on)\\s*[:=]"
motivo   = "CSP sem inline (D3)"
```

**5.2 — Tabela de equivalência para a migração Blade/askama → html-auy (colar em `docs/design/15-migracao-blade.md`)**

| Construção no askama | Equivalente em html-auy | Observação |
|---|---|---|
| `{{ x }}` | `Text::new(x)` / `Text::from(Msg::…)` | nunca `&str` cru |
| `{{ x \| safe }}` | **não existe** | renderize o nó, ou use `Markup` já pronto |
| `{{ x \| escape }}` | automático na construção | `\|e` é o comportamento padrão |
| `class="{{ c }}"` | `Class::of(&[Token::…])` | token fechado |
| `href="{{ u }}"` | `Href::internal(LUrl::…)` / `Href::external(…)` | esquema validado |
| `{% extends %}` + `{% block %}` | `fn shell(ctx, main: Markup) -> Markup` | herança vira chamada |
| `{% include %}` | chamada de função que devolve `Markup` | sem busca relativa |
| `{% macro %}` + `{{ caller() }}` | `fn f(children: impl Render)` | declarado, não implícito |
| `{% call(user) %}` | `fn f<T>(itens: &[T], linha: impl Fn(&T) -> Markup)` | tipado |
| `{% match %}` / `{% when %}` | `match` Rust | exaustivo pelo rustc |
| `{% let %}`/`{% set %}`/`{% decl %}` | `let` Rust | sem `decl`, sem borrow mágico |
| `{% for %}` + `loop.index` | `for` + `enumerate()` | índice explícito |
| `{% raw %}` | **não existe** | sem bypass |
| `\| json \| safe` em `<script>` | `JsonLd::graph(&g)` | escape fixo `\u003c` etc. |

**5.3 — Checklist de PR de view (acrescentar ao de `docs/design/10-ergonomia.md` §7)**

- [ ] Nenhum `|safe`, `escape = "none"`, `impl HtmlSafe` no diff (5.1).
- [ ] Todo `href`/`src` vem de `Href`/`Src`; nenhum literal de URL no markup.
- [ ] Nenhum `match` com braço `_` que engula variante nova sem revisão.
- [ ] Nenhum texto solto fora de `Text`/`Msg` — nem "só whitespace" (`03-template-enums.md:173`).
- [ ] Golden revisado: mudança de bytes é intencional e o `SIZE_HINT` mudou junto (A37).

**5.4 — Regra de descarte (colar em `docs/design/04-validacao.md` §4, item 1)**

```
error: conteúdo descartado em `shell`
   = regra: P1 — nenhum byte é escrito que não esteja no AST
   = ajuda: mova o texto para dentro do bloco, ou remova-o; a macro nunca ignora entrada
```

**5.5 — Nota de whitespace (colar em `docs/design/03-vocabulario-de-elementos.md` §7)**

> Não há modo de whitespace. O espaçamento entre elementos é decidido por `cq fmt` (fonte) e pela
> emissão canônica do codegen; dois renders do mesmo dado produzem os mesmos bytes, e o mesmo fonte
> formatado em outra máquina produz os mesmos bytes. Nenhum atributo de derive altera whitespace.

## 6. Armadilhas de complexidade

1. **Herança `extends`/`block`/`super()`.** É uma segunda linguagem de composição com regras posicionais (A11-A14), dois mecanismos de fechamento e `super()` sem parâmetros. Custo se entrar: resolver herança no parser, detectar bloco órfão, decidir conteúdo descartado e explicar `super()` em toda mensagem de erro. Substituível por chamada de função — D7.
2. **`caller()` e call blocks.** Nome reservado injetado (A16), arity variável (A17), valor de primeira classe e teste de existência em runtime (A18). Custo: três conceitos no parser e uma verificação de arity que o rustc não faz. `children: impl Render` cobre 100% dos casos do corpus.
3. **Três modos de whitespace + seis operadores.** Cada combinação modo × operador é uma célula de teste, e o resultado depende de config externa (A29-A31). Custo: golden por configuração; um bug de `~` é invisível até alguém comparar bytes.
4. **Escaper customizado por trait.** `write_escaped<W: fmt::Write>` é responsabilidade total do implementador (A7), sem contrato de cobertura. Custo: a garantia de segurança deixa de ser verificável e passa a depender de teste de terceiro. Fechado por D1/D2.
5. **`Values` / `Box<dyn Any>` / `get_value`.** Reintroduz resolução por string e dois erros de runtime (A35) e torna o render dependente de estado de request (contra P6). Custo: nenhum `Markup` pode ser cacheado com segurança. `Ctx` explícito resolve.
6. **`include` com duas raízes de busca.** Resolução relativa-ao-arquivo e depois base (A42): mover arquivo muda semântica e a colisão é silenciosa. Custo: resolvedor de paths com precedência e um teste por colisão.
7. **Template por variante de enum com herança parcial de atributos.** Cinco herdados, dois não (A32). Custo: matriz de configuração por variante e chance de escaping divergir dentro do mesmo enum. `match` em Rust devolve `Markup`.
8. **`askama.toml` com `dirs` por glob, `syntax` e `escaper`.** Três tabelas que mudam o significado do texto do template sem tocar no texto (A2, A6). Custo: o build passa a depender de um arquivo paralelo, e um erro em `[[escaper]]` desliga escape. A crate tem fonte única: `spec/elements.toml`.
9. **Gramática de `match` com `endwhen` opcional.** Exaustividade automática é boa (A34); o resto é custo: quatro combinações de fechamento e dois spellings por padrão (A33). `match` do Rust já dá exaustividade, uma grafia e spans corretos.
10. **`is defined`.** Existência como valor de expressão (A28), com limitação declarada a um nível de campo. Custo: uma semântica de "namespace de template" que não coincide com tipos; `Option<T>` já é o mecanismo.

## 7. Lacunas do corpus

1. **Não afirma ordem canônica de atributos.** Nem `07` nem `06` dizem se a emissão é na ordem do fonte, alfabética ou da spec. Para P6/D14 é o dado que falta; a crate deve declarar (ordem do TOML, 03 §7).
2. **Não trata estabilidade de bytes como contrato.** `SIZE_HINT` existe (`02:155`), mas nenhum capítulo fala de `ETag`, 304 ou comparação de snapshot. `[INFERENCE]` o askama não promete determinismo entre versões.
3. **Não define como um nome em `escape("…")` resolve** para um `[[escaper]]` registrado por extensão (A6). Sem essa regra, a crate não pode nem rejeitar a construção por analogia.
4. **Não cobre acessibilidade em nenhum nível.** Nenhuma menção a `alt`, `label/for`, `scope`, ordem de heading ou `aria-*`. A coluna "A11y embutida: ❌" de `14:24-35` é sustentada pela ausência total, não por uma decisão do corpus.
5. **Não cobre i18n/locale.** `LUrl` com locale injetado (02 §2) não tem análogo; o corpus não discute `lang`, `dir` nem texto traduzido — só literais.
6. **Não define o que acontece com `{{ }}` em atributo vs. texto** (A3): é a lacuna que justifica `AttrVal`, `Src`, `Href`.
7. **Não diz como `Ctx`/estado por request entraria sem `Values`.** O corpus oferece só `Any` indexado por string (A35); a alternativa explícita (parâmetro de função) fica sem discussão.
8. **Não há número de custo por template.** `05:157-158` mitiga, mas não mede; para o critério de `M1` (222 views) falta a ordem de grandeza de `cargo check` por `html!`.
9. **Não há política de erro para dado inválido.** O corpus tem `?` que aborta com `Error::Custom` (`07:662`) e panics aritméticos (`11:133`); `04-validacao.md:91` decide o oposto (ignorar dado inválido). A crate precisa manter a decisão dela.
10. **Não há teste de conformidade declarado.** MSRV (`11:11-13`) e "stable desde 0.1" são afirmações, não gates; `12-testes.md` da crate é mais rigoroso que qualquer coisa no corpus.
