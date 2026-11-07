//! video without captions: o bloco não compila (mensagem em `video_without_captions.stderr`).

use html::html;

fn main() {
    let _ = html! { video { track kind="subtitles" src=(s) srclang="pt" label="P"; } };
}
