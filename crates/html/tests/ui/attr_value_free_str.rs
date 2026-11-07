use html::html;

fn main() {
    let free = "solto";
    let _ = html! { abbr title=(free) { } };
}
