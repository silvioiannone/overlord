extern crate rustier;

use rustier::components::{
    AppFrame, Panel
};
use rustier::KeyCode;
use rustier::Rustier;

/// Entry point.
fn main() {
    let mut tui = Rustier::init();

    tui.event_handler.register_key(KeyCode::Esc, Box::new(move |_| {
        Rustier::quit();
        std::process::exit(0);
    }));

    let info_panel = Panel::default();

    let app_frame = AppFrame::default()
        .title("Overlord".to_string())
        .insert(Box::new(info_panel));

    tui.interface.add(Box::new(app_frame));

    tui.draw();
}
