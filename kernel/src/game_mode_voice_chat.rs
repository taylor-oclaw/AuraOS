extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct GameModeVoiceChat {
    participants: Vec<String>,
    messages: Vec<(String, String)>, // (sender, message)
    is_active: bool,
}

impl GameModeVoiceChat {
    pub fn new() -> Self {
        GameModeVoiceChat {
            participants: Vec::new(),
            messages: Vec::new(),
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
        }
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
    }

    pub fn send_message(&mut self, sender: String, message: String) -> bool {
        if self.is_active && self.participants.contains(&sender) {
            self.messages.push((sender, message));
            true
        } else {
            false
        }
    }

    pub fn get_messages(&self) -> Vec<(String, String)> {
        self.messages.clone()
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
}
