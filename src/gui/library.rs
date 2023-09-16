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
            text("Library").size(20).style(Text::Normal),
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
        let mut c = 0;

        scrollable(
            column(
                songs
                    .iter()
                    .map(|s| {
                        let style = match cur {
                            Some(value) if value.to_owned() == c => Text::Prim,
                            _ => Text::Default,
                        };
                        c += 1;
                        self.list_item(s, style, c - 1, false)
                    })
                    .collect(),
            )
            .padding([0, 15, 0, 5]),
        )
        .into()
    }
}
