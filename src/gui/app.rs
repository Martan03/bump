use iced::widget::{button, column, scrollable, text};
use iced::{executor, Alignment, Application, Command, Element, Renderer, Theme};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;

pub struct BumpApp {
    player: Player,
    library: Library,
    config: Config,
}

#[derive(Debug, Clone, Copy)]
pub enum BumpMessage {
    Update,
    Increment,
    Decrement,
    Play(Option<bool>),
    PlaySong(usize),
}

impl Application for BumpApp {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = BumpMessage;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            BumpApp {
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
            BumpMessage::Update => _ = self.library.find(&mut self.config),
            BumpMessage::Increment => {
                _ = self.player.next(&self.library);
            }
            BumpMessage::Decrement => {
                _ = self.player.prev(&self.library);
            }
            BumpMessage::Play(play) => {
                let playing = self.player.is_playing();
                _ = self.player.play(play.unwrap_or(!playing));
            }
            BumpMessage::PlaySong(id) => {
                _ = self.player.play_at(&self.library, id as i128, true);
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let active = self.player.get_current();
        column![
            button("Update library").on_press(BumpMessage::Update),
            button("Increment").on_press(BumpMessage::Increment),
            text(active),
            button("Decrement").on_press(BumpMessage::Decrement),
            button("Play").on_press(BumpMessage::Play(None)),
            self.vector_display(),
        ]
        .spacing(3)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl BumpApp {
    fn vector_display(&self) -> Element<BumpMessage> {
        let songs = self.library.get_songs();
        let mut c = 0;

        scrollable(column(
            songs
                .iter()
                .map(|s| {
                    c += 1;
                    button(text(format!("{}", s.get_name())))
                        .width(iced::Length::Fill)
                        .on_press(BumpMessage::PlaySong(c - 1))
                        .into()
                })
                .collect(),
        ).spacing(3))
        .into()
    }
}
