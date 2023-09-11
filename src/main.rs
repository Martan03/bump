use gui::app::BumpApp;
use iced::Application;
use iced::Settings;

mod player {
    pub mod player;
}

mod gui {
    pub mod app;
}

fn main() -> Result<(), iced::Error> {
    BumpApp::run(Settings::default())
}
