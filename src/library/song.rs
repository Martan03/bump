use audiotags::Tag;
use eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    /// Path to the song
    path: PathBuf,
    ///  Name of the song
    name: String,
    /// Song artist
    artist: String,
    /// Song length
    length: Duration,
}

impl Song {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let tag = Tag::new().read_from_path(path)?;
        Ok(Song {
            path: path.to_path_buf(),
            name: tag.title().unwrap_or("-").to_owned(),
            artist: tag.artist().unwrap_or("-").to_owned(),
            length: Duration::from_secs_f64(tag.duration().unwrap_or(0.0)),
        })
    }

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
    pub fn _get_length(&self) -> &Duration {
        &self.length
    }
}
