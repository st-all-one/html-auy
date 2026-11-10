//! `(a)(b)` é diagnosticado como concatenação, não como chamada.

use html_auy::html;

fn main() {
    let _ = html! { div { (a) (b) } };
}
