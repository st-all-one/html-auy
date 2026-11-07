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
| idioma da página | `ShellData` exige `Lang` |
| título | `Page` exige `title`/`heading` (forneça textos não vazios) |
| um `h1` | `Page` emite um; não escreva outro na view |
| skip link | `ShellData::skip(Text)` |
| status vivo | `Page` emite `role="status" aria-atomic="true"` persistente |
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

## Formulários

```rust
let nome = FieldState::new(AttrVal::new(&state.nome))
    .required(true)
    .help(Text::new("Como no documento"))
    .error(Text::new("Informe o nome completo"));

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
