extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_priority_infer_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_priority_infer_exit() {
    // Cleanup logic for the module
}

pub struct MeetingPriorityInfer {
    meetings: Vec<Meeting>,
}

impl MeetingPriorityInfer {
    pub fn new() -> Self {
        MeetingPriorityInfer {
            meetings: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, title: String, priority: u8) {
        let meeting = Meeting { title, priority };
        self.meetings.push(meeting);
    }

    pub fn get_meeting_count(&self) -> usize {
        self.meetings.len()
    }

    pub fn get_highest_priority_meeting(&self) -> Option<&Meeting> {
        self.meetings.iter().max_by_key(|m| m.priority)
    }

    pub fn remove_meeting(&mut self, title: &str) {
        self.meetings.retain(|m| m.title != title);
    }
}

struct Meeting {
    title: String,
    priority: u8,
}
