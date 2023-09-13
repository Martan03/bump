use eyre::Result;
use raplay::sink::CallbackInfo;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    gui::app::BumpMessage,
    library::{library::Library, song::Song},
};

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
    pub fn new(sender: UnboundedSender<BumpMessage>) -> Self {
        let mut sinker = Sinker::new();
        _ = sinker.song_end(move |info| match info {
            CallbackInfo::SourceEnded => {
                _ = sender.send(BumpMessage::SongEnd);
            }
            _ => todo!(),
        });
        Player {
            sinker,
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
        play: bool,
    ) -> Result<()> {
        self.set_current(library, index);
        self.load(library, play)?;
        Ok(())
    }

    pub fn is_playing(&self) -> bool {
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

    pub fn get_current_song(&self, library: &Library) -> Song {
        if self.state == PlayState::NotPlaying {
            return Song::default()
        }
        library.get_song(self.current)
    }
}
