//! literal class: o bloco não compila (mensagem em `literal_class.stderr`).

use html::html;

fn main() {
    let _ = html! { div class="card" { } };
}
