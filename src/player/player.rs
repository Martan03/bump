use std::{
    fs::{self, File},
    time::Duration,
};

use eyre::Result;
use log::error;
use place_macro::place;
use rand::seq::SliceRandom;
use raplay::{CallbackInfo, Timestamp};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    config::Config,
    generate_struct,
    gui::app::{BumpApp, Msg},
    library::{Library, Song},
};

use super::{sinker::Sinker, PlayerMsg};

/// State of the Player
#[derive(Debug, PartialEq)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
}

generate_struct! {
    pub Player {
        playlist: Vec<usize>,
        ;
        current: Option<usize>,
        volume: f32,
        mute: bool,
        ;
        sinker: Sinker,
        state: PlayState,
        shuffle_current: bool,
        volume_step: f32,
    }
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

        let data = match serde_json::from_str::<PlayerLoad>(
            &fs::read_to_string(path).unwrap_or("".to_owned()),
        ) {
            Ok(data) => data,
            Err(_) => PlayerLoad::default(),
        };

        let mut res = Self {
            sinker: Sinker::new(),
            state: PlayState::Stopped,
            current: data.current,
            volume: data.volume,
            mute: data.mute,
            playlist: data.playlist,
            shuffle_current: config.get_shuffle_current(),
            volume_step: 0.1,
            changed: true,
        };
        res.load_config(config);
        res.set_state(res.shuffle_current);
        res.init_sinker(lib, config, sender);
        res
    }

    /// Loads player config from config file
    pub fn load_config(&mut self, config: &Config) {
        self.shuffle_current = config.get_shuffle_current();
        self.volume_step = config.get_volume_step();
    }

    /// Saves player to the json
    pub fn save(&mut self, config: &Config) -> Result<()> {
        if !self.changed {
            return Ok(());
        }

        let data = PlayerSave {
            current: self.current,
            volume: self.volume,
            mute: self.mute,
            playlist: &self.playlist,
        };

        let path = config.get_player_path();
        File::create(&path)?;

        fs::write(path, serde_json::to_string::<PlayerSave>(&data)?)?;

        self.changed = false;

        Ok(())
    }

    /// Sets playing state based on the given bool
    pub fn play(&mut self, play: bool) {
        match self.sinker.play(play) {
            Ok(_) => self.set_state(play),
            Err(e) => error!("Failed to play/pause: {e}"),
        }
    }

    /// Sets playing state based on given option
    pub fn play_pause(&mut self, play: Option<bool>) {
        if self.is_stopped() {
            return;
        }
        let play = play.unwrap_or(!self.is_playing());
        self.play(play);
    }

    /// Stops the playback
    pub fn stop(&mut self) {
        _ = self.sinker.play(false);
        self.state = PlayState::Stopped;
        self.set_current(None);
    }

    /// Hard pauses the player
    pub fn hard_pause(&mut self) {
        if let Err(e) = self.sinker.hard_pause() {
            error!("Failed to hard pause: {e}");
        }
    }

    /// Plays next song
    pub fn next(&mut self, num: Option<usize>, lib: &Library) {
        let num = num.unwrap_or(1);
        if let Some(current) = self.get_current() {
            self.play_at(
                lib,
                (current + num) % self.get_playlist().len(),
                self.is_playing(),
            );
        }
    }

    /// Plays previous song
    pub fn prev(&mut self, num: Option<usize>, lib: &Library) {
        let num = num.unwrap_or(1);
        if let Some(current) = self.get_current() {
            self.play_at(
                lib,
                current.checked_sub(num).unwrap_or(self.playlist.len() - 1),
                self.is_playing(),
            );
        }
    }

    /// Plays song on given index
    pub fn play_at(&mut self, lib: &Library, index: usize, play: bool) {
        self.set_current(Some(index));
        self.try_load_song(lib, play);
    }

    /// Shuffles current playlist
    pub fn shuffle(&mut self) {
        let current = match self.get_current() {
            Some(current) => current,
            None => return,
        };
        let id = self.playlist.get(current).map(|c| *c);

        let mut rng = rand::thread_rng();
        self.playlist.shuffle(&mut rng);

        match id {
            Some(id) => self.find_current(id),
            None => return,
        };

        if !self.shuffle_current {
            if let Some(current) = self.current {
                self.playlist.swap(current, 0);
                self.current = Some(0);
            }
        }
    }

    /// Seeks to given position
    pub fn seek_to(&mut self, lib: &Library, time: Duration) -> Result<()> {
        if let Ok(timestamp) = self.sinker.get_timestamp() {
            if timestamp.total < time {
                self.next(Some(1), lib);
                return Ok(());
            }
        }
        self.sinker.seek_to(time)
    }

    //>=====================================================================<//
    //                           Getters & Setters                           //
    //>=====================================================================<//

    /// Checks if playback is playing
    pub fn is_playing(&self) -> bool {
        self.state == PlayState::Playing
    }

    /// Checks if playback is stopped
    pub fn is_stopped(&self) -> bool {
        self.state == PlayState::Stopped
    }

    /// Sets state based on boolean
    pub fn set_state(&mut self, play: bool) {
        self.state = match (self.current.is_some(), play) {
            (false, _) => PlayState::Stopped,
            (true, false) => PlayState::Paused,
            (true, true) => PlayState::Playing,
        };
    }

    /// Gets current id as option, returns None when playback stopped
    pub fn get_current_id(&self) -> Option<usize> {
        if self.get_current().is_none() || self.is_stopped() {
            return None;
        }
        self.playlist.get(self.current.unwrap()).map(|id| *id)
    }

    /// Gets currently playing song
    pub fn get_current_song(&self, lib: &Library) -> Song {
        if let Some(current) = self.get_current() {
            match self.playlist.get(current) {
                Some(index) if !self.is_stopped() => {
                    lib.get_song(index.to_owned())
                }
                _ => Song::default(),
            }
        } else {
            Song::default()
        }
    }

    /// Sets playback volume
    pub fn set_vol(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        match self.sinker.set_volume(volume) {
            Ok(_) => self.set_volume(volume),
            Err(e) => error!("Failed to set volume: {e}"),
        }
    }

    /// Sets volume up by given step
    pub fn volume_up(&mut self, step: Option<f32>) {
        let step = step.unwrap_or(self.volume_step);
        self.set_vol(self.volume + step);
    }

    /// Sets volume down by given step
    pub fn volume_down(&mut self, step: Option<f32>) {
        let step = step.unwrap_or(self.volume_step);
        self.set_vol(self.volume - step);
    }

    /// Sets mute with applying toggle
    pub fn mute(&mut self, mute: Option<bool>) {
        let mute = mute.unwrap_or(!self.get_mute());
        let volume = if mute { 0. } else { self.volume };
        match self.sinker.set_volume(volume) {
            Ok(_) => self.set_mute(mute),
            Err(e) => error!("Failed to set mute: {e}"),
        }
    }

    /// Gets currently playing song timestamp
    pub fn get_timestamp(&self) -> Timestamp {
        match self.sinker.get_timestamp() {
            Ok(t) if self.state != PlayState::Stopped => t,
            _ => Timestamp::new(
                Duration::from_secs_f32(0.),
                Duration::from_secs_f32(0.),
            ),
        }
    }

    /// Creates playlist from library
    pub fn create_playlist(&mut self, library: &Library, id: usize) {
        self.set_playlist((0..library.count()).collect());
        self.find_current(id);
    }
}

///>=======================================================================<///
///                         Player message handling                         ///
///>=======================================================================<///
impl BumpApp {
    /// Handles player update
    pub fn player_update(&mut self, msg: PlayerMsg) {
        match msg {
            PlayerMsg::Play(play) => {
                self.player.play_pause(play);
                self.hard_pause = None;
            }
            PlayerMsg::Next(val) if self.player.get_playlist().len() > 0 => {
                _ = self.player.next(val, &self.library)
            }
            PlayerMsg::Prev(val) if self.player.get_playlist().len() > 0 => {
                _ = self.player.prev(val, &self.library)
            }
            PlayerMsg::PlaySong(id, new) => {
                if new {
                    self.player.create_playlist(&self.library, id);
                } else {
                    self.player.find_current(id)
                }

                if let Some(current) = self.player.get_current() {
                    self.player.play_at(&self.library, current, true);
                    self.hard_pause = None;
                }
            }
            PlayerMsg::SeekTo(time) => {
                _ = self.player.seek_to(&self.library, time);
            }
            PlayerMsg::SongEnd => _ = self.player.next(Some(1), &self.library),
            PlayerMsg::Volume(vol) => _ = self.player.set_vol(vol),
            PlayerMsg::Mute(mute) => _ = self.player.mute(mute),
            PlayerMsg::Shuffle => self.player.shuffle(),
            PlayerMsg::VolumeUp(step) => self.player.volume_up(step),
            PlayerMsg::VolumeDown(step) => self.player.volume_down(step),
            _ => self.player.stop(),
        }
    }
}

//>=========================================================================<//
//                             Private functions                             //
//>=========================================================================<//
impl Player {
    /// Loads song from the library
    fn load_song(&mut self, lib: &Library, id: usize, play: bool) {
        match self.sinker.load(lib, self.get_playlist()[id], play) {
            Ok(_) => self.set_state(play),
            Err(e) => error!("Failed to load the song: {e}"),
        }
    }

    /// Tries to load a song from the library
    fn try_load_song(&mut self, lib: &Library, play: bool) {
        match self.get_current() {
            Some(current) if current < self.get_playlist().len() => {
                self.load_song(lib, current, play)
            }
            _ => self.stop(),
        }
    }

    /// Finds current
    fn find_current(&mut self, id: usize) {
        self.set_current(self.playlist.iter().position(|&x| x == id));
    }

    /// Initializes player sinker
    fn init_sinker(
        &mut self,
        lib: &Library,
        conf: &Config,
        sender: UnboundedSender<Msg>,
    ) {
        // Loads default song
        self.try_load_song(lib, conf.get_autoplay());
        // Sets volume
        if self.get_mute() {
            if let Err(_) = self.sinker.set_volume(0.) {
                self.set_mute(false);
            }
        } else {
            if let Err(_) = self.sinker.set_volume(self.get_volume()) {
                self.set_volume(1.);
            }
        }
        // Sets fade length
        _ = self.sinker.set_fade(conf.get_fade());
        // Sets gapless playing
        _ = self.sinker.set_gapless(conf.get_gapless());
        // Sets on song end function
        _ = self.sinker.song_end(move |info| match info {
            CallbackInfo::SourceEnded => {
                _ = sender.send(Msg::Plr(PlayerMsg::SongEnd))
            }
            CallbackInfo::PauseEnds(i) => _ = sender.send(Msg::HardPause(i)),
            _ => {}
        });
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            sinker: Sinker::new(),
            state: PlayState::Stopped,
            current: None,
            volume: 1.,
            mute: false,
            playlist: Vec::new(),
            shuffle_current: true,
            volume_step: 0.1,
            changed: true,
        }
    }
}

//>=========================================================================<//
//                 Structs for saving and loading the player                 //
//>=========================================================================<//
#[derive(Deserialize)]
struct PlayerLoad {
    /// Index of the currently playing song
    current: Option<usize>,
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
            current: None,
            volume: 1.,
            mute: false,
            playlist: Vec::new(),
        }
    }
}

#[derive(Serialize)]
struct PlayerSave<'a> {
    /// Index of the currently playing song
    current: Option<usize>,
    /// Volume of the playback
    volume: f32,
    /// When true playback is muted
    mute: bool,
    /// Current playlist
    playlist: &'a Vec<usize>,
}
