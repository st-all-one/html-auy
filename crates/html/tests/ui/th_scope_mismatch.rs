//! th scope mismatch: o bloco não compila (mensagem em `th_scope_mismatch.stderr`).

use html::html;

fn main() {
    let _ = html! { table { caption { m } thead { tr { th scope="row" { m } } } } };
}
