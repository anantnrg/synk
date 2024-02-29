use ui::Ui;
use xilem::{App, AppLauncher};

#[allow(dead_code)]
const APP_ID: &'static str = "synk";
// const JETBRAINS_MONO: &[u8] = include_bytes!("../resources/fonts/jetbrains_mono.ttf");

fn main() {
    let mut ui = Ui::new();
    let app = App::new(ui, Ui::run);
    AppLauncher::new(app).run();
}
