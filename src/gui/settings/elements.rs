use crate::gui::{
    app::Msg,
    svg_data::BIN,
    theme::{self, Text, Theme},
    widgets::{
        hover_grad::HoverGrad, svg_button::SvgButton,
        text_ellipsis::TextEllipsis, toggler::Toggler,
    },
};

use iced::{widget::row, Renderer};
use iced_core::Length;

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

/// Gets toggler component
pub fn toggler<'a, F>(text: String, val: bool, msg: F) -> Element<'a>
where
    F: Fn(bool) -> Msg + 'static,
{
    HoverGrad::new(
        Toggler::new(Some(text), val, move |val| msg(val))
            .width(Length::Shrink)
            .spacing(5)
            .into(),
    )
    .padding([4, 10, 4, 10])
    .width(Length::Shrink)
    .height(Length::Shrink)
    .into()
}

/// Gets removable item component
pub fn removable_item<'a>(content: String, msg: Msg) -> Element<'a> {
    HoverGrad::new(
        row![
            SvgButton::new(BIN.into())
                .width(20)
                .height(20)
                .style(theme::SvgButton::Remove)
                .on_press(msg),
            TextEllipsis::new(content).style(Text::Dark),
        ]
        .spacing(5)
        .into(),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([4, 10, 4, 10])
    .into()
}
