//! Campos: rótulo, `id`, `aria-describedby` e `aria-invalid` saem juntos (docs/05, docs/10).

use html::{AttrVal, Field, FieldName, FieldState, InputType, Msg, SelectOption, Text};

#[derive(Clone, Copy)]
struct M;

impl Msg for M {
    fn as_str(self) -> &'static str {
        "Nome completo"
    }
}

#[derive(Clone, Copy)]
struct Ajuda;

impl Msg for Ajuda {
    fn as_str(self) -> &'static str {
        "Como consta no documento"
    }
}

#[derive(Clone, Copy)]
struct Erro;

impl Msg for Erro {
    fn as_str(self) -> &'static str {
        "Informe o nome"
    }
}

#[test]
fn error() {
    let state = FieldState::new(AttrVal::new("ana"))
        .required(true)
        .help(Text::msg(Ajuda))
        .error(Text::msg(Erro));
    let drawn = Field::text(FieldName::new("sections[0][campo]"), M, &state);
    insta::assert_snapshot!("field_text_error", drawn.as_str());
}

#[test]
fn clean() {
    let state = FieldState::new(AttrVal::new(""));
    let drawn = Field::text(FieldName::new("nome"), M, &state);
    insta::assert_snapshot!("field_text_plain", drawn.as_str());
    assert!(!drawn.as_str().contains("aria-invalid"));
    assert!(!drawn.as_str().contains("-error"));
}

#[test]
fn textarea_escapes_value() {
    let state = FieldState::new(AttrVal::new("<script>alert(1)</script>"));
    let drawn = Field::textarea(FieldName::new("mensagem"), M, &state);
    insta::assert_snapshot!("field_textarea", drawn.as_str());
    assert!(!drawn.as_str().contains("<script>"));
}

#[test]
fn error_ids_are_the_contract() {
    let state = FieldState::new(AttrVal::new(""))
        .help(Text::msg(Ajuda))
        .error(Text::msg(Erro));
    let drawn = Field::text(FieldName::new("email"), M, &state);
    assert!(
        drawn
            .as_str()
            .contains("aria-describedby=\"f-email-help f-email-error\"")
    );
    assert!(drawn.as_str().contains("aria-invalid=\"true\""));
    // Os ids prometidos pelo `aria-describedby` existem no markup (senão o leitor de tela não lê).
    assert!(drawn.as_str().contains("id=\"f-email-help\""));
    assert!(drawn.as_str().contains("id=\"f-email-error\""));
    assert!(drawn.as_str().contains(">Informe o nome</span>"));
}

#[test]
fn indexed_name_becomes_slug_id() {
    let state = FieldState::new(AttrVal::new("x"));
    let drawn = Field::text(FieldName::new("sections[0][campo]"), M, &state);
    assert!(drawn.as_str().contains("id=\"f-sections-0-campo\""));
    assert!(drawn.as_str().contains("name=\"sections[0][campo]\""));
}

#[test]
fn input_types_emit_their_type() {
    let state = FieldState::new(AttrVal::new("x"));
    for (kind, expected) in [
        (InputType::Text, "text"),
        (InputType::Email, "email"),
        (InputType::Tel, "tel"),
        (InputType::Url, "url"),
        (InputType::Search, "search"),
        (InputType::Password, "password"),
    ] {
        let drawn = Field::input(FieldName::new("email"), M, kind, &state);
        assert!(
            drawn
                .as_str()
                .contains(&format!("<input type=\"{expected}\" id=\"f-email\"")),
            "{expected}: {}",
            drawn.as_str()
        );
        assert!(
            drawn
                .as_str()
                .contains("<label for=\"f-email\">Nome completo</label>")
        );
    }
    assert_eq!(
        Field::text(FieldName::new("email"), M, &state).as_str(),
        Field::input(FieldName::new("email"), M, InputType::Text, &state).as_str()
    );
}

#[test]
fn select_emits_options_as_data() {
    let state = FieldState::new(AttrVal::new("sp"))
        .required(true)
        .error(Text::msg(Erro))
        .help(Text::msg(Ajuda));
    let options = [
        SelectOption::new(AttrVal::new("rj"), Text::new("Rio de Janeiro")),
        SelectOption::new(AttrVal::new("sp"), Text::new("São Paulo")).selected(true),
        SelectOption::new(AttrVal::new("XX"), Text::new("Indisponível")).disabled(true),
    ];
    let drawn = Field::select(FieldName::new("uf"), M, &options, &state).expect("opções");
    let s = drawn.as_str();
    assert!(s.contains("<select id=\"f-uf\" name=\"uf\" required"));
    assert!(s.contains("aria-describedby=\"f-uf-help f-uf-error\""));
    assert!(s.contains("aria-invalid=\"true\""));
    assert!(s.contains("<option value=\"rj\">Rio de Janeiro</option>"));
    assert!(s.contains("<option value=\"sp\" selected>São Paulo</option>"));
    assert!(s.contains("<option value=\"XX\" disabled>Indisponível</option>"));
    assert!(s.contains("id=\"f-uf-help\""));
    assert!(s.contains("id=\"f-uf-error\""));
}

#[test]
fn select_escapes_value_and_label() {
    let state = FieldState::new(AttrVal::new(""));
    let options = [SelectOption::new(
        AttrVal::new("\"<x>"),
        Text::new("<script>alert(1)</script>"),
    )];
    let drawn = Field::select(FieldName::new("x"), M, &options, &state).expect("opções");
    assert!(!drawn.as_str().contains("<script>"));
    assert!(!drawn.as_str().contains("value=\"\"<x>"));
}

#[test]
fn select_without_a_selectable_option_is_none() {
    let state = FieldState::new(AttrVal::new(""));
    assert!(Field::select(FieldName::new("uf"), M, &[], &state).is_none());

    let only_disabled = [SelectOption::new(AttrVal::new("rj"), Text::new("RJ")).disabled(true)];
    assert!(Field::select(FieldName::new("uf"), M, &only_disabled, &state).is_none());

    let one_enabled = [
        SelectOption::new(AttrVal::new("rj"), Text::new("RJ")).disabled(true),
        SelectOption::new(AttrVal::new("sp"), Text::new("SP")),
    ];
    assert!(Field::select(FieldName::new("uf"), M, &one_enabled, &state).is_some());
}
