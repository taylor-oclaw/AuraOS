extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingTranscribeOffline {
    transcript: Vec<String>,
    participants: Vec<String>,
}

impl MeetingTranscribeOffline {
    pub fn new() -> Self {
        MeetingTranscribeOffline {
            transcript: Vec::new(),
            participants: Vec::new(),
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

    pub fn add_transcript(&mut self, speaker: &str, text: String) {
        let entry = String::from("info");
        self.transcript.push(entry);
    }

    pub fn get_transcript(&self) -> Vec<String> {
        self.transcript.clone()
    }

    pub fn list_participants(&self) -> Vec<String> {
        self.participants.clone()
    }
}
