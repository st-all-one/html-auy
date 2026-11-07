[Índice](README.md) · [Próximo →](01-instalacao.md)

# 00 — Filosofia do projeto

**Em uma frase.** HTML gerado não deve ser texto que alguém revisa depois; deve ser um artefato cuja
forma correta é a mais fácil de escrever e cuja forma errada não compila.

## O problema que justifica a biblioteca

Gerar HTML em Rust já é possível de várias formas. O `html-auy` existe porque quatro falhas
estruturais continuam aparecendo mesmo com macro, template compilado ou framework reativo:

| Falha | Exemplo | Onde ela aparece |
|---|---|---|
| atributo não validado | `a hreff="/x"` | no navegador, se alguém olhar |
| escape sem contexto | `</script>` dentro de JSON-LD | na página quebrada |
| acessibilidade como disciplina | `img` sem `alt`, `th` sem `scope` | no `axe`, depois |
| template em runtime | arquivo lido na request | no erro de produção |

A resposta do projeto não é “testar melhor” nem “revisar com mais cuidado”. É mudar o lugar onde o
erro é descoberto:

| Descoberta | Mecanismo | Custo para o consumidor |
|---|---|---|
| em tempo de compilação | tipo ou macro | zero em runtime |
| na montagem da página | `audit` | barato, offline |
| no navegador | `axe`/`sniffCSS`/teclado | gate do consumidor |

Quanto mais cedo o erro é descoberto, menos ele depende de disciplina.

## Os princípios, na forma que o usuário sente

### 1. O HTML é o artefato

A saída é `Markup`: bytes determinísticos. Não há VDOM, hidratação, marcadores de comentário ou
estado serializado dentro do documento. O mesmo dado produz os mesmos bytes.

Isso aparece em: `Page::markup()`, `ETag`, snapshots e no fato de `Markup` não aceitar `String`.

### 2. O contexto de escape é o tipo

Não existe “escapar na hora”. O valor carrega o contexto: `Text`, `AttrVal`, `Href`, `Src`,
`SrcSet`, `JsonLd`, `Timestamp`. O construtor seguro é o único caminho.

Isso aparece em: `img` exigir `Alt`, `href` não aceitar `&str`, JSON-LD escapar `<`/`>`/`&`/`'`.

### 3. Prevenção acima de detecção

Se um erro pode ser impedido por ausência de API, ele é impedido. `<form>` sem CSRF não compila
porque o token mora no método, não porque existe um lint. `Markup` não nasce de `String` porque o
construtor não existe.

Isso aparece em: `Form::open`, `CsrfToken`, `Method::Post`, `Markup` sem `From<String>`.

### 4. Acessibilidade por construção

O componente emite a semântica: `label` + `for`, `aria-describedby`, `aria-invalid`, `fieldset` +
`legend`, `scope`, `type="button"`, `alt` obrigatório, legendas de vídeo. O desenvolvedor escolhe o
componente, não a plumbing.

O que não puder ser impedido por tipo vira erro de macro ou regra de documento (`audit`). Não existe
aviso que não bloqueia.

### 5. Sem runtime

Nenhum `unsafe`, interpretador, leitura de arquivo por request, laço de reconciliação ou avaliação
de template no cliente. O binário é código Rust gerado.

Isso aparece em: `no_std` + `alloc`, `audit` como única dependência opcional, `Asset` em vez de
script inline.

### 6. Determinismo é uma feature

Ordem de atributos, `class`, `data-*`, `srcset`, assets e regras de documento é canônica. Dois
renders iguais produzem bytes iguais; o `ETag` é confiável e o snapshot é revisável.

Isso aparece em: `Assets` com ordem declarada, `SrcSet` ordenado, `Class` ordenada/deduplicada,
`Page` com sequência fixa de `<head>`/`<body>`.

### 7. O shell é dono do documento

`html`, `head`, `body`, `title` e `main` não existem no vocabulário de `html!`. O `Page` é a única
porta; ele emite o documento e o `h1`. A view começa no conteúdo do `<main>`.

Isso evita páginas sem `lang`, dois `h1`, `main` aninhado e `title` esquecido — não por revisão, por
construção.

### 8. O consumidor é dono da fronteira

O crate não conhece locale, rotas, sessão, HTTP, tema, CSS, banco ou headers. `Msg`, `LUrl`,
`ClassToken`, `Asset`, `JsonGraph`, `Render` e `Partial` são contratos; a política fica na aplicação.

Isso aparece em: `Msg` como trait, `LUrl` sem roteador, `Class` sem design system, `Partial` sem
transporte.

### 9. Escape hatch explícito, não caminho normal

`style=` existe porque há casos legítimos, mas é **auditado** e muda a CSP. `Render`/`AttrValue`
existem para extensão, mas quem implementa assume a confiança. O caminho padrão não tem inline,
`on*` ou `<style>`.

Isso aparece em: `audit` com `no_style_attribute`, CSP sem `unsafe-inline`, `Class` como caminho de
estilo.

### 10. Nada de açúcar que esconde preço

Toda economia de digitação precisa ser explicável em uma linha. Um componente que busca dados,
resolve locale ou decide status HTTP não entra: ele esconde uma fronteira. `html!` é açúcar
explicável — valida e escreve bytes.

## O que a filosofia rejeita

| Rejeitado | Motivo |
|---|---|
| template em runtime | quebra tipagem, `ETag` e auditabilidade |
| `Raw`/`PreEscaped` público | bypass de escape é o defeito que o projeto elimina |
| atributo arbitrário `attrs(…)` | validação que aceita tudo não valida |
| `<form>` no vocabulário | CSRF deixaria de ser tipo |
| VDOM/hidratação/ilhas | o HTML deixaria de ser o artefato |
| `on*`/`<style>`/script inline | CSP e comportamento fora do arquivo |
| custom elements/Shadow DOM | comportamento no cliente; elemento indefinido antes do JS |
| “sanitizar depois” | a página já foi emitida; o tipo é a prevenção |
| lint que não bloqueia | ruído; ou é erro, ou é teste que falha |

## O que a filosofia aceita como custo

| Custo | Por quê | Mitigação |
|---|---|---|
| mais tipos na assinatura | contexto de escape explícito | builders e `Msg` |
| sem template em arquivo | compilação e determinismo | funções Rust + snapshots |
| `h1` vem do `Page` | documento único e coeso | `heading` em `ShellData` |
| `Form` sem `class` | CSRF e contrato mínimo | envolva o form em um elemento |
| `Asset` com caminho `'static` | ordem e hash de build previsíveis | constantes geradas no build |
| `data-*` de root com valor `'static` | nome/valor validados | mapeie para `dark`/`light` |
| `Asset` sem SRI | limite atual do tipo | self-host + CSP; SRI em `Preload` |
| sem métricas de browser | o crate não abre navegador | `axe`, `sniffCSS`, teclado |
| `audit` opcional | não puxar parser no default | feature no CI |
| sem formatador de markup | a saída não depende do fonte | `rustfmt` no Rust; snapshot no HTML |

Esses custos são deliberados. Se uma mudança elimina um deles sem criar um caminho de escape, ela é
bem-vinda; se cria um caminho de escape para “resolver” o custo, ela contraria a filosofia.

## Como decidir se algo pertence ao crate

Pergunte, nesta ordem:

1. **O tipo pode impedir?** Se sim, é tipo (`Alt`, `Href`, `CsrfToken`).
2. **O macro pode validar no bloco?** Se sim, é invariante (`p` não contém `div`, `button` tem
   `type`).
3. **Só o documento inteiro sabe?** Se sim, é regra de `audit` (`main_unique`, `single_h1`).
4. **Só o navegador sabe?** Se sim, é gate do consumidor (`axe`, `sniffCSS`).
5. **É política da aplicação?** Então o crate fornece o dado, não a decisão (locale, tema, HTTP,
   sessão).

Se a resposta for “dá para aceitar qualquer string e validar depois”, a resposta é não.

## O que “pronto” significa

Uma funcionalidade não está pronta quando compila. Está pronta quando:

- o tipo torna o erro inexprimível, ou o macro recusa com span;
- a saída é determinística e tem teste de bytes;
- a documentação mostra o uso e o limite;
- o gate de acessibilidade/segurança aplicável foi executado;
- não há `unsafe`, código morto, warning ou placeholder.

## Resumo

1. **Tipo antes de convenção.**
2. **Compilador antes de runtime.**
3. **Documento antes de fragmento.**
4. **Determinismo antes de conveniência.**
5. **Fronteira explícita antes de mágica.**

O resto desta wiki é a aplicação dessas cinco frases à API.

---

[Índice](README.md) · [Próximo →](01-instalacao.md)
