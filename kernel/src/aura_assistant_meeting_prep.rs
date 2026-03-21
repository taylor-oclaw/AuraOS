extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct MeetingPrep {
    title: String,
    participants: Vec<String>,
    agenda_items: Vec<String>,
    location: String,
    time: String,
}

impl MeetingPrep {
    pub fn new(title: &str, location: &str, time: &str) -> Self {
        MeetingPrep {
            title: String::from(title),
            participants: Vec::new(),
            agenda_items: Vec::new(),
            location: String::from(location),
            time: String::from(time),
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn remove_participant(&mut self, participant: &str) {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.participants.remove(index);
        }
    }

    pub fn add_agenda_item(&mut self, item: &str) {
        self.agenda_items.push(String::from(item));
    }

    pub fn remove_agenda_item(&mut self, item: &str) {
        if let Some(index) = self.agenda_items.iter().position(|i| i == item) {
            self.agenda_items.remove(index);
        }
    }

    pub fn get_meeting_details(&self) -> String {
        let mut details = format!("Title: {}\n", self.title);
        details.push_str(format!("Location: {}\n", self.location).as_str());
        details.push_str(format!("Time: {}\n", self.time).as_str());
        details.push_str("Participants:\n");
        for participant in &self.participants {
            details.push_str(format!("- {}\n", participant).as_str());
        }
        details.push_str("Agenda Items:\n");
        for item in &self.agenda_items {
            details.push_str(format!("- {}\n", item).as_str());
        }
        details
    }
}
