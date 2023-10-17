use eyre::Result;
use log::error;
use place_macro::place;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
    time::Duration,
};

use crate::{generate_struct, gui::app::BumpApp, hotkeys::Hotkeys};

use super::ConfMsg;

generate_struct! {
    #[derive(Clone, Serialize, Deserialize)]
    pub Config {
        paths: Vec<PathBuf> => {
            if let Some(dir) = dirs::audio_dir() {
                vec![dir]
            } else {
                vec![PathBuf::from(".")]
            }
        },
        extensions: Vec<String> => {
            vec![
                "mp3".to_owned(),
                "flac".to_owned(),
                "m4a".to_owned(),
                "mp4".to_owned(),
            ]
        },
        library_path: PathBuf => Config::get_config_dir().join("library.json"),
        gui_path: PathBuf => Config::get_config_dir().join("gui.json"),
        player_path: PathBuf => Config::get_config_dir().join("player.json"),
        server_ip: String => "127.0.0.1".to_owned(),
        server_port: String => {
            #[cfg(debug_assertions)]
            {
                "23456".to_owned()
            }
            #[cfg(not(debug_assertions))]
            {
                "2867".to_owned()
            }
        },
        hotkeys: HashMap<String, String> => {
            let mut hotkeys = HashMap::new();
            hotkeys.insert("ctrl+alt+home".to_owned(), "pp".to_owned());
            hotkeys.insert("ctrl+alt+pg_up".to_owned(), "prev".to_owned());
            hotkeys.insert("ctrl+alt+pg_down".to_owned(), "next".to_owned());
            hotkeys.insert("ctrl+alt+up".to_owned(), "vu".to_owned());
            hotkeys.insert("ctrl+alt+down".to_owned(), "vd".to_owned());
            hotkeys
        },
        ;
        fade: Duration => Duration::from_millis(150),
        volume_step: f32 => 0.1,
        recursive_search: bool => true,
        shuffle_current: bool => false,
        autoplay: bool => false,
        start_load: bool => true,
        gapless: bool => false,
        enable_hotkeys: bool => true,
        ;
    }
}

impl Config {
    /// Loads config from config file
    pub fn load() -> Self {
        let mut path = Config::get_config_dir();
        path.push("config.json");

        match fs::read_to_string(path) {
            Err(_) => Config::default(),
            Ok(c) => match serde_json::from_str::<Config>(&c) {
                Err(_) => Config::default(),
                Ok(conf) => conf,
            },
        }
    }

    /// Saves config to the config directory
    pub fn save(&mut self) -> Result<()> {
        // When nothing changed, don't save
        if !self.changed {
            return Ok(());
        }

        let mut dir = Config::get_config_dir();
        fs::create_dir_all(&dir)?;

        dir.push("config.json");
        File::create(&dir)?;

        let text = serde_json::to_string_pretty::<Config>(self)?;
        fs::write(dir, text)?;

        self.changed = false;

        Ok(())
    }

    ///>===================================================================<///
    ///                          Getters & Setters                          ///
    ///>===================================================================<///

    /// Gets app id based on if running in debug or not
    pub fn get_app_id() -> String {
        #[cfg(debug_assertions)]
        {
            "bump_debug".to_owned()
        }
        #[cfg(not(debug_assertions))]
        {
            "bump".to_owned()
        }
    }

    /// Adds given path to paths
    pub fn add_path(&mut self, paths: Vec<PathBuf>) {
        self.changed = true;
        for path in paths {
            self.paths.push(path);
        }
    }

    /// Removes path on given index
    pub fn remove_path(&mut self, id: usize) {
        if id < self.paths.len() {
            self.changed = true;
            self.paths.remove(id);
        }
    }

    /// Gets config dir path
    pub fn get_config_dir() -> PathBuf {
        if let Some(mut dir) = dirs::config_dir() {
            dir.push(Config::get_app_id());
            dir
        } else {
            PathBuf::from(".")
        }
    }

    /// Gets server address
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.get_server_ip(), self.get_server_port())
    }

    /// Resets all the settings to the default value
    pub fn reset_all(&mut self) {
        *self = Config::default();
    }
}

/// Implements default for Config
impl Default for Config {
    fn default() -> Self {
        Config {
            paths: Config::default_paths(),
            extensions: Config::default_extensions(),
            library_path: Config::default_library_path(),
            gui_path: Config::default_gui_path(),
            player_path: Config::default_player_path(),
            recursive_search: Config::default_recursive_search(),
            shuffle_current: Config::default_shuffle_current(),
            fade: Config::default_fade(),
            volume_step: Config::default_volume_step(),
            autoplay: Config::default_autoplay(),
            start_load: Config::default_start_load(),
            gapless: Config::default_gapless(),
            changed: true,
            server_ip: Config::default_server_ip(),
            server_port: Config::default_server_port(),
            hotkeys: Config::default_hotkeys(),
            enable_hotkeys: Config::default_enable_hotkeys(),
        }
    }
}

///>=======================================================================<///
///                         Config message handling                         ///
///>=======================================================================<///
impl BumpApp {
    pub fn conf_update(&mut self, msg: ConfMsg) {
        match msg {
            ConfMsg::AddPath(paths) => self.config.add_path(paths),
            ConfMsg::RemPath(id) => self.config.remove_path(id),
            ConfMsg::EnableHotkeys(val) => self.enable_hotkeys(val),
            ConfMsg::RecursiveSearch(val) => {
                self.config.set_recursive_search(val);
            }
            ConfMsg::ShuffleCurrent(val) => {
                self.config.set_shuffle_current(val);
                self.player.load_config(&self.config);
            }
            ConfMsg::Autoplay(val) => self.config.set_autoplay(val),
            ConfMsg::StartLoad(val) => self.config.set_start_load(val),
            ConfMsg::Gapless(val) => self.config.set_gapless(val),
            ConfMsg::ResetAll => self.config.reset_all(),
        }
    }

    /// Enables/disables hotkeys
    fn enable_hotkeys(&mut self, val: bool) {
        if !val {
            if let Some(hotkeys) = &mut self.hotkeys {
                hotkeys.disable();
            }
            self.config.set_enable_hotkeys(val);
            return;
        }
        let mut hotkeys = Hotkeys::new();
        self.hotkeys = match hotkeys.init(&self.config, self.sender.clone()) {
            Ok(_) => Some(hotkeys),
            Err(e) => {
                self.config.set_enable_hotkeys(false);
                error!("{e}");
                return;
            }
        };
        self.config.set_enable_hotkeys(val);
    }
}
