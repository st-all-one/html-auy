[← Texto e i18n](05-texto-e-i18n.md) · [Índice](README.md) · [Próximo →](07-midia.md)

# 06 — Formulários

**Em uma frase.** `<form>` só existe por `Form::open`, e o token CSRF mora no **método**; os campos
existem por `Field`, que deriva `label`, `id`, `aria-describedby`, `aria-invalid` e `required` juntos.

## As duas portas

```rust
use html_auy::{
    Action, AttrVal, CsrfToken, Field, FieldName, FieldState, Form, Href, LUrl, Method, Text,
    html,
};

let action = Action::new(LUrl::new("/solicitacoes").expect("rota constante"));
let token = CsrfToken::new(csrf).expect("token não vazio");

let form = Form::open(
    &action,
    Method::Post(token),
    html! {
        (Field::text(FieldName::new("nome"), M::Name, &state.nome))
        button type="submit" { M::Send }
    },
);
```

`<form>` não está no vocabulário de `html!`. O caminho é:

1. montar os filhos como `Markup`;
2. chamar `Form::open` (ou `open_with_submit`);
3. inserir o resultado na view com `(form)`.

## Métodos e CSRF

| Método | `method` emitido | CSRF | `_method` |
|---|---|---|---|
| `Method::Get` | `get` | não | não |
| `Method::Post(token)` | `post` | sim (`_token`) | não |
| `Method::Put(token)` | `post` | sim | `put` |
| `Method::Patch(token)` | `post` | sim | `patch` |
| `Method::Delete(token)` | `post` | sim | `delete` |

- `CsrfToken::new("")` devolve `None`; não há formulário de escrita sem token.
- O crate **emite** o campo; validar o token e aplicá-lo à sessão é do servidor.
- `Form::open_with_submit(action, method, &SubmitKey, children)` acrescenta `_submit` para métodos
  não-GET. A chave é única por submissão; o servidor decide replay (`303` igual / `409` divergente).
- `Form::open` aceita apenas `Action` (`LUrl`, rota interna). Formulário para domínio externo não é
  representável.
- `Form::open` **não** recebe `class`/`id`/atributos extras. Para estilizar, envolva o formulário:

```rust
html! {
    section class="form-shell" {
        (form)
    }
}
```

## `FieldState`: valor, obrigatoriedade, erro e ajuda

```rust
let nome = FieldState::new(AttrVal::new(&state.nome))
    .required(true)
    .help(Text::msg(M::HelpName))
    .error(Text::msg(M::ErrorName));

let aceite = FieldState::new(AttrVal::new("sim"))
    .checked(state.accepted);
```

| Builder | Efeito |
|---|---|
| `FieldState::new(AttrVal)` | valor atual; vazio quando não há |
| `.required(bool)` | `required` + `*` fora do `<label>` |
| `.checked(bool)` | `checked` para checkbox/radio |
| `.autocomplete(Autocomplete)` | token validado de `autocomplete` |
| `.error(Text)` | `aria-invalid="true"` + `aria-describedby` + `<span id="…-error">` |
| `.help(Text)` | `aria-describedby` + `<span id="…-help">` |

O erro e a ajuda são `Text`; não há `String` cru. O `*` de obrigatório fica fora do `<label>` para
não entrar no nome acessível.

## Campos

### Texto, e-mail, data, número, etc.

```rust
let nome = Field::text(FieldName::new("nome"), M::Name, &nome);
let email = Field::input(
    FieldName::new("email"),
    M::Email,
    InputType::Email,
    &FieldState::new(AttrVal::new(&state.email))
        .autocomplete(Autocomplete::parse("email").expect("token")),
);
```

`Field::text` é `Field::input(..., InputType::Text, ...)`. `InputType` cobre os 14 tipos nativos:
`Text`, `Email`, `Tel`, `Url`, `Search`, `Password`, `Number`, `Date`, `Time`, `DateTimeLocal`,
`Month`, `Week`, `Color`, `Range`.

O HTML de `Field::text("nome", "Nome", state)` é, em forma canônica:

```html
<div><label for="f-nome">Nome</label><span aria-hidden="true">*</span><input type="text" id="f-nome" name="nome" value="Ana" required aria-describedby="f-nome-help f-nome-error"><span id="f-nome-help">Como no documento</span><span id="f-nome-error">Informe o nome completo</span></div>
```

- `FieldName::new("nome")` normaliza o `name` e deriva `id="f-nome"`.
- `name` é o nome de submissão; `id` é `f-` + nome normalizado.
- `aria-describedby` liga ajuda e erro na ordem; não escreva esses ids à mão.
- `autocomplete` só sai de `Autocomplete::parse`; não aceita string livre. Para passkey, `username webauthn` é um valor válido.

### `select`

```rust
use html_auy::{AttrVal, SelectOption};

let opcoes = [
    SelectOption::new(AttrVal::new("sp"), Text::msg(M::SaoPaulo)).selected(state.uf == "sp"),
    SelectOption::new(AttrVal::new("rj"), Text::msg(M::RioDeJaneiro)),
];

let select = Field::select(
    FieldName::new("uf"),
    M::State,
    &opcoes,
    &FieldState::new(AttrVal::new("")),
).expect("há opção selecionável");
```

- O estado inicial é `SelectOption::selected(true)`, não `FieldState.value`.
- `Field::select` devolve `None` se **todas** as opções estão `disabled` ou a lista está vazia.
- Opções `disabled` e `selected` são propriedades da opção.
- O `select` não emite `size`; o tamanho visual é CSS.

### `textarea`

```rust
let descricao = Field::textarea(
    FieldName::new("descricao"),
    M::Description,
    &FieldState::new(AttrVal::new(&state.description))
        .required(true)
        .help(Text::msg(M::HelpDescription)),
);
```

O valor vai como **texto filho**, não como `value` (que `<textarea>` ignora).

### `file`

```rust
let anexo = Field::file(
    FieldName::new("anexo"),
    M::Attachment,
    Some(AttrVal::new("image/png, application/pdf")),
    true,
    &FieldState::new(AttrVal::new("")).required(true),
);
```

`Field::file` nunca emite `value`; `accept` e `multiple` são explícitos.

### Checkbox e grupos

```rust
use html_auy::{AttrVal, ChoiceOption};

let aceite = Field::checkbox(
    FieldName::new("aceite"),
    M::Accept,
    &FieldState::new(AttrVal::new("sim")).checked(state.accepted),
);

let canais = [
    ChoiceOption::new(AttrVal::new("email"), Text::msg(M::Email)).checked(state.email),
    ChoiceOption::new(AttrVal::new("telefone"), Text::msg(M::Phone)),
];

let contato = Field::radio_group(
    FieldName::new("contato"),
    M::Contact,
    &canais,
    &FieldState::new(AttrVal::new("")).required(true),
).expect("há opção");
```

- `checkbox` usa `FieldState::checked`; valor só é emitido se não vazio.
- `checkbox_group` emite `<fieldset>` + `<legend>`, ids `f-<nome>-<índice>` e `name="<nome>[]"`.
- `radio_group` emite o mesmo, mas `name="<nome>"` sem colchetes.
- Em grupo, `required` do estado vira `aria-required` no `fieldset`; no rádio, também `required` em
  cada input.
- Grupo sem opção selecionável devolve `None`.

## Re-render de validação (422)

O crate não decide status HTTP. O padrão do consumidor é:

```rust
let nome = FieldState::new(AttrVal::new(&submitted.nome))
    .required(true)
    .error(Text::msg(M::ErrorName));

let form = Form::open(&action, Method::Post(token), html! {
    div role="alert" {
        a href="/solicitacoes#f-nome" { M::FixName }
    }
    (Field::text(FieldName::new("nome"), M::Name, &nome))
    button type="submit" { M::Send }
});
```

Regras:

- `422`: re-renderize **o mesmo formulário**, com valores submetidos, erros por campo e um resumo
  `role="alert"` com links para os ids `f-…`.
- `303`: sucesso de escrita; `Location` para a próxima tela.
- `400`: corpo malformado; `409`: conflito/replay divergente.
- O corpo de erro não é uma página genérica: o usuário precisa ver e corrigir o que enviou.

## Segurança e limites

- `CsrfToken` só garante que o campo foi emitido; a validação é do servidor.
- `FieldName` normaliza o nome, mas nomes de campo devem vir do schema, não de entrada arbitrária.
- `Field` não aceita `class`/`style`/atributos extras; estilize por `name`/`id` ou envolva o campo.
- `Field::file` não valida tipo/tamanho; isso é do servidor.
- `FieldState::error` mostra a mensagem; a decisão de exibir ou não é sua.
- O resumo `role="alert"` deve existir só quando há erro real; `role="alert"` estático não é
  anunciado.

## Erros comuns

| Sintoma | Correção |
|---|---|
| `form` não compila em `html!` | use `Form::open` e `(form)` |
| `CsrfToken::new` devolve `None` | token vazio; gere um token real |
| `Field::select` devolve `None` | todas as opções `disabled` ou lista vazia |
| `select` não mostra o valor salvo | marque `SelectOption::selected(true)`; `FieldState.value` não seleciona |
| `aria-describedby` aponta para id inexistente | use `Field::*`; o id `f-…` é derivado |
| erro não aparece | `FieldState::error(Text)` precisa ser passado ao `Field` |
| checkbox não envia | `name` do checkbox é o nome; grupo usa `[]` |

---

[← Texto e i18n](05-texto-e-i18n.md) · [Índice](README.md) · [Próximo →](07-midia.md)
