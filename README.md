# html-auy

**Um vocabulário HTML tipado para Rust, orientado a prevenção de erros, acessibilidade e determinismo.**

`html-auy` é um projeto **open source independente**, mantido como repositório próprio, que especifica e
implementa uma camada de renderização de HTML que trate o **HTML final como artefato**, e não como
projeção de uma árvore reativa.

> **Este projeto NÃO será publicado como crate.**
> Não há compromisso de semver público, nem obrigação de suportar consumidores externos.
> A biblioteca é de uso interno do projeto que a originou; a abertura do repositório é para
> **estudo, crítica e reuso por cópia/vendorização**. Ver [`docs/13-decisoes.md`](docs/13-decisoes.md), D0.

---

## O problema que este projeto resolve

Todo framework web em Rust resolve HTML de uma das quatro maneiras:

| Família | Representantes | O que falta |
|---|---|---|
| macro → `Markup`/`String` | maud, horrorshow, markup.rs | sem tipagem de atributo, sem escaping por contexto |
| template de arquivo em compile-time | askama, sailfish | `\|safe` como porta de escape, composição rígida |
| macro → árvore reativa/VDOM | leptos, yew, dioxus, sycamore, topcoat | o HTML deixa de ser o artefato; injeta script/estado |
| template em runtime sobre `Value` | minijinja, tera, handlebars | zero tipagem; erro só em request |

Nenhuma delas valida **atributo intrínseco** (`hreff`, `aria-lable`, `altt` compilam em todas), nenhuma
faz **escaping por contexto** (JSON-LD dentro de `<script>` é responsabilidade do chamador), e todas
deixam a **acessibilidade** como disciplina do desenvolvedor.

O quadrante vazio que este projeto ocupa:

> **props tipadas + `Markup` não reativo + atributo validado em compilação + acessibilidade por construção**

O escopo real que motivou o desenho: um portal municipal com 222 views, 6.765 tags e 63 elementos realmente usados (o `spec/elements.toml` declara 111)
distintos, exigência de **WCAG 2.2 AA**, **CSP sem inline**, **ETag/304**, **funcionamento sem JavaScript**
e um conjunto de gates de acessibilidade em CI. Os números que sustentam cada decisão estão em
[`docs/15-migracao-blade.md`](docs/15-migracao-blade.md).

---

## Estado do projeto

**0.1.0 (MVP, F-1 → F6.1).** O workspace `crates/` já contém o produto descrito em
[`docs/21-implementacao.md`](docs/21-implementacao.md): a biblioteca `html` (`no_std`, zero dependência
normal) a proc-macro `html-macros` e seu núcleo `html-macros-core`; a tabela gerada de `spec/elements.toml`; o shell
(`Page`/`View`/`Regions`), os componentes (`Form`/`Field`) e os builders de mídia (`Img`/`Picture`/`Video`/`Audio`/`Track`), além do `audit` opcional (`feature = "audit"`).

O gate é o do consumidor: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets
--all-features -- -D warnings`, `cargo test --workspace` (duas vezes), `cargo test -p html --features
audit`, `python3 tools/validate_spec.py`, `python3 tools/gen_spec.py --check` e
`python3 -m unittest discover -s tools/tests`.

Ficam para os marcos **medidos**: `M1`/`M2`/`M3` ([`docs/16-roadmap.md`](docs/16-roadmap.md)) comparam
linhas e tempo de correção contra o Blade. Os testes de aceitação de leitura (`acceptance_m1`, `m2`,
`port`) já provam a **igualdade byte a byte** entre a página sem macro e a com macro.

---

## Princípios (resumo executivo)

1. **O HTML é o artefato.** O que se escreve é o que sai, byte a byte, com escape automático e sem
   `Raw`/`PreEscaped` público.
2. **Contexto de escape é o tipo, não a posição sintática.** `Href`, `Src`, `Text`, `JsonLd`, `SrcSet`.
3. **Acessibilidade é ausência de API, não aviso.** `img` sem `Alt` não compila; nomear um `div` não
   cria landmark — o nome acessível existe só onde o elemento o expõe (`nav`, `section`, `dialog`).
4. **O componente carrega a acessibilidade.** O desenvolvedor escolhe o componente, não os atributos.
5. **Zero `unsafe`. Zero runtime. Zero JavaScript embutido.**
6. **Determinismo total**: ordem de atributos, de `srcset`, de tokens de tema e de assets é canônica.
7. **Erro de marcação é erro de compilação**, com mensagem acionável e testada.
8. **Um parser, três passes**: validar, formatar e gerar código compartilham o mesmo AST.

## Não-objetivos (resumo executivo)

- Não é um framework: não conhece HTTP, banco, sessão, i18n de domínio nem roteamento.
- Não aceita template em runtime, HTML vindo de banco ou `on*`; `style=` inline existe só como escape hatch auditado.
- Não é publicado como crate; não há semver público.
- Não substitui `axe`, `sniffCSS` ou testes de teclado — ele reduz o custo de chegar até eles.

Detalhes em [`docs/00-principios.md`](docs/00-principios.md) e [`docs/14-alternativas.md`](docs/14-alternativas.md).

---

## Como ler este repositório

| Documento | Assunto |
|---|---|
| [`docs/00-principios.md`](docs/00-principios.md) | problema, princípios, não-objetivos, glossário |
| [`docs/01-arquitetura.md`](docs/01-arquitetura.md) | os quatro crates internos, fronteiras, fluxo de request |
| [`docs/02-tipos-e-contextos.md`](docs/02-tipos-e-contextos.md) | `Markup`, valores tipados, regras de escape |
| [`docs/03-vocabulario-de-elementos.md`](docs/03-vocabulario-de-elementos.md) | `spec/elements.toml`, invariantes por elemento, conteúdo-modelo |
| [`docs/04-validacao.md`](docs/04-validacao.md) | os cinco níveis de validação e seus limites |
| [`docs/05-acessibilidade.md`](docs/05-acessibilidade.md) | WCAG 2.2 AA → mecanismo, regras advisory, ergonomia |
| [`docs/06-shell-e-tema.md`](docs/06-shell-e-tema.md) | base obrigatória, tokens, `/theme.css`, preview |
| [`docs/07-js-e-assets.md`](docs/07-js-e-assets.md) | onde o JavaScript vive, bundles, CSP, JSON-LD |
| [`docs/08-seguranca.md`](docs/08-seguranca.md) | modelo de ameaças do HTML, URLs, cadeia de suprimentos |
| [`docs/09-desempenho-e-cache.md`](docs/09-desempenho-e-cache.md) | alocação, `ETag` canônico, compressão |
| [`docs/10-ergonomia.md`](docs/10-ergonomia.md) | mensagens, formato recomendado, defaults do gerador, anti-ruído |
| [`docs/11-estabilidade.md`](docs/11-estabilidade.md) | MSRV, política de mudança, congelamento de spec |
| [`docs/12-testes.md`](docs/12-testes.md) | golden, property, fuzz, conformidade, e2e |
| [`docs/13-decisoes.md`](docs/13-decisoes.md) | D0–D12 com contexto, decisão e consequência |
| [`docs/14-alternativas.md`](docs/14-alternativas.md) | comparação com 9 alternativas e vereditos |
| [`docs/15-migracao-blade.md`](docs/15-migracao-blade.md) | números medidos, mapa Blade → html-auy, plano |
| [`docs/16-roadmap.md`](docs/16-roadmap.md) | M1/M2/M3, critérios go/no-go, riscos |
| [`docs/17-faq.md`](docs/17-faq.md) | as perguntas difíceis |
| [`docs/18-aprendizados/`](docs/18-aprendizados/README.md) | leitura dos 14 corpus técnicos: achados com citação, conflitos e decisões (D16–D20) |
| [`docs/19-revisao-arquitetural.md`](docs/19-revisao-arquitetural.md) | revisão crítica do próprio desenho: o que cortar, o que promete demais, o que garantir pelo compilador (D21–D22 aceitas; D27–D30 propostas) |
| [`docs/20-fronteira-de-aplicacao.md`](docs/20-fronteira-de-aplicacao.md) | os seis padrões de Blade que o crate não cobre: absorve ou não, como e por quê (D23–D26) |
| [`docs/21-implementacao.md`](docs/21-implementacao.md) | plano de execução: fases F0–F6, arquivos, assinaturas, casos `trybuild`/`insta` e DoD |
| [`docs/22-rigor.md`](docs/22-rigor.md) | rigor de compilação absorvido do `sniff-css`: duas camadas de lint, bans que importam aqui, deny/supply-chain, CI, golden e política de `#[expect]` |
| [`docs/23-acessibilidade-e-css.md`](docs/23-acessibilidade-e-css.md) | a fronteira do decidível: 25 regras medidas do `sniff` divididas em bloco / página / estilo computado |
| [`docs/24-exportacao.md`](docs/24-exportacao.md) | **pós-MVP:** exportar para PHP/Node/Java — o que viaja (spec + corpus), o que não (o macro) |
| [`docs/25-exemplo-de-uso.md`](docs/25-exemplo-de-uso.md) | **o uso final, por escrito:** as três telas do portal, o que o compilador recusa, os bytes que saem e o teste que roda no CI — critério de aceite de F0–F6 |
| [`docs/26-d29-consumidor.md`](docs/26-d29-consumidor.md) | D29 na prática: tema/CSS/`ETag` no consumidor e o desenho hipotético (recusado) dentro do crate |
| [`docs/27-licoes-do-eta.md`](docs/27-licoes-do-eta.md) | o que o `eta` faz bem, o que foi absorvido no 0.1.0 e o que foi recusado |
| [`crates/`](crates) | implementação do MVP: `html` (biblioteca), `html-macros` (`html!`) e `html-macros-core` (parser/validação/codegen) |
| [`spec/elements.toml`](spec/elements.toml) | tabela de elementos e invariantes (dado versionado) |
| [`spec/data.toml`](spec/data.toml) | tabela de `data-*` do consumidor (gate `tools/data_check.py`) |
| [`tools/`](tools) | gerador da tabela, validador da spec, gate `data-*` e seus testes |

---

## Licença

MIT. Ver [`LICENSE`](LICENSE).

O nome `html-auy` é mantido como proposto originalmente; não tem significado técnico interno.
