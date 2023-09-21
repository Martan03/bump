use iced::{
    widget::{column, container, text},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, Msg},
    theme::{Text, Theme},
    widgets::list_view::WrapBox,
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    /// Displays main page
    pub fn view_library(&self) -> Element {
        column![
            container(text("Library").size(25).style(Text::Light),).padding(5),
            self.list_header(false),
            self.library_songs(),
        ]
        .width(Length::Fill)
        .spacing(1)
        .into()
    }

    pub fn library_songs(&self) -> Element {
        let songs = self.library.get_songs();
        let cur = self.player.get_current();

        WrapBox::with_children(
            songs
                .iter()
                .enumerate()
                .map(|(c, s)| {
                    let style = match cur {
                        Some(value) if value.to_owned() == c => Text::Prim,
                        _ => Text::Default,
                    };
                    self.list_item(s, style, c, None, true)
                })
                .collect(),
            self.gui.get_wb_state(0),
        )
        .item_height(45)
        .scrollbar_button_height(15)
        .scrollbar_width(15)
        .padding([0, 5, 0, 5])
        .into()
    }
}
