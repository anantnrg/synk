use xilem::view::{button, div, View};

mod color;
mod window;

pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Ui {}
    }
    pub fn run(&mut self) -> impl View<Self> {
        div((button("hello", || {})))
    }
}
