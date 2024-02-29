use xilem::view::{button, div, View};

mod color;
pub mod window;

pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Ui {}
    }
    pub fn run(ui: &mut Self) -> impl View<Self> {
        div(button("hello", |_| {}))
    }
}
