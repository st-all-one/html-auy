//! pattern inline flag: flag inline/inline `(?i)` é recusada
//! (mensagem em `pattern_inline_flag.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { input type="text" name=(n) pattern="(?i)[a-z]+" aria_label=(l); };
}
