# Uso do `html-auy`

**Wiki operacional da série `0.1.x`.** Ela responde “como eu faço isto?” com código que segue a API
atual. A filosofia operacional está em [00 — Filosofia](00-filosofia.md); o *porquê* histórico das
decisões fica em [`docs/design/00`](../design/00-principios.md) a [`docs/design/27`](../design/27-licoes-do-eta.md). Aqui o foco
é a interface que o consumidor vê.

## O modelo em uma frase

```text
dados tipados → html! / componentes → Markup → View → Page → bytes
```

O crate não faz I/O, não lê ambiente, não resolve locale e não conhece HTTP. Ele recebe **dados já
decididos** e devolve **bytes determinísticos**.

| Peça | Papel |
|---|---|
| `html!` | valida o fragmento contra o vocabulário e gera `Markup` em tempo de compilação |
| `Markup` | HTML já renderizado; não há construtor público a partir de `String` |
| `View` | conteúdo do `<main>`, modais e assets exigidos pela tela |
| `ShellData` | dados do documento: título, `h1`, idioma, metadados, regiões, assets |
| `Page` | o único tipo que emite `<!DOCTYPE html>`, `<html>`, `<head>`, `<body>` e `<main>` |
| `Form` / `Field` | formulário com CSRF e campos com `label`/`id`/`aria-*` derivados |
| `audit` | passe opcional sobre o HTML final; feature `audit` |

## Começo em 30 segundos

```rust
use html_auy::{html, Lang, Page, ShellData, Text, View};

fn page() -> Page {
    let lang = Lang::parse("pt-BR").expect("constante");
    let data = ShellData::new(Text::new("Início"), Text::new("Início"), lang);
    let view = View::new(html! {
        p { Text::new("Olá, mundo.") }
    });
    Page::new(&data, view)
}
```

`Page` produz o documento completo, com `<title>`, `<h1>` e `<main>`. O guia passo a passo, incluindo
a saída exata e a fronteira HTTP, está em [Primeira página](02-primeira-pagina.md).

## Regras que explicam o resto

1. **Tipo antes de convenção.** `img` exige `Alt` e dimensões; `<form>` não compila sem
   `CsrfToken`; `Markup` não nasce de `String`.
2. **O shell é dono do documento.** `html`, `head`, `body`, `title`, `main` não existem dentro de
   `html!`; `Page` é a única porta.
3. **O contexto de escape está no tipo.** `Text`, `AttrVal`, `Href`, `Src`, `SrcSet`, `JsonLd` e
   `Timestamp` escolhem o escape na construção.
4. **Bytes canônicos.** Ordem de atributos, classes, `srcset`, assets e datas não depende de
   `HashMap`, locale ou indentação.
5. **Um `h1` por documento.** O `Page` o emite; a view começa no conteúdo do `<main>`.
6. **Sem JavaScript no markup.** Scripts são `Asset`s externos; `on*` e `<style>` não existem no
   vocabulário; `style=` é escape hatch auditado.
7. **O que o crate não vê, ele não promete.** Contraste, alvo de toque e layout computado são gates
   do consumidor (`axe`, `sniff`, e2e).

## Mapa da wiki

### Fundamentos

| Página | Pergunta que responde |
|---|---|
| [00 — Filosofia](00-filosofia.md) | Por que a API é assim e quais custos ela aceita? |
| [01 — Instalação](01-instalacao.md) | Como adicionar, qual feature ligar, qual MSRV, o que é `no_std`? |
| [02 — Primeira página](02-primeira-pagina.md) | Como sair de zero a um `Page` completo? |
| [03 — O macro `html!`](03-macro-html.md) | Qual é a gramática, o que compila e o que o compilador recusa? |
| [04 — Valores tipados](04-valores-tipados.md) | Qual tipo usar para texto, URL, imagem, data, classe e atributo? |
| [05 — Texto e i18n](05-texto-e-i18n.md) | Como entrar com texto, tradução e nomes acessíveis? |

### Interface

| Página | Pergunta que responde |
|---|---|
| [06 — Formulários](06-formularios.md) | Como montar `Form`, `Field`, CSRF, validação e re-render? |
| [07 — Mídia](07-midia.md) | Como emitir imagem responsiva, `picture`, vídeo, áudio e faixas? |
| [08 — Shell, assets e tema](08-shell-assets-e-tema.md) | Como controlar `head`, regiões, classes do root, CSS/JS e CDN? |

### Dados, segurança e integração

| Página | Pergunta que responde |
|---|---|
| [09 — Conteúdo dinâmico](09-conteudo-dinamico.md) | Como compor dados do banco, componentes, listas e classes dinâmicas? |
| [10 — Segurança](10-seguranca.md) | O que é impossível injetar, o que é escape hatch e como fica a CSP? |
| [11 — Acessibilidade](11-acessibilidade.md) | O que vem por construção e o que ainda precisa de gate? |
| [12 — HTTP e integrações](12-http-e-integracoes.md) | Como responder, cachear, fazer `303`/`422` e resposta parcial? |
| [16 — Projeto mínimo com Axum](16-projeto-minimo-axum.md) | Como juntar Cargo, adapter, views, assets, erros e formulários? |
| [17 — Guia de implementação](17-guia-de-implementacao.md) | Em que ordem implementar uma aplicação em fatias verticais? |
| [18 — Exemplo completo: portal governamental](18-exemplo-portal.md) | Como fica uma página inteira de portal municipal? |

### Qualidade e referência

| Página | Pergunta que responde |
|---|---|
| [13 — Testes e diagnóstico](13-testes-e-diagnostico.md) | Como testar, ler erros de compilação e usar `audit`? |
| [14 — Receitas](14-receitas.md) | Como resolver casos concretos sem reler a teoria? |
| [15 — Arquitetura](15-arquitetura.md) | Como a lib funciona por dentro e por que ela é assim? |
| [99 — Referência rápida](99-referencia-rapida.md) | Onde está a assinatura/construção de cada coisa? |

## Trilhas de leitura

- **Primeiro contato:** 00 → 01 → 02 → 03 → 04.
- **Formulário:** 02 → 06 → 10 → 12.
- **Migração de Blade/template:** 03 → 04 → 06 → 09 → 14.
- **Revisão de segurança:** 04 → 08 → 10 → 11.
- **Integração HTTP:** 02 → 08 → 12 → 16 → 13.
- **Primeiro projeto Axum:** 01 → 02 → 16 → 06 → 12.
- **Implementação end-to-end:** 00 → 01 → 02 → 17 → 16 → 06 → 12 → 13.
- **Página completa:** 02 → 16 → 18 → 12.
- **Entender a implementação:** 15 → 04 → 10 → 13.

## Convenções desta wiki

- `M` é um `enum` do consumidor que implementa `html_auy::Msg`; cada variante resolve um texto.
- `expect("…")` aparece em exemplos para deixar o valor visível. Em produção, propague o `Option`
  ou trate o dado na borda.
- Blocos marcados **“esboço”** dependem de tipos do seu domínio; os demais usam apenas a API pública.
- A saída é mostrada em uma linha porque o macro ignora whitespace e emite a forma canônica.
- “Não existe” significa que não há construtor público, atributo no vocabulário ou caminho de
  runtime; não é uma recomendação de estilo.

## O que a wiki não cobre

O crate **não** é um framework web, roteador, ORM, sistema de i18n, bundler, servidor de arquivos ou
motor de templates em runtime. Ele também não substitui `axe`, testes de teclado ou medição de
contraste. A fronteira está detalhada em [Segurança](10-seguranca.md) e
[Acessibilidade](11-acessibilidade.md).

---

**Comece por:** [00 — Filosofia](00-filosofia.md) e depois [01 — Instalação](01-instalacao.md).
