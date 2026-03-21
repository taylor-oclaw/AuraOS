extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_share_handler_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_share_handler_exit() {
    // Cleanup logic for the module
}

pub struct MeetingShareHandler {
    participants: Vec<String>,
    topics: Vec<String>,
    agenda: String,
    duration: u32,
    location: String,
}

impl MeetingShareHandler {
    pub fn new(participants: Vec<String>, topics: Vec<String>, agenda: String, duration: u32, location: String) -> Self {
        MeetingShareHandler {
            participants,
            topics,
            agenda,
            duration,
            location,
        }
    }

    pub fn add_participant(&mut self, participant: String) {
        self.participants.push(participant);
    }

    pub fn remove_participant(&mut self, participant: &str) -> bool {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.participants.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_topic(&mut self, topic: String) {
        self.topics.push(topic);
    }

    pub fn remove_topic(&mut self, topic: &str) -> bool {
        if let Some(index) = self.topics.iter().position(|t| t == topic) {
            self.topics.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_agenda(&mut self, new_agenda: String) {
        self.agenda = new_agenda;
    }

    pub fn get_meeting_details(&self) -> (Vec<String>, Vec<String>, String, u32, String) {
        (
            self.participants.clone(),
            self.topics.clone(),
            self.agenda.clone(),
            self.duration,
            self.location.clone(),
        
    }
}
