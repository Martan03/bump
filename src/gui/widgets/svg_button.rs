use iced::{Color, Element, Length, Rectangle};
use iced_core::layout::{self, Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::svg::{Handle, self};
use iced_core::widget::Tree;
use iced_core::{Layout, Widget};

use crate::gui::app::BumpMessage;

pub struct SvgButton {
    width: Length,
    height: Length,
    padding: u16,
    svg: Handle,
    on_press: Option<BumpMessage>,
}

impl SvgButton {
    pub fn new(svg: Handle) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            padding: 0,
            svg,
            on_press: None,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn on_press(mut self, message: BumpMessage) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for SvgButton
where
    Renderer: svg::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> layout::Node {
        let lim = limits.width(self.width).height(self.height);
        Node::new(lim.fill())
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        
        let svg_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        };

        let color = if cursor.is_over(bounds) {
            Some(Color::new(0.1, 0.7, 1.0, 1.0))
        } else {
            Some(Color::WHITE)
        };
        renderer.draw(self.svg.clone(), color, svg_bounds);
    }
}

impl<'a, Message, Renderer> From<SvgButton> for Element<'a, Message, Renderer>
where
    Renderer: svg::Renderer,
{
    fn from(svg_button: SvgButton) -> Self {
        Self::new(svg_button)
    }
}
