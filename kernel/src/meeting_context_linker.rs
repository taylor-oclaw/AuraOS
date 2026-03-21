extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingContextLinker {
    participants: Vec<String>,
    agenda_items: Vec<String>,
    start_time: u64,
    end_time: u64,
}

impl MeetingContextLinker {
    pub fn new(start_time: u64, end_time: u64) -> Self {
        MeetingContextLinker {
            participants: Vec::new(),
            agenda_items: Vec::new(),
            start_time,
            end_time,
        }
    }

    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
        }
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
    }

    pub fn add_agenda_item(&mut self, item: String) {
        if !self.agenda_items.contains(&item) {
            self.agenda_items.push(item);
        }
    }

    pub fn remove_agenda_item(&mut self, item: &str) {
        self.agenda_items.retain(|i| i != item);
    }

    pub fn get_meeting_details(&self) -> (Vec<String>, Vec<String>, u64, u64) {
        (
            self.participants.clone(),
            self.agenda_items.clone(),
            self.start_time,
            self.end_time,
        )
    }
}
