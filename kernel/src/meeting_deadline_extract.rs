extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingDeadlineExtractor {
    meetings: Vec<Meeting>,
}

impl MeetingDeadlineExtractor {
    pub fn new() -> Self {
        MeetingDeadlineExtractor {
            meetings: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, title: String, deadline: u64) {
        let meeting = Meeting { title, deadline };
        self.meetings.push(meeting);
    }

    pub fn get_all_meetings(&self) -> &Vec<Meeting> {
        &self.meetings
    }

    pub fn get_upcoming_meetings(&self, current_time: u64) -> Vec<&Meeting> {
        self.meetings.iter().filter(|m| m.deadline > current_time).collect()
    }

    pub fn remove_meeting_by_title(&mut self, title: &str) {
        self.meetings.retain(|m| m.title != title);
    }

    pub fn get_meeting_count(&self) -> usize {
        self.meetings.len()
    }
}

struct Meeting {
    title: String,
    deadline: u64,
}
