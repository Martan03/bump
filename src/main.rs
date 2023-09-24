use std::{env, io::prelude::*, net::TcpStream};

use config::config::Config;
use gui::app::BumpApp;
use gui::app::Msg;
use gui::app::PlayerMsg;
use gui::gui::Gui;
use iced::window;
use iced::window::PlatformSpecific;
use iced::Application;
use iced::Settings;

mod config {
    pub mod config;
}
mod gui {
    pub mod app;
    pub mod elements;
    pub mod gui;
    pub mod library;
    pub mod playlist;
    pub mod settings;
    pub mod svg_data;
    pub mod theme;
    pub mod widgets {
        pub mod list_view;
        pub mod svg_button;
    }
}
mod library {
    pub mod library;
    pub mod song;
}
mod player {
    pub mod player;
    pub mod sinker;
}

fn main() -> Result<(), iced::Error> {
    let args: Vec<_> = env::args().skip(1).collect();
    if !args.is_empty() {
        parse_args(args);
        return Ok(());
    }
    // on wayland, the app freezes when minimized, this is temporary workaround
    // until it is fixed
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    if let Err(_) = init_logger() {
        eprintln!("Failed to start logger");
    }

    BumpApp::run(make_settings())
}

/// Inits logger
fn init_logger() -> eyre::Result<()> {
    if let Ok(logger) = flexi_logger::Logger::try_with_env_or_str("warn") {
        logger
            .log_to_file(
                flexi_logger::FileSpec::default()
                    .directory(Config::get_config_dir().join("log")),
            )
            .start()?;
    }
    Ok(())
}

/// Makes window settings, loads saved settings
fn make_settings() -> Settings<(Config, Gui)> {
    let config = Config::load();
    let gui = Gui::load(&config);

    let icon = window::icon::from_rgba(
        include_bytes!("../assets/raw_img/icon_64.data")
            .to_owned()
            .into(),
        64,
        64,
    );
    let id = "bump";

    Settings {
        window: window::Settings {
            icon: icon.ok(),
            size: gui.get_size(),
            position: gui.get_pos(),
            platform_specific: PlatformSpecific {
                application_id: id.to_owned(),
            },
            ..Default::default()
        },
        id: Some(id.to_owned()),
        exit_on_close_request: false,
        flags: (config, gui),
        ..Default::default()
    }
}

/// Parses given arguments
fn parse_args(mut args: Vec<String>) {
    if let Some(arg) = args.get(0) {
        if arg == "h" || arg == "-h" || arg == "--help" {
            help();
        } else if arg == "i" || arg == "instance" {
            args.remove(0);
            if args.is_empty() {
                eprintln!("No instance arguments given");
            } else {
                parse_instance_args(args);
            }
        } else {
            eprintln!("Invalid argument: {arg}");
            return;
        }
    }
}

/// Prints help
/// TODO
fn help() {
    println!("This will be help one day");
}

/// Parses instance arguments
fn parse_instance_args(args: Vec<String>) {
    for arg in args {
        match arg.as_str() {
            "pp" | "play-pause" => send_message(Msg::Plr(PlayerMsg::Play(None))),
            "next" => send_message(Msg::Plr(PlayerMsg::Next)),
            "prev" => send_message(Msg::Plr(PlayerMsg::Prev)),
            "shuffle" | "mix" => send_message(Msg::Plr(PlayerMsg::Shuffle)),
            "exit" | "close" | "quit" => send_message(Msg::Close),
            _ => todo!(),
        }
    }
}

/// Sends given message to the server
fn send_message(msg: Msg) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:2867") {
        if let Ok(msg) = serde_json::to_string::<Msg>(&msg) {
            _ = stream.write(msg.as_bytes());
        }
    } else {
        eprintln!("Error sending message");
    }
}
