use iced::{
    widget::{text_input, Component},
    Element, Renderer,
};

#[derive(Debug, Clone)]
/// [`NumberInput`] events
#[allow(unused)]
pub enum Event {
    InputChanged(String),
}

pub struct NumberInput<Message> {
    value: Option<u32>,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
}

/// Creates new [`NumberInput`]
#[allow(unused)]
pub fn number_input<Message>(
    value: Option<u32>,
    on_change: impl Fn(Option<u32>) -> Message + 'static,
) -> NumberInput<Message> {
    NumberInput::new(value, on_change)
}

impl<Message> NumberInput<Message> {
    /// Constructs new [`NumberInput`]
    pub fn new(
        value: Option<u32>,
        on_change: impl Fn(Option<u32>) -> Message + 'static,
    ) -> Self {
        Self {
            value,
            on_change: Box::new(on_change),
        }
    }
}

impl<Message> Component<Message, Renderer> for NumberInput<Message> {
    type State = ();
    type Event = Event;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            Event::InputChanged(val) => {
                val.parse().ok().map(Some).map(self.on_change.as_ref())
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> Element<'_, Self::Event, Renderer> {
        text_input(
            "Enter a number:",
            self.value
                .as_ref()
                .map(u32::to_string)
                .as_deref()
                .unwrap_or(""),
        )
        .into()
    }
}
