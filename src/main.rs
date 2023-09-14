use std::env;

use config::config::Config;
use gui::app::BumpApp;
use gui::gui::Gui;
use iced::Settings;
use iced::window;
use iced::Application;

mod config {
    pub mod config;
}
mod gui {
    pub mod app;
    pub mod gui;
    pub mod theme;
    pub mod widgets {
        pub mod svg_button;
    }
}
mod library {
    pub mod library;
    pub mod song;
}
mod player {
    pub mod player;
    pub mod sinker;
}

fn main() -> Result<(), iced::Error> {
    // on wayland, the app freezes when minimized, this is temporary workaround
    // until it is fixed
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    let config = Config::load();
    let gui = Gui::load(&config);
    BumpApp::run(Settings {
        window: window::Settings {
            icon: window::icon::from_rgba(
                include_bytes!("../assets/raw_img/icon_64.data")
                    .to_owned()
                    .into(),
                64,
                64,
            )
            .ok(),
            size: gui.get_size(),
            position: gui.get_pos(),
            ..Default::default()
        },
        exit_on_close_request: false,
        flags: (config, gui),
        ..Default::default()
    })
}
