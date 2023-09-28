use std::cell::Cell;
use std::net::TcpListener;
use std::thread;
use std::time::{Duration, Instant};

use iced::widget::{column, row, Rule};
use iced::{executor, Application, Command, Element, Renderer, Subscription};
use iced_core::{window, Alignment, Event, Length};
use log::error;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::config::config::Config;
use crate::library::library::Library;
use crate::player::player::Player;
use crate::server::server::Server;

use super::gui::Gui;
use super::theme::Theme;

pub struct BumpApp {
    pub(super) player: Player,
    pub(super) library: Library,
    pub(super) config: Config,
    pub(super) gui: Gui,
    pub(super) sender: UnboundedSender<Msg>,
    pub(super) receiver: Cell<Option<UnboundedReceiver<Msg>>>,
    pub(super) theme: Theme,
    pub(super) page: Page,
    pub(super) hard_pause: Option<Instant>,
    listener: Cell<Option<TcpListener>>,
}

/// Messages to player
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerMsg {
    Play(Option<bool>),
    PlaySong(usize, bool),
    Next,
    Prev,
    SeekTo(Duration),
    SongEnd,
    Volume(f32),
    Mute(Option<bool>),
    Shuffle,
    VolumeUp(Option<f32>),
    VolumeDown(Option<f32>),
}

/// All pages enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Page {
    Library,
    Playlist,
    Settings,
}

/// Library messages
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LibMsg {
    LoadStart,
    LoadEnded,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConfMsg {
    RecursiveSearch(bool),
    ShuffleCurrent(bool),
    Autoplay(bool),
    StartLoad(bool),
    Gapless(bool),
}

/// Bump app messages
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Msg {
    Page(Page),
    Plr(PlayerMsg),
    Lib(LibMsg),
    Conf(ConfMsg),
    Tick,
    Move(i32, i32),
    Size(u32, u32),
    Close,
    #[serde(skip)]
    HardPause(Instant),
}

impl Application for BumpApp {
    type Executor = executor::Default;
    type Flags = (Config, Gui);
    type Theme = Theme;
    type Message = Msg;

    /// Creates new Application
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (BumpApp::new(flags.0, flags.1), Command::none())
    }

    /// Sets the title of the app
    fn title(&self) -> String {
        String::from("BUMP")
    }

    /// Handles app updates (messages)
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // Handle messages
        match message {
            Msg::Page(msg) => self.page = msg,
            Msg::Plr(msg) => self.player.handle_msg(msg, &mut self.library),
            Msg::Lib(msg) => {
                self.library
                    .handle_msg(&self.config, self.sender.clone(), msg)
            }
            Msg::Conf(msg) => self.config.handle_msg(msg),
            Msg::Tick => {}
            Msg::Move(x, y) => self.gui.set_pos(x, y),
            Msg::Size(w, h) => self.gui.set_size(w, h),
            Msg::Close => {
                self.save_all();
                return iced::window::close();
            }
            Msg::HardPause(i) => self.hard_pause = Some(i),
        };
        // Handle hard pause
        if let Some(i) = self.hard_pause {
            let now = Instant::now();
            if i <= now {
                self.player.hard_pause();
                self.hard_pause = None;
            }
        }
        Command::none()
    }

    /// Renders the view
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
            self.server_subscription(),
        ])
    }
}

impl BumpApp {
    /// Creates new BumpApp
    fn new(config: Config, gui: Gui) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel::<Msg>();
        let library = Library::load(&config);

        let listener = match TcpListener::bind(config.get_server_address()) {
            Ok(listener) => Some(listener),
            Err(_) => None,
        };

        let mut app = Self {
            player: Player::new(sender.clone(), &library, &config),
            library,
            config,
            gui,
            sender: sender,
            receiver: Cell::new(Some(receiver)),
            theme: Theme::default(),
            page: Page::Library,
            hard_pause: None,
            listener: Cell::new(listener),
        };

        if app.config.get_start_load() {
            app.library.start_find(&mut app.config, app.sender.clone());
        }
        app
    }

    /// Saves all things
    fn save_all(&self) {
        if let Err(e) = self.config.save() {
            error!("Failed to save config: {e}");
        }
        if let Err(e) = self.library.save(&self.config) {
            error!("Failed to save library: {e}");
        }
        if let Err(e) = self.gui.save(&self.config) {
            error!("Failed to save GUI state: {e}");
        }
        if let Err(e) = self.player.save(&self.config) {
            error!("Failed to save player state: {e}");
        }
    }

    //>=====================================================================<//
    //                             Subscriptions                             //
    //>=====================================================================<//

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

    /// Creates server subscription
    fn server_subscription(&self) -> Subscription<Msg> {
        if let Some(listener) = self.listener.take() {
            iced::subscription::unfold(
                "bump server".to_owned(),
                listener,
                |listener| async {
                    loop {
                        let stream = match listener.accept() {
                            Ok(stream) => stream,
                            _ => continue,
                        };

                        if let Some(msg) = Server::handle_client(stream.0) {
                            return (msg, listener);
                        }
                    }
                },
            )
        } else {
            iced::subscription::unfold(
                "bump server".to_owned(),
                (),
                |_| async {
                    loop {
                        println!("Duck");
                    }
                },
            )
        }
    }
}
