//! caption outside table: partes de tabela não são conteúdo de fluxo
//! (mensagem em `caption_outside_table.stderr`).

use html_auy::html;

fn main() {
    let _ = html! { div { caption { m } } };
}
