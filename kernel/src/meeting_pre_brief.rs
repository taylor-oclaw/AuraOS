extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_pre_brief_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_pre_brief_exit() {
    // Cleanup logic for the module
}

pub struct MeetingPreBrief {
    participants: Vec<String>,
    agenda: Vec<String>,
    location: String,
    time: String,
}

impl MeetingPreBrief {
    pub fn new(location: &str, time: &str) -> Self {
        MeetingPreBrief {
            participants: Vec::new(),
            agenda: Vec::new(),
            location: String::from(location),
            time: String::from(time),
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
    }

    pub fn add_agenda_item(&mut self, item: &str) {
        self.agenda.push(String::from(item));
    }

    pub fn remove_agenda_item(&mut self, item: &str) {
        self.agenda.retain(|i| i != item);
    }

    pub fn get_meeting_details(&self) -> String {
        let mut details = String::from("info");
        if !self.participants.is_empty() {
            details.push_str("\nParticipants:\n");
            for participant in &self.participants {
                details.push_str(participant);
                details.push('\n');
            }
        }
        if !self.agenda.is_empty() {
            details.push_str("Agenda:\n");
            for item in &self.agenda {
                details.push_str(item);
                details.push('\n');
            }
        }
        details
    }
}
