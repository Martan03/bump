use global_hotkey::hotkey::HotKey;

pub struct Hotkey {
    hotkey: HotKey,
    action: String,
}

impl Hotkey {
    /// Creates new [`Hotkey`]
    pub fn new(hotkey: HotKey, action: String) -> Self {
        Self { action, hotkey }
    }

    /// Gets hotkey
    pub fn get_hotkey(&self) -> HotKey {
        self.hotkey
    }

    /// Gets id of the hotkey
    pub fn get_id(&self) -> u32 {
        self.hotkey.id()
    }

    /// Gets hotkey action
    pub fn get_action(&self) -> &str {
        &self.action
    }
}

/// Implements clone for hotkey
impl Clone for Hotkey {
    fn clone(&self) -> Self {
        Self {
            hotkey: self.hotkey.clone(),
            action: self.action.clone(),
        }
    }
}
