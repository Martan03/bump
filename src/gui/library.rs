use iced::{
    widget::{column, container, scrollable, text},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, Msg},
    theme::{Text, Theme},
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    /// Displays main page
    pub fn view_library(&self) -> Element {
        column![
            text("Library").size(25).style(Text::Light),
            container(self.library_songs())
                .height(Length::Fill)
                .width(Length::Fill),
        ]
        .width(Length::Fill)
        .spacing(3)
        .padding(3)
        .into()
    }

    pub fn library_songs(&self) -> Element {
        let songs = self.library.get_songs();
        let cur = self.player.get_current();

        scrollable(
            column(
                songs
                    .iter()
                    .enumerate()
                    .map(|(c, s)| {
                        let style = match cur {
                            Some(value) if value.to_owned() == c => Text::Prim,
                            _ => Text::Default,
                        };
                        self.list_item(s, style, c, None)
                    })
                    .collect(),
            )
            .padding([0, 15, 0, 0]),
        )
        .into()
    }
}
