//! Aceite: uma página com o catálogo inteiro de campos gera IDs/ARIA coerentes e passa no audit.

#![expect(
    clippy::expect_used,
    reason = "teste de aceite: entradas fixas e válidas; o panic é diagnóstico"
)]

use html::{
    Action, AttrVal, Autocomplete, ChoiceOption, CsrfToken, Field, FieldName, FieldState, Form,
    InputType, LUrl, Lang, Method, Msg, Page, SelectOption, ShellData, Text, View, html,
};

#[derive(Clone, Copy)]
struct M;

impl Msg for M {
    fn as_str(self) -> &'static str {
        "Campo"
    }
}

fn lang() -> Lang {
    Lang::parse("pt-BR").expect("lang")
}

fn page() -> Page {
    let text = Field::text(
        FieldName::new("nome"),
        M,
        &FieldState::new(AttrVal::new("Ana")).required(true),
    );
    let email = Field::input(
        FieldName::new("email"),
        M,
        InputType::Email,
        &FieldState::new(AttrVal::new("ana@example.com"))
            .autocomplete(Autocomplete::parse("email").expect("token")),
    );
    let date = Field::input(
        FieldName::new("nascimento"),
        M,
        InputType::Date,
        &FieldState::new(AttrVal::new("1990-05-01")),
    );
    let notes = Field::textarea(
        FieldName::new("descricao"),
        M,
        &FieldState::new(AttrVal::new("Linha 1\nLinha 2")).help(Text::new("Detalhe o pedido")),
    );
    let options = [
        SelectOption::new(AttrVal::new("a"), Text::new("Assunto A")).selected(true),
        SelectOption::new(AttrVal::new("b"), Text::new("Assunto B")),
    ];
    let select = Field::select(
        FieldName::new("assunto"),
        M,
        &options,
        &FieldState::new(AttrVal::new("a")),
    )
    .expect("opções");
    let file = Field::file(
        FieldName::new("anexo"),
        M,
        Some(AttrVal::new("image/png, application/pdf")),
        true,
        &FieldState::new(AttrVal::new("ignorado")).required(true),
    );
    let checkbox = Field::checkbox(
        FieldName::new("aceite"),
        M,
        &FieldState::new(AttrVal::new("sim")).checked(true),
    );
    let choices = [
        ChoiceOption::new(AttrVal::new("email"), Text::new("E-mail")),
        ChoiceOption::new(AttrVal::new("telefone"), Text::new("Telefone")),
    ];
    let group = Field::radio_group(
        FieldName::new("contato"),
        M,
        &choices,
        &FieldState::new(AttrVal::new("")).required(true),
    )
    .expect("grupo");
    let checks = [
        ChoiceOption::new(AttrVal::new("news"), Text::new("Newsletter")).checked(true),
        ChoiceOption::new(AttrVal::new("offers"), Text::new("Ofertas")),
    ];
    let check_group = Field::checkbox_group(
        FieldName::new("preferencias"),
        M,
        &checks,
        &FieldState::new(AttrVal::new("")),
    )
    .expect("grupo");

    let action = Action::new(LUrl::new("/solicitacoes").expect("rota"));
    let token = CsrfToken::new("token-de-teste").expect("token");
    let form = Form::open(
        &action,
        Method::Post(token),
        html! {
            for field in [text, email, date, notes, select, file, checkbox, group, check_group] {
                (field)
            }
            button type="submit" { Text::new("Enviar") }
        },
    );

    let view = View::new(html! {
        section {
            h2 { Text::new("Solicitação") }
            (form)
        }
    });
    Page::new(
        &ShellData::new(Text::new("Solicitação"), Text::new("Solicitação"), lang()),
        view,
    )
}

#[test]
fn form_page_has_unique_ids_and_dangling_free_aria() {
    let drawn = page();
    let body = drawn.markup().as_str();
    let ids = body
        .match_indices("id=\"")
        .map(|(index, _)| {
            let rest = &body[index + 4..];
            rest.split('"').next().unwrap_or("")
        })
        .collect::<Vec<_>>();
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(ids.len(), unique.len(), "ids repetidos: {ids:?}");

    for (index, _) in body.match_indices("aria-describedby=\"") {
        let rest = &body[index + 18..];
        let value = rest.split('"').next().unwrap_or("");
        for id in value.split(' ') {
            assert!(
                body.contains(&format!("id=\"{id}\"")),
                "aria-describedby aponta para {id} inexistente"
            );
        }
    }
}

#[test]
fn form_page_never_emits_a_file_value() {
    let drawn = page();
    let body = drawn.markup().as_str();
    let file_start = body.find("type=\"file\"").expect("file");
    let file_end = file_start + body[file_start..].find('>').expect("fim") + 1;
    let file = &body[file_start..file_end];
    assert!(file.contains("accept=\"image/png, application/pdf\""));
    assert!(file.contains("multiple"));
    assert!(!file.contains("value="));
}

#[cfg(feature = "audit")]
#[test]
fn form_page_passes_the_document_audit() {
    assert_eq!(html::audit::check(page().markup().as_str()), Vec::new());
}

#[test]
fn validation_rerender_keeps_values_errors_and_alert_summary() {
    let state = FieldState::new(AttrVal::new("Ana"))
        .required(true)
        .error(Text::new("Informe um nome válido"));
    let field = Field::text(FieldName::new("nome"), M, &state);
    let action = Action::new(LUrl::new("/solicitacoes").expect("rota"));
    let token = CsrfToken::new("token-de-teste").expect("token");
    let form = Form::open(
        &action,
        Method::Post(token),
        html! {
            div role="alert" {
                a href=(html::Href::internal(LUrl::new("/solicitacoes#f-nome").expect("âncora"))) { (Text::new("Corrija o nome")) }
            }
            (field)
        },
    );
    let s = form.as_str();
    assert!(s.contains("value=\"Ana\""), "{s}");
    assert!(s.contains("aria-invalid=\"true\""), "{s}");
    assert!(s.contains("aria-describedby=\"f-nome-error\""), "{s}");
    assert!(s.contains("id=\"f-nome-error\""), "{s}");
    assert!(s.contains("role=\"alert\""), "{s}");
}
