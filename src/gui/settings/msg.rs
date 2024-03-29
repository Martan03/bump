use serde_derive::{Deserialize, Serialize};

use super::SettingsPage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettingsMsg {
    PickSearchPath,
    Page(SettingsPage),
    Fade(String),
    FadeSave,
    VolJump(String),
    VolJumpSave,
    Hotkey(String),
    HotkeySave,
}
