use std::collections::HashMap;

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use log::error;
use tokio::sync::mpsc::UnboundedSender;

use crate::{cli::instance::Instance, gui::app::Msg};

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
    pub fn init(
        &mut self,
        hotkeys: Vec<Hotkey>,
        sender: UnboundedSender<Msg>,
    ) {
        self.hotkeys = hotkeys;
        let mut actions: HashMap<u32, String> = HashMap::new();
        for hotkey in self.hotkeys.iter() {
            let hk = hotkey.get_hotkey();
            println!("{}", hotkey.to_string());
            if let Err(e) = self.manager.register(hk) {
                error!("Failed to register the hotkey {e}");
            } else {
                actions.insert(hk.id(), hotkey.get_action().to_owned());
            }
        }

        let sender = sender.clone();
        GlobalHotKeyEvent::set_event_handler(Some(
            move |e: GlobalHotKeyEvent| {
                if let Some(action) = actions.get(&e.id) {
                    if let Some(msg) = Instance::get_action_msg(action) {
                        _ = sender.send(msg);
                    }
                }
            },
        ));
    }
}
