use iced::{
    widget::{button, column, text},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, Msg},
    theme::{Button, Text, Theme},
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_settings(&self) -> Element {
        column![
            text("Settings").size(25).style(Text::Normal),
            button("Update library")
                .on_press(Msg::Update)
                .style(Button::Primary),
        ]
        .width(Length::Fill)
        .spacing(3)
        .padding(3)
        .into()
    }
}
