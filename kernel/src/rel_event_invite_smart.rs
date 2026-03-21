extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct RelEventInviteSmart {
    event_name: String,
    participants: Vec<String>,
    max_participants: usize,
}

impl RelEventInviteSmart {
    pub fn new(event_name: &str, max_participants: usize) -> Self {
        RelEventInviteSmart {
            event_name: String::from(event_name),
            participants: Vec::new(),
            max_participants,
        }
    }

    pub fn add_participant(&mut self, participant: &str) -> bool {
        if self.participants.len() < self.max_participants {
            self.participants.push(String::from(participant));
            true
        } else {
            false
        }
    }

    pub fn remove_participant(&mut self, participant: &str) -> bool {
        let pos = self.participants.iter().position(|p| p == participant);
        if let Some(index) = pos {
            self.participants.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        self.participants.len() >= self.max_participants
    }

    pub fn list_participants(&self) -> Vec<String> {
        self.participants.clone()
    }

    pub fn event_name(&self) -> &str {
        &self.event_name
    }
}
