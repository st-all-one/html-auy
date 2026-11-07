//! media xor src: o bloco não compila (mensagem em `media_xor_src.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { video src=(s) { media_source src=(s) type="video/mp4"; track kind="captions" src=(s) srclang="pt" label="P"; } };
}
