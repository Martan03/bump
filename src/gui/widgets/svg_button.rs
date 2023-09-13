use iced::Length;
use iced_core::event::Status;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::{self, Cursor};
use iced_core::renderer::Style;
use iced_core::svg::{self, Handle};
use iced_core::widget::{tree, Tree};
use iced_core::{
    touch, Clipboard, Color, Element, Event, Layout, Padding, Pixels,
    Rectangle, Shell, Widget,
};

pub struct SvgButton<Message>
where
    Message: Clone,
{
    width: Length,
    max_width: f32,
    height: Length,
    max_height: f32,
    padding: Padding,
    svg: Handle,
    on_press: Option<Message>,
}

impl<Message> SvgButton<Message>
where
    Message: Clone,
{
    pub fn new(svg: Handle) -> Self {
        Self {
            width: Length::Fill,
            max_width: f32::INFINITY,
            height: Length::Fill,
            max_height: f32::INFINITY,
            padding: Padding::ZERO,
            svg,
            on_press: None,
        }
    }

    /// Sets the width of the [`SvgButton`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the maximum width of the [`SvgButton`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the height of the [`SvgButton`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum height of the [`SvgButton`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`SvgButton`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for SvgButton<Message>
where
    Renderer: svg::Renderer,
    Message: Clone,
{
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> Node {
        let lim = limits
            .max_width(self.max_width)
            .width(self.width)
            .height(self.height);
        Node::new(lim.fill())
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = state.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        return Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Left,
            ))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.clone() {
                    let state = state.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }

                        return Status::Captured;
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = state.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        Status::Ignored
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
            x: bounds.x + self.padding.left,
            y: bounds.y + self.padding.right,
            width: bounds.width - self.padding.left - self.padding.right,
            height: bounds.height - self.padding.left - self.padding.right,
        };

        let color = if cursor.is_over(bounds) {
            Some(Color::new(0.1, 0.7, 1.0, 1.0))
        } else {
            Some(Color::WHITE)
        };
        renderer.draw(self.svg.clone(), color, svg_bounds);
    }
}

impl<'a, Message, Renderer> From<SvgButton<Message>>
    for Element<'a, Message, Renderer>
where
    Renderer: svg::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(button: SvgButton<Message>) -> Self {
        Self::new(button)
    }
}

/// The local state of a [`SvgButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}
