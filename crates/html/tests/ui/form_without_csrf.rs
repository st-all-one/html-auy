use html::{Action, Form, LUrl, Markup, Method, Text};

fn main() {
    let action = Action::new(LUrl::new("/x").expect("caminho"));
    let children = Markup::from_render(&Text::new("campos"));
    let _ = Form::open(&action, Method::Post, children);
}
