use iced::{
    widget::{component, text_input, Component},
    Element, Renderer,
};

use crate::gui::theme::Theme;

#[derive(Debug, Clone)]
/// [`Input`] events
pub enum Event {
    Changed(String),
}

pub struct Input<Message> {
    placeholder: Option<String>,
    value: Option<String>,
    on_change: Box<dyn Fn(Option<String>) -> Message>,
}

/// Creates new [`Input`]
#[allow(unused)]
pub fn input<Message>(
    placeholder: Option<String>,
    value: Option<String>,
    on_change: impl Fn(Option<String>) -> Message + 'static,
) -> Input<Message> {
    Input::new(placeholder, value, on_change)
}

impl<Message> Input<Message> {
    /// Constructs new [`Input`]
    pub fn new(
        placeholder: Option<String>,
        value: Option<String>,
        on_change: impl Fn(Option<String>) -> Message + 'static,
    ) -> Self {
        Self {
            placeholder,
            value,
            on_change: Box::new(on_change),
        }
    }
}

impl<Message> Component<Message, Renderer<Theme>> for Input<Message> {
    type State = ();
    type Event = Event;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            Event::Changed(val) => {
                val.parse().ok().map(Some).map(self.on_change.as_ref())
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> Element<'_, Self::Event, Renderer<Theme>> {
        text_input(
            self.placeholder.as_deref().unwrap_or(""),
            self.value.as_deref().unwrap_or(""),
        )
        .into()
    }
}

impl<'a, Message> From<Input<Message>>
    for Element<'a, Message, Renderer<Theme>>
where
    Message: 'a,
{
    fn from(input: Input<Message>) -> Self {
        component(input)
    }
}
