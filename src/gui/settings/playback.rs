use iced::{
    widget::{column, container, row, scrollable, text, text_input},
    Renderer,
};
use iced_core::{Length, Padding};

use crate::{
    config::ConfMsg,
    gui::{
        app::{BumpApp, Msg},
        svg_data::TICK,
        theme::{Text, Theme},
        widgets::{hover_grad::HoverGrad, svg_button::SvgButton},
    },
};

use super::{elements::toggler, SettingsMsg};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn playback_settings(&self) -> Element {
        scrollable(
            column![
                toggler(
                    "Shuffle currently playing song".to_owned(),
                    self.config.get_shuffle_current(),
                    |val| Msg::Conf(ConfMsg::ShuffleCurrent(val))
                ),
                toggler(
                    "Auto play on startup".to_owned(),
                    self.config.get_autoplay(),
                    |val| Msg::Conf(ConfMsg::Autoplay(val))
                ),
                toggler(
                    "Gapless playback".to_owned(),
                    self.config.get_gapless(),
                    |val| Msg::Conf(ConfMsg::Gapless(val))
                ),
                column![
                    text("Fade play/pause:").style(Text::Normal),
                    HoverGrad::new(
                        row![
                            container(
                                SvgButton::new(TICK.into())
                                    .width(15)
                                    .height(15)
                                    .on_press(Msg::Settings(
                                        SettingsMsg::FadeSave
                                    )),
                            )
                            .height(30)
                            .padding(3)
                            .center_x()
                            .center_y(),
                            text_input("0.150", &self.settings.fade).on_input(
                                |val| Msg::Settings(SettingsMsg::Fade(val))
                            )
                        ]
                        .into()
                    )
                    .height(Length::Shrink),
                ]
                .spacing(3),
                column![
                    text("Volume step:").style(Text::Normal),
                    HoverGrad::new(
                        row![
                            container(
                                SvgButton::new(TICK.into())
                                    .width(15)
                                    .height(15)
                                    .on_press(Msg::Settings(
                                        SettingsMsg::VolJumpSave
                                    )),
                            )
                            .height(30)
                            .padding(3)
                            .center_x()
                            .center_y(),
                            text_input("0.1", &self.settings.vol_jmp)
                                .on_input(|val| Msg::Settings(
                                    SettingsMsg::VolJump(val)
                                ))
                        ]
                        .into()
                    )
                    .height(Length::Shrink),
                ]
                .spacing(3)
            ]
            .padding(Padding::from([5, 15])),
        )
        .into()
    }
}
