use crate::{
    config::config::Config,
    gui::app::{LibMsg, Msg},
};
use std::{
    fs::{self, read_dir, File},
    thread::{self, JoinHandle},
};

use super::song::Song;
use eyre::Result;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Serialize, Deserialize)]
pub struct Library {
    /// All songs in library
    songs: Vec<Song>,
    #[serde(skip)]
    load_process: Option<JoinHandle<Library>>,
}

impl Library {
    /// Loads songs from the library
    pub fn load(config: &Config) -> Library {
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
    pub fn save(&self, config: &Config) -> Result<()> {
        let path = config.get_library_path();
        File::create(&path)?;

        let text = serde_json::to_string::<Library>(self)?;
        fs::write(path, text)?;

        Ok(())
    }

    /// Finds songs from song directories
    pub fn find(&mut self, config: &Config) {
        let mut paths = config.get_paths().clone();
        let mut i = 0;

        for s in &mut self.songs {
            s.set_deleted(true);
        }

        while i < paths.len() {
            let dir = &paths[i];
            i += 1;
            let dir = match read_dir(dir) {
                Ok(dir) => dir,
                Err(_) => continue,
            };

            for f in dir {
                let f = match f {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let path = f.path();

                if path.is_dir() {
                    if config.get_recursive_search() {
                        paths.push(path.clone());
                    }
                    continue;
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
                        for s in &mut self.songs {
                            if s.get_path() == song.get_path() {
                                s.set_deleted(false);
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

    /// Gets song on given ID
    pub fn get_song(&self, id: usize) -> Song {
        if let Some(song) = self.songs.get(id) {
            song.clone()
        } else {
            Song::default()
        }
    }

    /// Gets number of songs in the library
    pub fn count(&self) -> usize {
        self.songs.len()
    }

    /// Starts finding new songs
    pub fn start_find(&mut self, conf: &Config, sender: UnboundedSender<Msg>) {
        let mut lib = self.clone();
        let config = conf.clone();
        let load = thread::spawn(move || {
            lib.find(&config);

            _ = sender.send(Msg::Lib(LibMsg::LoadEnded));

            lib
        });
        self.load_process = Some(load);
    }

    /// Ends finding new songs
    pub fn end_find(&mut self) {
        if let Some(process) = self.load_process.take() {
            self.songs = process.join().unwrap().songs;
        }
    }

    pub fn handle_msg(
        &mut self,
        config: &Config,
        sender: UnboundedSender<Msg>,
        msg: LibMsg,
    ) {
        match msg {
            LibMsg::LoadStart => self.start_find(config, sender),
            LibMsg::LoadEnded => self.end_find(),
        }
    }
}

/// Implements default for Library
impl Default for Library {
    fn default() -> Self {
        Library {
            songs: Vec::new(),
            load_process: None,
        }
    }
}

impl Clone for Library {
    fn clone(&self) -> Self {
        Self {
            songs: self.songs.clone(),
            load_process: None,
        }
    }
}
