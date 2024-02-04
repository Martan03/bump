use iced::{
    widget::{button, column, scrollable, text},
    Renderer,
};
use iced_core::{Length, Padding};

use crate::{
    config::ConfMsg,
    gui::{
        app::{BumpApp, LibMsg, Msg},
        theme::{Button, Text, Theme},
        widgets::hover_grad::HoverGrad,
    },
};

use super::{
    elements::{removable_item, toggler},
    SettingsMsg,
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn library_settings(&self) -> Element {
        scrollable(
            column![
                column![
                    text("Songs loading:").height(22).style(Text::Normal),
                    button(
                        HoverGrad::new(
                            text("Update library").style(Text::Normal).into()
                        )
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(Padding::from([3, 5]))
                    )
                    .on_press(Msg::Lib(LibMsg::LoadStart)),
                    toggler(
                        "Update library on start".to_owned(),
                        self.config.get_start_load(),
                        |val| Msg::Conf(ConfMsg::StartLoad(val))
                    ),
                    toggler(
                        "Recursive search for songs".to_owned(),
                        self.config.get_recursive_search(),
                        |val| Msg::Conf(ConfMsg::RecursiveSearch(val))
                    ),
                ],
                self.get_paths_input(),
            ]
            .spacing(5)
            .padding(Padding::from([5, 15])),
        )
        .into()
    }

    fn get_paths_input(&self) -> Element {
        let mut items: Vec<Element> = Vec::new();

        items.push(
            text("Songs search paths:")
                .style(Text::Normal)
                .height(22)
                .into(),
        );
        for (i, path) in self.config.get_paths().iter().enumerate() {
            items.push(removable_item(
                path.to_string_lossy().to_string(),
                Msg::Conf(ConfMsg::RemPath(i)),
            ));
        }
        items.push(
            button("Add path")
                .on_press(Msg::Settings(SettingsMsg::PickSearchPath))
                .style(Button::Primary)
                .into(),
        );

        column(items).spacing(3).into()
    }
}
