use iced::widget::{
    button, column, container, row, scrollable, slider, text, Space,
};
use iced::Renderer;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Alignment, Length};

use crate::library::song::Song;

use super::app::{BumpApp, Msg, PlayerMsg, PageMsg};
use super::svg_data::{pp_icon, vol_icon, NEXT, PREV};
use super::theme::{Button, Container, Text, Theme};
use super::widgets::svg_button::SvgButton;

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn menu(&self) -> Element {
        container(
            column![
                button("Library").on_press(Msg::Page(PageMsg::Library)),
                button("Playlist").on_press(Msg::Page(PageMsg::Playlist)),
            ]
        )
        .width(175)
        .height(Length::Fill)
        .style(Container::Separate)
        .into()
    }

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
                        self.list_item(s, style, c - 1)
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
        .on_press(Msg::Plr(PlayerMsg::PlaySong(c)))
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

    /// Gets player bar
    pub fn player_bar(&self) -> Element {
        let (time, len) = self.player.get_timestamp();

        container(column![
            slider(0.0..=len.as_secs_f32(), time.as_secs_f32(), |v| {
                Msg::Plr(PlayerMsg::SeekTo(v))
            })
            .height(4)
            .step(0.01),
            row![
                container(self.title_bar(),).width(Length::FillPortion(1)),
                self.play_menu(),
                container(self.volume_menu(),).width(Length::FillPortion(1)),
            ]
            .height(Length::Fill)
            .padding(5)
            .align_items(Alignment::Center)
        ])
        .padding([1, 0, 0, 0])
        .align_y(Vertical::Center)
        .height(60)
        .style(Container::Dark)
        .into()
    }

    /// Gets title bar
    fn title_bar(&self) -> Element {
        let song = self.player.get_current_song(&self.library);
        column![
            text(song.get_name()).size(16).style(Text::Light),
            text(song.get_artist()).size(14).style(Text::Dark),
        ]
        .into()
    }

    /// Gets play menu with buttons to play, play next,...
    fn play_menu(&self) -> Element {
        row![
            SvgButton::new(PREV.into())
                .width(16)
                .height(16)
                .on_press(Msg::Plr(PlayerMsg::Prev)),
            SvgButton::new(pp_icon(self.player.is_playing()))
                .width(30)
                .height(30)
                .on_press(Msg::Plr(PlayerMsg::Play(None))),
            SvgButton::new(NEXT.into())
                .width(16)
                .height(16)
                .on_press(Msg::Plr(PlayerMsg::Next)),
        ]
        .align_items(Alignment::Center)
        .spacing(20)
        .into()
    }

    /// Gets volume menu
    fn volume_menu(&self) -> Element {
        container(
            row![
                SvgButton::new(vol_icon(
                    self.player.get_volume(),
                    self.player.get_mute()
                ))
                .width(20)
                .height(20)
                .on_press(Msg::Plr(PlayerMsg::Mute(None))),
                text(format!("{:.0}", self.player.get_volume() * 100.0))
                    .width(28)
                    .style(Text::Normal),
                slider(0.0..=1., self.player.get_volume(), |v| {
                    Msg::Plr(PlayerMsg::Volume(v))
                })
                .step(0.01)
                .width(100),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Right)
        .into()
    }
}
