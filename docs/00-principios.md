# 00 — Princípios, escopo e glossário

## 1. O problema, com precisão

Gerar HTML em Rust é resolvido, hoje, por quatro famílias de solução (detalhadas em
[`14-alternativas.md`](14-alternativas.md)). Todas compartilham três falhas estruturais:

### 1.1 Atributo intrínseco não é validado

Em maud, askama, sailfish, leptos, yew, dioxus, sycamore e topcoat:

```rust
html! { a hreff="/noticias" { "Notícias" } }   // compila em todos eles
```

O nome do atributo é apenas um identificador. Só props de **componente** são tipadas — e mesmo assim
porque o componente é uma `fn`. O elemento `<a>` em si é texto livre. Consequência prática: erros de
atributo só aparecem no navegador, e apenas se alguém olhar.

### 1.2 Escaping não conhece contexto

Todo escape automático disponível é **HTML-escape de texto e de valor de atributo**. Nada mais.
O caso que dói em produção:

```rust
// JSON-LD dentro de <script>: se um título contiver "</script>", a página quebra.
html! { script type="application/ld+json" { (serde_json::to_string(&graph)?) } }
```

O maud mantém a issue #181 aberta justamente por não ter escaping context-aware. A resposta correta
não é "escapar melhor": é **o tipo do valor conhecer o contexto** — `JsonLd` escapa `<`, `>`, `&`,
`'` para `\u003c`, `\u003e`, `\u0026`, `\u0027` **sempre**, sem o chamador poder esquecer.

### 1.3 Acessibilidade é disciplina

`alt` vazio silencioso é o padrão em quase todo componente de imagem do ecossistema. `aria-label` em
`<div>` sem papel, `role="button"` em `<button>`, `th` sem `scope`, `input` sem `label` — nada disso é
detectado antes do axe em runtime, e o axe só vê as páginas que o teste visita.

## 2. Princípios

### P1 — O HTML é o artefato
A saída é `Markup`: bytes determinísticos que o autor pode prever. Nada de VDOM, hidratação,
marcadores de comentário ou estado serializado dentro do documento.

### P2 — Contexto de escape é o tipo
Um valor carrega o contexto em que será inserido. Não existe função que escape "na hora"; o escape
acontece na construção do valor, e só existe um caminho de construção por contexto.

### P3 — Prevenção acima de detecção
Se um erro pode ser impedido por ausência de API, ele **é** impedido. `img` sem `alt` não compila
porque `alt` é parâmetro obrigatório — não porque existe um teste.

### P4 — Acessibilidade por construção, aviso só para o que resta
O componente emite a semântica (`label`+`for`, `aria-describedby`, `scope`, `aria-labelledby`,
`type="button"`, alvo mínimo de toque). O desenvolvedor escolhe o componente. O que não puder ser
impedido por tipo vira **erro de macro** ou **teste de página** (N4). Não existe aviso com ratchet: um
aviso que não bloqueia é documentação de pior qualidade (D28/A4).

### P5 — Sem runtime
Nenhum `unsafe`. Nenhum interpretador. Nenhuma leitura de arquivo em tempo de request. Nenhum laço de
reconciliação. O binário final é código Rust gerado.

### P6 — Determinismo
Ordem de atributos, de elementos em `srcset`, de tokens de tema, de assets e de regras de CSS é
canônica e estável. Dois renders com o mesmo dado produzem bytes idênticos, para que `ETag` seja
confiável.

### P7 — Erro de marcação é erro de compilação
Com span no ponto exato e mensagem acionável, testada por `trybuild`.

### P8 — Um parser, três passes
Parse, validação, formatação e geração de código compartilham o mesmo AST. É o que torna o
`fmt` barato (~250 LOC) em vez de um projeto de gramática próprio.

### P9 — Nada de açúcar que esconde preço
Toda economia de digitação tem que ser explicável em uma linha. `div` implícito por classe é aceito
porque economiza e não esconde nada; "componente mágico que busca dados" não é.

### P10 — O projeto pode morrer barato
Três marcos com critérios numéricos ([`16-roadmap.md`](16-roadmap.md)). Se o primeiro falhar,
o projeto para e a alternativa (maud + camada de valores tipados, ~600 LOC) já resolve 70%.

## 3. Não-objetivos permanentes

| Não-objetivo | Por quê |
|---|---|
| Framework web (HTTP, rotas, sessão, banco) | escopo; o crate é só a camada de HTML |
| Template em runtime (arquivo lido em execução) | quebra tipagem, `ETag` e auditabilidade |
| HTML/CSS vindo de banco | conteúdo é dado; apresentação é código |
| `style=` e atributos `on*` | CSP sem inline, `ADR` de estilo, e nenhum ganho real |
| `Raw`/`PreEscaped` público | bypass de escape é o defeito que o projeto existe para eliminar |
| Reatividade, hidratação, ilhas | o HTML deixaria de ser o artefato |
| Elementos personalizados (`<my-*>`, `customElements.define`, Shadow DOM, `<slot>`) | comportamento passaria a viver no cliente, o HTML deixaria de ser o artefato e o elemento indefinido pisca antes do JS (D20) |
| Publicação como crate com semver público | ver D0 em [`13-decisoes.md`](13-decisoes.md) |
| Substituir `axe`/`sniffCSS` | o projeto reduz o custo de chegar até eles, não os troca |

## 4. Glossário

| Termo | Definição |
|---|---|
| **`Markup`** | tipo único renderizável; envolve `String` com escape já aplicado |
| **valor tipado** | tipo cuja construção garante um contexto (`Href`, `Src`, `JsonLd`, `Alt`, `SrcSet`) |
| **invariante** | regra que o vocabulário impõe a um elemento (`img` exige `alt`) |
| **conteúdo-modelo** | conjunto de filhos permitidos por elemento (`<p>` não pode conter `<div>`) |
| **primitivo** | componente aprovado que emite semântica completa (`Field`, `Dialog`, `Picture`) |
| **elemento cru** | elemento usado diretamente no markup, sem primitivo |
| ~~**advisory**~~ | *(removido — D28/A4)* regra de lint que não bloqueia; hoje: ou é erro, ou é teste que falha |
| ~~**ratchet**~~ | *(removido — D28/A4)* contador que não podia aumentar; a medição de elementos crus continua só como relatório |
| **spec** | `spec/elements.toml`, a tabela de elementos e invariantes, versionada |
| **shell** | a base estrutural obrigatória (`<html>`, header, `<main>`, footer) |
| **parcial autocontido** | componente que declara props, lógica de derivação e assets no mesmo arquivo |
