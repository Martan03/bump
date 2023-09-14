use std::time::Duration;

use eyre::Result;
use rand::seq::SliceRandom;
use raplay::sink::CallbackInfo;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    gui::app::{Msg, PlayerMsg},
    library::{library::Library, song::Song},
};

use super::sinker::Sinker;

#[derive(PartialEq)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
}

pub struct Player {
    sinker: Sinker,
    state: PlayState,
    current: usize,
    volume: f32,
    mute: bool,
    playlist: Vec<usize>,
}

impl Player {
    /// Constructs new Player
    pub fn new(sender: UnboundedSender<Msg>, library: &Library) -> Self {
        let mut sinker = Sinker::new();
        _ = sinker.song_end(move |info| match info {
            CallbackInfo::SourceEnded => {
                _ = sender.send(Msg::Plr(PlayerMsg::SongEnd));
            }
            _ => todo!(),
        });
        Player {
            sinker,
            state: PlayState::Stopped,
            current: usize::MAX,
            volume: 1.,
            mute: false,
            playlist: (0..library.count()).collect(),
        }
    }

    /// Loads song from the library
    pub fn load(&mut self, library: &Library, play: bool) -> Result<()> {
        self.set_state(play);
        self.sinker
            .load(library, self.playlist[self.current], play)?;
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
        self.set_current(index);
        self.load(library, play)?;
        Ok(())
    }

    pub fn shuffle(&mut self) {
        let id = self.playlist[self.current];
        let mut rng = rand::thread_rng();
        self.playlist.shuffle(&mut rng);

        self.find_current(id)
    }

    pub fn is_playing(&self) -> bool {
        self.state == PlayState::Playing
    }

    pub fn _is_stopped(&self) -> bool {
        self.state == PlayState::Stopped
    }

    pub fn set_state(&mut self, play: bool) {
        self.state = if play {
            PlayState::Playing
        } else {
            PlayState::Paused
        }
    }

    pub fn set_current(&mut self, index: i128) {
        let count = self.playlist.len() - 1;
        if index < 0 {
            self.current = count;
        } else if index as usize > count {
            self.current = 0;
        } else {
            self.current = index as usize;
        }
    }

    pub fn get_current(&self) -> Option<&usize> {
        if self.state == PlayState::Stopped {
            None
        } else {
            self.playlist.get(self.current)
        }
    }

    /// Gets currently playing song
    pub fn get_current_song(&self, library: &Library) -> Song {
        match self.playlist.get(self.current) {
            Some(index) if self.state != PlayState::Stopped => {
                library.get_song(index.to_owned())
            }
            _ => Song::default(),
        }
    }

    /// Gets current volume of the playback
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Sets playback volume
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        match self.sinker.set_volume(volume) {
            Ok(_) => {
                self.volume = volume;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Gets whether playback is muted
    pub fn get_mute(&self) -> bool {
        self.mute
    }

    /// Sets mute
    pub fn set_mute(&mut self, mute: bool) -> Result<()> {
        let volume = if mute { 0. } else { self.volume };
        match self.sinker.set_volume(volume) {
            Ok(_) => {
                self.mute = mute;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Gets currently playing song timestamp
    pub fn get_timestamp(&self) -> (Duration, Duration) {
        match self.sinker.get_timestamp() {
            Ok((t, l)) => (t, l),
            Err(_) => {
                (Duration::from_secs_f32(0.), Duration::from_secs_f32(0.))
            }
        }
    }

    /// Seeks to given position
    pub fn seek_to(&mut self, time: Duration) -> Result<()> {
        self.sinker.seek_to(time)
    }

    /// Handles player messages
    pub fn handle_msg(&mut self, msg: PlayerMsg, library: &Library) {
        match msg {
            PlayerMsg::Play(play) => {
                _ = self.play(play.unwrap_or(!self.is_playing()));
            }
            PlayerMsg::PlaySong(id) => {
                self.create_playlist(library, id);
                _ = self.play_at(library, self.current as i128, true)
            }
            PlayerMsg::Next => _ = self.next(library),
            PlayerMsg::Prev => _ = self.prev(library),
            PlayerMsg::SeekTo(secs) => {
                _ = self.seek_to(Duration::from_secs_f32(secs));
            }
            PlayerMsg::SongEnd => _ = self.next(library),
            PlayerMsg::Volume(vol) => _ = self.set_volume(vol),
            PlayerMsg::Mute(mute) => {
                _ = self.set_mute(mute.unwrap_or(!self.get_mute()))
            }
            PlayerMsg::Shuffle => self.shuffle(),
        }
    }

    fn create_playlist(&mut self, library: &Library, id: usize) {
        self.playlist = (0..library.count()).collect();

        self.find_current(id);
    }

    fn find_current(&mut self, id: usize) {
        if let Some(index) = self.playlist.iter().position(|&x| x == id) {
            self.current = index;
        } else {
            self.current = 0;
        }
    }
}
