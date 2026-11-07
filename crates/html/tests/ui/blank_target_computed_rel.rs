//! blank target computed rel: `rel` computado com `target="_blank"` não compila
//! (mensagem em `blank_target_computed_rel.stderr`).

use html::html;

fn main() {
    let _ = html! { a href=(h) target="_blank" rel=(r) { m } };
}
