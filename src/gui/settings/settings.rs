use std::path::PathBuf;

use iced::{
    widget::{button, column, text},
    Command, Renderer,
};
use iced_core::Length;

use crate::gui::{
    app::{BumpApp, ConfMsg, LibMsg, Msg},
    theme::{Button, Text, Theme},
    widgets::{
        hover_grad::HoverGrad,
        toggler::Toggler,
    },
};

use super::{elements::removable_item, SettingsMsg};

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
                |val| Msg::Conf(ConfMsg::StartLoad(val))
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
            toggler(
                "Enable hotkeys".to_owned(),
                self.config.get_enable_hotkeys(),
                |val| Msg::Conf(ConfMsg::EnableHotkeys(val))
            ),
            self.get_paths_input(),
        ]
        .width(Length::Fill)
        .spacing(2)
        .padding(3)
        .into()
    }

    /// Settings update function
    pub fn settings_update(&mut self, msg: SettingsMsg) -> Command<Msg> {
        match msg {
            SettingsMsg::PickSearchPath => {
                Command::perform(pick_folder(), |path| match path {
                    Some(path) => Msg::Conf(ConfMsg::AddPath(path)),
                    None => Msg::Tick,
                })
            }
        }
    }

    fn get_paths_input(&self) -> Element {
        let mut items: Vec<Element> = Vec::new();

        items.push(text("Songs search paths:").style(Text::Normal).into());
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

pub async fn pick_folder() -> Option<PathBuf> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a folder...")
        .pick_folder()
        .await;
    if let Some(handle) = handle {
        Some(handle.path().into())
    } else {
        None
    }
}
