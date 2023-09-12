use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Paths where songs are saved
    paths: Vec<PathBuf>,
    extensions: Vec<String>,
}

impl Config {
    /// Loads config from config
    pub fn load() -> Self {
        Config {
            paths: vec![Config::default_songs_path()],
            extensions: vec![
                "mp3".to_owned(),
                "flac".to_owned(),
                "m4a".to_owned(),
                "mp4".to_owned()
            ],
        }
    }

    /// Gets all paths songs are saved in
    pub fn get_paths(&mut self) -> &Vec<PathBuf> {
        &self.paths
    }

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
        if let Some(dir) = dirs::config_dir() {
            dir
        } else {
            PathBuf::from(".")
        }
    }
}
