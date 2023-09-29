use std::time::Duration;

use iced::widget::{
    button, column, container, row, slider, svg, text, Rule, Space,
};
use iced::Renderer;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Alignment, Length};

use crate::library::song::Song;

use super::app::{BumpApp, Msg, Page, PlayerMsg};
use super::svg_data::{pp_icon, vol_icon, ICON, NEXT, PREV};
use super::theme::{
    self, Button, Container, SvgButton as SvgTheme, Text, Theme,
};
use super::widgets::hover_grad::HoverGrad;
use super::widgets::svg_button::SvgButton;
use super::widgets::text_ellipsis::TextEllipsis;

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    /// Gets app menu
    pub fn menu(&self) -> Element {
        column![
            container(svg(ICON).width(50).height(50),)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            Space::new(Length::Shrink, 5),
            self.menu_button("Library", Page::Library),
            self.menu_button("Playlist", Page::Playlist),
            Space::new(Length::Shrink, Length::Fill),
            self.menu_button("Settings", Page::Settings),
        ]
        .width(175)
        .height(Length::Fill)
        .padding(10)
        .into()
    }

    /// Create menu button
    fn menu_button<'a>(&self, data: &'a str, page: Page) -> Element<'a> {
        HoverGrad::new(
            button(data)
                .width(Length::Fill)
                .style(Button::Menu(self.page == page))
                .on_press(Msg::Page(page))
                .into()
        )
        .height(Length::Shrink)
        .into()
    }

    /// Create list header
    pub fn list_header(&self, numbered: bool) -> Element {
        fn header_item<'a>(data: &'a str, fill: u16) -> Element<'a> {
            TextEllipsis::new(data)
                .width(Length::FillPortion(fill))
                .style(Text::Darker)
                .size(15)
                .into()
        }
        let mut items: Vec<Element> = Vec::new();
        if numbered {
            items.push(header_item("#", 1));
        }
        items.extend([
            header_item("Title / Artist", 10),
            header_item("Album / Year", 9),
            header_item("Length / Genre", 1),
        ]);
        column![
            row(items)
                .height(20)
                .padding([0, 25, 0, 10])
                .spacing(3)
                .align_items(Alignment::Center),
            Rule::horizontal(2)
        ]
        .into()
    }

    /// Gets button for list item data and add bottom "border"
    pub fn list_item(
        &self,
        s: &Song,
        style: Text,
        c: usize,
        num: Option<usize>,
        new: bool,
    ) -> Element {
        HoverGrad::new(
            button(
                column![
                    Space::new(Length::Shrink, Length::FillPortion(1)),
                    self.list_item_data(s, style, num),
                    Space::new(Length::Shrink, Length::FillPortion(1)),
                    // Creates bottom border
                    Rule::horizontal(1).style(theme::Rule::Separate(1)),
                ]
                .padding([0, 6, 0, 6]),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .style(Button::Item)
            .on_press(Msg::Plr(PlayerMsg::PlaySong(c, new)))
            .into(),
        )
        .height(45)
        .into()
    }

    /// Gets list item data
    fn list_item_data(
        &self,
        s: &Song,
        style: Text,
        num: Option<usize>,
    ) -> Element {
        let mut items: Vec<Element> = Vec::new();
        if let Some(num) = num {
            items.push(
                TextEllipsis::new(num.to_string())
                    .width(Length::FillPortion(1))
                    .style(Text::Darker)
                    .into(),
            );
        }
        items.extend([
            self.list_item_col(
                s.get_name().to_owned(),
                style,
                s.get_artist().to_owned(),
                10,
            ),
            self.list_item_col(
                s.get_album().to_owned(),
                style,
                s.get_year_str(),
                9,
            ),
            self.list_item_col(
                s.get_length_str(),
                style,
                s.get_genre().to_owned(),
                1,
            ),
        ]);
        row(items).spacing(3).align_items(Alignment::Center).into()
    }

    /// Gets column of the list item
    fn list_item_col(
        &self,
        top: String,
        style: Text,
        bottom: String,
        p: u16,
    ) -> Element {
        // Gets top text
        fn top_text<'a>(data: String, style: Text) -> Element<'a> {
            TextEllipsis::new(data).size(15).style(style).into()
        }
        // Gets bottom text
        fn bottom_text<'a>(data: String) -> Element<'a> {
            TextEllipsis::new(data).size(11).style(Text::Darker).into()
        }

        column![top_text(top, style), bottom_text(bottom)]
            .width(Length::FillPortion(p))
            .into()
    }

    /// Gets player bar
    pub fn player_bar(&self) -> Element {
        let (time, len) = self.player.get_timestamp();

        container(column![
            slider(0.0..=len.as_secs_f32(), time.as_secs_f32(), |v| {
                Msg::Plr(PlayerMsg::SeekTo(Duration::from_secs_f32(v)))
            })
            .height(4)
            .step(0.01),
            row![
                container(self.title_bar()).width(Length::FillPortion(1)),
                self.play_menu(),
                container(self.volume_menu()).width(Length::FillPortion(1)),
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
            TextEllipsis::new(song.get_name().to_owned())
                .size(16)
                .style(Text::Light)
                .ellipsis("..."),
            TextEllipsis::new(song.get_artist().to_owned())
                .size(14)
                .style(Text::Dark)
                .ellipsis("..."),
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
                .padding(8)
                .style(SvgTheme::Circle(30.))
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
