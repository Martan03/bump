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

use crate::config::{ConfMsg, Config};
use crate::hotkeys::Hotkeys;
use crate::library::Library;
use crate::player::{Player, PlayerMsg};
use crate::server::Server;

use super::gui::Gui;
use super::settings::{Settings, SettingsMsg};
use super::theme::Theme;

pub struct BumpApp {
    pub player: Player,
    pub library: Library,
    pub config: Config,
    pub gui: Gui,
    pub sender: UnboundedSender<Msg>,
    pub receiver: Cell<Option<UnboundedReceiver<Msg>>>,
    pub theme: Theme,
    pub page: Page,
    pub hard_pause: Option<Instant>,
    listener: Cell<Option<TcpListener>>,
    pub hotkeys: Option<Hotkeys>,
    pub settings: Settings,
}

/// All pages enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Page {
    Library,
    Playlist,
    Settings,
}

/// Library messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LibMsg {
    LoadStart,
    LoadEnded,
}

/// Bump app messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Msg {
    Page(Page),
    Plr(PlayerMsg),
    Lib(LibMsg),
    Conf(ConfMsg),
    Settings(SettingsMsg),
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
            Msg::Page(msg) => {
                self.gui.get_wb_state_mut(1).get_mut().scroll_to =
                    self.player.get_current();
                self.page = msg
            }
            Msg::Plr(msg) => self.player_update(msg),
            Msg::Lib(msg) => {
                self.library
                    .handle_msg(&self.config, self.sender.clone(), msg)
            }
            Msg::Conf(msg) => self.conf_update(msg),
            Msg::Settings(msg) => {
                return self.settings_update(msg);
            }
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
        let settings = Settings::new(&config);

        let mut app = Self {
            player: Player::new(sender.clone(), &library, &config),
            library,
            config,
            gui,
            sender,
            receiver: Cell::new(Some(receiver)),
            theme: Theme::default(),
            page: Page::Library,
            hard_pause: None,
            listener: Cell::new(listener),
            hotkeys: None,
            settings,
        };

        app.enable_hotkeys(app.config.get_enable_hotkeys());
        if app.config.get_start_load() {
            app.library.start_find(&app.config, app.sender.clone());
        }
        app
    }

    /// Saves all things
    fn save_all(&mut self) {
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
            format!("{} receiver", Config::get_app_id()),
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
            format!("{} tick", Config::get_app_id()),
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
                format!("{} server", Config::get_app_id()),
                listener,
                |listener| async {
                    loop {
                        let stream = match listener.accept() {
                            Ok(stream) => stream,
                            _ => continue,
                        };

                        if let Some(msg) = Server::handle_client(&stream.0) {
                            Server::send_cli_response(&stream.0, "Ok");
                            return (msg, listener);
                        }
                        Server::send_cli_response(
                            &stream.0,
                            "Error receiving message",
                        );
                    }
                },
            )
        } else {
            iced::subscription::unfold(
                format!("{} server", Config::get_app_id()),
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
