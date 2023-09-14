use eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
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
}

impl Config {
    /// Loads config from config
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
        let mut dir = Config::get_config_dir();
        fs::create_dir_all(&dir)?;

        dir.push("config.json");
        File::create(&dir)?;

        let text = serde_json::to_string_pretty::<Config>(self)?;
        fs::write(dir, text)?;

        Ok(())
    }

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

    /// Gets valid extensions
    pub fn get_extensions(&mut self) -> &Vec<String> {
        &self.extensions
    }

    /// Gets default songs path
    pub fn default_songs_path() -> PathBuf {
        if let Some(dir) = dirs::audio_dir() {
            dir
        } else {
            PathBuf::from(".")
        }
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
}

impl Default for Config {
    /// Sets default values for Config
    fn default() -> Self {
        let mut library_path = Config::get_config_dir();
        library_path.push("library.json");
        let mut gui_path = Config::get_config_dir();
        gui_path.push("gui.json");
        Config {
            paths: vec![Config::default_songs_path()],
            extensions: vec![
                "mp3".to_owned(),
                "flac".to_owned(),
                "m4a".to_owned(),
                "mp4".to_owned(),
            ],
            library_path,
            gui_path,
        }
    }
}
