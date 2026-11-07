//! required caption: o bloco não compila (mensagem em `required_caption.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { table { thead { tr { th scope="col" { m } } } } };
}
