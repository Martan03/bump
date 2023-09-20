use eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
    time::Duration,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Paths where songs are saved
    paths: Vec<PathBuf>,
    /// Valid file extensions
    extensions: Vec<String>,
    /// Path to the library
    library_path: PathBuf,
    /// Path to the gui state json
    gui_path: PathBuf,
    /// Path to the player file
    player_path: PathBuf,
    /// Whether it should use recursive search when finding songs
    recursive_search: bool,
    /// When true shuffles currently playing song as well
    shuffle_current: bool,
    /// Fade length of the playback when pausing    
    fade: Duration,
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
    pub fn save(&self) -> Result<()> {
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

        Ok(())
    }

    ///==================
    /// Getters & Setters
    ///==================

    /// Gets all paths songs are saved in
    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.paths
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
    pub fn get_extensions(&mut self) -> &Vec<String> {
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

    /// Gets whether songs finder should use recursive search
    pub fn get_recursive_search(&self) -> bool {
        self.recursive_search
    }

    /// Gets fade length
    pub fn get_fade(&self) -> Duration {
        self.fade
    }

    ///======================
    /// Default config values
    ///======================

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
}

impl Default for Config {
    /// Sets default values for Config
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
            changed: false,
        }
    }
}
