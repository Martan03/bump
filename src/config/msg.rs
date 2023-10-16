use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfMsg {
    AddPath(Vec<PathBuf>),
    RemPath(usize),
    EnableHotkeys(bool),
    RecursiveSearch(bool),
    ShuffleCurrent(bool),
    Autoplay(bool),
    StartLoad(bool),
    Gapless(bool),

    ResetAll,
}
