use askama::Template;
use axum::{Router, response::Html, routing::get};

#[derive(Template)]
#[template(path = "index.html")]
struct PortalTemplate {
    title: &'static str,
    heading: &'static str,
}

async fn index() -> Html<String> {
    let page = PortalTemplate {
        title: "Portal Oficial Integrado - Município",
        heading: "Transparência, Cidadania e Gestão Democrática Municipal",
    }
    .render()
    .expect("template válido");
    Html(page)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("bind 127.0.0.1:3001");
    println!("askama demo: http://127.0.0.1:3001");
    axum::serve(listener, app).await.expect("server");
}

#[cfg(test)]
mod tests {
    use super::PortalTemplate;
    use askama::Template;

    #[test]
    fn page_has_the_same_document_shape() {
        let s = PortalTemplate {
            title: "Portal Oficial Integrado - Município",
            heading: "Transparência, Cidadania e Gestão Democrática Municipal",
        }
        .render()
        .expect("template válido");
        assert_eq!(s.matches("<h1").count(), 1);
        assert_eq!(s.matches("<div").count(), 204);
        assert_eq!(s.matches("<a ").count(), 56);
        assert_eq!(s.matches("<button").count(), 8);
        assert_eq!(s.matches("<img").count(), 2);
        assert_eq!(s.matches("<section").count(), 6);
        assert_eq!(s.matches("<article").count(), 5);
        assert!(!s.contains("href=\"/#"));
    }
}
