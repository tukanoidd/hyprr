mod app;
mod scrollable_list;

use color_eyre::eyre;
use iced::window::Position;
use iced::{window, Application, Settings};

use crate::gui::app::GuiApp;

pub fn execute() -> eyre::Result<()> {
    let settings = Settings {
        id: Some("tukanoidd.hyprr".to_string()),
        window: window::Settings {
            position: Position::Centered,
            size: (1080, 720),
            min_size: Some((1080, 720)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: false,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        flags: (),
        default_font: None,
        default_text_size: 20,
        text_multithreading: true,
        antialiasing: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    };

    GuiApp::run(settings)?;

    Ok(())
}
