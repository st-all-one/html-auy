//! pattern flags: o bloco não compila (mensagem em `pattern_flags.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { label { input type="text" name=(n) pattern="/[a-z]+/i"; } };
}
