extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarMeetingCost {
    meetings: Vec<(String, u32)>, // (meeting_name, cost)
}

impl CalendarMeetingCost {
    pub fn new() -> Self {
        CalendarMeetingCost {
            meetings: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, name: String, cost: u32) {
        self.meetings.push((name, cost));
    }

    pub fn total_cost(&self) -> u32 {
        self.meetings.iter().map(|(_, cost)| cost).sum()
    }

    pub fn find_meeting_by_name(&self, name: &str) -> Option<&(String, u32)> {
        self.meetings.iter().find(|&&(ref meeting_name, _)| meeting_name == name)
    }

    pub fn remove_meeting_by_name(&mut self, name: &str) -> bool {
        let pos = self.meetings.iter().position(|&(ref meeting_name, _)| meeting_name == name);
        if let Some(index) = pos {
            self.meetings.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_meetings(&self) -> Vec<&(String, u32)> {
        self.meetings.iter().collect()
    }
}
