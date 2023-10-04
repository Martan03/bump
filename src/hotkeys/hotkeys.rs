use std::collections::HashMap;

use eyre::Result;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use log::error;
use tokio::sync::mpsc::UnboundedSender;

use crate::{cli::instance::Instance, config::config::Config, gui::app::Msg};

use super::hotkey::Hotkey;

pub struct Hotkeys {
    manager: Option<GlobalHotKeyManager>,
    actions: HashMap<u32, String>,
}

impl Hotkeys {
    /// Creates new hotkeys
    pub fn new() -> Self {
        Self {
            manager: None,
            actions: HashMap::new(),
        }
    }

    /// Inits and registers hotkeys
    pub fn init(
        &mut self,
        conf: &Config,
        sender: UnboundedSender<Msg>,
    ) -> Result<()> {
        let mngr = GlobalHotKeyManager::new()?;
        for (hk, act) in conf.get_hotkeys().iter() {
            let hotkey =
                match Hotkey::new_from_str(hk.to_owned(), act.to_owned()) {
                    Ok(hotkey) => hotkey,
                    Err(e) => {
                        error!("Failed to read hotkey: {e}");
                        continue;
                    }
                };

            let hk = hotkey.get_hotkey();
            match mngr.register(hk) {
                Ok(_) => {
                    self.actions
                        .insert(hk.id(), hotkey.get_action().to_owned());
                }
                Err(e) => error!("Failed to register the hotkey {e}"),
            }
        }

        let sender = sender.clone();
        let actions = self.actions.clone();
        GlobalHotKeyEvent::set_event_handler(Some(
            move |e: GlobalHotKeyEvent| {
                if let Some(action) = actions.get(&e.id) {
                    if let Some(msg) = Instance::get_action_msg(action) {
                        _ = sender.send(msg);
                    }
                }
            },
        ));
        self.manager = Some(mngr);
        Ok(())
    }

    pub fn disable(&mut self) {
        self.manager = None;
    }
}
