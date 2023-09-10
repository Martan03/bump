use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

#[derive(Debug)]
pub struct Counter {
    count: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Counter { count: 0 }
    }

    fn title(&self) -> String {
        String::from("BUMP")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<CounterMessage> {
        column![
            button("Increment").on_press(CounterMessage::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(CounterMessage::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
