mod app;
mod tabs;

use color_eyre::eyre;

use crate::gui::app::App;

pub fn execute() -> eyre::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native("Hyprr", options, Box::new(|cc| Box::new(App::new(cc))))
        .map_err(|err| eyre::eyre!("{err}"))
}
