extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_stranger_mode_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_stranger_mode_exit() {
    // Cleanup logic for the module
}

pub struct ToneStrangerMode {
    active: bool,
    users: Vec<String>,
    banned_words: Vec<String>,
}

impl ToneStrangerMode {
    pub fn new() -> Self {
        ToneStrangerMode {
            active: false,
            users: Vec::new(),
            banned_words: Vec::new(),
        }
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

    pub fn add_user(&mut self, username: &str) {
        if !self.users.contains(&username.to_string()) {
            self.users.push(username.to_string());
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        self.users.retain(|user| user != username);
    }

    pub fn add_banned_word(&mut self, word: &str) {
        if !self.banned_words.contains(&word.to_string()) {
            self.banned_words.push(word.to_string());
        }
    }

    pub fn remove_banned_word(&mut self, word: &str) {
        self.banned_words.retain(|w| w != word);
    }

    pub fn check_message(&self, message: &str) -> bool {
        if !self.active {
            return true;
        }

        for banned_word in &self.banned_words {
            if message.contains(banned_word) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_stranger_mode() {
        let mut mode = ToneStrangerMode::new();
        assert!(!mode.is_active());

        mode.activate();
        assert!(mode.is_active());

        mode.deactivate();
        assert!(!mode.is_active());

        mode.add_user("alice");
        assert_eq!(mode.users.len(), 1);

        mode.remove_user("alice");
        assert_eq!(mode.users.len(), 0);

        mode.add_banned_word("badword");
        assert_eq!(mode.banned_words.len(), 1);

        mode.remove_banned_word("badword");
        assert_eq!(mode.banned_words.len(), 0);

        mode.activate();
        mode.add_banned_word("forbidden");
        assert!(!mode.check_message("This is a forbidden message"));
        assert!(mode.check_message("This is a safe message"));
    }
}
