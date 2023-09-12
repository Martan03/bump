use crate::config::config::Config;
use std::fs::{self, read_dir, File};

use super::song::Song;
use eyre::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Library {
    /// All songs in library
    songs: Vec<Song>,
}

impl Library {
    /// Loads songs from the library
    pub fn load(config: &mut Config) -> Library {
        let path = config.get_library_path();

        match fs::read_to_string(path) {
            Err(_) => Library::default(),
            Ok(l) => match serde_json::from_str::<Library>(&l) {
                Err(_) => Library::default(),
                Ok(lib) => lib,
            },
        }
    }

    /// Saves songs to the library
    pub fn save(&self) -> Result<()> {
        let mut dir = Config::get_config_dir();
        fs::create_dir_all(&dir)?;
        dir.push("library.json");
        File::create(&dir)?;

        let text = serde_json::to_string::<Library>(self)?;
        fs::write(dir, text)?;

        Ok(())
    }

    /// Finds songs from song directories
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

    /// Gets songs from the library
    pub fn get_songs(&self) -> &Vec<Song> {
        &self.songs
    }

    /// Gets number of songs in the library
    pub fn count(&self) -> usize {
        self.songs.len()
    }
}

impl Default for Library {
    /// Creates default Library
    fn default() -> Self {
        Library { songs: Vec::new() }
    }
}
