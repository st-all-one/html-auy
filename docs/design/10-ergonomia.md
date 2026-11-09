# 10 — Ergonomia de desenvolvimento

## 1. O que uma pessoa precisa saber para escrever uma view

Só isto:

| Construção | Uso |
|---|---|
| `@if cond { … } @else { … }` | condicional |
| `for x in itens { … }` | repetição |
| `fn nome(p: Dados) -> Markup { html! { … } }` | a view é uma função |
| `Msg::X` | texto (traduzido) |
| — | `<html>`, `<head>`, `<body>`, `<title>` e `<h1>` **não** se escrevem: o `Page` os emite |

Não precisa de: `&`, lifetimes, `move`, `.into()`, `impl Trait`, closures, `?` dentro do bloco.
Tudo isso fica na assinatura dos primitivos e dos valores tipados.

**Cheat sheet de 12 linhas** acompanha o repositório do consumidor e é o único material de onboarding
necessário para quem já sabe HTML.

## 2. Formato das mensagens de erro

Toda mensagem tem três partes, nesta ordem:

```
error: <o que está errado>
  --> <arquivo>:<linha>:<coluna>
   = regra: <WCAG SC | invariante da spec>
   = ajuda: <como corrigir, com o código exato>
```

Exemplos normativos (todos com teste `trybuild`):

```
error: `img` exige `alt`
   = regra: WCAG 1.1.1 / spec:element.img
   = ajuda: `Alt::new("Descrição objetiva")` ou `Alt::decorative()`

error: atributo desconhecido `hreff` em `a`
   = ajuda: atributos válidos em `a`: href, hreflang, rel, target, download, referrerpolicy

error: `<div>` não é permitido dentro de `<p>`
   = regra: conteúdo-modelo (spec:element.p)
   = ajuda: use `<span>` (phrasing) ou mova o `<div>` para fora do `<p>`

error: `label for="nome"` não tem `id="nome"` no mesmo bloco
   = regra: WCAG 1.3.1
   = ajuda: use `Field::text(FieldName::new("nome"), Msg::Name, &state)` (gera label+id+aria)

> Link com texto ambíguo (WCAG 2.4.4) é regra de **N4/pós-MVP**: o macro não compara variantes de
> `Msg`, e o `audit` ainda não implementa a checagem.
```

## 3. Formatação

O macro **ignora whitespace**: a saída não depende de indentação, então não existe formatador de marcação.
O fonte é formatado pelo `rustfmt` como qualquer Rust — a marcação dentro do `html!` é texto que o
`rustfmt` não toca, e está tudo bem: nada depende dela.

`cq fmt` (na revisão de 2026-11-06, D22) sai por isso — o item abaixo descreve o formato que se recomenda
escrever, não um comando que o impõe.

- Formato recomendado (4 espaços, um elemento por linha, ordem de atributos = ordem da spec).
- Regras fixas: 4 espaços de indentação; **um elemento por linha**; ordem de atributos = ordem da
  spec; quebra de linha em 100 colunas; texto curto na mesma linha do elemento; `@if`/`for` com corpo
  entre chaves.
- Não formata Rust fora do macro (`rustfmt` cuida disso).
- **Nada depende do fonte**: não há teste de idempotência de formatação porque não há formatador — o
  teste que existe é o de **saída** (o mesmo corpo com 2, 4 ou 8 espaços gera bytes idênticos).

## 4. O que o codegen sintetiza (e o que é erro)

`cq fix` foi cortado (D28/A3): correção mecânica e inequívoca vira **default do codegen**; o que não é
inequívoco vira erro de compilação, nunca sugestão silenciosa.

| Comportamento | Como |
|---|---|
| `target="_blank"` sem `rel` → `rel="noopener noreferrer"` | sintetizado |
| `target="_blank"` com `rel` literal sem `noopener` | **erro** (`blank_target_implies_noopener`); `rel` computado também |
| `svg` sem nome acessível → `aria-hidden="true" focusable="false"` | sintetizado; atributo explícito vence |
| ordem dos atributos = `required ++ allowed` da ordem declarada na spec | determinística |
| `class` vazio, `srcset`/`sizes` sem valor válido | omitidos pelo `Writer` |
| `button` sem `type` | **erro**: `type` é obrigatório (nunca o submit implícito) |
| `th` sem `scope` | **erro**: `scope` é obrigatório |
| `img` com legenda solta → `<figure>`+`<figcaption>` | **não** existe: o autor escreve `figure`/`figcaption` |
| `role` redundante | **não** é removido pelo codegen: `role` é allowlist por elemento |

## 5. Anti-ruído (regra dura)

- Advisory conta por **página**, não por ocorrência.
- Regra que dispara mais de 3× na mesma página é **bug do primitivo**: abre issue no primitivo, não
  no call site.
- Nenhuma regra de N4 pode disparar em PR pequeno: o alvo é **zero ocorrência** nos arquivos de referência (`M3`), não um limite que desce.
- Falso-positivo acima de 5% das ocorrências invalida a regra até ser redesenhada.

## 6. Fluxo de trabalho

```
1. escreve a view          → cargo clippy / cargo check (erros de N1/N2)
2. renders + snapshot      → insta review (é aqui que o HTML se lê)
3. e2e/axe/sniff           → N5 no CI do consumidor
4. cargo fmt               → só o Rust em volta, nunca a marcação
```

*(O `cq fmt`/`cq check`/`cq fix` saíram na revisão — [`19`](19-revisao-arquitetural.md) A2/A3 e D28.)*

## 7. Revisão de código (checklist de PR de view)

- [ ] Nenhum elemento cru que tenha primitivo equivalente.
- [ ] Todo texto visível é `Msg` (nenhum literal).
- [ ] Todo link interno é `LUrl`.
- [ ] Nenhum `data-*` inventado (só os declarados).
- [ ] Snapshot revisado: a mudança de HTML é intencional e legível.
- [ ] Orçamento de elementos crus não aumentou.

## 8. Edição de texto (copy) por quem não é de Rust

- Textos vivem no catálogo de tradução (`.ftl`/enum gerado), não na view.
- Alterar copy = alterar um arquivo de tradução; nada de Rust.
- Revisão de HTML por quem é de HTML/CSS = revisar o **snapshot** (`.snap`), que é o único arquivo
  HTML do repositório.
- Se o time de design precisar mexer em estrutura, o caminho é primitivo + snapshot, não markup livre.

## 9. Suporte de editor

- `rust-analyzer` funciona **dentro** do macro para expressões Rust (variáveis, chamadas, tipos), com
  spans preservados.
- Não há hover/rename semântico de "tag HTML"; renomear um campo de struct quebra a compilação (é o
  que importa).
- `rustfmt` no pre-commit dá o formato estável do Rust em volta; a marcação é livre (a saída não depende dela).

## 10. Custos honestos de ergonomia

| Custo | Mitigação |
|---|---|
| markup dentro de `.rs` | snapshots revisáveis (o `.snap` é o HTML legível) |
| props opcionais via builder | assinatura legível; construtor recebe só o obrigatório |
| `Msg` para todo texto | gerado do catálogo; autocomplete do enum |
| sem formatador de marcação | nada depende do formato: indentação é livre |
| erro de macro pode ser verboso | mensagens normativas + `trybuild` garantindo texto |
