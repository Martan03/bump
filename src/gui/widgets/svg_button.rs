use iced::mouse;
use iced::{Color, Element, Length, Rectangle, Size};
use iced_core::layout::{self, Limits};
use iced_core::{renderer, widget, Layout, Widget};

pub struct SvgButton {
    width: f32,
    height: f32,
    path: String,
}

impl SvgButton {
    pub fn new(width: f32, height: f32, path: String) -> Self {
        Self {
            width,
            height,
            path,
        }
    }
}

pub fn svg_button(width: f32, height: f32, path: String) -> SvgButton {
    SvgButton::new(width, height, path)
}

impl<Message, Renderer> Widget<Message, Renderer> for SvgButton
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, _limits: &Limits) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: self.width.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Renderer> From<SvgButton> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(svg_button: SvgButton) -> Self {
        Self::new(svg_button)
    }
}
