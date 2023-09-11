use std::{path::PathBuf, time::Duration};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    /// Path to the song
    path: PathBuf,
    ///  Name of the song
    name: String,
    /// Song artist
    artist: String,
    /// Song length
    length: Duration
}

impl Song {
    /// Gets song path
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets song name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets song artist
    pub fn get_artist(&self) -> &str {
        &self.artist
    }

    /// Gets song length
    pub fn get_length(&self) -> &Duration {
        &self.length
    }
}
