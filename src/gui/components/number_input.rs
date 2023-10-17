use iced::Renderer;
use iced_lazy::Component;
use iced_native::Element;

#[derive(Debug, Clone)]
/// [`NumberInput`] events
pub enum Event {
    InputChanged(String),
}

pub struct NumberInput<Message> {
    value: Option<u32>,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
}

/// Creates new [`NumberInput`]
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
        event: Event,
    ) -> Option<Message> {
        match event {
            Event::InputChanged(value) => {
                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    value.parse().ok().map(Some).map(self.on_change.as_ref())
                }
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> Element<'_, Self::Event, Renderer> {
        todo!()
    }
}
