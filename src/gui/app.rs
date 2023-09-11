use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox};

use crate::player::player::Player;

pub struct Counter {
    count: i32,
    player: Player
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    Increment,
    Decrement,
    Play(Option<bool>)
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Counter {
            count: 0,
            player: Player::new()
        }
    }

    fn title(&self) -> String {
        String::from("BUMP")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1,
            CounterMessage::Play(play) => {
                let playing = self.player.is_playing();
                _ = self.player.play(play.unwrap_or(!playing));
            }
        };
    }

    fn view(&self) -> Element<CounterMessage> {
        column![
            button("Increment").on_press(CounterMessage::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(CounterMessage::Decrement),
            button("Play").on_press(CounterMessage::Play(None))
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
