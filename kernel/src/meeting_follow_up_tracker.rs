extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MeetingFollowUpTracker {
    meetings: Vec<Meeting>,
}

impl MeetingFollowUpTracker {
    pub fn new() -> Self {
        MeetingFollowUpTracker {
            meetings: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, title: String, date: String, participants: Vec<String>) {
        let meeting = Meeting {
            title,
            date,
            participants,
            follow_ups: Vec::new(),
        };
        self.meetings.push(meeting);
    }

    pub fn get_meetings(&self) -> &Vec<Meeting> {
        &self.meetings
    }

    pub fn add_follow_up(&mut self, meeting_title: &str, follow_up: String) {
        if let Some(meeting) = self.meetings.iter_mut().find(|m| m.title == meeting_title) {
            meeting.follow_ups.push(follow_up);
        }
    }

    pub fn get_follow_ups_for_meeting(&self, meeting_title: &str) -> Option<&Vec<String>> {
        self.meetings
            .iter()
            .find(|m| m.title == meeting_title)
            .map(|m| &m.follow_ups)
    }

    pub fn remove_meeting(&mut self, meeting_title: &str) {
        self.meetings.retain(|m| m.title != meeting_title);
    }
}

#[derive(Debug)]
struct Meeting {
    title: String,
    date: String,
    participants: Vec<String>,
    follow_ups: Vec<String>,
}
