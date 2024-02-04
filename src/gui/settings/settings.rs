use std::{path::PathBuf, time::Duration};

use eyre::{Report, Result};
use iced::{
    widget::{button, column, row, text},
    Command, Renderer,
};
use iced_core::{Length, Padding};
use serde_derive::{Deserialize, Serialize};

use crate::{
    config::{ConfMsg, Config},
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
    pub fade: String,
    pub vol_jmp: String,
}

impl Settings {
    /// Creates new [`Settings`] struct
    pub fn new(config: &Config) -> Self {
        let fade_secs = config.get_fade().as_secs();
        let fade_millis = config.get_fade().subsec_millis();
        let fade = format!(
            "{:02}:{:02}.{:02}",
            fade_secs / 60,
            fade_secs % 60,
            fade_millis,
        );

        let vol_jmp = format!("{}", config.get_volume_step());

        Self {
            fade,
            vol_jmp,
            ..Default::default()
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            page: SettingsPage::Library,
            fade: "00:00.150".to_owned(),
            vol_jmp: "0.1".to_owned(),
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
            SettingsMsg::Fade(val) => {
                self.settings.fade = val;
                Command::none()
            }
            SettingsMsg::FadeSave => {
                if let Ok(val) = self.convert_to_duration(&self.settings.fade)
                {
                    self.config.set_fade(val);
                    self.player.fade(self.config.get_fade());
                }
                Command::none()
            }
            SettingsMsg::VolJump(val) => {
                self.settings.vol_jmp = val;
                Command::none()
            }
            SettingsMsg::VolJumpSave => {
                if let Ok(val) = self.settings.vol_jmp.parse::<f32>() {
                    self.config.set_volume_step(val);
                    self.player.volume_step(self.config.get_volume_step());
                }
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

    fn convert_to_duration(&self, dur: &str) -> Result<Duration> {
        if let Some(minute_sep) = dur.find(':') {
            if let Some(second_sep) = dur[minute_sep + 1..].find('.') {
                let minutes: u64 = dur[..minute_sep].parse()?;
                let seconds: u64 = dur
                    [minute_sep + 1..minute_sep + second_sep + 1]
                    .parse()?;
                let milliseconds: u64 =
                    dur[minute_sep + second_sep + 2..].parse()?;
                let nanoseconds: u32 =
                    format!("{:09}", milliseconds * 1_000_000).parse()?;

                return Ok(Duration::new(minutes * 60 + seconds, nanoseconds));
            }
        }

        Err(Report::msg("Invalid format"))
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
