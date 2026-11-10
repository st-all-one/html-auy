# 20 — Fronteira de aplicação: o que o crate absorve do Blade

> **O que é:** a decisão, caso a caso, sobre os seis padrões de Blade que o crate **não** cobre
> ([`19`](19-revisao-arquitetural.md) §3). Para cada um: absorve ou não, como, por quê, e o custo.
> **Critério de aceitação:** um item só entra se **elimina uma classe de defeito** (não conveniência),
> cabe em **~100 LOC** e é cobrado por **`trybuild`**. Conveniência que não remove defeito fica no
> consumidor.

## 1. Medição que originou a pergunta

Corpus real: **225 `.blade.php`** (23,6k LOC, 2.084 usos de componente, 6.765 tags).

| Padrão | Ocorrências | O que ele resolve hoje |
|---|---|---|
| `$attributes` / `->merge()` | **478** / 15 | componente "burro": repassa atributo que não conhece |
| `@stack` / `@push` | **5** / 3 | contribuição para uma região do shell, de qualquer profundidade |
| `@csrf` / `@method` | **142** / 57 | protocolo de formulário (POST + método semântico) |
| `old()` / `$errors` / `session()` | **188** / **204** / **189** | estado implícito da request |
| `trans_choice`/`@choice` (plural) | **22** | pluralização por CLDR (1 comentário / 2 comentários) |
| `x-dynamic-component` + registry | 1 + `WidgetRegistry` | componente escolhido por string em runtime |

## 2. Decisão

| # | Caso | Absorve? | Mecanismo | Escopo | Por quê |
|---|---|---|---|---|---|
| **c** | `@csrf`/`@method` | **sim** | `Form::open(action, method, children)` com `Method::Post(token)`; `form` com `only = "component"` | `type` | CSRF faltando é **falha de segurança**, não de layout. `html!` não emite `<form>`: o token passa a ser o argumento que não se esquece |
| **b** | `@stack`/`@push` | **sim** | `View` = mônade de contribuições (`main`, `modals`, `assets`) devolvida para cima | `type` | pilha ambiente é ordem implícita de flush; valor devolvido é testável e não tem estado global |
| **d** | `old()`/`$errors` | **sim (a forma, não a fonte)** | `FieldState { valor, erro, ajuda }` + `Field::text(name, label, &state)` | `type` | é exatamente onde mora o defeito de a11y (`aria-invalid`/`aria-describedby` divergentes); hoje são 2 componentes duplicando a lógica |
| **a** | `$attributes` | **parcial** | `Class` combinável (`Class::of(&[…]).extend(…)`); `data-*` fechado por **gate**; **sem** bag | `block` + gate | 478 usos existem porque o componente não conhece o chamador. Em Rust o componente devolve `Markup` com as classes dele; o que sobra é classe do chamador e `data-*`, e só isso deve ser componível |
| **e** | plural (`trans_choice`/`@choice`) | **não** | `Msg` é handle; plural é `impl From<Msg> for Text`/`Display` do próprio enum no consumidor, nunca no crate | — | absorver plural = absorver dados CLDR dos 4 locales: dependência e política, zero garantia de HTML |
| **f** | componente por string | **não** | `dyn Widget -> Markup` no consumidor | — | não existir tag dinâmica **é** a garantia; o registry já é resolvido por trait object |

## 3. As formas de API

### 3.1 `Form` — o CSRF como argumento obrigatório

```rust
// html_auy::form — o crate não emite token; exige um.
pub struct CsrfToken<'a>(&'a str);
pub enum Method<'a> {
    Get,
    Post(CsrfToken<'a>), Put(CsrfToken<'a>), Patch(CsrfToken<'a>), Delete(CsrfToken<'a>),
}

impl Form {
    /// Emite `<form>`, o `_token` e — para PUT/PATCH/DELETE — o `_method` oculto.
    /// Sem `CsrfToken` não há `Post`/`Put`/`Patch`/`Delete`: `form` tem `only = "component"`,
    /// então `html!` recusa `<form>`.
    pub fn open(action: &Action<'_>, method: Method<'_>, children: Markup) -> Markup;
}
```

Spec já aplicado: `[element.form] only = "component"` + invariante **`form_requires_csrf`** (escopo `type`),
documentada em [`04`](04-validacao.md) §4.1. Consumidor: `CsrfToken::from(session.token())` — uma linha,
um lugar.

### 3.2 `View` — a pilha vira valor devolvido

```rust
pub struct View { main: Markup, modals: Vec<Markup>, assets: Assets }

impl View {
    pub fn new(main: impl Into<Markup>) -> Self;
    pub fn modal(mut self, m: impl Into<Markup>) -> Self;
    pub fn asset(mut self, a: Asset) -> Self;
    pub fn merge(self, other: View) -> Self;   // soma modals/assets; `main` é o do shell
}

// no formulário, 4 níveis abaixo:
View::new(html! { Form::open(&action, Method::Post(csrf)) { … } }).modal(html! { dialog { … } })
// no topo:
Page::new(&data, view)     // o shell consome `main`, `modals` e `assets` — uma vez, na ordem dele
```

Por que não uma pilha ambiente: (1) a ordem de flush deixa de ser implícita; (2) `assert!(view.modals.len() == 1)`
é teste; (3) `Page` continua função pura, sem `static`/`thread_local`.

### 3.3 `FieldState` — o estado da request com um tipo só

```rust
pub struct FieldState<'a> {
    value: AttrVal<'a>,
    required: bool,
    checked: bool,
    error: Option<Text<'a>>,   // já resolvido: `Text::msg(Erro::Obrigatorio)` (D31)
    help: Option<Text<'a>>,
    autocomplete: Option<Autocomplete<'a>>,
}

impl<'a> FieldState<'a> {
    pub const fn new(value: AttrVal<'a>) -> Self;
    pub const fn required(self, yes: bool) -> Self;
    pub const fn checked(self, yes: bool) -> Self;
    pub const fn error(self, text: Text<'a>) -> Self;
    pub const fn help(self, text: Text<'a>) -> Self;
    pub const fn autocomplete(self, token: Autocomplete<'a>) -> Self;
}

impl Field {
    /// Deriva `id`, `for`, `aria-describedby` (com os `<span>` de ajuda/erro), `aria-invalid`,
    /// `required` e o `*` **fora** do `<label>`.
    pub fn text<M: Msg>(name: FieldName<'_>, label: M, state: &FieldState<'_>) -> Markup;
    pub fn input<M: Msg>(name: FieldName<'_>, label: M, kind: InputType, state: &FieldState<'_>) -> Markup;
    pub fn textarea<M: Msg>(name: FieldName<'_>, label: M, state: &FieldState<'_>) -> Markup;
    pub fn file<M: Msg>(name: FieldName<'_>, label: M, accept: Option<AttrVal<'_>>, multiple: bool, state: &FieldState<'_>) -> Markup;
    pub fn checkbox<M: Msg>(name: FieldName<'_>, label: M, state: &FieldState<'_>) -> Markup;
    pub fn checkbox_group<M: Msg>(name: FieldName<'_>, legend: M, options: &[ChoiceOption<'_>], state: &FieldState<'_>) -> Option<Markup>;
    pub fn radio_group<M: Msg>(name: FieldName<'_>, legend: M, options: &[ChoiceOption<'_>], state: &FieldState<'_>) -> Option<Markup>;
    pub fn select<M: Msg>(name: FieldName<'_>, label: M, options: &[SelectOption<'_>], state: &FieldState<'_>) -> Option<Markup>; // None: nenhuma opção selecionável
}
```

`old()` e `$errors` continuam sendo do consumidor — mas **uma** vez: `FormState::from_request(request)`
produz os `FieldState`, e as 188 leituras de old + 114 `@error` espalhadas deixam de existir. `FieldName`
tipado (`FieldName::new("sections[0][props][campo]")`) valida a sintaxe de nome indexado — o `str_replace`
que `select.blade.php` e `form-group.blade.php` repetem.

### 3.4 `Class` combinável e `data-*` fechado (o que substitui o bag)

```rust
pub struct Class { tokens: Vec<&'static str> }
impl Class {
    pub fn of<T: ClassToken>(tokens: &[T]) -> Self;  // ordem canônica no render
    pub fn extend(self, other: Class) -> Self;       // dedupe
}
```

`data-*` **não** vira enum no crate: os nomes são o contrato do JS **do consumidor** e mudam com ele
(150 nomes distintos no corpus de origem). O macro valida a **sintaxe** do nome; o conjunto fechado é um
**gate** (`tools/data-check.py`) que confere tabela ↔ uso ↔ leitor no JS. Colocar 150 nomes de um produto
dentro do crate seria acoplar o crate a um produto por um defeito de baixa severidade — o mesmo raciocínio
de §2 (e), aplicado a `data-*`.

`Class::extend` cobre o único caso legítimo do `->merge()` (`select select-bordered` + `w-full` do chamador).
`data-*` passa a ter conjunto fechado — o que já era a intenção em [`07`](07-js-e-assets.md) §3, mas sem
mecanismo. **Bag genérico não entra**: aceitar atributo desconhecido num componente é reabrir o vocabulário
depois de tê-lo fechado; o preço é enumerar o passthrough, e o benefício é que um `data-*` inventado falha.

## 4. O que fica de fora, definitivamente

| Não entra | Motivo |
|---|---|
| attribute bag (`$attributes`) | reabre o vocabulário; a garantia é a recusa |
| pilha ambiente (`@push` implícito) | ordem implícita; `View` faz o mesmo com valor |
| tag/componente por string | nome inválido vira `match` exaustivo, não `None` silencioso |
| plural/CLDR | dados de locale, não semântica de HTML |
| `style=`, `on*`, `{!! $var !!}` | o escape hatch que o projeto existe para fechar (`Markup` cobre o caso legítimo) |
| token/CSRF **gerado** | o crate exige o token, não o produz (sessão é do consumidor) |

## 5. Teste de aceitação (antes de escrever o crate inteiro)

Portar **dois componentes reais** do projeto de origem e medir:

| Componente | Hoje | Critério |
|---|---|---|
| `Media/View/picture.blade.php` (77 linhas) | `alt` default `''`, dimensões opcionais, `srcset` montado à mão, classe string | ≤ **1,3×** LOC e **≥ 3** defeitos recusados que o Blade permite (`alt` vazio, dims, `sizes`, classe) |
| `Cms/components/select.blade.php` (110 linhas) | 2× `str_replace` de id, `aria-describedby` montado à mão, bag de atributos | ≤ **1,3×** LOC e `aria-*` **derivado** (nenhum `aria-*` escrito à mão) |

Se algum passar de 1,3×, **o DSL tem buraco de ergonomia** — conserte o buraco ou corte o item; não
"aceite porque é mais seguro".

## 6. Decisões e sequência

| ID | Decisão | Status | Custo |
|---|---|---|---|
| **D23** | `<form>` só via `Form::open`, com o `CsrfToken` dentro do `Method`; `form` com `only = "component"` | **aceita**; spec e código prontos | `trybuild` de `Form::open` no macro e de `Method::Post` sem token |
| **D24** | `View` (mônade de contribuições) substitui `@stack`/`@push` | **aceita** | ~40 LOC |
| **D25** | `FieldState` + `Field` derivam `aria-*`; `FieldName` valida nome indexado | **aceita** | ~90 LOC |
| **D26** | `Class` combinável; `data-*` fechado por **gate** (não pelo macro); **sem** attribute bag | **aceita** | ~60 LOC |
| — | plural e componente dinâmico **não** entram (§2, §4) | decidido | 0 |

Plano de execução com arquivos, assinaturas e casos de teste: [`21-implementacao.md`](21-implementacao.md).

Ordem: **D23 → D26 → D25 → D24** (segurança, vocabulário, acessibilidade, composição). Cada passo é
validado pelo porte de §5; nenhum deles muda a fronteira "compilador recusa × gate falha" — apenas move
mais três defeitos para o lado do compilador.
