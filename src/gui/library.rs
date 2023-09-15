use iced::{widget::{column, row, container}, Renderer};
use iced_core::{Length, Alignment};

use super::{app::{BumpApp, Msg}, theme::Theme};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    /// Displays main page
    pub fn view_main_page(&self) -> Element {
        column![
            row![
                self.menu(),
                container(self.songs_list()).width(Length::Fill),
                /*
                button("Shuffle")
                    .style(Button::Primary)
                    .on_press(Msg::Plr(PlayerMsg::Shuffle)),
                button("Update library")
                    .style(Button::Primary)
                    .on_press(Msg::Update),
                */
            ]
            .height(Length::Fill)
            .spacing(3),
            self.player_bar(),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}
