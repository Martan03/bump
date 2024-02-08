use eyre::{Report, Result};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};

use super::{
    code::{get_code_string, string_to_code},
    modifiers::{get_modifier_string, string_to_modifier},
};

pub struct Hotkey {
    modifiers: Modifiers,
    code: Code,
    action: String,
}

impl Hotkey {
    /// Creates new [`Hotkey`]
    pub fn new(modifiers: Modifiers, code: Code, action: String) -> Self {
        Self {
            modifiers,
            code,
            action,
        }
    }

    /// Creates new [`Hotkey`] by given String
    pub fn new_from_str(hotkey: String, action: String) -> Result<Self> {
        let parts = hotkey.split('+');
        let mut mods = Modifiers::empty();
        let mut code: Option<Code> = None;
        for part in parts {
            let part = &part.to_lowercase().replace('-', "_");
            if let Some(modifier) = string_to_modifier(part) {
                mods |= modifier;
            } else if code.is_some() {
                return Err(Report::msg("Multiple codes"));
            } else if let Some(c) = string_to_code(part) {
                code = Some(c);
            } else {
                return Err(Report::msg("Key not supported: {part}"));
            }
        }

        match code {
            Some(code) => Ok(Hotkey::new(mods, code, action)),
            None => Err(Report::msg("No code given")),
        }
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

impl ToString for Hotkey {
    fn to_string(&self) -> String {
        get_modifier_string(&self.modifiers)
            + "+"
            + get_code_string(&self.code)
    }
}

/// Implements clone for hotkey
impl Clone for Hotkey {
    fn clone(&self) -> Self {
        Self {
            modifiers: self.modifiers,
            code: self.code,
            action: self.action.clone(),
        }
    }
}
