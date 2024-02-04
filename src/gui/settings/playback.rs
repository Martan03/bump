use iced::{
    widget::{column, scrollable},
    Renderer,
};
use iced_core::Padding;

use crate::{
    config::ConfMsg,
    gui::{
        app::{BumpApp, Msg},
        theme::Theme,
    },
};

use super::elements::toggler;

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
                    "Automatically start playing song on start".to_owned(),
                    self.config.get_autoplay(),
                    |val| Msg::Conf(ConfMsg::Autoplay(val))
                ),
                toggler(
                    "Play songs without gap between them".to_owned(),
                    self.config.get_gapless(),
                    |val| Msg::Conf(ConfMsg::Gapless(val))
                ),
            ]
            .padding(Padding::from([5, 15])),
        )
        .into()
    }
}
