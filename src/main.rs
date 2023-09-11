use gui::app::BumpApp;
use iced::Application;
use iced::Settings;

mod gui {
    pub mod app;
}

mod library {
    pub mod library;
    pub mod song;
}

mod player {
    pub mod player;
}

fn main() -> Result<(), iced::Error> {
    BumpApp::run(Settings::default())
}
