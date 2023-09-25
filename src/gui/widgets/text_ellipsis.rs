use std::borrow::Cow;

use iced_core::{
    alignment::{Horizontal, Vertical},
    layout,
    text::{self, Shaping},
    Color, Element, Length, Padding, Text, Widget,
};

pub struct TextEllipsis<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    content: Cow<'a, str>,
    width: Length,
    height: Length,
    padding: Padding,
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    size: Option<f32>,
    font: Option<Renderer::Font>,
    shaping: Shaping,
    ellipsis: Cow<'a, str>,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Renderer> TextEllipsis<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a [`TextEllipsis`] with given content
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::new(0.),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Top,
            size: None,
            font: None,
            shaping: Shaping::Basic,
            ellipsis: "".into(),
            style: Default::default(),
        }
    }

    /// Sets the width of the [`TextEllipsis`]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`TextEllipsis`]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`TextEllipsis`]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the size of the [`TextEllipsis`]
    pub fn size(mut self, size: impl Into<f32>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the font of the [`TextEllipsis`]
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the ellipsis of the [`TextEllipsis`]
    pub fn ellipsis(mut self, ellipsis: impl Into<Cow<'a, str>>) -> Self {
        self.ellipsis = ellipsis.into();
        self
    }

    /// Sets the style of the [`TextEllipsis`]
    pub fn style(
        mut self,
        style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for TextEllipsis<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let bounds = renderer.measure(
            &self.content,
            size,
            size.into(),
            self.font.unwrap_or_else(|| renderer.default_font()),
            limits.max(),
            self.shaping,
        );

        let size = limits.resolve(bounds);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        _state: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as iced_core::Renderer>::Theme,
        _style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        _cursor: iced_core::mouse::Cursor,
        _viewport: &iced_core::Rectangle,
    ) {
        let bounds = layout.bounds();

        renderer.fill_text(Text {
            content: &self.content,
            size: self.size.unwrap_or_else(|| renderer.default_size()),
            bounds,
            line_height: Default::default(),
            color: Color::from_rgb(1., 1., 1.),
            font: self.font.unwrap_or_else(|| renderer.default_font()),
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
            shaping: self.shaping,
        });
    }
}

impl<'a, Message, Renderer> From<TextEllipsis<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(
        text: TextEllipsis<'a, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(text)
    }
}

pub trait StyleSheet {
    type Style: Default;

    fn foreground(&self, style: &Self::Style) -> Option<Color> {
        _ = style;
        None
    }
}
