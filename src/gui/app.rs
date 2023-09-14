use std::cell::Cell;
use std::thread;
use std::time::{Duration, Instant};

use iced::widget::{
    button, column, container, row, slider, svg, text,
};
use iced::{
    executor, Alignment, Application, Command, Element, Renderer, Subscription,
};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{window, Event, Length};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;

use super::gui::Gui;
use super::theme::{Button, Container, Text, Theme};
use super::widgets::svg_button::SvgButton;

pub struct BumpApp {
    pub(super) player: Player,
    pub(super) library: Library,
    pub(super) config: Config,
    pub(super) gui: Gui,
    _sender: UnboundedSender<Msg>,
    pub(super) receiver: Cell<Option<UnboundedReceiver<Msg>>>,
    pub(super) theme: Theme,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerMsg {
    Play(Option<bool>),
    PlaySong(usize),
    Next,
    Prev,
    SeekTo(f32),
    SongEnd,
    Volume(f32),
    Mute(Option<bool>),
    Shuffle,
}

#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Plr(PlayerMsg),
    Update,
    Tick,
    Move(i32, i32),
    Size(u32, u32),
    Close,
}

impl Application for BumpApp {
    type Executor = executor::Default;
    type Flags = (Config, Gui);
    type Theme = Theme;
    type Message = Msg;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (BumpApp::new(flags.0, flags.1), Command::none())
    }

    fn title(&self) -> String {
        String::from("BUMP")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Msg::Plr(msg) => self.player.handle_msg(msg, &self.library),
            Msg::Update => _ = self.library.find(&mut self.config),
            Msg::Tick => {}
            Msg::Move(x, y) => self.gui.set_pos(x, y),
            Msg::Size(w, h) => self.gui.set_size(w, h),
            Msg::Close => {
                _ = self.config.save();
                _ = self.library.save(&self.config);
                _ = self.gui.save(&self.config);
                return iced::window::close();
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Msg, Renderer<Theme>> {
        column![
            row![
                button("Shuffle")
                    .style(Button::Primary)
                    .on_press(Msg::Plr(PlayerMsg::Shuffle)),
                button("Update library")
                    .style(Button::Primary)
                    .on_press(Msg::Update),
            ]
            .spacing(3),
            container(self.songs_list()).height(Length::FillPortion(1)),
            self.bottom_bar(),
        ]
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
                    Some(Msg::Close)
                }
                Event::Window(window::Event::Moved { x, y }) => {
                    Some(Msg::Move(x, y))
                }
                Event::Window(window::Event::Resized { width, height }) => {
                    Some(Msg::Size(width, height))
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
                    (Msg::Tick, t + tick)
                },
            ),
        ])
    }
}

impl BumpApp {
    fn new(config: Config, gui: Gui) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel::<Msg>();
        let library = Library::load(&config);

        BumpApp {
            player: Player::new(sender.clone(), &library),
            library,
            config,
            gui,
            _sender: sender,
            receiver: Cell::new(Some(receiver)),
            theme: Theme::default(),
        }
    }

    fn bottom_bar(&self) -> Element<'_, Msg, Renderer<Theme>> {
        let (time, len) = self.player.get_timestamp();

        container(column![
            slider(0.0..=len.as_secs_f32(), time.as_secs_f32(), |v| {
                Msg::Plr(PlayerMsg::SeekTo(v))
            })
            .height(4)
            .step(0.01),
            row![
                container(self.title_bar(),).width(Length::FillPortion(1)),
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

    fn title_bar(&self) -> Element<'_, Msg, Renderer<Theme>> {
        let song = self.player.get_current_song(&self.library);
        column![
            text(song.get_name()).size(16).style(Text::Light),
            text(song.get_artist()).size(14).style(Text::Dark),
        ]
        .into()
    }

    fn play_menu(&self) -> Element<'_, Msg, Renderer<Theme>> {
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
                .on_press(Msg::Plr(PlayerMsg::Prev)),
            SvgButton::new(pp_handle)
                .width(30)
                .height(30)
                .on_press(Msg::Plr(PlayerMsg::Play(None))),
            SvgButton::new(next_handle)
                .width(16)
                .height(16)
                .on_press(Msg::Plr(PlayerMsg::Next)),
        ]
        .align_items(Alignment::Center)
        .spacing(20)
        .into()
    }

    fn volume_menu(&self) -> Element<'_, Msg, Renderer<Theme>> {
        let mut icon = "assets/icons/volume_100.svg";
        if self.player.get_mute() {
            icon = "assets/icons/volume_muted.svg";
        } else if self.player.get_volume() < 0.33 {
            icon = "assets/icons/volume_33.svg";
        } else if self.player.get_volume() < 0.66 {
            icon = "assets/icons/volume_66.svg";
        }
        let handle = svg::Handle::from_path(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            icon
        ));

        container(
            row![
                SvgButton::new(handle)
                    .width(20)
                    .height(20)
                    .on_press(Msg::Plr(PlayerMsg::Mute(None))),
                text(format!("{:.0}", self.player.get_volume() * 100.0))
                    .width(28)
                    .style(Text::Normal),
                slider(0.0..=1., self.player.get_volume(), |v| {
                    Msg::Plr(PlayerMsg::Volume(v))
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
