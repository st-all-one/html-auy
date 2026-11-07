# Acessibilidade no `html-auy`

Este documento descreve o modelo de acessibilidade implementado pelo crate, a fronteira entre as garantias de compilação e os gates do consumidor, e as APIs concretas que uma aplicação deve usar. Ele complementa [`05-acessibilidade.md`](05-acessibilidade.md), que mantém o mapeamento WCAG 2.2 AA/eMAG.

O alvo é **WCAG 2.2 AA** no contexto do projeto. Acessibilidade não é uma revisão feita no fim: ela é emitida pelo vocabulário e pelos primitivos, e depois verificada sobre o documento final onde o compilador local não consegue ver a página inteira.

## 1. Estratégia

O projeto usa quatro camadas, em ordem decrescente de força:

1. **Ausência de API / exigência de tipo.** O errado não constrói: `img` sem `Alt`, `Video::new` sem legendas, formulário de escrita sem `CsrfToken`.
2. **Defaults dos primitivos e validação da macro.** `Field`, `Img`, `Page`, `ShellData` e `Form` emitem a plumbing; `html!` recusa aninhamento inválido, atributo obrigatório ausente, `label for` órfão e atributo proibido no bloco local.
3. **Audit de documento (`N4`).** `html_auy::audit::check` roda sobre o HTML montado atrás de `feature = "audit"`. Ele pega problemas que nenhum fragmento isolado decide: `id` duplicado, múltiplos `h1`, salto de nível, colisão de nomes de landmark, referência ARIA órfã e `aria-current` repetido.
4. **Gates do consumidor no navegador (`N5`).** `axe`, `sniffCSS`, testes de teclado e testes ponta a ponta cobrem estilo computado, contraste, alvo de toque, ordem de foco, oclusão e comportamento pós-swap.

Não existe camada advisory. Uma regra ou faz código válido deixar de compilar, ou é um teste que falha no CI. Aviso que dispara vinte vezes por página não é tratado como garantia de acessibilidade.

## 2. O que o crate garante

### 2.1 Imagens e conteúdo não textual

- `Img::new` exige `Src`, `Alt`, `WidthPx` e `HeightPx`.
- `Alt::new("")` significa decoração explícita; não existe caminho para alt vazio acidental.
- SVG decorativo tem caminho tipado; SVG informativo exige nome.
- As dimensões intrínsecas são obrigatórias para evitar layout shift e espaço reservado vazio.

```rust
let image = Img::new(
    Src::parse("/media/capa.avif").expect("fonte válida"),
    Alt::new("Ônibus no terminal"),
    WidthPx::new(720),
    HeightPx::new(480),
);
```

### 2.2 Formulários e nomes acessíveis

`Field` deriva as peças que normalmente são esquecidas:

- `<label for="...">` e o `id` do input;
- `aria-describedby` para os `<span>` de ajuda e erro;
- `aria-invalid="true"` quando há erro;
- `required` quando obrigatório;
- o `*` de obrigatório fora do `<label>`, para não entrar no nome acessível;
- `<fieldset>` + `<legend>` para grupos de checkbox/radio.

```rust
let name = FieldState::new(AttrVal::new(&state.name))
    .required(true)
    .help(Text::new("Como consta no documento"))
    .error(Text::new("Informe o nome completo"));

let field = Field::text(FieldName::new("nome"), M::Name, &name);
```

Regras que decorrem do bloco local:

- controle sem nome acessível não compila;
- `aria-hidden="true"` em controle focável é recusado;
- `label for` precisa resolver um `id` do mesmo bloco;
- `th` exige `scope`;
- `button` exige `type`;
- `autocomplete` só sai de `Autocomplete::parse`; é vocabulário fechado, não string livre.

O caminho de passkey/autenticação é tipado:

```rust
let state = FieldState::new(AttrVal::new(""))
    .autocomplete(Autocomplete::parse("username webauthn").expect("token válido"));

let password = Field::input(
    FieldName::new("usuario"),
    M::User,
    InputType::Password,
    &state,
);
```

O crate emite o contrato acessível do formulário; o consumidor é dono do fluxo de login, da política de CAPTCHA, do comportamento de colar e da cerimônia de passkey.

### 2.3 Estrutura da página, skip link e ajuda

`Page` é o único construtor de documento. Ele emite:

- `<!DOCTYPE html>`, `<html lang>`, `<head>` e `<body>`;
- `<title>` a partir do `ShellData` obrigatório;
- um `<main id="main" tabindex="-1">`;
- uma região viva persistente `<div role="status" aria-atomic="true"></div>`;
- um `<h1>` a partir do `ShellData` no caminho normal;
- o skip link quando `ShellData::skip` é fornecido.

O skip link é o primeiro elemento focável:

```rust
let data = ShellData::new(
    Text::new("Solicitações"),
    Text::new("Solicitações"),
    Lang::parse("pt-BR").expect("idioma"),
)
.skip(Text::new("Pular para o conteúdo"));
```

A ajuda consistente ocupa um slot fixo do shell:

```rust
let help = html! {
    p { Text::new("Precisa de ajuda? Fale com o atendimento.") }
};

let data = ShellData::new(title, heading, lang).help(help);
```

`help` é emitido depois do header e antes de `before_main`/`<main>`, então ocupa a mesma posição estrutural em todas as páginas que fornecem o bloco. O consumidor decide o conteúdo e se o slot existe.

### 2.4 Landmarks, headings e `aria-current`

- `Regions` é dono das contribuições de header, before-main, after-main e footer.
- Com dois ou mais landmarks `nav`/`aside`, cada um precisa de nome acessível distinto.
- O `audit` cobra `landmark_names_distinct` e `landmark_names_required`.
- `Page` emite um `h1`; a view normalmente usa `h2`–`h6`; o `audit` cobra exatamente um `h1` e nenhum salto de nível.
- `aria-current` é único dentro de cada `<nav>` mais próximo; navegações separadas podem ter seu próprio item atual. A regra de audit ignora `aria-current="false"`.

```rust
let mut regions = Regions::new();
regions.header = Some(html! {
    header {
        nav aria_label=(Label::text(M::MainNav)) {
            a href=(home) aria_current="page" { M::Home }
        }
    }
});
```

### 2.5 Região viva

O shell sempre emite uma região de status persistente e vazia:

```html
<div role="status" aria-atomic="true"></div>
```

O consumidor a preenche por swap ou atualização de script. Uma região inserida apenas depois da atualização pode não ser anunciada de forma confiável. Não adicione um segundo `aria-live` a um `role="status"`/`role="alert"`; o papel já implica o comportamento de região viva.

### 2.6 Alternativas de mídia

O vocabulário de mídia separa legendas, audiodescrição e alternativa textual completa:

- `Video::new(src, Track::captions(...))` exige legendas para vídeo pré-gravado (WCAG 1.2.2).
- `Track::descriptions(src, label)` é a faixa tipada de audiodescrição (WCAG 1.2.3/1.2.5).
- `Video::transcript(Transcript)` é a alternativa textual completa para vídeo quando não há faixa descritiva; não substitui a faixa de audiodescrição exigida por 1.2.5.
- `Audio::transcript(Transcript)` é a alternativa textual para áudio pré-gravado (WCAG 1.2.1). `Track` não substitui transcrição de áudio, porque áudio não tem área visual de legenda.

```rust
use html_auy::{Audio, Href, LUrl, Src, Text, Transcript};

let transcript = Transcript::new(
    Href::internal(LUrl::new("/media/episodio-transcricao").expect("rota")),
    Text::new("Transcrição do episódio"),
);

let audio = Audio::new(Src::parse("/media/episodio.mp3").expect("fonte"))
    .transcript(transcript);
```

`Audio::transcript` e `Video::transcript` envolvem a mídia em `<figure>` e adicionam `<figcaption>` com o link da transcrição. A figure é a relação semântica entre a mídia e sua alternativa.

### 2.7 ARIA e atributos

- `aria_label` aceita `Label` (`Label::text`/`Label::sr_only`), não `&str`.
- `role` é enum por elemento, não string livre.
- `aria-checked`, `aria-expanded` e `aria-current` só existem onde o papel permite.
- `aria-label` sobre texto visível divergente é recusado.
- `data-*` é escalar e validado; não serialize JSON no atributo.
- `accesskey` é proibido: a ativação varia por navegador/leitor de tela e pode colidir com atalhos nativos.
- `popover="hint"` é valor global declarado para dicas/tooltips; a resolução de `popovertarget` é cobrada pelo audit.
- `draggable` é proibido; interações de arrastar precisam de alternativa explícita por botão/teclado.

### 2.8 `Alt+1` e atalhos próprios

O crate não implementa `Alt+1`. Ele fornece o skip link e a navegação por landmarks. `accesskey` é proibido por decisão. Se um produto exige `Alt+1`, isso é um listener de teclado ou uma decisão de plataforma do consumidor, não um atributo do vocabulário HTML. O crate não consegue garantir a ativação entre combinações de navegador e leitor de tela.

## 3. Audit de documento

Ative `audit` nos testes ou no CI:

```toml
[dev-dependencies]
html-auy = { version = "0.1", features = ["audit"] }
```

```rust
let findings = html_auy::audit::check(page.markup().as_str());
assert!(findings.is_empty(), "{findings:#?}");
```

`Finding` expõe a regra, a mensagem e informação do nó; a saída é determinística e segue a ordem do documento, com a ordem fixa das regras desempatando achados no mesmo nó.

As 13 regras são:

| Regra | O que detecta |
|---|---|
| `main_unique` | zero ou mais de um `<main>` |
| `single_h1` | zero ou mais de um `<h1>` |
| `heading_levels_no_skips` | `h2` → `h4` e saltos equivalentes |
| `ids_unique` | `id` repetido |
| `landmark_names_distinct` | landmarks do mesmo tipo com o mesmo nome |
| `landmark_names_required` | `nav`/`aside` repetidos sem nome acessível |
| `aria_current_unique_per_nav` | mais de um `aria-current` atual no mesmo `<nav>`; `false` é ignorado |
| `aria_refs_resolve` | `aria-labelledby`/`describedby`/`controls` apontando para id inexistente |
| `popover_target_resolves` | `popovertarget` sem `popover` correspondente |
| `no_content_derived_identifiers` | `id`/`name` fora do alfabeto estável gerado |
| `no_inline_script` | `<script>` sem `src` e que não é `application/ld+json` |
| `no_style_attribute` | qualquer `style=` |
| `no_event_handler` | qualquer `on*` |

`audit` não é sanitizador. É um teste de documento final. Ele não prova que o alt é útil, que o nome do link é descritivo no contexto ou que o layout renderizado é utilizável.

## 4. O que continua gate do consumidor

O crate não mede nem garante:

- contraste contra fundos compostos;
- tamanho de alvo em pixels CSS;
- foco visível, foco não obscurecido ou ordem de foco dentro de diálogos/popovers;
- redimensionamento de texto, reflow e espaçamento de texto;
- restrição de orientação;
- animação, `prefers-reduced-motion`, `forced-colors`, `prefers-contrast` ou transparência reduzida;
- oclusão, cabeçalho fixo cobrindo foco ou casos de transform/overflow;
- comportamento pós-swap do htmx;
- a qualidade, e não apenas a presença, do alt e do texto do link.

Use `axe`, `sniffCSS`, testes de teclado no navegador e revisão de conteúdo para essas camadas. O projeto entrega mecanismos e um audit de documento, não um navegador.

## 5. Testes de acessibilidade

O repositório verifica os mecanismos em vários níveis:

- testes `trybuild` para markup recusado e diagnóstico acionável;
- testes golden/snapshot para a saída HTML canônica;
- testes unitários e de propriedade para invariantes de valores;
- fuzz do parser fora do workspace principal;
- testes de `audit` sobre páginas completas com `feature = "audit"`;
- validação da spec e rastreabilidade da documentação em `tools/validate_spec.py`;
- gates do consumidor com `axe`/`sniffCSS`/teclado/e2e.

Uma mudança que altere um primitivo de acessibilidade deve atualizar o teste de compilação, a saída golden e o comportamento de audit correspondente. Uma regra nova que só o documento final decide pertence ao `audit`, não ao `html!`.

## 6. Regras de projeto

1. **Correto por default.** O primitivo emite a plumbing; o desenvolvedor escolhe o componente, não o ARIA.
2. **Escapulidas visíveis.** `Alt::decorative()`, labels sr-only e links de transcrição explícitos tornam a exceção visível.
3. **Nada de advisory silencioso.** Uma regra ou falha a compilação, ou falha um teste.
4. **Uma fonte de estrutura de documento.** `Page` é dono do documento; a view é dona do conteúdo do `<main>`; as regiões são donas dos landmarks.
5. **Regra ruidosa é bug do primitivo.** Se uma regra dispara repetidamente na mesma página, corrija o primitivo em vez de pedir workaround em cada view.
6. **A fronteira é declarada.** Se o crate não consegue observar, o consumidor precisa do gate.

## 7. Documentos relacionados

- [`05-acessibilidade.md`](05-acessibilidade.md) — mapeamento completo WCAG 2.2 AA/eMAG em português.
- [`04-validacao.md`](04-validacao.md) — níveis de validação e escopos das invariantes.
- [`23-acessibilidade-e-css.md`](23-acessibilidade-e-css.md) — o que é decidível no crate versus CSS de navegador.
- [`usage/11-acessibilidade.md`](usage/11-acessibilidade.md) — guia operacional de acessibilidade.
- [`12-testes.md`](12-testes.md) — camadas de teste e gates de CI.
