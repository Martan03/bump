use crate::gui::{
    app::Msg,
    svg_data::BIN,
    theme::{Text, Theme},
    widgets::{
        hover_grad::HoverGrad, svg_button::SvgButton,
        text_ellipsis::TextEllipsis,
    },
};

use iced::{widget::row, Renderer};
use iced_core::Length;

type Element<'a> = iced::Element<'a, Msg, Renderer<Theme>>;

pub fn removable_item<'a>(content: String, msg: Msg) -> Element<'a> {
    HoverGrad::new(
        row![
            SvgButton::new(BIN.into())
                .width(20)
                .height(20)
                .on_press(msg),
            TextEllipsis::new(content).style(Text::Normal),
        ].spacing(3)
        .into(),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([3, 10, 3, 10])
    .into()
}
