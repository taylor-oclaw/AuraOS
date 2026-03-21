extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct VoiceShortcutTrigger {
    shortcuts: Vec<String>,
    active: bool,
}

impl VoiceShortcutTrigger {
    pub fn new() -> Self {
        VoiceShortcutTrigger {
            shortcuts: Vec::new(),
            active: false,
        }
    }

    pub fn add_shortcut(&mut self, shortcut: String) {
        if !self.shortcuts.contains(&shortcut) {
            self.shortcuts.push(shortcut);
        }
    }

    pub fn remove_shortcut(&mut self, shortcut: &str) -> bool {
        let index = self.shortcuts.iter().position(|s| s == shortcut);
        match index {
            Some(i) => {
                self.shortcuts.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn list_shortcuts(&self) -> Vec<String> {
        self.shortcuts.clone()
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
