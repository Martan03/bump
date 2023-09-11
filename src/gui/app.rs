use iced::widget::{button, column, text};
use iced::{executor, Alignment, Application, Command, Element, Renderer};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;

pub struct BumpApp {
    count: i32,
    player: Player,
    library: Library,
    config: Config,
}

#[derive(Debug, Clone, Copy)]
pub enum BumpMessage {
    Increment,
    Decrement,
    Play(Option<bool>),
}

impl Application for BumpApp {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = iced::Theme;
    type Message = BumpMessage;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            BumpApp {
                count: 0,
                player: Player::new(),
                library: Library::new(),
                config: Config::load(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("BUMP")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            BumpMessage::Increment => self.count += 1,
            BumpMessage::Decrement => self.count -= 1,
            BumpMessage::Play(play) => {
                self.library.find(&mut self.config);
                let playing = self.player.is_playing();
                _ = self.player.play(play.unwrap_or(!playing));
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![
            button("Increment").on_press(BumpMessage::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(BumpMessage::Decrement),
            button("Play").on_press(BumpMessage::Play(None))
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
