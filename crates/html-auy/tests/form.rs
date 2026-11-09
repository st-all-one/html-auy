//! `<form>`: método, token e `_method` (docs/design/08).

use html_auy::{Action, CsrfToken, Form, LUrl, Markup, SubmitKey, Text};

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: caminho fixo e válido"
)]
fn action() -> Action<'static> {
    Action::new(LUrl::new("/noticias").expect("caminho interno"))
}

fn children() -> Markup {
    Markup::from_render(&Text::new("campos"))
}

#[expect(
    clippy::expect_used,
    reason = "auxiliar de teste: token fixo e não vazio"
)]
#[expect(
    clippy::missing_const_for_fn,
    reason = "o `expect` do auxiliar de teste não é const"
)]
fn token() -> CsrfToken<'static> {
    CsrfToken::new("abc123").expect("token não vazio")
}

#[test]
fn get() {
    let drawn = Form::open(&action(), html_auy::Method::Get, children());
    assert_eq!(
        drawn.as_str(),
        "<form method=\"get\" action=\"/noticias\">campos</form>"
    );
}

#[test]
fn post_has_only_the_token() {
    let drawn = Form::open(&action(), html_auy::Method::Post(token()), children());
    assert_eq!(
        drawn.as_str(),
        "<form method=\"post\" action=\"/noticias\">\
         <input type=\"hidden\" name=\"_token\" value=\"abc123\">campos</form>"
    );
}

#[test]
fn put() {
    let action = Action::new(LUrl::new("/solicitacoes/nova").expect("caminho interno"));
    let token = CsrfToken::new("tok").expect("token");
    let drawn = Form::open(&action, html_auy::Method::Put(token), children());
    assert!(drawn.as_str().contains(
        "<form method=\"post\" action=\"/solicitacoes/nova\">\
         <input type=\"hidden\" name=\"_token\" value=\"tok\">\
         <input type=\"hidden\" name=\"_method\" value=\"put\">"
    ));
}

#[test]
fn put_patch_delete_add_method() {
    for (method, hidden) in [
        (html_auy::Method::Put(token()), "put"),
        (html_auy::Method::Patch(token()), "patch"),
        (html_auy::Method::Delete(token()), "delete"),
    ] {
        let drawn = Form::open(&action(), method, children());
        assert!(
            drawn
                .as_str()
                .contains(&format!("name=\"_method\" value=\"{hidden}\"")),
            "método oculto de {hidden}"
        );
    }
}

#[test]
fn token_value_is_escaped() {
    let raw = CsrfToken::new("a&b\"c").expect("token não vazio");
    let drawn = Form::open(&action(), html_auy::Method::Post(raw), children());
    assert!(drawn.as_str().contains("value=\"a&amp;b&quot;c\""));
}

#[test]
fn empty_token_is_rejected() {
    assert!(CsrfToken::new("").is_none());
}

#[test]
fn get_never_emits_a_token() {
    let drawn = Form::open(&action(), html_auy::Method::Get, children());
    assert!(!drawn.as_str().contains("_token"));
    assert!(!drawn.as_str().contains("_method"));
}

#[test]
fn submit_key_is_emitted_after_token_and_method() {
    let key = SubmitKey::new("once-1").expect("chave");
    let drawn = Form::open_with_submit(&action(), html_auy::Method::Put(token()), &key, children());
    let s = drawn.as_str();
    let token = s.find("name=\"_token\"").expect("token");
    let method = s.find("name=\"_method\"").expect("method");
    let submit = s.find("name=\"_submit\"").expect("submit");
    assert!(token < method && method < submit, "{s}");
    assert!(s.contains("name=\"_submit\" value=\"once-1\""));
}

#[test]
fn submit_key_is_not_emitted_for_get() {
    let key = SubmitKey::new("once-1").expect("chave");
    let drawn = Form::open_with_submit(&action(), html_auy::Method::Get, &key, children());
    assert!(!drawn.as_str().contains("_submit"));
}

#[test]
fn submit_key_is_escaped_and_empty_is_rejected() {
    let key = SubmitKey::new("a&b\"c").expect("chave");
    let drawn =
        Form::open_with_submit(&action(), html_auy::Method::Post(token()), &key, children());
    assert!(drawn.as_str().contains("value=\"a&amp;b&quot;c\""));
    assert!(SubmitKey::new("").is_none());
}
