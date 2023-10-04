use iced_core::{
    gradient::Linear,
    layout::{Limits, Node},
    mouse::Cursor,
    renderer::{Quad, Style},
    widget::Tree,
    Background, Color, Degrees, Element, Gradient, Layout, Length, Padding,
    Rectangle, Size, Vector, Widget,
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
        let limits = limits.width(self.width).height(self.height);

        let content_limits = limits.pad(self.padding);

        let content = self
            .content
            .as_widget()
            .layout(renderer, &content_limits)
            .translate(Vector::new(self.padding.left, self.padding.top));
        let content_size = content.size();

        let min = limits.min();
        let limits = limits
            .min_width(min.width + self.padding.horizontal())
            .min_height(min.height + self.padding.vertical());

        let w = match self.width {
            Length::Fill | Length::FillPortion(_) => limits.max().width,
            Length::Shrink => content_size.width + self.padding.horizontal(),
            Length::Fixed(n) => n,
        };
        let h = match self.height {
            Length::Fill | Length::FillPortion(_) => limits.max().height,
            Length::Shrink => content_size.height + self.padding.vertical(),
            Length::Fixed(n) => n,
        };

        Node::with_children(Size::new(w, h), vec![content])
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
        let bounds = layout.bounds();

        let grad_style = if cursor.is_over(bounds) {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        let grad_style = if let Some(grad_style) = grad_style {
            grad_style
        } else {
            self.content.as_widget().draw(
                &state.children[0],
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                viewport,
            );
            return;
        };
        let grad_len = bounds.width * grad_style.len_percent;

        let mut center = if let Some(pos) = cursor.position() {
            pos.x - bounds.x
        } else {
            self.content.as_widget().draw(
                &state.children[0],
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                viewport,
            );
            return;
        };

        let left = center - grad_len;
        let right = center + grad_len;

        let (mut left, left_col) = if left > 0. {
            (left, 1.)
        } else {
            (0., center / grad_len)
        };
        let (mut right, right_col) = if right < bounds.width {
            (right, 1.)
        } else {
            (bounds.width, (bounds.width - center) / grad_len)
        };

        center /= bounds.width;
        left /= bounds.width;
        right /= bounds.width;

        let mut grad = Linear::new(Degrees(180.));

        grad = grad.add_stop(
            left,
            Color::from_rgba(
                grad_style.center_col.r * (1. - left_col)
                    + grad_style.side_col.r * left_col,
                grad_style.center_col.g * (1. - left_col)
                    + grad_style.side_col.g * left_col,
                grad_style.center_col.b * (1. - left_col)
                    + grad_style.side_col.b * left_col,
                grad_style.center_col.a * (1. - left_col)
                    + grad_style.side_col.a * left_col,
            ),
        );

        grad = grad.add_stop(center, grad_style.center_col);

        grad = grad.add_stop(
            right,
            Color::from_rgba(
                grad_style.center_col.r * (1. - right_col)
                    + grad_style.side_col.r * right_col,
                grad_style.center_col.g * (1. - right_col)
                    + grad_style.side_col.g * right_col,
                grad_style.center_col.b * (1. - right_col)
                    + grad_style.side_col.b * right_col,
                grad_style.center_col.a * (1. - right_col)
                    + grad_style.side_col.a * right_col,
            ),
        );

        let quad = Quad {
            bounds,
            border_radius: grad_style.border_radius.into(),
            border_width: 0.,
            border_color: Color::TRANSPARENT,
        };

        renderer.fill_quad(quad, Background::Gradient(Gradient::Linear(grad)));

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
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
    pub center_col: Color,
    pub side_col: Color,
    pub len_percent: f32,
}

pub trait StyleSheet {
    type Style: Default;

    fn active(&self, style: &Self::Style) -> Option<Appearance>;

    fn hovered(&self, style: &Self::Style) -> Option<Appearance>;
}
