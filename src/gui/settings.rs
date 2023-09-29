use iced::{
    widget::{button, column, row, text, text_input},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, ConfMsg, LibMsg, Msg},
    svg_data::BIN,
    theme::{Button, Text, Theme},
    widgets::{
        svg_button::SvgButton, text_ellipsis::TextEllipsis, toggler::Toggler,
    },
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
            self.get_paths_input(),
        ]
        .width(Length::Fill)
        .spacing(3)
        .padding(3)
        .into()
    }

    fn get_paths_input(&self) -> Element {
        let mut items: Vec<Element> = Vec::new();

        items.push(text("Songs search paths:").style(Text::Normal).into());
        for (i, path) in self.config.get_paths().iter().enumerate() {
            items.push(
                self.get_remove_input(path.to_string_lossy().to_string(), i),
            );
        }
        items.push(text_input("path", "").into());

        column(items).spacing(3).into()
    }

    fn get_remove_input(&self, text: String, id: usize) -> Element {
        row![
            SvgButton::new(BIN.into())
                .width(20)
                .height(20)
                .on_press(Msg::Conf(ConfMsg::RemPath(id))),
            TextEllipsis::new(text).style(Text::Normal),
        ]
        .spacing(3)
        .into()
    }
}
