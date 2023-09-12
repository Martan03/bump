use crate::config::config::Config;
use std::fs::read_dir;

use super::song::Song;
use eyre::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Library {
    /// All songs in library
    songs: Vec<Song>,
}

impl Library {
    /// Creates empty library
    pub fn new() -> Self {
        Library { songs: Vec::new() }
    }

    pub fn find(&mut self, config: &mut Config) {
        let mut paths = config.get_paths().clone();
        let mut i = 0;

        while i < paths.len() {
            let dir = &paths[i];
            i += 1;
            let dir = match read_dir(dir) {
                Ok(dir) => dir,
                Err(_) => continue,
            };

            for f in dir {
                let f = f.expect("Failed to read file");
                let path = f.path();

                if path.is_dir() {
                    paths.push(path.clone());
                }

                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy();
                    if !config
                        .get_extensions()
                        .iter()
                        .any(|e| e == ext.as_ref())
                    {
                        continue;
                    }
                    if let Ok(song) = Song::load(&path) {
                        let mut exist = false;
                        for s in self.get_songs() {
                            if s.get_path() == song.get_path() {
                                exist = true;
                                break;
                            }
                        }
                        if !exist {
                            self.songs.push(song);
                        }
                    }
                }
            }
        }
    }

    /// Loads songs from the library (TODO)
    /// pub fn load() {}

    /// Saves songs to the library (TODO)
    /*
    pub fn save(&self) -> Result<()> {
        let dir = Config::get_config_dir();
        let path = std::path::Path::new(&filename);
        let prefix = path
        .parent()
        .ok_or(Report::msg("Error creating stats directory"))?;
    std::fs::create_dir_all(&prefix)?;
    std::fs::File::create(&path)?;
    
    let text = serde_json::to_string_pretty::<Stats>(self)?;
    std::fs::write(&path, text)?;
    
    Ok(())
}
*/


    pub fn get_songs(&self) -> &Vec<Song> {
        &self.songs
    }

    pub fn count(&self) -> usize {
        self.songs.len()
    }
}
