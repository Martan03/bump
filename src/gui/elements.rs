use iced::widget::{
    button, column, container, row, scrollable, text, Space,
};
use iced::Renderer;
use iced_core::Length;

use crate::library::song::Song;

use super::app::{BumpApp, Msg, PlayerMsg};
use super::theme::{Button, Container, Text, Theme};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    /// Gets songs list element
    pub fn songs_list(&self) -> Element {
        let songs = self.library.get_songs();
        let cur = self.player.get_current();
        let mut c = 0;

        scrollable(
            column(
                songs
                    .iter()
                    .map(|s| {
                        let style = match cur {
                            Some(value) if value.to_owned() == c => Text::Prim,
                            _ => Text::Default,
                        };
                        c += 1;
                        self.list_item(s, style, c)
                    })
                    .collect(),
            )
            .padding([0, 15, 0, 5]),
        )
        .into()
    }

    /// Gets button for list item data and add bottom "border"
    pub fn list_item(&self, s: &Song, style: Text, c: usize) -> Element {
        button(
            column![
                Space::new(Length::Shrink, Length::FillPortion(1)),
                self.list_item_data(s, style),
                Space::new(Length::Shrink, Length::FillPortion(1)),
                // Creates bottom border
                container("")
                    .width(Length::Fill)
                    .height(1)
                    .style(Container::Separate),
            ]
            .padding([0, 6, 0, 6]),
        )
        .height(45)
        .width(Length::Fill)
        .padding(0)
        .style(Button::Item)
        .on_press(Msg::Plr(PlayerMsg::PlaySong(c - 1)))
        .into()
    }

    /// Gets list item data
    fn list_item_data(&self, s: &Song, style: Text) -> Element {
        row![
            self.list_item_col(s.get_name(), style, s.get_artist(), 11),
            self.list_item_col(s.get_album(), style, &s.get_year_str(), 11),
            self.list_item_col(&s.get_length_str(), style, s.get_genre(), 1),
        ]
        .height(Length::Shrink)
        .spacing(3)
        .into()
    }

    /// Gets column of the list item
    fn list_item_col(
        &self,
        top: &str,
        style: Text,
        bottom: &str,
        p: u16,
    ) -> Element {
        // Gets top text
        fn top_text(data: &str, style: Text) -> Element<'static> {
            text(data).size(15).style(style).into()
        }
        // Gets bottom text
        fn bottom_text(data: &str) -> Element<'static> {
            text(data).size(11).style(Text::Darker).into()
        }

        column![top_text(top, style), bottom_text(bottom),]
            .height(Length::Shrink)
            .width(Length::FillPortion(p))
            .into()
    }
}
