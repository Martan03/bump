use std::borrow::Cow;

use iced_core::{
    alignment::{Horizontal, Vertical},
    layout,
    text::{self, Shaping},
    Color, Element, Length, Padding, Pixels, Size, Text, Widget,
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
            ellipsis: "...".into(),
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

    /// Sets the horizontal alignment of the [`TextEllipsis`]
    pub fn horizontal_alignment(mut self, alignment: Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the [`TextEllipsis`]
    pub fn vertical_alignment(mut self, alignment: Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the size of the [`TextEllipsis`]
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    /// Sets the font of the [`TextEllipsis`]
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the shaping of the [`TextEllipsis`]
    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
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
        let limits = limits
            .width(self.width)
            .height(self.height)
            .pad(self.padding);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let width = renderer.measure_width(
            &self.content,
            size,
            self.font.unwrap_or_else(|| renderer.default_font()),
            self.shaping,
        ) + 0.1;

        let lim = limits.min_width(width).min_height(size * 1.3);
        let w = match self.width {
            Length::Fill | Length::FillPortion(_) => lim.max().width,
            Length::Shrink => lim.min().width,
            Length::Fixed(n) => n,
        };

        let h = match self.height {
            Length::Fill | Length::FillPortion(_) => lim.max().height,
            Length::Shrink => lim.min().height,
            Length::Fixed(n) => n,
        };

        layout::Node::new(Size::new(w, h))
    }

    fn draw(
        &self,
        _state: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_core::Renderer>::Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        _cursor: iced_core::mouse::Cursor,
        _viewport: &iced_core::Rectangle,
    ) {
        let mut bounds = layout.bounds();
        bounds.x += self.padding.left;
        bounds.y += self.padding.top;

        bounds.x = match self.horizontal_alignment {
            Horizontal::Left => bounds.x,
            Horizontal::Center => bounds.center_x(),
            Horizontal::Right => bounds.width + bounds.x,
        };

        bounds.y = match self.vertical_alignment {
            Vertical::Top => bounds.y,
            Vertical::Center => bounds.center_y(),
            Vertical::Bottom => bounds.height + bounds.y,
        };

        let size = self.size.unwrap_or_else(|| renderer.default_size());
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        let mut text_width =
            renderer.measure_width(&self.content, size, font, self.shaping);

        if text_width <= bounds.width {
            renderer.fill_text(Text {
                content: &self.content,
                size,
                bounds,
                line_height: Default::default(),
                color: theme
                    .foreground(&self.style)
                    .unwrap_or(style.text_color),
                font,
                horizontal_alignment: self.horizontal_alignment,
                vertical_alignment: self.vertical_alignment,
                shaping: self.shaping,
            });
            return;
        }

        let ellipsis_width =
            renderer.measure_width(&self.ellipsis, size, font, self.shaping);
        let width = bounds.width - ellipsis_width;

        let mut chars_fit = self.content.chars().count();
        while text_width > width && chars_fit > 0 {
            chars_fit -= 1;
            text_width = renderer.measure_width(
                &self.content[..chars_fit],
                size,
                font,
                self.shaping,
            );
        }
        let mut content = self.content[..chars_fit].to_owned();
        content += &self.ellipsis;

        renderer.fill_text(Text {
            content: &content,
            size,
            bounds,
            line_height: Default::default(),
            color: theme.foreground(&self.style).unwrap_or(style.text_color),
            font,
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
