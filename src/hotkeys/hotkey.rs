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
}
