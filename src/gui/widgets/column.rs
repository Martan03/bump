//! Distribute content vertically.
#![allow(unused)]
use iced::mouse;
use iced_core::{
    event,
    layout::{Limits, Node},
    overlay, renderer, svg,
    widget::{Operation, Tree},
    Alignment, Clipboard, Element, Event, Layout, Length, Padding, Pixels,
    Rectangle, Shell, Vector, Widget,
};

const SCROLLBAR_WIDTH: f32 = 20.;

/// A container that distributes its contents vertically.
#[allow(missing_debug_implementations)]
pub struct ListView<'a, Message, Renderer: svg::Renderer> {
    width: Length,
    max_width: f32,
    height: Length,
    max_height: f32,
    spacing: f32,
    padding: Padding,
    align_items: Alignment,
    state: State,
    children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer: svg::Renderer> ListView<'a, Message, Renderer> {
    /// Creates an empty [`ListView`].
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    /// Creates a [`ListView`] with the given elements.
    pub fn with_children(
        children: Vec<Element<'a, Message, Renderer>>,
    ) -> Self {
        ListView {
            width: Length::Shrink,
            max_width: f32::INFINITY,
            height: Length::Shrink,
            max_height: f32::INFINITY,
            spacing: 0.0,
            padding: Padding::ZERO,
            align_items: Alignment::Start,
            state: State::default(),
            children,
        }
    }

    /// Sets the width of the [`ListView`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the maximum width of the [`ListView`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the height of the [`ListView`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`ListView`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the horizontal alignment of the contents of the [`ListView`] .
    pub fn align_items(mut self, align: Alignment) -> Self {
        self.align_items = align;
        self
    }

    /// Adds an element to the [`ListView`].
    pub fn push(
        mut self,
        child: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message, Renderer: svg::Renderer> Default
    for ListView<'a, Message, Renderer>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for ListView<'a, Message, Renderer>
where
    Renderer: svg::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);
        let size = limits.fill();

        let mut pos = 0.;

        let children = self
            .children
            .iter()
            .map(|c| {
                let node = c
                    .as_widget()
                    .layout(renderer, &limits)
                    .translate(Vector::new(0., pos));
                pos += node.size().height;
                node
            })
            .collect();

        Node::with_children(size, children)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                })
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state, layout, cursor, viewport, renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer)
    }
}

impl<'a, Message, Renderer> From<ListView<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: svg::Renderer + 'a,
{
    fn from(list_view: ListView<'a, Message, Renderer>) -> Self {
        Self::new(list_view)
    }
}

pub struct State {
    offset: f32,
    pub scroll_to: Option<usize>,
}

impl State {
    fn new() -> Self {
        Self {
            offset: 0.,
            scroll_to: None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
