use std::fs::{self, File};

use eyre::Result;
use iced::window::Position;
use serde_derive::{Deserialize, Serialize};

use crate::config::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gui {
    width: u32,
    height: u32,
    pos_x: i32,
    pos_y: i32,
}

impl Gui {
    /// Loads GUI from the json file
    pub fn load(config: &Config) -> Self {
        let path = config.get_gui_path();

        match fs::read_to_string(path) {
            Err(_) => Gui::default(),
            Ok(g) => match serde_json::from_str::<Gui>(&g) {
                Err(_) => Gui::default(),
                Ok(gui) => gui,
            },
        }
    }

    /// Saves gui state to json file
    pub fn save(&self, config: &Config) -> Result<()> {
        let file = config.get_gui_path();
        File::create(&file)?;

        let text = serde_json::to_string_pretty::<Gui>(self)?;
        fs::write(file, text)?;

        Ok(())
    }

    /// Gets window size
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Sets window size to given value
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Gets window position
    pub fn get_pos(&self) -> Position {
        if self.pos_x == i32::MAX || self.pos_y == i32::MAX {
            Position::Default
        } else {
            Position::Specific(self.pos_x, self.pos_y)
        }
    }

    /// Sets window position
    pub fn set_pos(&mut self, pos_x: i32, pos_y: i32) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            width: 1280,
            height: 720,
            pos_x: i32::MAX,
            pos_y: i32::MAX,
        }
    }
}
