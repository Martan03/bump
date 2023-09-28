use iced::{
    widget::{button, column, text, toggler},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, ConfMsg, LibMsg, Msg},
    theme::{Button, Text, Theme},
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_settings(&self) -> Element {
        column![
            text("Settings").size(25).style(Text::Normal),
            button("Update library")
                .on_press(Msg::Lib(LibMsg::LoadStart))
                .style(Button::Primary),
            toggler(
                "Update library on start".to_owned(),
                self.config.get_start_load(),
                |val| { Msg::Conf(ConfMsg::StartLoad(val)) }
            ),
            toggler(
                "Recursive search for songs".to_owned(),
                self.config.get_recursive_search(),
                |val| { Msg::Conf(ConfMsg::RecursiveSearch(val)) }
            ),
            toggler(
                "Shuffle currently playing song".to_owned(),
                self.config.get_shuffle_current(),
                |val| { Msg::Conf(ConfMsg::ShuffleCurrent(val)) }
            ),
            toggler(
                "Automatically start playing song on start".to_owned(),
                self.config.get_autoplay(),
                |val| { Msg::Conf(ConfMsg::Autoplay(val)) }
            ),
            toggler(
                "Play songs without gap between them".to_owned(),
                self.config.get_gapless(),
                |val| { Msg::Conf(ConfMsg::Gapless(val)) }
            ),
        ]
        .width(Length::Fill)
        .spacing(3)
        .padding(3)
        .into()
    }
}
