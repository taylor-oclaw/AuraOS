extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MeetingRecurringDetect {
    meetings: Vec<Meeting>,
}

impl MeetingRecurringDetect {
    pub fn new() -> Self {
        MeetingRecurringDetect {
            meetings: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, meeting: Meeting) {
        self.meetings.push(meeting);
    }

    pub fn remove_meeting(&mut self, id: u32) -> Option<Meeting> {
        let pos = self.meetings.iter().position(|m| m.id == id);
        match pos {
            Some(index) => Some(self.meetings.remove(index)),
            None => None,
        }
    }

    pub fn get_meeting(&self, id: u32) -> Option<&Meeting> {
        self.meetings.iter().find(|m| m.id == id)
    }

    pub fn list_all_meetings(&self) -> &Vec<Meeting> {
        &self.meetings
    }

    pub fn detect_recurring_meetings(&self) -> Vec<String> {
        let mut recurring = Vec::new();
        for i in 0..self.meetings.len() {
            for j in (i + 1)..self.meetings.len() {
                if self.meetings[i].is_similar(&self.meetings[j]) {
                    if !recurring.contains(&self.meetings[i].title) {
                        recurring.push(self.meetings[i].title.clone());
                    }
                }
            }
        }
        recurring
    }
}

#[derive(Debug, Clone)]
struct Meeting {
    id: u32,
    title: String,
    start_time: u64,
    end_time: u64,
    recurrence_pattern: RecurrencePattern,
}

impl Meeting {
    fn is_similar(&self, other: &Meeting) -> bool {
        self.title == other.title && self.recurrence_pattern == other.recurrence_pattern
    }
}

#[derive(Debug, Clone, PartialEq)]
enum RecurrencePattern {
    Daily,
    Weekly(Vec<u8>), // Days of the week (0-6)
    Monthly(u8),     // Day of the month
    Yearly(u8, u8),  // Month and day of the month
}
