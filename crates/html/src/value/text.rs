//! Texto, rótulo, identificador e nome de campo.

use crate::markup::Render;
use crate::msg::Msg;
use alloc::borrow::Cow;
use alloc::string::String;

/// Valor de atributo genérico vindo do consumidor (escapado na escrita).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttrVal<'a>(&'a str);

impl<'a> AttrVal<'a> {
    /// Envolve um texto que será escapado ao virar atributo.
    #[must_use]
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }

    /// O texto cru.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Rótulo de controle: visível ou só para leitor de tela.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label<'a> {
    text: &'a str,
    sr_only: bool,
}

impl<'a> Label<'a> {
    /// Rótulo visível.
    #[must_use]
    pub fn text<M: Msg>(message: M) -> Label<'static> {
        Label {
            text: message.as_str(),
            sr_only: false,
        }
    }

    /// Rótulo só para tecnologia assistiva: renderiza `<span class="sr-only">`.
    ///
    /// A classe `sr-only` é vocabulário **do crate**: o CSS do consumidor precisa defini-la
    /// (`position:absolute; width:1px; height:1px; overflow:hidden; clip-path:inset(50%)`).
    #[must_use]
    pub fn sr_only<M: Msg>(message: M) -> Label<'static> {
        Label {
            text: message.as_str(),
            sr_only: true,
        }
    }

    /// O texto do rótulo (sem a marcação).
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.text
    }
}

impl Render for Label<'_> {
    fn render_to(&self, out: &mut String) {
        if self.sr_only {
            out.push_str(r#"<span class="sr-only">"#);
            crate::escape::text(out, self.text);
            out.push_str("</span>");
        } else {
            crate::escape::text(out, self.text);
        }
    }
}

/// Identificador de elemento: normalizado, nunca derivado de dado do usuário.
///
/// A normalização garante o alfabeto do HTML (`[A-Za-z0-9_-]`): qualquer outro caractere vira
/// `-`, as pontas são aparadas e o vazio vira `id`. Como a normalização pode mudar o texto, o
/// valor é guardado em [`Cow`]: já válido = emprestado, corrigido = próprio.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<'a>(Cow<'a, str>);

impl<'a> Id<'a> {
    /// Normaliza `value` para um id válido (nunca falha).
    #[must_use]
    pub fn new(value: &'a str) -> Self {
        Self(normalize(value, is_id_char, "id"))
    }

    /// `Some` só quando `value` já é um id válido (nada a normalizar).
    #[must_use]
    pub fn try_new(value: &'a str) -> Option<Self> {
        let valid = !value.is_empty()
            && value.chars().all(is_id_char)
            && !value.starts_with('-')
            && !value.ends_with('-');
        if valid {
            Some(Self(Cow::Borrowed(value)))
        } else {
            None
        }
    }

    /// O id normalizado.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Nome de campo de formulário (`name`), também base do `id` do controle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldName<'a>(Cow<'a, str>);

impl<'a> FieldName<'a> {
    /// Normaliza `value` para um nome de campo válido (nunca falha).
    #[must_use]
    pub fn new(value: &'a str) -> Self {
        Self(normalize(value, is_field_char, "field"))
    }

    /// `Some` só quando `value` já é um nome de campo válido.
    #[must_use]
    pub fn try_new(value: &'a str) -> Option<Self> {
        let valid =
            !value.is_empty() && value.chars().all(is_field_char) && !value.starts_with('-');
        if valid {
            Some(Self(Cow::Borrowed(value)))
        } else {
            None
        }
    }

    /// O nome como vai no atributo `name`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Deriva o `id` do controle: prefixo `f-`, `[`/`]`/`.`/espaço viram `-`, sem hífen duplo.
    ///
    /// `sections[0][campo]` → `f-sections-0-campo`.
    pub fn id_into(&self, out: &mut String) {
        out.push_str("f-");
        push_slug(out, &self.0);
    }
}

const fn is_id_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

const fn is_field_char(ch: char) -> bool {
    is_id_char(ch) || ch == '[' || ch == ']' || ch == '.'
}

fn normalize<'a>(
    value: &'a str,
    allowed: fn(char) -> bool,
    fallback: &'static str,
) -> Cow<'a, str> {
    let already_valid = !value.is_empty()
        && value.chars().all(allowed)
        && !value.starts_with('-')
        && !value.ends_with('-');
    if already_valid {
        return Cow::Borrowed(value);
    }
    let mut owned = String::with_capacity(value.len());
    for ch in value.chars() {
        owned.push(if allowed(ch) { ch } else { '-' });
    }
    let trimmed = owned.trim_matches('-');
    if trimmed.is_empty() {
        Cow::Owned(String::from(fallback))
    } else {
        Cow::Owned(String::from(trimmed))
    }
}

/// Anexa `value` como slug: o que não é alfanumérico/\vira um único `-`, sem pontas.
fn push_slug(out: &mut String, value: &str) {
    let mut pending_dash = false;
    for ch in value.chars() {
        if !is_id_char(ch) {
            pending_dash = true;
            continue;
        }
        if pending_dash {
            push_separator(out);
        }
        pending_dash = false;
        out.push(ch);
    }
}

fn push_separator(out: &mut String) {
    if !out.ends_with('-') {
        out.push('-');
    }
}
