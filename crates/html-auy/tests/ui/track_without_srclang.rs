//! track without srclang: o bloco não compila (mensagem em `track_without_srclang.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { video { track kind="captions" src=(s) srclang="pt" label="P"; track kind="subtitles" src=(s) label="L"; } };
}
