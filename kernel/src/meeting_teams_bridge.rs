extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_teams_bridge_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_teams_bridge_exit() {
    // Cleanup logic for the module
}

pub struct MeetingTeamsBridge {
    participants: Vec<String>,
    meetings: Vec<String>,
}

impl MeetingTeamsBridge {
    pub fn new() -> Self {
        MeetingTeamsBridge {
            participants: Vec::new(),
            meetings: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
        }
    }

    pub fn remove_participant(&mut self, participant: &str) -> bool {
        let index = self.participants.iter().position(|p| p == participant);
        if let Some(i) = index {
            self.participants.remove(i);
            true
        } else {
            false
        }
    }

    pub fn list_participants(&self) -> Vec<String> {
        self.participants.clone()
    }

    pub fn schedule_meeting(&mut self, meeting: String) {
        if !self.meetings.contains(&meeting) {
            self.meetings.push(meeting);
        }
    }

    pub fn cancel_meeting(&mut self, meeting: &str) -> bool {
        let index = self.meetings.iter().position(|m| m == meeting);
        if let Some(i) = index {
            self.meetings.remove(i);
            true
        } else {
            false
        }
    }

    pub fn list_meetings(&self) -> Vec<String> {
        self.meetings.clone()
    }
}
