# 16 — Roadmap, marcos e critérios de parada

> **Documento histórico.** O desenho abaixo antecede o 0.1.0 público; nomes `html-core`/`html-spec`/`html-macro` referem-se a crates da fase interna. O estado atual está em [`01`](01-arquitetura.md) e D35 em [`13`](13-decisoes.md).


## 1. Escopo de código previsto

| Peça | Conteúdo | LOC (anterior) | LOC (decidido, D27–D32) |
|---|---|---|---|
| `html-auy` | `Markup`, escaper, `Render`, 15 valores tipados, `Class`/`Tone`, `Msg` (D31) | ~600 | ~600 |
| `html_auy::spec` | tabela dos 111 elementos em Rust, **gerada uma vez** de `spec/elements.toml` | — | ~400 + `tools/gen_spec.py` ~120 |
| `html-spec` (`build.rs`) | gerador em tempo de build + esquema | ~300 + gerado | — (D27) |
| `html_auy::shell` | `Page`, `ShellData`, `View` (D24), `Form` (D23), `Field` (D25); sem tema (D29) | ~430 | ~430 |
| `html-macro` | parser, validação, codegen (sem `fmt`) | ~1.900 | ~1.650 |
| `audit` (`N4`) | `html5ever` + as 6 invariantes `output` (hoje 8), atrás de `feature = "audit"` | ~400 (crate + binário) | ~250 |
| `cq` (binário) | check/fmt/fix/audit/spec | ~700 | — (D28) |
| testes | golden, property, `trybuild`, fuzz, paridade da spec, conformidade | ~450 | ~450 |
| **total** | | **~4.780 LOC** | **~3.780 LOC** (+ ~120 Python) |

> **Decidido em 2026-11-06:** D27–D32 aceitas ([`13`](13-decisoes.md)). A coluna "anterior" é
> **registro histórico**; a coluna "decidido" é o desenho a implementar.
>
> **MVP × descartado.** O MVP é **F-1 → F6 + F6.1** ([`21`](21-implementacao.md) §0). O corpus de
> conformidade exportado (F7) e a exportação ([`24`](24-exportacao.md)) estão **descartados**
> ([ADR 0002](../adr/0002-sem-exportacao-fora-do-rust.md)).
>
> **Vigente × proposta.** A coluna *vigente* é o desenho atual (4 crates + `cq`). A *proposta* é o corte
> de [`19`](19-revisao-arquitetural.md) §8 (**D27–D30**, aceitas em 2026-11-06). As decisões **D21–D26**
> também estão aceitas e valem nas duas colunas: escopo `block|type|output` (D21), documento só no `Page`
> (D22), `Form`+CSRF (D23), `View` (D24), `Field`/`FieldState` (D25) e `Class` combinável (D26).

Referência de esforço: 6–10 semanas de uma pessoa experiente em Rust, com agentes, para chegar ao
`M2` com testes. O `M1` é 2 semanas.

## 2. Marcos

### M1 — "o vocabulário tipado já vale a pena" (2 semanas)

**Entrega.** `html-auy` + tabela gerada + uma página real (lista de notícias) escrita **sem
macro**, com primitivos.

**Critério de sucesso (numérico).**
1. Linhas da página ≤ **1,4×** as linhas de Blade equivalentes.
2. `cargo check` sem nenhum `allow`.
3. Zero regressão nos gates de acessibilidade da página.
4. `SrcSet`, `Alt`, `Href` cobrem 100% dos usos da página.

**Se falhar:** parar o projeto e adotar **maud + camada de valores tipados** (as ~600 LOC de
`html-auy`). Isso já resolve ~70% do problema e é o plano B formal (ver
[`14-alternativas.md`](14-alternativas.md) §3).

### M2 — "o macro paga a própria existência" (3–4 semanas)

**Entrega.** `html!` com validação completa + `trybuild` de todas as mensagens normativas.
(*Sem* `cq fmt`: a macro ignora whitespace, então não há o que formatar — [`10`](10-ergonomia.md) §3.)

**Critério de sucesso.**
1. A mesma página do `M1` reescrita em `html!` com **menos** linhas que a versão sem macro.
2. Toda mensagem de erro foi lida por alguém que **não** é o autor, e a pessoa corrigiu o código sem
   ajuda.
3. O formato do fonte **não altera** a saída: o mesmo corpo com 2, 4 ou 8 espaços gera bytes idênticos.
4. Nenhum falso-positivo da validação em 30 views convertidas.

**Se falhar:** publicar só `html-auy` internamente e usar maud para marcação. O `fmt`/validação
viram backlog, não bloqueiam o consumidor.

### M3 — "o lint de página ganha o que o compilador não vê" (2 semanas)

**Entrega.** `html::audit` (N4) rodando no CI do consumidor sobre fixtures e páginas do e2e, com relatório
informativo de elementos crus.

**Critério de sucesso.**
1. Encontra **≥ 3 defeitos reais** que os gates atuais não pegam (medido em uma varredura única).
2. Tempo < 100 ms por página.
3. Zero falso-positivo nos arquivos de referência.

**Se falhar:** manter só N1/N2 e confiar em axe/`sniffCSS` para página inteira.

## 3. Riscos e mitigação

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|
| parser da macro quebra com mudança de `rustc` | média | alto | MSRV + CI stable/beta + fuzz + dependências mínimas |
| mensagens de erro ruins afastam o time | média | alto | `trybuild` + revisão por não-autor (M2.2) |
| `spec/elements.toml` e tabela divergem | alta | baixo | teste de paridade em todo `cargo test` (D27) |
| ergonomia pior que Blade → abandono | média | alto | M1 mede linhas; M2 mede tempo de correção |
| escopo cresce para "framework" | alta | alto | D3/D4: atributo novo exige decisão registrada |
| formatter vira produto | média | alto | `fmt` só reemite do AST; sem gramática própria (D6) |

## 4. Critérios de abandono (kill criteria)

O projeto é encerrado, sem drama, se qualquer um ocorrer:

1. `M1` não atinge ≤ 1,4× de linhas;
2. `M2` não reduz linhas **e** não melhora a taxa de correção de erro por não-autor;
3. o custo de manutenção do parser (horas/ano) supera o custo dos defeitos que ele evita — medido em
   dois ciclos de atualização de `rustc`;
4. o consumidor adota um framework que passe a validar atributo intrínseco (ver
   [`14-alternativas.md`](14-alternativas.md) §5).

Ao encerrar, o repositório permanece como **registro de decisão** (este conjunto de documentos) e o
`html-auy` é vendorizado como parte do consumidor, sem macro.

## 5. Sequência de implementação (fora dos marcos)

```
1. escaper + Markup + property tests          (base de tudo)
2. tabela de elementos em Rust (sem build.rs) (D27)
3. html! com parse + codegen sem validação    (prova o pipeline)
4. validação N2 completa + trybuild
5. output/N4 como dev-dependency + teste      (D30)
6. corpus de conformidade                     (descartado, F7 — ADR 0002)
```

Cada passo tem que deixar o repositório **verde** (`cargo fmt --check` + `clippy -D warnings` +
`cargo test`), porque o consumidor vendoriza a cada passo. Não existe `cq check`: é a decisão D28.