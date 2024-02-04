use iced::{
    widget::{column, container, row, scrollable, text_input},
    Renderer,
};
use iced_core::{Length, Padding};

use crate::{
    config::ConfMsg,
    gui::{
        app::{BumpApp, Msg},
        svg_data::PLUS,
        theme::Theme,
        widgets::{hover_grad::HoverGrad, svg_button::SvgButton},
    },
};

use super::{
    elements::{removable_item, toggler},
    SettingsMsg,
};

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

        col = col.push(
            HoverGrad::new(
                row![
                    container(
                        SvgButton::new(PLUS.into())
                            .width(15)
                            .height(15)
                            .on_press(Msg::Settings(SettingsMsg::HotkeySave)),
                    )
                    .height(30)
                    .padding(3)
                    .center_x()
                    .center_y(),
                    text_input("hotkey: action", &self.settings.hotkey)
                        .on_input(|val| Msg::Settings(SettingsMsg::Hotkey(
                            val
                        )))
                ]
                .into(),
            )
            .height(Length::Shrink),
        );

        scrollable(col.padding(Padding::from([5, 15]))).into()
    }
}
