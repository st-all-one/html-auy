//! required caption: o bloco não compila (mensagem em `required_caption.stderr`).

use html::html;

fn main() {
    let _ = html! { table { thead { tr { th scope="col" { m } } } } };
}
