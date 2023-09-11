use super::song::Song;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Library {
    /// All songs in library
    songs: Vec<Song>
}

impl Library {
    /// Creates empty library
    pub fn new() -> Self {
        Library {
            songs: Vec::new()
        }
    }

    /// Loads songs from the library (TODO)
    pub fn load() {

    }

    /// Saves songs to the library (TODO)
    pub fn save() {

    }
}
