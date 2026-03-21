extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneGroupChatAdapt {
    participants: Vec<String>,
    messages: Vec<(String, String)>, // (participant, message)
}

impl ToneGroupChatAdapt {
    pub fn new() -> Self {
        ToneGroupChatAdapt {
            participants: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        if !self.participants.contains(&participant.to_string()) {
            self.participants.push(participant.to_string());
        }
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
        self.messages.retain(|(sender, _)| sender != participant);
    }

    pub fn send_message(&mut self, sender: &str, message: &str) -> bool {
        if self.participants.contains(&sender.to_string()) {
            self.messages.push((sender.to_string(), message.to_string()));
            true
        } else {
            false
        }
    }

    pub fn get_messages(&self) -> Vec<(String, String)> {
        self.messages.clone()
    }

    pub fn get_participants(&self) -> Vec<String> {
        self.participants.clone()
    }
}
