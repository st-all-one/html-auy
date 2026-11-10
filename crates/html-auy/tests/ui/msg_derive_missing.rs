//! Toda variante de `Msg` precisa de `#[msg("…")]`.

#[derive(Clone, Copy, html_auy::Msg)]
enum Bad {
    A,
    #[msg("ok")]
    B,
}

fn main() {}
