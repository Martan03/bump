use std::cell::Cell;
use std::thread;
use std::time::{Duration, Instant};

use iced::widget::{
    button, column, container, row, scrollable, slider, svg, text,
};
use iced::{
    executor, Alignment, Application, Command, Element, Renderer, Subscription,
};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{window, Event, Length};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::library::song::Song;
use crate::player::player::Player;

use super::theme::{Button, Container, Text, Theme};
use super::widgets::svg_button::SvgButton;

pub struct BumpApp {
    player: Player,
    library: Library,
    config: Config,
    _sender: UnboundedSender<BumpMessage>,
    receiver: Cell<Option<UnboundedReceiver<BumpMessage>>>,
    theme: Theme,
}

#[derive(Debug, Clone, Copy)]
pub enum BumpMessage {
    Update,
    Next,
    Prev,
    Play(Option<bool>),
    PlaySong(usize),
    SeekTo(f32),
    SongEnd,
    Volume(f32),
    Mute(Option<bool>),
    Tick,
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
                theme: Theme::default(),
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
            BumpMessage::SeekTo(secs) => {
                _ = self.player.seek_to(Duration::from_secs_f32(secs));
            }
            BumpMessage::SongEnd => {
                _ = self.player.next(&self.library);
            }
            BumpMessage::Volume(vol) => _ = self.player.set_volume(vol),
            BumpMessage::Mute(mute) => {
                let mute = mute.unwrap_or(!self.player.get_mute());
                _ = self.player.set_mute(mute);
            }
            BumpMessage::Tick => {}
            BumpMessage::Close => {
                _ = self.config.save();
                _ = self.library.save();
                return iced::window::close();
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, BumpMessage, Renderer<Theme>> {
        let active = self.player.get_current();

        column![
            button("Update library")
                .style(Button::Primary)
                .on_press(BumpMessage::Update),
            text(format!("{}/{}", active, self.library.count())),
            container(self.vector_display(),).height(Length::FillPortion(1)),
            self.bottom_bar(),
        ]
        .spacing(3)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
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
            iced::subscription::unfold(
                "69 tick".to_owned(),
                Instant::now(),
                |t| async move {
                    let delta = Instant::now() - t;
                    let tick = Duration::from_secs(1);
                    if delta < tick {
                        thread::sleep(tick - delta);
                    }
                    (BumpMessage::Tick, t + tick)
                },
            ),
        ])
    }
}

impl BumpApp {
    fn vector_display(&self) -> Element<'_, BumpMessage, Renderer<Theme>> {
        let songs = self.library.get_songs();
        let mut c = 0;

        scrollable(
            column(
                songs
                    .iter()
                    .map(|s| {
                        c += 1;
                        let total_secs = s.get_length().as_secs();
                        let mins = total_secs / 60;
                        let secs = mins % 60;
                        let time = if total_secs > 0 {
                            format!("{:02}:{:02}", mins, secs)
                        } else {
                            "--:--".to_owned()
                        };
                        button(row![
                            text(s.get_name()).width(Length::FillPortion(1)),
                            text(s.get_artist()).width(Length::FillPortion(1)),
                            text(time).width(50),
                        ])
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

    fn bottom_bar(&self) -> Element<'_, BumpMessage, Renderer<Theme>> {
        let song = self.player.get_current_song(&self.library);
        let (time, len) = self.player.get_timestamp();

        container(column![
            slider(0.0..=len.as_secs_f32(), time.as_secs_f32(), |v| {
                BumpMessage::SeekTo(v)
            })
            .height(4)
            .step(0.01),
            row![
                container(self.title_bar(song),).width(Length::FillPortion(1)),
                self.play_menu(),
                container(self.volume_menu(),).width(Length::FillPortion(1)),
            ]
            .height(Length::Fill)
            .padding(5)
            .align_items(Alignment::Center)
        ])
        .align_y(Vertical::Center)
        .height(60)
        .style(Container::Dark)
        .into()
    }

    fn title_bar(
        &self,
        song: Song,
    ) -> Element<'_, BumpMessage, Renderer<Theme>> {
        column![
            text(song.get_name()).size(16).style(Text::Light),
            text(song.get_artist()).size(14).style(Text::Dark),
        ]
        .into()
    }

    fn play_menu(&self) -> Element<'_, BumpMessage, Renderer<Theme>> {
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
                .width(16)
                .height(16)
                .on_press(BumpMessage::Prev),
            SvgButton::new(pp_handle)
                .width(30)
                .height(30)
                .on_press(BumpMessage::Play(None)),
            SvgButton::new(next_handle)
                .width(16)
                .height(16)
                .on_press(BumpMessage::Next),
        ]
        .align_items(Alignment::Center)
        .spacing(20)
        .into()
    }

    fn volume_menu(&self) -> Element<'_, BumpMessage, Renderer<Theme>> {
        let mut icon = "assets/icons/volume.svg";
        if self.player.get_mute() {
            icon = "assets/icons/volume_muted.svg";
        }
        let handle = svg::Handle::from_path(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            icon
        ));

        container(
            row![
                SvgButton::new(handle)
                    .width(18)
                    .height(18)
                    .on_press(BumpMessage::Mute(None)),
                text(format!("{:.0}", self.player.get_volume() * 100.0))
                    .width(28)
                    .style(Text::Normal),
                slider(0.0..=1., self.player.get_volume(), |v| {
                    BumpMessage::Volume(v)
                })
                .step(0.01)
                .width(100),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Right)
        .into()
    }
}
