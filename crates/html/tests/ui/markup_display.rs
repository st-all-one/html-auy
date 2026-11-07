use html::{Markup, Text};

fn main() {
    let markup = Markup::from_render(&Text::new("x"));
    println!("{markup}");
}
