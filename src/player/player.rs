// Hi Bonny4, I'm using your library
// There's so much, I know! But I'm tired now

use std::{fs::File, path::PathBuf};

use eyre::Result;
use raplay::{
    source::{symph::SymphOptions, Symph},
    Sink,
};

#[derive(PartialEq)]
pub enum PlayState {
    NotPlaying,
    Playing,
    Paused,
}

pub struct Player {
    sink: Sink,
    symph: SymphOptions,
    state: PlayState,
}

impl Player {
    pub fn new() -> Self {
        Player {
            sink: Sink::default(),
            symph: SymphOptions::default(),
            state: PlayState::NotPlaying,
        }
    }

    pub fn load(&mut self, path: &PathBuf, play: bool) -> Result<()> {
        let file = File::open(path)?;
        let src = Symph::try_new(file, &self.symph)?;
        self.sink.load(src, play)?;
        Ok(())
    }

    pub fn is_playing(&mut self) -> bool {
        self.state == PlayState::Playing
    }

    pub fn play(&mut self, play: bool) -> Result<()> {
        self.state = if play {
            PlayState::Playing
        } else {
            PlayState::Paused
        };
        self.sink.play(play)?;
        Ok(())
    }
}
