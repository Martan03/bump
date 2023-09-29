use iced_core::{
    gradient::Linear,
    layout::{Limits, Node},
    mouse::Cursor,
    renderer::{Quad, Style},
    widget::Tree,
    Background, Color, Degrees, Element, Gradient, Layout, Length, Padding,
    Rectangle, Size, Widget,
};

pub struct HoverGrad<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    width: Length,
    height: Length,
    padding: Padding,
    content: Element<'a, Message, Renderer>,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> HoverGrad<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`HoverGrad`]
    pub fn new(content: Element<'a, Message, Renderer>) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            padding: Padding::from(0),
            content,
            style: Default::default(),
        }
    }

    /// Sets the width of the [`HoverGrad`]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`HoverGrad`]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`HoverGrad`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the style of the [`HoverGrad`]
    pub fn style(
        mut self,
        style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for HoverGrad<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .width(self.width)
            .height(self.height)
            .pad(self.padding);

        let child = self.content.as_widget().layout(renderer, &limits);

        let min = limits.min();
        let limits = limits
            .min_width(min.width + self.padding.horizontal())
            .min_height(min.height + self.padding.vertical());

        let w = match self.width {
            Length::Fill | Length::FillPortion(_) => limits.max().width,
            Length::Shrink => limits.min().width,
            Length::Fixed(n) => n,
        };
        let h = match self.height {
            Length::Fill | Length::FillPortion(_) => limits.max().height,
            Length::Shrink => limits.min().height,
            Length::Fixed(n) => n,
        };

        Node::with_children(Size::new(w, h), vec![child])
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut state.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            )
        })
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced_core::Event,
        layout: Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> iced_core::event::Status {
        self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_core::Renderer>::Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );

        let bounds = layout.bounds();

        let grad_style = if cursor.is_over(bounds) {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        let grad_style = if let Some(grad_style) = grad_style {
            grad_style
        } else {
            return;
        };

        let pos = if let Some(p) = cursor.position() {
            p
        } else {
            return;
        };

        // in pixels
        let mut center = pos.x - bounds.x;
        let left = center - grad_style.fade_len;
        let right = center + grad_style.fade_len;

        let (mut left, l_mul) = if left > 0. {
            (left, 0.)
        } else {
            (0., left.abs() / grad_style.fade_len)
        };

        let (mut right, r_mul) = if right < bounds.width {
            (right, 0.)
        } else {
            (bounds.width, (right - bounds.width) / grad_style.fade_len)
        };

        center /= bounds.width;
        left /= bounds.width;
        right /= bounds.width;

        let mut grad = Linear::new(Degrees(180.));

        let m = grad_style.mouse_color;
        let f = grad_style.fade_color;
        grad = grad.add_stop(
            left,
            Color::from_rgba(
                m.r * l_mul + f.r * (1. - l_mul),
                m.g * l_mul + f.g * (1. - l_mul),
                m.b * l_mul + f.b * (1. - l_mul),
                m.a * l_mul + f.a * (1. - l_mul),
            ),
        );

        grad = grad.add_stop(center, m);

        grad = grad.add_stop(
            right,
            Color::from_rgba(
                m.r * r_mul + f.r * (1. - r_mul),
                m.g * r_mul + f.g * (1. - r_mul),
                m.b * r_mul + f.b * (1. - r_mul),
                m.a * r_mul + f.a * (1. - r_mul),
            ),
        );

        let quad = Quad {
            bounds,
            border_radius: grad_style.border_radius.into(),
            border_width: 0.,
            border_color: Color::TRANSPARENT,
        };

        renderer.fill_quad(quad, Background::Gradient(Gradient::Linear(grad)));
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_core::overlay::Element<'b, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut state.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<HoverGrad<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer + 'a,
    Renderer::Theme: StyleSheet,
    Message: 'a,
{
    fn from(value: HoverGrad<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}

pub struct Appearance {
    pub border_radius: f32,
    pub mouse_color: Color,
    pub fade_color: Color,
    pub fade_len: f32,
}

pub trait StyleSheet {
    type Style: Default;

    fn active(&self, style: &Self::Style) -> Option<Appearance>;

    fn hovered(&self, style: &Self::Style) -> Option<Appearance>;
}
