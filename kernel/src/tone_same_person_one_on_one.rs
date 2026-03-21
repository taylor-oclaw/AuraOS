extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_same_person_one_on_one_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_same_person_one_on_one_exit() {
    // Cleanup logic for the module
}

pub struct ToneSamePersonOneOnOne {
    user_id: String,
    conversation_history: Vec<String>,
}

impl ToneSamePersonOneOnOne {
    pub fn new(user_id: &str) -> Self {
        ToneSamePersonOneOnOne {
            user_id: String::from(user_id),
            conversation_history: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.conversation_history.push(String::from(message));
    }

    pub fn get_conversation_history(&self) -> &[String] {
        &self.conversation_history
    }

    pub fn clear_conversation_history(&mut self) {
        self.conversation_history.clear();
    }

    pub fn last_message(&self) -> Option<&str> {
        self.conversation_history.last().map(|s| s.as_str())
    }
}
