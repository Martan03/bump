use std::path::PathBuf;

use iced::{
    widget::{button, column, row, text},
    Command, Renderer,
};
use iced_core::{Length, Padding};
use serde_derive::{Deserialize, Serialize};

use crate::{
    config::ConfMsg,
    gui::{
        app::{BumpApp, Msg},
        theme::{Button, Theme},
        widgets::hover_grad::HoverGrad,
    },
};

use super::SettingsMsg;

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettingsPage {
    Library,
    Playback,
    Hotkeys,
}

pub struct Settings {
    page: SettingsPage,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            page: SettingsPage::Library,
        }
    }
}

impl BumpApp {
    pub fn view_settings(&self) -> Element {
        column![
            self.settings_menu(),
            match self.settings.page {
                SettingsPage::Library => self.library_settings(),
                SettingsPage::Playback => self.playback_settings(),
                SettingsPage::Hotkeys => self.hotkeys_settings(),
            }
        ]
        .width(Length::Fill)
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
            SettingsMsg::Page(page) => {
                self.settings.page = page;
                Command::none()
            }
        }
    }

    /// Displays settings menu
    fn settings_menu(&self) -> Element {
        row![
            button(
                HoverGrad::new(text("Library").into())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::from([3, 5]))
            )
            .style(Button::Menu(self.settings.page == SettingsPage::Library))
            .on_press(Msg::Settings(SettingsMsg::Page(SettingsPage::Library))),
            button(
                HoverGrad::new(text("Playback").into())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::from([3, 5]))
            )
            .style(Button::Menu(self.settings.page == SettingsPage::Playback))
            .on_press(Msg::Settings(SettingsMsg::Page(
                SettingsPage::Playback
            ))),
            button(
                HoverGrad::new(text("Hotkeys").into())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::from([3, 5]))
            )
            .style(Button::Menu(self.settings.page == SettingsPage::Hotkeys))
            .on_press(Msg::Settings(SettingsMsg::Page(SettingsPage::Hotkeys))),
        ]
        .spacing(5)
        .into()
    }
}

pub async fn pick_folder() -> Option<Vec<PathBuf>> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose folders...")
        .pick_folders()
        .await;
    if let Some(handle) = handle {
        let paths: Vec<PathBuf> =
            handle.iter().map(|path| path.path().to_owned()).collect();
        Some(paths)
    } else {
        None
    }
}
