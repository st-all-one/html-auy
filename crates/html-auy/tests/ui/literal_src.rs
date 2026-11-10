//! literal src remoto não é aceito: URL externa exige `Src::parse`.

use html_auy::html;

fn main() {
    let _ = html! { img src="https://cdn.example/a.png" alt=(html_auy::Alt::decorative()) width=(html_auy::WidthPx::new(1)) height=(html_auy::HeightPx::new(1)); };
}
