//! Campos de formulário: um controle, seu rótulo e o `aria-*` que a acessibilidade exige.

use crate::escape;
use crate::markup::Markup;
use crate::msg::{Msg, Text};
use crate::value::{AttrVal, Autocomplete, FieldName};
use alloc::format;
use alloc::string::String;

/// Estado do campo: valor, obrigatoriedade, erro e ajuda.
#[derive(Debug, Clone)]
pub struct FieldState<'a> {
    value: AttrVal<'a>,
    required: bool,
    checked: bool,
    error: Option<Text<'a>>,
    help: Option<Text<'a>>,
    autocomplete: Option<Autocomplete<'a>>,
}

impl<'a> FieldState<'a> {
    /// Campo com o valor atual (vazio quando não há).
    #[must_use]
    pub const fn new(value: AttrVal<'a>) -> Self {
        Self {
            value,
            required: false,
            checked: false,
            error: None,
            help: None,
            autocomplete: None,
        }
    }

    /// Marca o campo como obrigatório (`required` + `*` fora do rótulo).
    #[must_use]
    pub const fn required(mut self, yes: bool) -> Self {
        self.required = yes;
        self
    }

    /// Estado de `checkbox`/`radio` (`checked`).
    #[must_use]
    pub const fn checked(mut self, yes: bool) -> Self {
        self.checked = yes;
        self
    }

    /// Token de `autocomplete` validado.
    #[must_use]
    pub const fn autocomplete(mut self, value: Autocomplete<'a>) -> Self {
        self.autocomplete = Some(value);
        self
    }

    /// Mensagem de erro (liga `aria-invalid` e `aria-describedby`).
    #[must_use]
    pub const fn error(mut self, text: Text<'a>) -> Self {
        self.error = Some(text);
        self
    }

    /// Texto de ajuda ligado por `aria-describedby`.
    #[must_use]
    pub const fn help(mut self, text: Text<'a>) -> Self {
        self.help = Some(text);
        self
    }
}

/// Tipo de `<input>` que o [`Field`] sabe rotular.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// `type="text"`.
    Text,
    /// `type="email"`.
    Email,
    /// `type="tel"`.
    Tel,
    /// `type="url"`.
    Url,
    /// `type="search"`.
    Search,
    /// `type="password"`.
    Password,
    /// `type="number"`.
    Number,
    /// `type="date"`.
    Date,
    /// `type="time"`.
    Time,
    /// `type="datetime-local"`.
    DateTimeLocal,
    /// `type="month"`.
    Month,
    /// `type="week"`.
    Week,
    /// `type="color"`.
    Color,
    /// `type="range"`.
    Range,
}

impl InputType {
    /// O valor do atributo `type`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
            Self::Password => "password",
            Self::Number => "number",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Color => "color",
            Self::Range => "range",
        }
    }
}

/// Opção de [`Field::select`]: valor, rótulo e estado próprio.
#[derive(Debug, Clone, Copy)]
pub struct SelectOption<'a> {
    value: AttrVal<'a>,
    label: Text<'a>,
    selected: bool,
    disabled: bool,
}

impl<'a> SelectOption<'a> {
    /// Opção com valor e rótulo; não selecionada e habilitada.
    #[must_use]
    pub const fn new(value: AttrVal<'a>, label: Text<'a>) -> Self {
        Self {
            value,
            label,
            selected: false,
            disabled: false,
        }
    }

    /// Marca a opção como `selected` (o consumidor decide qual, inclusive por dado).
    #[must_use]
    pub const fn selected(mut self, yes: bool) -> Self {
        self.selected = yes;
        self
    }

    /// Marca a opção como `disabled`.
    #[must_use]
    pub const fn disabled(mut self, yes: bool) -> Self {
        self.disabled = yes;
        self
    }
}

/// Opção de checkbox/radio em grupo.
#[derive(Debug, Clone, Copy)]
pub struct ChoiceOption<'a> {
    value: AttrVal<'a>,
    label: Text<'a>,
    checked: bool,
    disabled: bool,
}

impl<'a> ChoiceOption<'a> {
    /// Opção com valor e rótulo; desmarcada e habilitada.
    #[must_use]
    pub const fn new(value: AttrVal<'a>, label: Text<'a>) -> Self {
        Self {
            value,
            label,
            checked: false,
            disabled: false,
        }
    }

    /// Marca a opção como `checked`.
    #[must_use]
    pub const fn checked(mut self, yes: bool) -> Self {
        self.checked = yes;
        self
    }

    /// Marca a opção como `disabled`.
    #[must_use]
    pub const fn disabled(mut self, yes: bool) -> Self {
        self.disabled = yes;
        self
    }
}

/// Campos do crate: `label` + `id` + `aria-*` decididos juntos, nunca à mão.
pub struct Field;

impl Field {
    /// `<input type="text">` com rótulo, ajuda e erro ligados.
    #[must_use]
    pub fn text<M: Msg>(name: FieldName<'_>, label: M, state: &FieldState<'_>) -> Markup {
        Self::input(name, label, InputType::Text, state)
    }

    /// `<input type="{kind}">` com o mesmo contrato de [`Field::text`].
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn input<M: Msg>(
        name: FieldName<'_>,
        label: M,
        kind: InputType,
        state: &FieldState<'_>,
    ) -> Markup {
        let id = identifier(&name);
        let mut out = String::with_capacity(192);
        open_block(&mut out, &id, label, state.required);
        out.push_str("<input type=\"");
        out.push_str(kind.as_str());
        out.push_str("\" id=\"");
        escape::attr(&mut out, &id);
        out.push_str("\" name=\"");
        escape::attr(&mut out, name.as_str());
        out.push_str("\" value=\"");
        escape::attr(&mut out, state.value.as_str());
        out.push('"');
        autocomplete_attr(&mut out, state);
        state_attributes(&mut out, &id, state);
        out.push('>');
        described(&mut out, &id, state);
        out.push_str("</div>");
        Markup(out)
    }

    /// `<select>` com opções como dado: valor, rótulo e `selected`/`disabled` por opção.
    ///
    /// Devolve `None` quando não há nenhuma opção selecionável (lista vazia ou todas `disabled`) —
    /// a mesma regra que o macro `html!` cobra via `select_has_selectable_option`.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn select<M: Msg>(
        name: FieldName<'_>,
        label: M,
        options: &[SelectOption<'_>],
        state: &FieldState<'_>,
    ) -> Option<Markup> {
        if !options.iter().any(|option| !option.disabled) {
            return None;
        }
        let id = identifier(&name);
        let mut out = String::with_capacity(options.len().saturating_mul(64).saturating_add(192));
        open_block(&mut out, &id, label, state.required);
        out.push_str("<select id=\"");
        escape::attr(&mut out, &id);
        out.push_str("\" name=\"");
        escape::attr(&mut out, name.as_str());
        out.push('"');
        autocomplete_attr(&mut out, state);
        state_attributes(&mut out, &id, state);
        out.push('>');
        for option in options {
            out.push_str("<option value=\"");
            escape::attr(&mut out, option.value.as_str());
            out.push('"');
            if option.selected {
                out.push_str(" selected");
            }
            if option.disabled {
                out.push_str(" disabled");
            }
            out.push('>');
            escape::text(&mut out, option.label.as_str());
            out.push_str("</option>");
        }
        out.push_str("</select>");
        described(&mut out, &id, state);
        out.push_str("</div>");
        Some(Markup(out))
    }

    /// `<textarea>` com o mesmo contrato de `text`.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn textarea<M: Msg>(name: FieldName<'_>, label: M, state: &FieldState<'_>) -> Markup {
        let id = identifier(&name);
        let mut out = String::with_capacity(192);
        open_block(&mut out, &id, label, state.required);
        out.push_str("<textarea id=\"");
        escape::attr(&mut out, &id);
        out.push_str("\" name=\"");
        escape::attr(&mut out, name.as_str());
        out.push('"');
        autocomplete_attr(&mut out, state);
        state_attributes(&mut out, &id, state);
        out.push('>');
        escape::text(&mut out, state.value.as_str());
        out.push_str("</textarea>");
        described(&mut out, &id, state);
        out.push_str("</div>");
        Markup(out)
    }

    /// `<input type="file">`; `value` não é emitido (arquivo não é preenchido pelo servidor).
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn file<M: Msg>(
        name: FieldName<'_>,
        label: M,
        accept: Option<AttrVal<'_>>,
        multiple: bool,
        state: &FieldState<'_>,
    ) -> Markup {
        let id = identifier(&name);
        let mut out = String::with_capacity(192);
        open_block(&mut out, &id, label, state.required);
        out.push_str("<input type=\"file\" id=\"");
        escape::attr(&mut out, &id);
        out.push_str("\" name=\"");
        escape::attr(&mut out, name.as_str());
        out.push('"');
        if let Some(accept) = accept {
            out.push_str(" accept=\"");
            escape::attr(&mut out, accept.as_str());
            out.push('"');
        }
        if multiple {
            out.push_str(" multiple");
        }
        state_attributes(&mut out, &id, state);
        out.push('>');
        described(&mut out, &id, state);
        out.push_str("</div>");
        Markup(out)
    }

    /// `<input type="checkbox">` com um único rótulo.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn checkbox<M: Msg>(name: FieldName<'_>, label: M, state: &FieldState<'_>) -> Markup {
        let id = identifier(&name);
        let mut out = String::with_capacity(192);
        open_block(&mut out, &id, label, state.required);
        out.push_str("<input type=\"checkbox\" id=\"");
        escape::attr(&mut out, &id);
        out.push_str("\" name=\"");
        escape::attr(&mut out, name.as_str());
        out.push('"');
        if !state.value.as_str().is_empty() {
            out.push_str(" value=\"");
            escape::attr(&mut out, state.value.as_str());
            out.push('"');
        }
        if state.checked {
            out.push_str(" checked");
        }
        state_attributes(&mut out, &id, state);
        out.push('>');
        described(&mut out, &id, state);
        out.push_str("</div>");
        Markup(out)
    }

    /// Grupo de checkboxes com `fieldset`/`legend`; `None` sem opção selecionável.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn checkbox_group<M: Msg>(
        name: FieldName<'_>,
        legend: M,
        options: &[ChoiceOption<'_>],
        state: &FieldState<'_>,
    ) -> Option<Markup> {
        choice_group(&name, legend, options, GroupKind::Checkbox, state)
    }

    /// Grupo de radios com `fieldset`/`legend`; `None` sem opção selecionável.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "o `FieldName` é consumo: evita clonar o `Cow` do chamador"
    )]
    #[must_use]
    pub fn radio_group<M: Msg>(
        name: FieldName<'_>,
        legend: M,
        options: &[ChoiceOption<'_>],
        state: &FieldState<'_>,
    ) -> Option<Markup> {
        choice_group(&name, legend, options, GroupKind::Radio, state)
    }
}

fn identifier(name: &FieldName<'_>) -> String {
    let mut id = String::with_capacity(name.as_str().len().saturating_add(2));
    name.id_into(&mut id);
    id
}

/// `autocomplete` quando o chamador declarou um token válido.
fn autocomplete_attr(out: &mut String, state: &FieldState<'_>) {
    if let Some(token) = state.autocomplete {
        out.push_str(" autocomplete=\"");
        escape::attr(out, token.as_str());
        out.push('"');
    }
}

/// `<fieldset>` com `legend` e os `aria-*` do grupo.
fn open_group(out: &mut String, id: &str, legend: impl Msg, state: &FieldState<'_>) {
    out.push_str("<fieldset id=\"");
    escape::attr(out, id);
    out.push('"');
    if state.help.is_some() || state.error.is_some() {
        out.push_str(" aria-describedby=\"");
        if state.help.is_some() {
            escape::attr(out, id);
            out.push_str("-help");
        }
        if state.help.is_some() && state.error.is_some() {
            out.push(' ');
        }
        if state.error.is_some() {
            escape::attr(out, id);
            out.push_str("-error");
        }
        out.push('"');
    }
    if state.error.is_some() {
        out.push_str(" aria-invalid=\"true\"");
    }
    if state.required {
        out.push_str(" aria-required=\"true\"");
    }
    out.push_str("><legend>");
    escape::text(out, legend.as_str());
    out.push_str("</legend>");
}

#[derive(Clone, Copy)]
enum GroupKind {
    Checkbox,
    Radio,
}

impl GroupKind {
    const fn tag(self) -> &'static str {
        match self {
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
        }
    }

    const fn brackets(self) -> bool {
        matches!(self, Self::Checkbox)
    }

    const fn required_on_input(self) -> bool {
        matches!(self, Self::Radio)
    }
}

fn choice_group<M: Msg>(
    name: &FieldName<'_>,
    legend: M,
    options: &[ChoiceOption<'_>],
    kind: GroupKind,
    state: &FieldState<'_>,
) -> Option<Markup> {
    if !options.iter().any(|option| !option.disabled) {
        return None;
    }
    let id = identifier(name);
    let mut out = String::with_capacity(options.len().saturating_mul(96).saturating_add(192));
    open_group(&mut out, &id, legend, state);
    for (index, option) in options.iter().enumerate() {
        let option_id = format!("{id}-{index}");
        out.push_str("<label for=\"");
        escape::attr(&mut out, &option_id);
        out.push_str("\"><input type=\"");
        out.push_str(kind.tag());
        out.push_str("\" id=\"");
        escape::attr(&mut out, &option_id);
        out.push_str("\" name=\"");
        escape::attr(&mut out, name.as_str());
        if kind.brackets() {
            out.push_str("[]");
        }
        out.push_str("\" value=\"");
        escape::attr(&mut out, option.value.as_str());
        out.push('"');
        if option.checked {
            out.push_str(" checked");
        }
        if option.disabled {
            out.push_str(" disabled");
        }
        if kind.required_on_input() && state.required {
            out.push_str(" required");
        }
        out.push('>');
        escape::text(&mut out, option.label.as_str());
        out.push_str("</label>");
    }
    described(&mut out, &id, state);
    out.push_str("</fieldset>");
    Some(Markup(out))
}

/// `<div><label for=…>{rótulo}</label>`, com o `*` **fora** do rótulo (não entra no nome acessível).
fn open_block(out: &mut String, id: &str, label: impl Msg, required: bool) {
    out.push_str("<div><label for=\"");
    escape::attr(out, id);
    out.push_str("\">");
    escape::text(out, label.as_str());
    out.push_str("</label>");
    if required {
        out.push_str("<span aria-hidden=\"true\">*</span>");
    }
}

/// `required`, `aria-describedby` (ajuda → erro) e `aria-invalid`.
fn state_attributes(out: &mut String, id: &str, state: &FieldState<'_>) {
    if state.required {
        out.push_str(" required");
    }
    if state.help.is_some() || state.error.is_some() {
        out.push_str(" aria-describedby=\"");
        if state.help.is_some() {
            escape::attr(out, id);
            out.push_str("-help");
        }
        if state.help.is_some() && state.error.is_some() {
            out.push(' ');
        }
        if state.error.is_some() {
            escape::attr(out, id);
            out.push_str("-error");
        }
        out.push('"');
    }
    if state.error.is_some() {
        out.push_str(" aria-invalid=\"true\"");
    }
}

/// Os `span` que os ids de `aria-describedby` prometem: ajuda, depois erro.
fn described(out: &mut String, id: &str, state: &FieldState<'_>) {
    for (suffix, text) in [("-help", &state.help), ("-error", &state.error)] {
        let Some(text) = text else {
            continue;
        };
        out.push_str("<span id=\"");
        escape::attr(out, id);
        out.push_str(suffix);
        out.push_str("\">");
        escape::text(out, text.as_str());
        out.push_str("</span>");
    }
}
