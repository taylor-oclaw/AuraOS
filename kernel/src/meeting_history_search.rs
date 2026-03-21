extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingHistorySearch {
    history: Vec<String>,
}

impl MeetingHistorySearch {
    pub fn new() -> Self {
        MeetingHistorySearch {
            history: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, meeting: String) {
        self.history.push(meeting);
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        self.history
            .iter()
            .filter(|meeting| meeting.contains(query))
            .cloned()
            .collect()
    }

    pub fn get_all_meetings(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn remove_meeting(&mut self, index: usize) -> Option<String> {
        if index < self.history.len() {
            Some(self.history.remove(index))
        } else {
            None
        }
    }

    pub fn count_meetings(&self) -> usize {
        self.history.len()
    }
}
