use global_hotkey::hotkey::{HotKey, Modifiers, Code};

pub struct Hotkey {
    modifiers: Modifiers,
    code: Code,
    action: String,
}

impl Hotkey {
    /// Creates new [`Hotkey`]
    pub fn new(modifiers: Modifiers, code: Code, action: String) -> Self {
        Self { modifiers, code, action }
    }

    /// Gets hotkey
    pub fn get_hotkey(&self) -> HotKey {
        HotKey::new(Some(self.modifiers), self.code)
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
            modifiers: self.modifiers.clone(),
            code: self.code.clone(),
            action: self.action.clone(),
        }
    }
}
