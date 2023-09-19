use std::{
    fs::{self, File},
    time::Duration,
};

use eyre::Result;
use rand::seq::SliceRandom;
use raplay::sink::CallbackInfo;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    config::config::Config,
    gui::app::{Msg, PlayerMsg},
    library::{library::Library, song::Song},
};

use super::sinker::Sinker;

#[derive(Debug, PartialEq)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    #[serde(skip, default = "default_sinker")]
    sinker: Sinker,
    #[serde(skip, default = "default_state")]
    state: PlayState,
    current: usize,
    volume: f32,
    mute: bool,
    playlist: Vec<usize>,
    #[serde(skip, default = "default_shuffle_current")]
    shuffle_current: bool,
}

impl Player {
    /// Constructs new Player
    pub fn new(
        sender: UnboundedSender<Msg>,
        library: &Library,
        config: &Config,
    ) -> Self {
        let mut plr = Player::load(config, library);
        if plr.playlist.is_empty() && library.count() > 0 {
            plr.playlist = (0..library.count()).collect();
        }
        plr.shuffle_current = config.get_shuffle_current();
        _ = plr.sinker.song_end(move |info| match info {
            CallbackInfo::SourceEnded => {
                _ = sender.send(Msg::Plr(PlayerMsg::SongEnd));
            }
            _ => todo!(),
        });
        plr
    }

    /// Loads player from the json
    pub fn load(config: &Config, library: &Library) -> Self {
        /// Clears current and sets state to stopped
        fn clear_current(mut plr: Player) -> Player {
            plr.current = usize::MAX;
            plr.state = PlayState::Stopped;
            plr
        }

        let path = config.get_player_path();

        match fs::read_to_string(path) {
            Err(_) => Player::default(),
            Ok(p) => match serde_json::from_str::<Player>(&p) {
                Err(_) => Player::default(),
                Ok(mut plr) => {
                    match plr.sinker.set_volume(plr.volume) {
                        Err(_) => plr.volume = 1.0,
                        _ => {}
                    }
                    if let Some(id) = plr.playlist.get(plr.current) {
                        match plr.sinker.load(library, id.to_owned(), false) {
                            Ok(_) => plr,
                            Err(_) => clear_current(plr),
                        }
                    } else {
                        clear_current(plr)
                    }
                }
            },
        }
    }

    /// Saves player to the json
    pub fn save(&self, config: &Config) -> Result<()> {
        let path = config.get_player_path();
        File::create(&path)?;

        let text = serde_json::to_string::<Player>(self)?;
        fs::write(path, text)?;

        Ok(())
    }

    /// Loads song from the library
    pub fn load_song(&mut self, library: &Library, play: bool) -> Result<()> {
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
    pub fn next(&mut self, library: &mut Library) -> Result<()> {
        let play = self.is_playing();
        self.play_at(library, self.current as i128 + 1, play)?;
        Ok(())
    }

    /// Plays previous song
    pub fn prev(&mut self, library: &mut Library) -> Result<()> {
        let play = self.is_playing();
        self.play_at(library, self.current as i128 - 1, play)?;
        Ok(())
    }

    pub fn play_at(
        &mut self,
        library: &mut Library,
        index: i128,
        play: bool,
    ) -> Result<()> {
        self.set_current(index);
        self.load_song(library, play)?;
        match self.sinker.get_timestamp() {
            Ok((_, l)) => {
                library.set_song_length(self.playlist[self.current], l)
            }
            Err(_) => {}
        }
        Ok(())
    }

    pub fn shuffle(&mut self) {
        let id = if let Some(i) = self.get_current() {
            i.to_owned()
        } else {
            usize::MAX
        };

        if id != usize::MAX && self.shuffle_current {
            self.playlist.remove(self.current);
        }

        let mut rng = rand::thread_rng();
        self.playlist.shuffle(&mut rng);

        if id != usize::MAX && self.shuffle_current {
            self.playlist.insert(0, id);
            self.set_current(0);
        } else {
            self.find_current(id);
        }
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

    pub fn get_playlist(&self) -> &Vec<usize> {
        &self.playlist
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
    pub fn handle_msg(&mut self, msg: PlayerMsg, library: &mut Library) {
        match msg {
            PlayerMsg::Play(play) => {
                if self.state != PlayState::Stopped {
                    _ = self.play(play.unwrap_or(!self.is_playing()));
                }
            }
            PlayerMsg::PlaySong(id, new) => {
                if new {
                    self.create_playlist(library, id);
                } else {
                    self.find_current(id)
                }

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

impl Default for Player {
    fn default() -> Self {
        Self {
            sinker: Sinker::new(),
            state: PlayState::Stopped,
            current: usize::MAX,
            volume: 1.,
            mute: false,
            playlist: Vec::new(),
            shuffle_current: true,
        }
    }
}

fn default_sinker() -> Sinker {
    Sinker::new()
}

fn default_state() -> PlayState {
    PlayState::Paused
}

fn default_shuffle_current() -> bool {
    true
}
