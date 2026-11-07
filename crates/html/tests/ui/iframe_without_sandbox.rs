//! iframe without sandbox: o bloco não compila (mensagem em `iframe_without_sandbox.stderr`).

use html::html;

fn main() {
    let _ = html! { iframe src=(s) title="vídeo" { } };
}
