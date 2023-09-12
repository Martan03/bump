// Hi Bonny4, I'm using your library
// There's so much, I know! But I'm tired now

use std::{fs::File, path::PathBuf};

use eyre::Result;

use crate::library::library::Library;

use super::sinker::Sinker;

#[derive(PartialEq)]
pub enum PlayState {
    NotPlaying,
    Playing,
    Paused,
}

pub struct Player {
    sinker: Sinker,
    state: PlayState,
    current: usize,
}

impl Player {
    /// Constructs new Player
    pub fn new() -> Self {
        Player {
            sinker: Sinker::new(),
            state: PlayState::NotPlaying,
            current: 0,
        }
    }

    /// Loads song from the library
    pub fn load(&mut self, library: &Library, play: bool) -> Result<()> {
        self.set_state(play);
        self.sinker.load(library, self.current, play)?;
        Ok(())
    }

    /// Sets playing state based on the given bool
    pub fn play(&mut self, play: bool) -> Result<()> {
        self.set_state(play);
        self.sinker.play(play)?;
        Ok(())
    }

    /// Plays next song
    pub fn next(&mut self, library: &Library) -> Result<()> {
        let play = self.is_playing();
        self.play_at(library, self.current as i128 + 1, play)?;
        Ok(())
    }

    /// Plays previous song
    pub fn prev(&mut self, library: &Library) -> Result<()> {
        let play = self.is_playing();
        self.play_at(library, self.current as i128 - 1, play)?;
        Ok(())
    }

    pub fn play_at(
        &mut self,
        library: &Library,
        index: i128,
        play: bool
    ) -> Result<()> {
        self.set_current(library, index);
        self.load(library, play)?;
        Ok(())
    }

    pub fn is_playing(&mut self) -> bool {
        self.state == PlayState::Playing
    }

    pub fn set_state(&mut self, play: bool) {
        self.state = if play {
            PlayState::Playing
        } else {
            PlayState::Paused
        }
    }

    pub fn set_current(&mut self, library: &Library, index: i128) {
        let count = library.count() - 1;
        if index < 0 {
            self.current = count;
        } else if index as usize > count {
            self.current = 0;
        } else {
            self.current = index as usize;
        }
    }

    pub fn get_current(&self) -> usize {
        self.current
    }
}
