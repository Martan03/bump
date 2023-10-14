use std::{
    cell::Cell,
    fs::{self, File},
};

use eyre::Result;
use iced::window::Position;
use serde_derive::{Deserialize, Serialize};

use crate::config::Config;

use super::widgets::list_view;

#[derive(Clone, Serialize, Deserialize)]
pub struct Gui {
    /// Window width
    width: u32,
    /// Window height
    height: u32,
    /// Window position on x coordinate
    pos_x: i32,
    /// Window position on y coordinate
    pos_y: i32,
    #[serde(skip)]
    wb_states: Vec<Cell<list_view::State>>,
    /// Stores whether Gui variables changed
    #[serde(skip, default)]
    changed: bool,
}

impl Gui {
    /// Loads Gui from the json file
    pub fn load(config: &Config) -> Self {
        let path = config.get_gui_path();

        let mut gui = match fs::read_to_string(path) {
            Err(_) => Gui::default(),
            Ok(g) => match serde_json::from_str::<Gui>(&g) {
                Err(_) => Gui::default(),
                Ok(gui) => gui,
            },
        };
        gui.wb_states = vec![Cell::<list_view::State>::default(); 2];
        gui
    }

    /// Saves gui state to json file
    pub fn save(&self, config: &Config) -> Result<()> {
        // If Gui haven't change, don't save
        if !self.changed {
            return Ok(());
        }

        let file = config.get_gui_path();
        File::create(&file)?;

        let text = serde_json::to_string::<Gui>(self)?;
        fs::write(file, text)?;

        Ok(())
    }

    //>=====================================================================<//
    //                           Getters & Setters                           //
    //>=====================================================================<//

    /// Gets window size
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Sets window size to given value
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.changed = true;
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
        self.changed = true;
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }

    /// Gets WrapBox state on given index
    pub fn get_wb_state(&self, index: usize) -> &Cell<list_view::State> {
        &self.wb_states[index]
    }
}

/// Implements default for GUI
impl Default for Gui {
    fn default() -> Self {
        Gui {
            width: 1280,
            height: 720,
            pos_x: i32::MAX,
            pos_y: i32::MAX,
            wb_states: vec![Cell::<list_view::State>::default(); 2],
            changed: false,
        }
    }
}
