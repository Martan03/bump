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
        for hotkey in self.hotkeys.iter() {
            if let Err(e) = self.manager.register(hotkey.get_hotkey()) {
                error!("Failed to register the hotkey {e}");
            }
        }

        let hotkeys: Vec<_> = self.hotkeys.clone();
        let sender = sender.clone();
        GlobalHotKeyEvent::set_event_handler(Some(move |e: GlobalHotKeyEvent| {
            for hotkey in &hotkeys {
                if hotkey.get_id() == e.id {
                    if let Some(msg) =
                        Instance::get_action_msg(hotkey.get_action())
                    {
                        _ = sender.send(msg);
                    }
                }
            }
        }));
    }
}
