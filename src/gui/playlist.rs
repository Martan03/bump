use iced::{
    widget::{button, column, row, text, Space},
    Renderer,
};
use iced_core::Length;

use super::{
    app::{BumpApp, Msg, PlayerMsg},
    theme::{Button, Text, Theme},
    widgets::list_view::WrapBox,
};

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

impl BumpApp {
    pub fn view_playlist(&self) -> Element {
        column![
            row![
                text("Playlist").size(25).style(Text::Light),
                Space::new(Length::Fill, Length::Shrink),
                button("Shuffle")
                    .style(Button::Primary)
                    .on_press(Msg::Plr(PlayerMsg::Shuffle)),
            ]
            .padding(5),
            self.list_header(true),
            self.playlist_songs(),
        ]
        .width(Length::Fill)
        .spacing(1)
        .into()
    }

    fn playlist_songs(&self) -> Element {
        let cur = self.player.get_current();

        WrapBox::with_children(
            self.player
                .get_playlist()
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    let c = p.to_owned();
                    let s = self.library.get_song(c);
                    if s.get_deleted() {
                        None
                    } else {
                        let style = match cur {
                            Some(value) if value.to_owned() == c => Text::Prim,
                            _ => Text::Default,
                        };
                        Some(self.list_item(&s, style, c, Some(i + 1), false))
                    }
                })
                .collect(),
            self.gui.get_wb_state(1),
        )
        .item_height(45)
        .scrollbar_button_height(15)
        .scrollbar_width(15)
        .padding([0, 5, 0, 5])
        .into()
    }
}
