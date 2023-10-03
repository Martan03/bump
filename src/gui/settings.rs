use std::path::PathBuf;

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
        hover_grad::HoverGrad, svg_button::SvgButton,
        text_ellipsis::TextEllipsis, toggler::Toggler,
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
        items.push(
            text_input("path", "")
                .on_submit(Msg::Conf(ConfMsg::AddPath(PathBuf::from(""))))
                .into(),
        );

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

/// Gets toggler widget
pub fn toggler<'a, F>(text: String, val: bool, msg: F) -> Element<'a>
where
    F: Fn(bool) -> Msg + 'static,
{
    HoverGrad::new(
        Toggler::new(Some(text), val, move |val| msg(val))
            .width(Length::Shrink)
            .spacing(5)
            .into(),
    )
    .padding([3, 10, 3, 10])
    .width(Length::Shrink)
    .height(Length::Shrink)
    .into()
}
