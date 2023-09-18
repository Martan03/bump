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
    /// Song album
    album: String,
    /// Song release year
    year: i32,
    /// Song length
    length: Duration,
    /// Song genre
    genre: String,
}

impl Song {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let tag = Tag::new().read_from_path(path)?;

        Ok(Song {
            path: path.to_path_buf(),
            name: tag.title().unwrap_or("-").to_owned(),
            artist: tag.artist().unwrap_or("-").to_owned(),
            album: tag.album_title().unwrap_or("-").to_owned(),
            year: tag.year().unwrap_or(i32::MAX),
            length: Duration::from_secs_f64(tag.duration().unwrap_or(0.0)),
            genre: tag.genre().unwrap_or("-").to_owned(),
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

    /// Gets album of the song
    pub fn get_album(&self) -> &str {
        &self.album
    }

    /// Gets year the song was released in
    pub fn get_year(&self) -> i32 {
        self.year
    }

    /// Gets year string the song was released in, if no year returns '-'
    pub fn get_year_str(&self) -> String {
        if self.get_year() == i32::MAX {
            "-".to_owned()
        } else {
            self.get_year().to_string()
        }
    }

    /// Gets song length
    pub fn get_length(&self) -> &Duration {
        &self.length
    }

    /// Gets song length as string, when no length returns '--:--'
    pub fn get_length_str(&self) -> String {
        let mut total_secs = self.get_length().as_secs();

        let days = total_secs / 86400;
        total_secs %= 86400;
        let hours = total_secs / 3600;
        total_secs %= 3600;
        let mins = total_secs / 60;
        let secs = total_secs % 60;

        if days > 0 {
            format!("{}d:{:02}:{:02}:{:02}", days, hours, mins, secs)
        } else if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, mins, secs)
        } else if total_secs > 0 {
            format!("{:02}:{:02}", mins, secs)
        } else {
            "--:--".to_owned()
        }
    }

    /// Set songs length to given length
    pub fn set_length(&mut self, len: Duration) {
        self.length = len;
    }

    pub fn get_genre(&self) -> &str {
        &self.genre
    }
}

impl Default for Song {
    fn default() -> Self {
        Self {
            path: Default::default(),
            name: "Not playing".to_owned(),
            artist: Default::default(),
            album: Default::default(),
            year: Default::default(),
            length: Default::default(),
            genre: Default::default(),
        }
    }
}
