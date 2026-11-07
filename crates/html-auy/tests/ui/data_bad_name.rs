//! data bad name: o bloco não compila (mensagem em `data_bad_name.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div data_Foo="x" { } };
}
