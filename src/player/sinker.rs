use std::{fs::File, time::Duration};

use eyre::{Report, Result};
use raplay::{
    source::{symph::SymphOptions, Symph},
    CallbackInfo, Sink, Timestamp,
};

use crate::library::library::Library;

/// Implements core player functions
pub struct Sinker {
    sink: Sink,
    symph: SymphOptions,
}

impl Sinker {
    /// Creates new sinker
    pub fn new() -> Self {
        Self {
            sink: Sink::default(),
            symph: SymphOptions::default(),
        }
    }

    /// Loads given song
    pub fn load(
        &mut self,
        library: &Library,
        index: usize,
        play: bool,
    ) -> Result<()> {
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

    /// Creates hard pause - completely paused audio
    pub fn hard_pause(&mut self) -> Result<()> {
        self.sink.hard_pause()?;
        Ok(())
    }

    /// Calls function on song end
    pub fn song_end<F>(&mut self, f: F) -> Result<()>
    where
        F: Send + 'static + FnMut(CallbackInfo),
    {
        self.sink.on_callback(Some(f))?;
        Ok(())
    }

    /// Sets the playback volume
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        self.sink.volume(volume * volume)?;
        Ok(())
    }

    /// Gets timestamp of currently playing song
    pub fn get_timestamp(&self) -> Result<Timestamp> {
        match self.sink.get_timestamp() {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()),
        }
    }

    /// Seeks to given position
    pub fn seek_to(&mut self, time: Duration) -> Result<()> {
        self.sink.seek_to(time)?;
        Ok(())
    }

    /// Sets fade length
    pub fn set_fade(&mut self, fade: Duration) -> Result<()> {
        self.sink.set_fade_len(fade)?;
        Ok(())
    }

    /// Sets gapless to given value
    pub fn set_gapless(&mut self, enable: bool) {
        self.symph.format.enable_gapless = enable;
    }
}
