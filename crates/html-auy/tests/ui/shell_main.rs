//! shell main: o bloco não compila (mensagem em `shell_main.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { main { m } };
}
