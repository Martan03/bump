use gui::app::Counter;
use iced::Sandbox;
use iced::Settings;

mod player {
    pub mod player;
}

mod gui {
    pub mod app;
}

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}