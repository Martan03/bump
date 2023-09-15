use iced::{widget::{column, row, container, scrollable, button}, Renderer};
use iced_core::{Length, Alignment};

use super::{app::{BumpApp, Msg, PlayerMsg}, theme::{Theme, Text, Button}};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_playlist(&self) -> Element {
        column![
            row![
                self.menu(),
                column![
                    button("Shuffle")
                        .style(Button::Primary)
                        .on_press(Msg::Plr(PlayerMsg::Shuffle)),
                    container(self.playlist_songs()).width(Length::Fill),
                ]
                .width(Length::Fill),
                /*
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

    fn playlist_songs(&self) -> Element {
        let cur = self.player.get_current();

        scrollable(
            column(
                self.player.get_playlist()
                    .iter()
                    .map(|p| {
                        let c = p.to_owned();
                        let song = self.library.get_song(c);
                        let style = match cur {
                            Some(value) if value.to_owned() == c => Text::Prim,
                            _ => Text::Default,
                        };
                        self.list_item(&song, style, c)
                    })
                    .collect(),
            )
            .padding([0, 15, 0, 5]),
        )
        .into()
    }
}
