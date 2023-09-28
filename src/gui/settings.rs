use iced::{
    widget::{button, column, text},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, ConfMsg, LibMsg, Msg},
    theme::{Button, Text, Theme},
    widgets::toggler::Toggler,
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_settings(&self) -> Element {
        column![
            text("Settings").size(25).style(Text::Normal),
            button("Update library")
                .on_press(Msg::Lib(LibMsg::LoadStart))
                .style(Button::Primary),
            Toggler::new(
                "Update library on start".to_owned(),
                self.config.get_start_load(),
                |val| { Msg::Conf(ConfMsg::StartLoad(val)) }
            )
            .spacing(3),
            Toggler::new(
                "Recursive search for songs".to_owned(),
                self.config.get_recursive_search(),
                |val| { Msg::Conf(ConfMsg::RecursiveSearch(val)) }
            )
            .spacing(3),
            Toggler::new(
                "Shuffle currently playing song".to_owned(),
                self.config.get_shuffle_current(),
                |val| { Msg::Conf(ConfMsg::ShuffleCurrent(val)) }
            )
            .spacing(3),
            Toggler::new(
                "Automatically start playing song on start".to_owned(),
                self.config.get_autoplay(),
                |val| { Msg::Conf(ConfMsg::Autoplay(val)) }
            )
            .spacing(3),
            Toggler::new(
                "Play songs without gap between them".to_owned(),
                self.config.get_gapless(),
                |val| { Msg::Conf(ConfMsg::Gapless(val)) }
            )
            .spacing(3),
        ]
        .width(Length::Fill)
        .spacing(3)
        .padding(3)
        .into()
    }
}
