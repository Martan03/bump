use std::env;

use cli::Cli;
use config::Config;
use gui::app::BumpApp;
use gui::gui::Gui;
use iced::window;
use iced::window::PlatformSpecific;
use iced::Application;
use iced::Settings;

mod cli;
mod config;
mod core;
mod gui;
mod hotkeys;
mod library;
mod player;
mod server;

fn main() -> Result<(), iced::Error> {
    let config = Config::load();

    let args: Vec<_> = env::args().skip(1).collect();
    if !args.is_empty() {
        let mut cli = Cli::new(&config);
        cli.parse(args);
        return Ok(());
    }
    // on wayland, the app freezes when minimized, this is temporary workaround
    // until it is fixed
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    if let Err(_) = init_logger() {
        eprintln!("Failed to start logger");
    }

    BumpApp::run(make_settings(config))
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
fn make_settings(config: Config) -> Settings<(Config, Gui)> {
    let gui = Gui::load(&config);

    let icon = window::icon::from_rgba(
        include_bytes!("../assets/raw_img/icon_64.data")
            .to_owned()
            .into(),
        64,
        64,
    );

    let id = Config::get_app_id();

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
        id: Some(id),
        exit_on_close_request: false,
        flags: (config, gui),
        ..Default::default()
    }
}
