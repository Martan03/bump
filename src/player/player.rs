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

/// State of the Player
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

        let state = if config.get_autoplay() {
            PlayState::Playing
        } else {
            PlayState::Paused
        };
        let mut res = Self {
            sinker: Sinker::new(),
            state,
            current: data.current,
            volume: data.volume,
            mute: data.mute,
            playlist: data.playlist,
            shuffle_current: config.get_shuffle_current(),
        };

        res.init_sinker(lib, config, sender);
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

    /// Sets playing state based on the given bool
    pub fn play(&mut self, play: bool) -> Result<()> {
        self.set_state(play);
        self.sinker.play(play)?;
        Ok(())
    }

    /// Plays next song
    pub fn next(&mut self, lib: &mut Library) -> Result<()> {
        self.play_at(lib, self.current as i128 + 1, self.is_playing())?;
        Ok(())
    }

    /// Plays previous song
    pub fn prev(&mut self, lib: &mut Library) -> Result<()> {
        self.play_at(lib, self.current as i128 - 1, self.is_playing())?;
        Ok(())
    }

    /// Plays song on given index
    pub fn play_at(
        &mut self,
        lib: &mut Library,
        index: i128,
        play: bool,
    ) -> Result<()> {
        self.load_song(lib, play)?;
        self.set_current(index);
        if let Ok((_, l)) = self.sinker.get_timestamp() {
            lib.set_song_length(self.playlist[self.current], l);
        }
        Ok(())
    }

    /// Loads song from the library
    fn load_song(&mut self, lib: &Library, play: bool) -> Result<()> {
        self.set_state(play);
        if self.current != usize::MAX {
            self.sinker.load(lib, self.playlist[self.current], play)?;
        }
        Ok(())
    }

    /// Shuffles current playlist
    pub fn shuffle(&mut self) {
        if let Some(id) = self.get_current() {
            let mut rng = rand::thread_rng();
            if self.shuffle_current {
                self.playlist.shuffle(&mut rng);
                self.find_current(id);
            } else {
                self.playlist.swap(self.current, 0);
                self.playlist[1..].shuffle(&mut rng);
                self.set_current(0);
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

    //>=====================================================================<//
    //                           Getters & Setters                           //
    //>=====================================================================<//

    /// Checks if playback is playing
    pub fn is_playing(&self) -> bool {
        self.state == PlayState::Playing
    }

    /// Checks if playback is stopped
    pub fn _is_stopped(&self) -> bool {
        self.state == PlayState::Stopped
    }

    /// Sets state based on boolean
    pub fn set_state(&mut self, play: bool) {
        self.state = if play {
            PlayState::Playing
        } else {
            PlayState::Paused
        }
    }

    /// Gets current as option, returns None when playback stopped
    pub fn get_current(&self) -> Option<usize> {
        if self.state == PlayState::Stopped {
            None
        } else {
            self.playlist.get(self.current).map(|c| *c)
        }
    }

    /// Sets current with overflow check
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

    /// Gets currently playing song
    pub fn get_current_song(&self, lib: &Library) -> Song {
        match self.playlist.get(self.current) {
            Some(index) if self.state != PlayState::Stopped => {
                lib.get_song(index.to_owned())
            }
            _ => Song::default(),
        }
    }

    /// Gets playlist
    pub fn get_playlist(&self) -> &Vec<usize> {
        &self.playlist
    }

    /// Gets current volume of the playback
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Sets playback volume
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        self.sinker.set_volume(volume)?;
        self.volume = volume;
        Ok(())
    }

    /// Gets whether playback is muted
    pub fn get_mute(&self) -> bool {
        self.mute
    }

    /// Sets mute
    pub fn set_mute(&mut self, mute: bool) -> Result<()> {
        let volume = if mute { 0. } else { self.volume };
        self.sinker.set_volume(volume)?;
        self.mute = mute;
        Ok(())
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

    //>=====================================================================<//
    //                           Private functions                           //
    //>=====================================================================<//

    /// Creates playlist from library
    fn create_playlist(&mut self, library: &Library, id: usize) {
        self.playlist = (0..library.count()).collect();
        self.find_current(id);
    }

    /// Finds current
    fn find_current(&mut self, id: usize) {
        if let Some(index) = self.playlist.iter().position(|&x| x == id) {
            self.current = index;
        } else {
            self.current = usize::MAX;
        }
    }

    /// Initializes player sinker
    fn init_sinker(
        &mut self,
        lib: &Library,
        conf: &Config,
        sender: UnboundedSender<Msg>,
    ) {
        // Loads default song
        if let Err(_) = self.load_song(lib, conf.get_autoplay()) {
            self.current = usize::MAX;
            self.state = PlayState::Stopped;
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
        // Sets fade length
        _ = self.sinker.set_fade(conf.get_fade());
        // Sets gapless playing
        _ = self.sinker.set_gapless(conf.get_gapless());
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

//>=========================================================================<//
//                 Structs for saving and loading the player                 //
//>=========================================================================<//
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
