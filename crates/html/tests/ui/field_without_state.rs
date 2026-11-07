use html::{Field, FieldName, Msg};

#[derive(Clone, Copy)]
struct M;

impl Msg for M {
    fn as_str(self) -> &'static str {
        "Rótulo"
    }
}

fn main() {
    let _ = Field::text(FieldName::new("nome"), M);
}
