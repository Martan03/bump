use std::path::PathBuf;

use iced::{
    widget::{button, column, text},
    Command, Renderer,
};
use iced_core::Length;

use crate::gui::{
    app::{BumpApp, ConfMsg, LibMsg, Msg},
    theme::{Button, Text, Theme},
};

use super::{elements::{removable_item, toggler}, SettingsMsg};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_settings(&self) -> Element {
        column![
            text("Settings").size(25).style(Text::Normal),
            button("Update library")
                .on_press(Msg::Lib(LibMsg::LoadStart))
                .style(Button::Primary),
            text("Songs loading:").height(22).style(Text::Normal),
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
            self.get_paths_input(),
            text("Playback:").height(22).style(Text::Normal),
            toggler(
                "Shuffle currently playing song".to_owned(),
                self.config.get_shuffle_current(),
                |val| Msg::Conf(ConfMsg::ShuffleCurrent(val))
            ),
            toggler(
                "Automatically start playing song on start".to_owned(),
                self.config.get_autoplay(),
                |val| Msg::Conf(ConfMsg::Autoplay(val))
            ),
            toggler(
                "Play songs without gap between them".to_owned(),
                self.config.get_gapless(),
                |val| Msg::Conf(ConfMsg::Gapless(val))
            ),
            text("Hotkeys:").height(22).style(Text::Normal),
            toggler(
                "Enable hotkeys".to_owned(),
                self.config.get_enable_hotkeys(),
                |val| Msg::Conf(ConfMsg::EnableHotkeys(val))
            ),
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
                Command::perform(pick_folder(), |paths| match paths {
                    Some(paths) => Msg::Conf(ConfMsg::AddPath(paths)),
                    None => Msg::Tick,
                })
            }
        }
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

pub async fn pick_folder() -> Option<Vec<PathBuf>> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose folders...")
        .pick_folders()
        .await;
    if let Some(handle) = handle {
        let mut paths: Vec<PathBuf> = Vec::new();
        for path in handle {
            paths.push(path.path().into())
        }
        Some(paths)
    } else {
        None
    }
}
