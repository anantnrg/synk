pub mod colors;

use colors::Colors;
use freya::prelude::*;

pub fn render_ui() -> Element {
    let colors = Colors::new();
    println!("{0}", colors.background);
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            background: "{colors.background}",
            main_align: "center",
            cross_align: "center",
            label { "hello" }
        }
    }
}
