//! input list aponta para datalist ausente: o bloco não compila.

use html_auy::html;

fn main() {
    let _ = html! {
        label for="cidade" { m }
        input type="text" name="cidade" id="cidade" list="ausente";
    };
}
