use std::env;

use gui::app::BumpApp;
use iced::window;
use iced::Application;
use iced::Settings;

mod config {
    pub mod config;
}
mod gui {
    pub mod app;
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
    // on wayland, the app freezes when not drawn, this is temporary workaround
    // until it is fixed
    env::set_var("WINIT_UNIX_BACKEND", "x11");
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
            ..Default::default()
        },
        exit_on_close_request: false,
        ..Default::default()
    })
}
