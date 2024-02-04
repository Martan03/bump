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

use super::elements::{removable_item, toggler};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn hotkeys_settings(&self) -> Element {
        let mut col = column![toggler(
            "Enable hotkeys".to_owned(),
            self.config.get_enable_hotkeys(),
            |val| Msg::Conf(ConfMsg::EnableHotkeys(val))
        ),];

        for (key, val) in self.config.get_hotkeys().iter() {
            col = col.push(removable_item(format!("{key}: {val}"), Msg::Tick));
        }

        scrollable(col.padding(Padding::from([5, 15]))).into()
    }
}
