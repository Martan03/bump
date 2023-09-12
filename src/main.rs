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
    BumpApp::run(Settings::default())
}
