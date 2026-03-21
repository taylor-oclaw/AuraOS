extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct VoicePrivacyMute {
    is_muted: bool,
    allowed_users: Vec<String>,
    blocked_words: Vec<String>,
    log: Vec<String>,
}

impl VoicePrivacyMute {
    pub fn new() -> Self {
        VoicePrivacyMute {
            is_muted: false,
            allowed_users: Vec::new(),
            blocked_words: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn mute(&mut self) {
        self.is_muted = true;
        self.log.push(String::from("Muted voice input."));
    }

    pub fn unmute(&mut self) {
        self.is_muted = false;
        self.log.push(String::from("Unmuted voice input."));
    }

    pub fn is_voice_muted(&self) -> bool {
        self.is_muted
    }

    pub fn add_allowed_user(&mut self, user: String) {
        if !self.allowed_users.contains(&user) {
            self.allowed_users.push(user);
            self.log.push(format!("Added allowed user: {}", user));
        }
    }

    pub fn remove_allowed_user(&mut self, user: &str) {
        if let Some(index) = self.allowed_users.iter().position(|u| u == user) {
            self.allowed_users.remove(index);
            self.log.push(format!("Removed allowed user: {}", user));
        }
    }

    pub fn add_blocked_word(&mut self, word: String) {
        if !self.blocked_words.contains(&word) {
            self.blocked_words.push(word);
            self.log.push(format!("Added blocked word: {}", word));
        }
    }

    pub fn remove_blocked_word(&mut self, word: &str) {
        if let Some(index) = self.blocked_words.iter().position(|w| w == word) {
            self.blocked_words.remove(index);
            self.log.push(format!("Removed blocked word: {}", word));
        }
    }

    pub fn check_message(&self, message: &str, user: &str) -> bool {
        if !self.allowed_users.contains(&user.to_string()) {
            self.log.push(format!("User {} is not allowed to speak.", user));
            return false;
        }

        for blocked_word in &self.blocked_words {
            if message.contains(blocked_word) {
                self.log.push(format!("Blocked word '{}' found in message from {}", blocked_word, user));
                return false;
            }
        }

        true
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_privacy_mute() {
        let mut vpm = VoicePrivacyMute::new();
        assert!(!vpm.is_voice_muted());

        vpm.mute();
        assert!(vpm.is_voice_muted());

        vpm.unmute();
        assert!(!vpm.is_voice_muted());

        vpm.add_allowed_user(String::from("Alice"));
        assert_eq!(vpm.allowed_users.len(), 1);

        vpm.remove_allowed_user("Alice");
        assert_eq!(vpm.allowed_users.len(), 0);

        vpm.add_blocked_word(String::from("secret"));
        assert_eq!(vpm.blocked_words.len(), 1);

        vpm.remove_blocked_word("secret");
        assert_eq!(vpm.blocked_words.len(), 0);

        vpm.add_allowed_user(String::from("Bob"));
        assert!(vpm.check_message("This is a test message.", "Bob"));
        assert!(!vpm.check_message("This is a secret message.", "Bob"));

        let log = vpm.get_log();
        assert_eq!(log.len(), 7);
    }
}
