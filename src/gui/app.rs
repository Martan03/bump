use std::cell::Cell;
use std::thread;
use std::time::{Duration, Instant};

use iced::widget::{column, row, Rule};
use iced::{executor, Application, Command, Element, Renderer, Subscription};
use iced_core::{window, Alignment, Event, Length};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;

use super::gui::Gui;
use super::theme::Theme;

pub struct BumpApp {
    pub(super) player: Player,
    pub(super) library: Library,
    pub(super) config: Config,
    pub(super) gui: Gui,
    _sender: UnboundedSender<Msg>,
    pub(super) receiver: Cell<Option<UnboundedReceiver<Msg>>>,
    pub(super) theme: Theme,
    pub(super) page: Page,
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
pub enum Page {
    Library,
    Playlist,
    Settings,
}

#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Page(Page),
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
            Msg::Page(msg) => self.page = msg,
            Msg::Plr(msg) => self.player.handle_msg(msg, &self.library),
            Msg::Update => _ = self.library.find(&mut self.config),
            Msg::Tick => {}
            Msg::Move(x, y) => self.gui.set_pos(x, y),
            Msg::Size(w, h) => self.gui.set_size(w, h),
            Msg::Close => {
                _ = self.config.save();
                _ = self.library.save(&self.config);
                _ = self.gui.save(&self.config);
                _ = self.player.save(&self.config);
                return iced::window::close();
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Msg, Renderer<Theme>> {
        let page = match self.page {
            Page::Library => self.view_library(),
            Page::Playlist => self.view_playlist(),
            Page::Settings => self.view_settings(),
        };

        column![
            row![self.menu(), Rule::vertical(1), page,]
                .height(Length::Fill)
                .spacing(3),
            self.player_bar(),
        ]
        .align_items(Alignment::Center)
        .into()
    }

    /// Sets app theme
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    /// Creates app subscriptions
    fn subscription(&self) -> Subscription<Msg> {
        Subscription::batch([
            self.receiver_subscription(),
            self.window_subscription(),
            self.tick_subscription(Duration::from_secs(1)),
        ])
    }
}

impl BumpApp {
    fn new(config: Config, gui: Gui) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel::<Msg>();
        let library = Library::load(&config);

        BumpApp {
            player: Player::new(sender.clone(), &library, &config),
            library,
            config,
            gui,
            _sender: sender,
            receiver: Cell::new(Some(receiver)),
            theme: Theme::default(),
            page: Page::Library,
        }
    }

    /// Creates receiver subscription
    fn receiver_subscription(&self) -> Subscription<Msg> {
        iced::subscription::unfold(
            "bump receiver".to_owned(),
            self.receiver.take(),
            |receiver| async {
                let mut receiver = receiver.unwrap();
                let message = receiver.recv().await.unwrap();
                (message, Some(receiver))
            },
        )
    }

    /// Creates window subscription (Close, move, resize)
    fn window_subscription(&self) -> Subscription<Msg> {
        iced::subscription::events_with(|event, _| match event {
            Event::Window(window::Event::CloseRequested) => Some(Msg::Close),
            Event::Window(window::Event::Moved { x, y }) => {
                Some(Msg::Move(x, y))
            }
            Event::Window(window::Event::Resized { width, height }) => {
                Some(Msg::Size(width, height))
            }
            _ => None,
        })
    }

    /// creates tick subcription that's sending message every `tick`
    fn tick_subscription(&self, tick: Duration) -> Subscription<Msg> {
        iced::subscription::unfold(
            "bump tick".to_owned(),
            Instant::now(),
            move |t| async move {
                let delta = Instant::now() - t;
                if delta < tick {
                    thread::sleep(tick - delta);
                }
                (Msg::Tick, t + tick)
            },
        )
    }
}
