extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShortcutManager {
    shortcuts: Vec<(String, String)>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        ShortcutManager {
            shortcuts: Vec::new(),
        }
    }

    pub fn add_shortcut(&mut self, key_sequence: &str, action: &str) {
        self.shortcuts.push((String::from(key_sequence), String::from(action)));
    }

    pub fn remove_shortcut(&mut self, key_sequence: &str) -> bool {
        let len_before = self.shortcuts.len();
        self.shortcuts.retain(|(k, _)| k != key_sequence);
        self.shortcuts.len() != len_before
    }

    pub fn get_action(&self, key_sequence: &str) -> Option<&String> {
        for (k, a) in &self.shortcuts {
            if k == key_sequence {
                return Some(a);
            }
        }
        None
    }

    pub fn list_shortcuts(&self) -> &Vec<(String, String)> {
        &self.shortcuts
    }

    pub fn clear(&mut self) {
        self.shortcuts.clear();
    }

    pub fn count(&self) -> usize {
        self.shortcuts.len()
    }
}
