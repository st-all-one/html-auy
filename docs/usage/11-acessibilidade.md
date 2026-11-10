[← Segurança](10-seguranca.md) · [Índice](README.md) · [Próximo →](12-http-e-integracoes.md)

# 11 — Acessibilidade

**Em uma frase.** Acessibilidade é emitida: o que o compilador consegue decidir vira tipo ou erro de
macro; o que depende do documento inteiro vira `audit`; o que depende do navegador continua no gate
do consumidor.

## O que vem por construção

| Requisito | Mecanismo |
|---|---|
| alternativa de imagem | `Img` exige `Alt`; `Alt::decorative()` é explícito |
| dimensões intrínsecas | `Img` exige `WidthPx`/`HeightPx` |
| legendas de vídeo | `Video::new` exige `Track::captions` |
| audiodescrição de vídeo | `Track::descriptions`; alternativa completa por `Video::transcript` |
| transcrição de áudio | `Audio::transcript(Transcript)` |
| passkey | `Autocomplete` aceita `username webauthn`; sem bloquear colar/gerenciador |
| idioma da página | `ShellData` exige `Lang` |
| título | `Page` exige `title`/`heading` (forneça textos não vazios) |
| um `h1` | `Page` emite um; não escreva outro na view |
| skip link | `ShellData::skip(Text)` |
| status vivo | `Page` emite `role="status" aria-atomic="true"` persistente |
| ajuda consistente | `ShellData::help(Markup)` fixa o slot entre header e `main` |
| nome de campo | `Field` exige `label` e deriva `id`/`aria-*` |
| erro de campo | `FieldState::error(Text)` liga `aria-invalid`/`aria-describedby` |
| grupo de escolha | `checkbox_group`/`radio_group` emitem `fieldset`+`legend` |
| link/botão com nome | o macro exige conteúdo ou nome acessível |
| `th` | exige `scope` |
| `iframe` | exige `title` |
| `dialog` | exige nome acessível |
| `a target="_blank"` | ganha `rel="noopener noreferrer"` |
| SVG decorativo | ganha `aria-hidden`/`focusable` quando sem nome |
| `aria-label` | exige `Label` (mensagem), nunca `&str` |

O que não compila não precisa ser revisado. O que depende de composição de página é o próximo nível.

## Shell: a base que não se repete

```rust
let data = ShellData::new(title, heading, lang)
    .skip(Text::msg(M::Skip))
    .regions(regions);
```

- `Page` emite `<html lang="…">`, `<title>`, `<main id="main" tabindex="-1">` e um `<h1>`.
- O skip link é o **primeiro focável** e aponta para `#main`.
- A região `role="status"` é persistente, vazia no primeiro render; swaps a preenchem.
- `noscript` fica depois do status e antes do header; o skip continua primeiro.
- Com dois ou mais `nav`/`aside`, cada um precisa de nome acessível. Com dois landmarks do mesmo
  tipo e o mesmo nome, `audit` acusa `landmark_names_distinct`.
- A ordem do DOM é a ordem visual; o crate não oferece `order`/`flex-direction: reverse` no CSS.
- `header`/`footer`/`aside`/`nav` são regiões; um slot vazio não é emitido.

### Skip link: um `<a>`, não um `<nav>` invisível

O “link para pular blocos” não é um elemento `nav` escondido. É um link comum, colocado antes de
qualquer conteúdo repetido:

```html
<a class="skip-link" href="#main">Pular para o conteúdo</a>
```

O `Page` o emite **quando** `ShellData::skip(Text)` recebe um texto. O destino é fixo:
`<main id="main" tabindex="-1">`; o `tabindex="-1"` é o que permite ao navegador mover o foco para
o `main` ao ativar o link. O link vem antes do header, da região de status e do `noscript`, portanto
é o primeiro focável da página.

O crate **não** decide tudo:

| Parte | Quem assume |
|---|---|
| markup, classe, `href="#main"`, ordem e alvo focável | `html-auy` (`ShellData::skip` + `Page`) |
| texto traduzido | consumidor (`Text::msg`) |
| esconder até o foco e mostrar no `:focus` | CSS do consumidor |
| existir ou não na página | consumidor: `.skip` é opt-in; o crate não tem locale para inventar o texto |
| `Alt+1` | ninguém no crate; `accesskey` é proibido (conflita com navegador/leitor de tela) |

Um CSS mínimo, no `@layer a11y` do consumidor:

```css
.skip-link {
  position: absolute;
  inset-block-start: 0;
  inset-inline-start: 0;
  z-index: 100;
  padding: 0.75rem 1rem;
  background: Canvas;
  color: CanvasText;
  transform: translateY(-150%);
}

.skip-link:focus {
  transform: translateY(0);
}
```

`Canvas`/`CanvasText` são cores de sistema e sobrevivem ao `forced-colors`. Não use
`display: none`, `visibility: hidden` ou `aria-hidden`: isso remove o link do teclado e do leitor de
tela. Se houver barra fixa, acrescente `scroll-margin-top` ao `#main`.

**Sobre `Alt+1`:** não é um atalho HTML padrão. O único atributo nativo é `accesskey="1"`, cuja
ativação varia por navegador/SO (`Alt+1`, `Alt+Shift+1`, `Ctrl+Option+1`) e conflita com atalhos do
leitor de tela; por isso o vocabulário o recusa. `aria-keyshortcuts` apenas **anuncia** um atalho
existente, não o implementa, e também não entra no vocabulário. O caminho suportado é `Tab` +
`Enter` no skip link; leitores de tela também pulam por landmarks. Se o produto exigir `Alt+1`, isso
é um listener no JS do consumidor, com o custo de conflito e de manutenção — não uma garantia do
crate.

## Formulários

```rust
let nome = FieldState::new(AttrVal::new(&state.nome))
    .required(true)
    .help(Text::msg(M::HelpName))
    .error(Text::msg(M::ErrorName));

let campo = Field::text(FieldName::new("nome"), M::Name, &nome);
```

O componente emite:

- `<label for="f-nome">Nome</label>`;
- `id="f-nome"`, `name="nome"`;
- `aria-describedby="f-nome-help f-nome-error"` quando há ajuda/erro;
- `aria-invalid="true"` quando há erro;
- `required` e o `*` **fora** do label (o asterisco não entra no nome acessível).

Grupos:

- `checkbox_group`/`radio_group` usam `<fieldset>` + `<legend>`;
- ids `f-<nome>-<índice>`;
- `name[]` apenas em checkbox;
- `aria-required` no grupo; `required` em cada rádio quando obrigatório;
- `None` quando todas as opções estão `disabled`.

O resumo de erro deve ser `role="alert"` **quando há erro**, com links para os ids dos campos. Um
`role="alert"` estático não é anunciado.

## Mídia

- `Img` exige `Alt`; use `Alt::decorative()` só para imagem puramente decorativa.
- `Picture` exige um `Img` de fallback; `Source` exige `srcset` e `sizes`.
- `Video` exige `captions`; `controls` é o padrão.
- `Track::subtitles` exige `Lang`; `Track::captions` não.
- `preload` de imagem deve reutilizar as mesmas instâncias de `SrcSet`/`Sizes` do `<img>`.
- O crate não mede contraste, alvo de toque ou legibilidade da imagem.

## `audit`: as regras de documento

Habilite `features = ["audit"]` e rode no CI:

```rust
#[test]
fn paginas_passam_no_audit() {
    for page in fixtures() {
        let findings = html_auy::audit::check(page.markup().as_str());
        assert!(findings.is_empty(), "{findings:#?}");
    }
}
```

| Regra | Corrige |
|---|---|
| `main_unique` | exatamente um `<main>` |
| `single_h1` | exatamente um `<h1>` |
| `heading_levels_no_skips` | `h2` → `h4` pulando nível |
| `ids_unique` | `id` repetido |
| `landmark_names_distinct` | dois landmarks com o mesmo nome |
| `landmark_names_required` | `nav`/`aside` repetidos sem nome |
| `aria_current_unique_per_nav` | mais de um `aria-current` atual no mesmo `<nav>` (valor diferente de `false`) |
| `aria_refs_resolve` | `aria-labelledby`/`describedby`/`controls` órfãos |
| `popover_target_resolves` | `popovertarget` sem `popover` |
| `no_content_derived_identifiers` | `id`/`name` instáveis |
| `no_inline_script` | script sem `src`/JSON-LD |
| `no_style_attribute` | `style=` |
| `no_event_handler` | `on*` |

`audit` é determinístico e não mede estilo computado. Ele não substitui `axe`, teclado ou
`sniffCSS`.

## O que ainda é gate do consumidor

| Preocupação | Ferramenta |
|---|---|
| contraste real (fundos compostos) | `axe`/medição |
| alvo de toque, foco visível, overflow | `sniffCSS` |
| ordem de tabulação em diálogo/popover | teste de teclado |
| foco após swap parcial | `axe` depois da troca |
| texto alternativo útil (não só presente) | revisão de conteúdo |
| idioma de partes | `lang` correto no dado |
| contraste de estados (hover/focus/disabled) | CSS + medição |
| `prefers-reduced-motion` | CSS |
| `forced-colors`, `prefers-contrast`, `prefers-reduced-transparency` | CSS do consumidor; o tema não emite essas camadas |
| espaçamento de texto (WCAG 1.4.12) | CSS: `rem`, `line-height`, sem altura fixa em texto |
| ajuda consistente entre páginas (WCAG 3.2.6) | `ShellData::help` fixa o slot; o consumidor decide o conteúdo |
| entrada redundante e autenticação (WCAG 3.3.7/3.3.8) | fluxo do consumidor; `Field` não bloqueia colar/gerenciador de senha |

## Erros comuns

| Sintoma | Correção |
|---|---|
| `audit` acusa `single_h1` | remova o `h1` da view; o `Page` já tem um |
| `nav` sem nome | `nav aria_label=(Label::text(M::MainNav))` |
| `aria-describedby` órfão | use `Field::*`; não escreva ids à mão |
| link só com ícone | texto visível ou `Label::sr_only` no conteúdo do link |
| botão sem `type` | `type="button"`/`"submit"`/`"reset"` |
| `style=` no HTML | use classe/token; se inevitável, aceite o finding e ajuste a CSP |
| `role="alert"` vazio na página | só emita o alerta quando houver erro |

## Foco e resposta parcial

O crate modela foco pós-swap em `Partial::focus(Id)` e regiões em `Partial::update(...)`. O
adaptador do consumidor deve:

1. marcar `aria-busy="true"` na região durante a requisição;
2. remover no swap;
3. mover o foco para o título/controle indicado;
4. rodar `axe` depois do swap.

Os detalhes estão em [HTTP e integrações](12-http-e-integracoes.md).

---

[← Segurança](10-seguranca.md) · [Índice](README.md) · [Próximo →](12-http-e-integracoes.md)
