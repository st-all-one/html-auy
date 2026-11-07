//! aria not allowed here: o elemento não declara `aria-*` (mensagem em `aria_not_allowed_here.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { p aria_live="polite" { } };
}
