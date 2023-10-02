use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use log::error;

use super::hotkey::Hotkey;

pub struct Hotkeys {
    manager: GlobalHotKeyManager,
    hotkeys: Vec<Hotkey>,
}

impl Hotkeys {
    pub fn new(hotkeys: Vec<Hotkey>) -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();

        for hotkey in hotkeys.iter() {
            if let Err(e) = manager.register(hotkey.get_hotkey()) {
                error!("Failed to register the hotkey {e}");
            }

            GlobalHotKeyEvent::set_event_handler(Some(|e| {
                println!("{:?}", e);
            }))
        }

        Self { manager, hotkeys }
    }
}
