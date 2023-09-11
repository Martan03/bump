// Hi Bonny4, I'm using your library
// There's so much, I know! But I'm tired now

use std::fs::File;

use eyre::Result;
use raplay::{Sink, source::{Symph, symph::SymphOptions}};

pub enum PlayState {
    NotPlaying,
    Playing,
    Paused
}   

pub struct Player {
    sink: Sink,
    symph: SymphOptions,
    state: PlayState
}

impl Player {
    pub fn new() -> Self {
        Player { 
            sink: Sink::default(),
            symph: SymphOptions::default(),
            state: PlayState::NotPlaying
        }
    }

    pub fn load(&mut self, path: String) -> Result<()> {
        let file = File::open(path)?;
        let src = Symph::try_new(file, &self.symph)?;
        self.sink.load(src, true)?;
        Ok(())
    }

    pub fn togglePlay(&mut self) -> Result<()> {
        match self.state {
            PlayState::Playing => {
                self.state = PlayState::Paused;
                self.sink.pause()?;
            }
            _ => {
                self.state = PlayState::Playing;
                self.sink.resume()?;
            }
        }
        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        print!("test");
        self.load("/home/martan03/Music/Imagine Dragons - Mercury - Act 1/".to_owned())?;
        self.togglePlay()?;
        Ok(())
    }
}

