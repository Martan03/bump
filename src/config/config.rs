use eyre::Result;
use log::error;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
    time::Duration,
};

use crate::gui::app::{BumpApp, ConfMsg};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    /// Paths where songs are saved
    #[serde(default = "Config::get_default_song_paths")]
    paths: Vec<PathBuf>,
    /// Valid file extensions
    #[serde(default = "Config::get_default_extensions")]
    extensions: Vec<String>,
    /// Path to the library
    #[serde(default = "Config::get_default_library_path")]
    library_path: PathBuf,
    /// Path to the gui state json
    #[serde(default = "Config::get_default_gui_path")]
    gui_path: PathBuf,
    /// Path to the player file
    #[serde(default = "Config::get_default_player_path")]
    player_path: PathBuf,
    /// Whether it should use recursive search when finding songs
    #[serde(default = "Config::get_default_recursive_search")]
    recursive_search: bool,
    /// When true shuffles currently playing song as well
    #[serde(default = "Config::get_default_shuffle_current")]
    shuffle_current: bool,
    /// Fade length of the playback when pausing
    #[serde(default = "Config::get_default_fade")]
    fade: Duration,
    /// When true automatically starts playing last played song after start
    #[serde(default = "Config::get_default_autoplay")]
    autoplay: bool,
    /// When true loads new songs on app start
    #[serde(default = "Config::get_default_start_load")]
    start_load: bool,
    /// When true plays songs without the gaps in between
    #[serde(default = "Config::get_default_gapless")]
    gapless: bool,
    /// IP of the server
    #[serde(default = "Config::get_default_server_ip")]
    server_ip: String,
    /// Port of the server
    #[serde(default = "Config::get_default_server_port")]
    server_port: String,
    /// App hotkeys
    #[serde(default = "Config::get_default_hotkeys")]
    hotkeys: HashMap<String, String>,
    enable_hotkeys: bool,
    /// True when anything in config changed, else false
    #[serde(skip, default)]
    changed: bool,
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

    /// Gets all paths songs are saved in
    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    /// Adds given path to paths
    pub fn add_path(&mut self, path: PathBuf) {
        self.changed = true;
        self.paths.push(path);
    }

    /// Removes path on given index
    pub fn remove_path(&mut self, id: usize) {
        if id < self.paths.len() {
            self.changed = true;
            self.paths.remove(id);
        }
    }

    /// Gets library path
    pub fn get_library_path(&self) -> &PathBuf {
        &self.library_path
    }

    /// Gets gui path
    pub fn get_gui_path(&self) -> &PathBuf {
        &self.gui_path
    }

    /// Gets player path
    pub fn get_player_path(&self) -> &PathBuf {
        &self.player_path
    }

    /// Gets valid extensions
    pub fn get_extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    /// Gets config dir path
    pub fn get_config_dir() -> PathBuf {
        if let Some(mut dir) = dirs::config_dir() {
            dir.push("bump");
            dir
        } else {
            PathBuf::from(".")
        }
    }

    /// Gets shuffle current
    pub fn get_shuffle_current(&self) -> bool {
        self.shuffle_current
    }

    /// Sets shuffle current to given value
    pub fn set_shuffle_current(&mut self, val: bool) {
        self.changed = true;
        self.shuffle_current = val;
    }

    /// Gets whether songs finder should use recursive search
    pub fn get_recursive_search(&self) -> bool {
        self.recursive_search
    }

    /// Sets recursive search to given value
    pub fn set_recursive_search(&mut self, val: bool) {
        self.changed = true;
        self.recursive_search = val;
    }

    /// Gets fade length
    pub fn get_fade(&self) -> Duration {
        self.fade
    }

    /// Gets whether song should start playing on start
    pub fn get_autoplay(&self) -> bool {
        self.autoplay
    }

    /// Sets autoplay to given value
    pub fn set_autoplay(&mut self, val: bool) {
        self.changed = true;
        self.autoplay = val;
    }

    /// Gets whether songs should load on start
    pub fn get_start_load(&self) -> bool {
        self.start_load
    }

    /// Sets start load to given value
    pub fn set_start_load(&mut self, val: bool) {
        self.changed = true;
        self.start_load = val;
    }

    /// Gets whether gapless is enabled
    pub fn get_gapless(&self) -> bool {
        self.gapless
    }

    /// Sets gapless to given value
    pub fn set_gapless(&mut self, val: bool) {
        self.changed = true;
        self.gapless = val;
    }

    /// Gets server ip
    pub fn get_server_ip(&self) -> &str {
        &self.server_ip
    }

    /// Gets server port
    pub fn get_server_port(&self) -> &str {
        &self.server_port
    }

    /// Gets server address
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server_ip, self.server_port)
    }

    /// Gets hotkeys
    pub fn get_hotkeys(&self) -> &HashMap<String, String> {
        &self.hotkeys
    }

    /// Gets enable hotkeys
    pub fn get_enable_hotkeys(&self) -> bool {
        self.enable_hotkeys
    }

    pub fn set_enable_hotkeys(&mut self, val: bool) {
        self.changed = true;
        self.enable_hotkeys = val;
    }

    ///>===================================================================<///
    ///                        Default Config values                        ///
    ///>===================================================================<///

    /// Gets default songs path
    fn get_default_song_paths() -> Vec<PathBuf> {
        if let Some(dir) = dirs::audio_dir() {
            vec![dir]
        } else {
            vec![PathBuf::from(".")]
        }
    }

    /// Gets default path to library
    fn get_default_library_path() -> PathBuf {
        Config::get_config_dir().join("library.json")
    }

    /// Gets default path to gui state file
    fn get_default_gui_path() -> PathBuf {
        Config::get_config_dir().join("gui.json")
    }

    /// Gets default path to player state file
    fn get_default_player_path() -> PathBuf {
        Config::get_config_dir().join("player.json")
    }

    /// Gets default extensions list
    fn get_default_extensions() -> Vec<String> {
        vec![
            "mp3".to_owned(),
            "flac".to_owned(),
            "m4a".to_owned(),
            "mp4".to_owned(),
        ]
    }

    /// Gets default recursive search
    fn get_default_recursive_search() -> bool {
        true
    }

    /// Gets default shuffle current
    fn get_default_shuffle_current() -> bool {
        false
    }

    /// Gets default fade length
    fn get_default_fade() -> Duration {
        Duration::from_millis(150)
    }

    /// Gets default autoplay value
    fn get_default_autoplay() -> bool {
        false
    }

    /// Gets default start load
    fn get_default_start_load() -> bool {
        true
    }

    /// Gets defautl gapless
    fn get_default_gapless() -> bool {
        false
    }

    /// Gets default server ip
    fn get_default_server_ip() -> String {
        "127.0.0.1".to_owned()
    }

    /// Gets default server port
    fn get_default_server_port() -> String {
        "2867".to_owned()
    }

    /// Gets default hotkeys
    fn get_default_hotkeys() -> HashMap<String, String> {
        let mut hotkeys = HashMap::new();
        hotkeys.insert("ctrl+alt+home".to_owned(), "pp".to_owned());
        hotkeys.insert("ctrl+alt+pg_up".to_owned(), "prev".to_owned());
        hotkeys.insert("ctrl+alt+pg_down".to_owned(), "next".to_owned());
        hotkeys.insert("ctrl+alt+up".to_owned(), "vu".to_owned());
        hotkeys.insert("ctrl+alt+down".to_owned(), "vd".to_owned());
        hotkeys
    }

    /// Gets default enable hotkeys
    fn get_default_enable_hotkeys() -> bool {
        true
    }
}

/// Implements default for Config
impl Default for Config {
    fn default() -> Self {
        let mut library_path = Config::get_config_dir();
        let mut gui_path = library_path.clone();
        let mut player_path = library_path.clone();
        library_path.push("library.json");
        gui_path.push("gui.json");
        player_path.push("player.json");

        Config {
            paths: Config::get_default_song_paths(),
            extensions: Config::get_default_extensions(),
            library_path: Config::get_default_library_path(),
            gui_path: Config::get_default_gui_path(),
            player_path: Config::get_default_player_path(),
            recursive_search: Config::get_default_recursive_search(),
            shuffle_current: Config::get_default_shuffle_current(),
            fade: Config::get_default_fade(),
            autoplay: Config::get_default_autoplay(),
            start_load: Config::get_default_start_load(),
            gapless: Config::get_default_gapless(),
            changed: true,
            server_ip: Config::get_default_server_ip(),
            server_port: Config::get_default_server_port(),
            hotkeys: Config::get_default_hotkeys(),
            enable_hotkeys: Config::get_default_enable_hotkeys(),
        }
    }
}

///>=======================================================================<///
///                         Config message handling                         ///
///>=======================================================================<///
impl BumpApp {
    pub fn conf_update(&mut self, msg: ConfMsg) {
        match msg {
            ConfMsg::RecursiveSearch(val) => {
                self.config.set_recursive_search(val)
            }
            ConfMsg::ShuffleCurrent(val) => {
                self.config.set_shuffle_current(val)
            }
            ConfMsg::Autoplay(val) => self.config.set_autoplay(val),
            ConfMsg::StartLoad(val) => self.config.set_start_load(val),
            ConfMsg::Gapless(val) => self.config.set_gapless(val),
            ConfMsg::RemPath(id) => self.config.remove_path(id),
            ConfMsg::AddPath(path) => self.config.add_path(path),
            ConfMsg::EnableHotkeys(mut val) => {
                let hotkeys = if let Some(hotkeys) = &mut self.hotkeys {
                    hotkeys
                } else {
                    error!("Failed to set enable hotkeys to: {val}");
                    return;
                };
                if val {
                    if let Err(_) =
                        hotkeys.init(&self.config, self.sender.clone())
                    {
                        error!("Failed to initialize hotkeys");
                        val = false;
                    }
                } else {
                    hotkeys.disable();
                }
                self.config.set_enable_hotkeys(val);
            }
        }
    }
}
