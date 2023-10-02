use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use log::error;

use super::hotkey::Hotkey;

pub struct Hotkeys {
    manager: GlobalHotKeyManager,
    hotkeys: Vec<Hotkey>,
}

impl Hotkeys {
    /// Creates new hotkeys
    pub fn new() -> Self {
        Self {
            manager: GlobalHotKeyManager::new().unwrap(),
            hotkeys: Vec::new(),
        }
    }

    /// Inits and registers hotkeys
    pub fn init(&mut self, hotkeys: Vec<Hotkey>) {
        for hotkey in hotkeys.iter() {
            if let Err(e) = self.manager.register(hotkey.get_hotkey()) {
                error!("Failed to register the hotkey {e}");
            }

            GlobalHotKeyEvent::set_event_handler(Some(|e| {
                println!("{:?}", e);
            }))
        }
    }
}
