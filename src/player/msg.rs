use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PlayerMsg {
    Play(Option<bool>),
    Next(Option<usize>),
    Prev(Option<usize>),
    PlaySong(usize, bool),
    SeekTo(Duration),
    SongEnd,
    Volume(f32),
    VolumeUp(Option<f32>),
    VolumeDown(Option<f32>),
    Mute(Option<bool>),
    Shuffle,
}
