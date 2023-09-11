use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox};

use crate::player::player::Player;

pub struct Counter {
    count: i32,
    player: Player
}

#[derive(Debug, Clone, Copy)]
pub enum PlayMessage {
    Play,
    Pause,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    Increment,
    Decrement,
    Play(PlayMessage)
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
            CounterMessage::Play(PlayMessage::Play) => _ = self.player.play(),
            CounterMessage::Play(PlayMessage::Pause) => _ = self.player.togglePlay(),
        };
    }

    fn view(&self) -> Element<CounterMessage> {
        column![
            button("Increment").on_press(CounterMessage::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(CounterMessage::Decrement),
            button("Play").on_press(CounterMessage::Play(PlayMessage::Play))
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
