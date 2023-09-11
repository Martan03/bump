use iced::widget::{button, column, text, Text, Column};
use iced::{executor, Alignment, Application, Command, Element, Renderer};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;

pub struct BumpApp {
    count: usize,
    player: Player,
    library: Library,
    config: Config,
}

#[derive(Debug, Clone, Copy)]
pub enum BumpMessage {
    Increment,
    Decrement,
    Play(Option<bool>),
    PlaySong(Option<usize>)
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
            BumpMessage::Increment => {
                self.count += 1;
                let songs = self.library.get_songs();
                self.count %= songs.len();
                let playing = self.player.is_playing();
                _ = self.player.load(
                    songs[self.count].get_path(),
                    playing
                );
            },
            BumpMessage::Decrement => {
                let songs = self.library.get_songs();
                if self.count > 0 {
                    self.count -= 1;
                } else {
                    self.count = songs.len() - 1;
                }
                let playing = self.player.is_playing();
                _ = self.player.load(
                    songs[self.count].get_path(),
                    playing
                );
            },
            BumpMessage::Play(play) => {
                let playing = self.player.is_playing();
                _ = self.player.play(play.unwrap_or(!playing));
            },
            BumpMessage::PlaySong(id) => {
                let songs = self.library.get_songs();
                let playing = self.player.is_playing();
                _ = self.player.load(
                    songs[id.unwrap_or(0)].get_path(),
                    playing
                );
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![
            button("Increment").on_press(BumpMessage::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(BumpMessage::Decrement),
            button("Play").on_press(BumpMessage::Play(None)),
            self.vector_display(),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

impl BumpApp {
    fn vector_display(&self) -> Element<BumpMessage> {
        let songs = self.library.get_songs();
        let song_elements: Vec<Element<_>> = songs.iter().map(|song| {
            Text::new(song.get_name()).into()
        }).collect();
    
        // Combine the elements into a single element using the + operator
        let combined_elements: Element<BumpMessage> = song_elements
            .into_iter()
            .fold(Column::new().spacing(20), |column, element| {
                column.push(element)
            })
            .into();
    
        combined_elements
    }
}
