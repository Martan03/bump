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
    pub fn hotkeys_settings(&self) -> Element {
        scrollable(
            column![toggler(
                "Enable hotkeys".to_owned(),
                self.config.get_enable_hotkeys(),
                |val| Msg::Conf(ConfMsg::EnableHotkeys(val))
            ),]
            .padding(Padding::from([5, 15])),
        )
        .into()
    }
}
