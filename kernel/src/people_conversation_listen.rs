extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PeopleConversationListen {
    participants: Vec<String>,
    conversation_history: Vec<String>,
}

impl PeopleConversationListen {
    pub fn new(participants: Vec<&str>) -> Self {
        let mut participant_strings = Vec::new();
        for participant in participants {
            participant_strings.push(String::from(participant));
        }
        PeopleConversationListen {
            participants: participant_strings,
            conversation_history: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, name: &str) {
        self.participants.push(String::from(name));
    }

    pub fn remove_participant(&mut self, name: &str) -> bool {
        if let Some(index) = self.participants.iter().position(|x| x == name) {
            self.participants.remove(index);
            true
        } else {
            false
        }
    }

    pub fn start_conversation(&mut self, message: &str) {
        self.conversation_history.push(String::from(message));
    }

    pub fn get_participants(&self) -> Vec<String> {
        self.participants.clone()
    }

    pub fn get_conversation_history(&self) -> Vec<String> {
        self.conversation_history.clone()
    }
}
