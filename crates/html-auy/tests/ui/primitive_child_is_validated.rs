//! primitive child is validated: filhos de primitivo passam pela mesma validação
//! (mensagem em `primitive_child_is_validated.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { Comp() { script { m } } };
}
