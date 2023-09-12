use gui::app::BumpApp;
use iced::Application;
use iced::Settings;

mod config {
    pub mod config;
}
mod gui {
    pub mod app;
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
    //env::set_var("WINIT_UNIX_BACKEND", "x11");
    BumpApp::run(Settings::default())
}
