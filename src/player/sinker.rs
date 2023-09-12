use std::fs::File;

use eyre::{Report, Result};
use raplay::{
    source::{symph::SymphOptions, Symph},
    Sink,
};

use crate::library::library::Library;

/// Implements core player functions
pub struct Sinker {
    sink: Sink,
    symph: SymphOptions,
}

impl Sinker {
    /// Constructs new Sinker
    pub fn new() -> Self {
        Sinker {
            sink: Sink::default(),
            symph: SymphOptions::default(),
        }
    }

    /// Loads given song
    pub fn load(&mut self, library: &Library, index: usize, play: bool) -> Result<()> {
        let song = library.get_songs().get(index);
        if song.is_none() {
            return Err(Report::msg("Song can't be accessed"));
        }
        let file = File::open(library.get_songs()[index].get_path())?;
        let src = Symph::try_new(file, &self.symph)?;
        self.sink.load(src, play)?;
        Ok(())
    }

    /// Sets the play state based on given bool
    pub fn play(&mut self, play: bool) -> Result<()> {
        self.sink.play(play)?;
        Ok(())
    }
}
