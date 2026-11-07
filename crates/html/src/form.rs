//! `<form>`: o único elemento que `html!` recusa, porque exige CSRF por tipo.

use crate::escape;
use crate::markup::Markup;
use crate::value::LUrl;
use alloc::string::String;

/// Ação do formulário (`action`): caminho interno já validado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action<'a>(LUrl<'a>);

impl<'a> Action<'a> {
    /// Envolve a URL de destino.
    #[must_use]
    pub const fn new(url: LUrl<'a>) -> Self {
        Self(url)
    }

    /// A URL emitida.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Método HTTP do formulário. O token mora **no método**: `Post`/`Put`/`Patch`/`Delete` não
/// existem sem `CsrfToken`, e `Get` não pede um (formulário de busca não inventa token).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method<'a> {
    /// `get`: tudo na query string, sem token.
    Get,
    /// `post`.
    Post(CsrfToken<'a>),
    /// `post` + `_method=put`.
    Put(CsrfToken<'a>),
    /// `post` + `_method=patch`.
    Patch(CsrfToken<'a>),
    /// `post` + `_method=delete`.
    Delete(CsrfToken<'a>),
}

impl<'a> Method<'a> {
    /// O valor de `method` no elemento.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post(_) | Self::Put(_) | Self::Patch(_) | Self::Delete(_) => "post",
        }
    }

    /// O valor de `_method`, quando o método não é `get` nem `post`.
    #[must_use]
    pub const fn hidden(&self) -> Option<&'static str> {
        match self {
            Self::Get | Self::Post(_) => None,
            Self::Put(_) => Some("put"),
            Self::Patch(_) => Some("patch"),
            Self::Delete(_) => Some("delete"),
        }
    }

    /// O token exigido pelo método (`None` em `get`).
    const fn token(self) -> Option<CsrfToken<'a>> {
        match self {
            Self::Get => None,
            Self::Post(token) | Self::Put(token) | Self::Patch(token) | Self::Delete(token) => {
                Some(token)
            }
        }
    }
}

/// Token CSRF já decidido pelo consumidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CsrfToken<'a>(&'a str);

impl<'a> CsrfToken<'a> {
    /// `None` para token vazio: formulário sem CSRF não compila.
    #[must_use]
    pub const fn new(token: &'a str) -> Option<Self> {
        if token.is_empty() {
            None
        } else {
            Some(Self(token))
        }
    }

    /// O valor do campo `_token`.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Chave de submissão idempotente: o servidor a valida e rejeita replay.
///
/// O crate só emite o campo `_submit`; a política de uso único é do handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubmitKey<'a>(&'a str);

impl<'a> SubmitKey<'a> {
    /// `None` para chave vazia.
    #[must_use]
    pub const fn new(key: &'a str) -> Option<Self> {
        if key.is_empty() {
            None
        } else {
            Some(Self(key))
        }
    }

    /// O valor do campo `_submit`.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// O `<form>`: `Form::open(ação, método, filhos)`.
///
/// Só existe por esta porta: o elemento não é do vocabulário de `html!`
/// (invariante `form_requires_csrf`), e o método carrega o token.
pub struct Form;

impl Form {
    /// Emite `<form>` com o token e o `_method` que o método exige.
    #[must_use]
    pub fn open(action: &Action<'_>, method: Method<'_>, children: Markup) -> Markup {
        Self::open_inner(action, method, None, children)
    }

    /// Como [`Form::open`], mas acrescenta o campo `_submit` para métodos não-GET.
    ///
    /// A chave deve ser única por submissão e validada no servidor; replay com o mesmo corpo
    /// retorna o resultado original (`303`), replay com corpo diferente retorna `409`.
    #[must_use]
    pub fn open_with_submit(
        action: &Action<'_>,
        method: Method<'_>,
        submit: &SubmitKey<'_>,
        children: Markup,
    ) -> Markup {
        Self::open_inner(action, method, Some(submit), children)
    }

    fn open_inner(
        action: &Action<'_>,
        method: Method<'_>,
        submit: Option<&SubmitKey<'_>>,
        children: Markup,
    ) -> Markup {
        let children = children.into_string();
        let mut out = String::with_capacity(64usize.saturating_add(children.len()));
        out.push_str("<form method=\"");
        out.push_str(method.as_str());
        out.push_str("\" action=\"");
        escape::attr(&mut out, action.as_str());
        out.push_str("\">");

        if let Some(token) = method.token() {
            out.push_str("<input type=\"hidden\" name=\"_token\" value=\"");
            escape::attr(&mut out, token.as_str());
            out.push_str("\">");
        }
        if let Some(hidden) = method.hidden() {
            out.push_str("<input type=\"hidden\" name=\"_method\" value=\"");
            out.push_str(hidden);
            out.push_str("\">");
        }
        if method.token().is_some()
            && let Some(submit) = submit
        {
            out.push_str("<input type=\"hidden\" name=\"_submit\" value=\"");
            escape::attr(&mut out, submit.as_str());
            out.push_str("\">");
        }
        out.push_str(&children);
        out.push_str("</form>");
        Markup(out)
    }
}
