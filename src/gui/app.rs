use std::cell::Cell;

use iced::widget::{
    button, column, container, row, scrollable, svg, text, Space,
};
use iced::{
    executor, Alignment, Application, Command, Element, Renderer,
    Subscription, Theme,
};
use iced_core::{window, Event, Length};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;

use super::widgets::svg_button::SvgButton;

pub struct BumpApp {
    player: Player,
    library: Library,
    config: Config,
    _sender: UnboundedSender<BumpMessage>,
    receiver: Cell<Option<UnboundedReceiver<BumpMessage>>>,
}

#[derive(Debug, Clone, Copy)]
pub enum BumpMessage {
    Update,
    Next,
    Prev,
    Play(Option<bool>),
    PlaySong(usize),
    SongEnd,
    Close,
}

impl Application for BumpApp {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = BumpMessage;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (sender, receiver) = mpsc::unbounded_channel::<BumpMessage>();
        let mut config = Config::load();
        let library = Library::load(&mut config);
        (
            BumpApp {
                player: Player::new(sender.clone()),
                library,
                config,
                _sender: sender,
                receiver: Cell::new(Some(receiver)),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("BUMP")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            BumpMessage::Update => _ = self.library.find(&mut self.config),
            BumpMessage::Next => {
                _ = self.player.next(&self.library);
            }
            BumpMessage::Prev => {
                _ = self.player.prev(&self.library);
            }
            BumpMessage::Play(play) => {
                let playing = self.player.is_playing();
                _ = self.player.play(play.unwrap_or(!playing));
            }
            BumpMessage::PlaySong(id) => {
                _ = self.player.play_at(&self.library, id as i128, true);
            }
            BumpMessage::SongEnd => {
                _ = self.player.next(&self.library);
            }
            BumpMessage::Close => {
                _ = self.config.save();
                _ = self.library.save();
                return iced::window::close();
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let active = self.player.get_current();

        column![
            button("Update library").on_press(BumpMessage::Update),
            text(active),
            self.bottom_bar(),
            self.vector_display(),
        ]
        .spacing(3)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            iced::subscription::unfold(
                "69".to_owned(),
                self.receiver.take(),
                |receiver| async {
                    let mut receiver = receiver.unwrap();
                    let message = receiver.recv().await.unwrap();
                    (message, Some(receiver))
                },
            ),
            iced::subscription::events_with(|event, _| match event {
                Event::Window(window::Event::CloseRequested) => {
                    Some(BumpMessage::Close)
                }
                _ => None,
            }),
        ])
    }
}

impl BumpApp {
    fn vector_display(&self) -> Element<BumpMessage> {
        let songs = self.library.get_songs();
        let mut c = 0;

        scrollable(
            column(
                songs
                    .iter()
                    .map(|s| {
                        c += 1;
                        button(text(format!(
                            "{} - {}",
                            s.get_name(),
                            s.get_artist()
                        )))
                        .width(iced::Length::Fill)
                        .on_press(BumpMessage::PlaySong(c - 1))
                        .into()
                    })
                    .collect(),
            )
            .spacing(3),
        )
        .into()
    }

    fn bottom_bar(&self) -> Element<BumpMessage> {
        row![
            container(
                self.title_bar(),
            ).width(Length::FillPortion(1)),
            self.play_menu(),
            Space::new(Length::FillPortion(1), Length::Shrink),
        ]
        .align_items(Alignment::Center)
        .into()
    }

    fn title_bar(&self) -> Element<BumpMessage> {
        let song = self.player.get_current_song(&self.library);
        column![
            text(song.get_name()).size(16),
            text(song.get_artist()).size(14),
        ]
        .into()
    }

    fn play_menu(&self) -> Element<BumpMessage> {
        let mut pp_icon = "assets/icons/play.svg";
        if self.player.is_playing() {
            pp_icon = "assets/icons/pause.svg";
        }
        let pp_handle = svg::Handle::from_path(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            pp_icon
        ));

        let prev_handle = svg::Handle::from_path(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "assets/icons/prev.svg"
        ));

        let next_handle = svg::Handle::from_path(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "assets/icons/next.svg"
        ));

        row![
            SvgButton::new(prev_handle)
                .width(18)
                .height(18)
                .on_press(BumpMessage::Prev),
            SvgButton::new(pp_handle)
                .width(33)
                .height(33)
                .on_press(BumpMessage::Play(None)),
            SvgButton::new(next_handle)
                .width(18)
                .height(18)
                .on_press(BumpMessage::Next),
        ]
        .height(70)
        .align_items(Alignment::Center)
        .spacing(20)
        .into()
    }
}
