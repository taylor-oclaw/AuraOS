extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_patience_mode_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_patience_mode_exit() {
    // Cleanup logic for the module
}

pub struct SpeechPatienceMode {
    patience_level: u32,
    messages: Vec<String>,
}

impl SpeechPatienceMode {
    pub fn new(patience_level: u32) -> Self {
        SpeechPatienceMode {
            patience_level,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        if self.messages.len() < self.patience_level as usize {
            self.messages.push(message);
        }
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn has_reached_patience_limit(&self) -> bool {
        self.messages.len() >= self.patience_level as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_patience_mode() {
        let mut mode = SpeechPatienceMode::new(3);
        assert_eq!(mode.get_messages().len(), 0);

        mode.add_message(String::from("Hello"));
        mode.add_message(String::from("World"));
        assert_eq!(mode.get_messages().len(), 2);

        mode.add_message(String::from("Test"));
        assert_eq!(mode.get_messages().len(), 3);
        assert!(mode.has_reached_patience_limit());

        mode.add_message(String::from("Should not add"));
        assert_eq!(mode.get_messages().len(), 3);

        mode.clear_messages();
        assert_eq!(mode.get_messages().len(), 0);
    }
}
