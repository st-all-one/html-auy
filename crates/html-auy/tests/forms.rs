//! Formulários: tipos nativos, `autocomplete`, arquivo, checkbox, grupos e estado inicial.

#![expect(
    clippy::expect_used,
    reason = "teste: entradas fixas e válidas; o panic é diagnóstico"
)]

use html_auy::{
    AttrVal, Autocomplete, ChoiceOption, Field, FieldName, FieldState, InputType, Msg,
    SelectOption, Text,
};

#[derive(Clone, Copy)]
struct M;

impl Msg for M {
    fn as_str(self) -> &'static str {
        "Campo"
    }
}

#[derive(Clone, Copy)]
struct Ajuda;

impl Msg for Ajuda {
    fn as_str(self) -> &'static str {
        "Ajuda"
    }
}

#[derive(Clone, Copy)]
struct Erro;

impl Msg for Erro {
    fn as_str(self) -> &'static str {
        "Erro"
    }
}

fn src(value: &str) -> html_auy::Src<'_> {
    html_auy::Src::parse(value).expect("src válido")
}

#[test]
fn every_input_type_has_a_native_name() {
    let state = FieldState::new(AttrVal::new("x"));
    for (kind, expected) in [
        (InputType::Text, "text"),
        (InputType::Email, "email"),
        (InputType::Tel, "tel"),
        (InputType::Url, "url"),
        (InputType::Search, "search"),
        (InputType::Password, "password"),
        (InputType::Number, "number"),
        (InputType::Date, "date"),
        (InputType::Time, "time"),
        (InputType::DateTimeLocal, "datetime-local"),
        (InputType::Month, "month"),
        (InputType::Week, "week"),
        (InputType::Color, "color"),
        (InputType::Range, "range"),
    ] {
        let drawn = Field::input(FieldName::new("campo"), M, kind, &state);
        assert!(
            drawn
                .as_str()
                .contains(&format!("<input type=\"{expected}\" id=\"f-campo\"")),
            "{expected}: {}",
            drawn.as_str()
        );
    }
}

#[test]
fn autocomplete_is_emitted_only_when_valid_and_declared() {
    let state = FieldState::new(AttrVal::new("ana@example.com"));
    let without = Field::text(FieldName::new("email"), M, &state);
    assert!(!without.as_str().contains("autocomplete="));

    let state = state.autocomplete(Autocomplete::parse("email").expect("token"));
    let with = Field::text(FieldName::new("email"), M, &state);
    assert!(with.as_str().contains(" autocomplete=\"email\""));

    let area = Field::textarea(FieldName::new("bio"), M, &state);
    assert!(area.as_str().contains(" autocomplete=\"email\""));

    let options = [SelectOption::new(AttrVal::new("a"), Text::new("A"))];
    let select = Field::select(FieldName::new("uf"), M, &options, &state).expect("opções");
    assert!(select.as_str().contains(" autocomplete=\"email\""));
}

#[test]
fn file_has_no_value_and_emits_accept_multiple() {
    let state = FieldState::new(AttrVal::new("ignorado")).required(true);
    let drawn = Field::file(
        FieldName::new("anexo"),
        M,
        Some(AttrVal::new("image/png, application/pdf")),
        true,
        &state,
    );
    let s = drawn.as_str();
    assert!(s.contains("<input type=\"file\" id=\"f-anexo\" name=\"anexo\""));
    assert!(s.contains(" accept=\"image/png, application/pdf\""));
    assert!(s.contains(" multiple"));
    assert!(s.contains(" required"));
    assert!(!s.contains("ignorado"));
    assert!(!s.contains("value="));
}

#[test]
fn single_checkbox_uses_field_state() {
    let state = FieldState::new(AttrVal::new("sim")).checked(true);
    let drawn = Field::checkbox(FieldName::new("aceite"), M, &state);
    let s = drawn.as_str();
    assert!(s.contains(
        "<input type=\"checkbox\" id=\"f-aceite\" name=\"aceite\" value=\"sim\" checked"
    ));
    assert!(s.contains("id=\"f-aceite\""));
    assert!(s.contains(" for=\"f-aceite\""));
}

#[test]
fn checkbox_group_is_a_fieldset_with_unique_ids() {
    let state = FieldState::new(AttrVal::new(""))
        .required(true)
        .help(Text::msg(Ajuda))
        .error(Text::msg(Erro));
    let options = [
        ChoiceOption::new(AttrVal::new("vermelho"), Text::new("Vermelho")).checked(true),
        ChoiceOption::new(AttrVal::new("verde"), Text::new("Verde")),
        ChoiceOption::new(AttrVal::new("azul"), Text::new("Azul")).disabled(true),
    ];
    let drawn = Field::checkbox_group(FieldName::new("cores"), M, &options, &state).expect("grupo");
    let s = drawn.as_str();
    assert!(s.starts_with("<fieldset id=\"f-cores\""));
    assert!(s.contains("aria-describedby=\"f-cores-help f-cores-error\""));
    assert!(s.contains("aria-invalid=\"true\""));
    assert!(s.contains("aria-required=\"true\""));
    assert!(s.contains("<legend>Campo</legend>"));
    assert!(s.contains("name=\"cores[]\""));
    assert!(s.contains("id=\"f-cores-0\""));
    assert!(s.contains("id=\"f-cores-1\""));
    assert!(s.contains("id=\"f-cores-2\""));
    assert!(s.contains("value=\"vermelho\" checked"));
    assert!(s.contains("value=\"azul\" disabled"));
    assert!(s.contains(">Vermelho</label>"));
    assert!(s.contains("id=\"f-cores-help\""));
    assert!(s.contains("id=\"f-cores-error\""));
}

#[test]
fn radio_group_does_not_add_brackets_and_is_required_on_inputs() {
    let state = FieldState::new(AttrVal::new("")).required(true);
    let options = [
        ChoiceOption::new(AttrVal::new("mensal"), Text::new("Mensal")).checked(true),
        ChoiceOption::new(AttrVal::new("anual"), Text::new("Anual")),
    ];
    let drawn = Field::radio_group(FieldName::new("plano"), M, &options, &state).expect("grupo");
    let s = drawn.as_str();
    assert!(s.contains("<legend>Campo</legend>"));
    assert!(s.contains("name=\"plano\""));
    assert!(!s.contains("name=\"plano[]\""));
    assert!(s.contains("type=\"radio\""));
    assert!(s.contains("value=\"mensal\" checked required"));
    assert!(s.contains("value=\"anual\" required"));
}

#[test]
fn choice_groups_are_none_without_a_selectable_option() {
    let state = FieldState::new(AttrVal::new(""));
    assert!(Field::checkbox_group(FieldName::new("x"), M, &[], &state).is_none());
    assert!(Field::radio_group(FieldName::new("x"), M, &[], &state).is_none());

    let disabled = [ChoiceOption::new(AttrVal::new("a"), Text::new("A")).disabled(true)];
    assert!(Field::checkbox_group(FieldName::new("x"), M, &disabled, &state).is_none());
    assert!(Field::radio_group(FieldName::new("x"), M, &disabled, &state).is_none());
}

#[test]
fn choice_labels_and_values_are_escaped() {
    let state = FieldState::new(AttrVal::new(""));
    let options = [ChoiceOption::new(
        AttrVal::new("\"<x>"),
        Text::new("<script>alert(1)</script>"),
    )];
    let drawn = Field::radio_group(FieldName::new("x"), M, &options, &state).expect("grupo");
    assert!(!drawn.as_str().contains("<script>"));
    assert!(!drawn.as_str().contains("value=\"\"<x>"));
}

#[test]
fn src_accepts_internal_and_http_but_not_executable_schemes() {
    assert_eq!(src("/a.png").as_str(), "/a.png");
    assert_eq!(
        src("https://cdn.example/a.png").as_str(),
        "https://cdn.example/a.png"
    );
    assert!(html_auy::Src::parse("javascript:alert(1)").is_none());
    assert!(html_auy::Src::parse("data:image/png;base64,AAAA").is_none());
    assert!(html_auy::Src::parse("//cdn.example/a.png").is_none());
}
