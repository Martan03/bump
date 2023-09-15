use std::env;

use config::config::Config;
use gui::app::BumpApp;
use gui::gui::Gui;
use iced::window;
use iced::window::PlatformSpecific;
use iced::Application;
use iced::Settings;

mod config {
    pub mod config;
}
mod gui {
    pub mod app;
    pub mod elements;
    pub mod gui;
    pub mod svg_data;
    pub mod theme;
    pub mod widgets {
        pub mod list_view;
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

    BumpApp::run(make_settings())
}

/// Makes window settings, loads saved settings
fn make_settings() -> Settings<(Config, Gui)> {
    let config = Config::load();
    let gui = Gui::load(&config);

    let icon = window::icon::from_rgba(
        include_bytes!("../assets/raw_img/icon_64.data")
            .to_owned()
            .into(),
        64,
        64,
    );
    let id = "bump";

    Settings {
        window: window::Settings {
            icon: icon.ok(),
            size: gui.get_size(),
            position: gui.get_pos(),
            platform_specific: PlatformSpecific {
                application_id: id.to_owned(),
            },
            ..Default::default()
        },
        id: Some(id.to_owned()),
        exit_on_close_request: false,
        antialiasing: true,
        flags: (config, gui),
        ..Default::default()
    }
}
