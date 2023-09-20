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

pub struct Player {
    /// Wrapper around sink that plays audio
    sinker: Sinker,
    /// State of the player
    state: PlayState,
    /// Index of the currently playing song
    current: usize,
    /// Volume of the playback
    volume: f32,
    /// When true playback is muted
    mute: bool,
    /// Current playlist
    playlist: Vec<usize>,
    /// True when shuffle should shuffle currently playing song
    shuffle_current: bool,
}

impl Player {
    /// Constructs new Player
    pub fn new(
        sender: UnboundedSender<Msg>,
        library: &Library,
        config: &Config,
    ) -> Self {
        let mut plr = Player::load(config, library, sender);
        if plr.playlist.is_empty() && library.count() > 0 {
            plr.playlist = (0..library.count()).collect();
        }
        plr
    }

    /// Loads player from the json
    pub fn load(
        config: &Config,
        lib: &Library,
        sender: UnboundedSender<Msg>,
    ) -> Self {
        let path = config.get_player_path();

        let data = match fs::read_to_string(path) {
            Err(_) => PlayerLoad::default(),
            Ok(p) => match serde_json::from_str::<PlayerLoad>(&p) {
                Err(_) => PlayerLoad::default(),
                Ok(plr) => plr,
            },
        };

        let mut res = Self {
            sinker: Sinker::new(),
            state: PlayState::Paused,
            current: data.current,
            volume: data.volume,
            mute: data.mute,
            playlist: data.playlist,
            shuffle_current: config.get_shuffle_current(),
        };

        res.init_sinker(lib, sender);
        res
    }

    /// Saves player to the json
    pub fn save(&self, config: &Config) -> Result<()> {
        let data = PlayerSave {
            current: self.current,
            volume: self.volume,
            mute: self.mute,
            playlist: &self.playlist,
        };

        let path = config.get_player_path();
        File::create(&path)?;

        let text = serde_json::to_string::<PlayerSave>(&data)?;
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

        if id != usize::MAX && !self.shuffle_current {
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
            self.current = usize::MAX;
        }
    }

    /// Initializes player sinker
    fn init_sinker(&mut self, lib: &Library, sender: UnboundedSender<Msg>) {
        // Loads default song
        if let Some(id) = self.playlist.get(self.current) {
            if let Err(_) = self.sinker.load(lib, id.to_owned(), false) {
                self.current = usize::MAX;
                self.state = PlayState::Stopped;
            }
        }
        // Sets volume
        if self.mute {
            if let Err(_) = self.sinker.set_volume(0.) {
                self.mute = false;
            }
        } else {
            if let Err(_) = self.sinker.set_volume(self.volume) {
                self.volume = 1.;
            }
        }
        // Sets on song end function
        _ = self.sinker.song_end(move |info| match info {
            CallbackInfo::SourceEnded => {
                _ = sender.send(Msg::Plr(PlayerMsg::SongEnd));
            }
            _ => todo!(),
        });
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

///==========================================
/// Structs for saving and loading the player
///==========================================
#[derive(Deserialize)]
struct PlayerLoad {
    /// Index of the currently playing song
    current: usize,
    /// Volume of the playback
    volume: f32,
    /// When true playback is muted
    mute: bool,
    /// Current playlist
    playlist: Vec<usize>,
}

impl Default for PlayerLoad {
    /// Default values for PlayerLoad
    fn default() -> Self {
        Self {
            current: usize::MAX,
            volume: 1.,
            mute: false,
            playlist: Vec::new(),
        }
    }
}

#[derive(Serialize)]
struct PlayerSave<'a> {
    /// Index of the currently playing song
    current: usize,
    /// Volume of the playback
    volume: f32,
    /// When true playback is muted
    mute: bool,
    /// Current playlist
    playlist: &'a Vec<usize>,
}
