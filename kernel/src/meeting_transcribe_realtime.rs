extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn meeting_transcribe_realtime_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn meeting_transcribe_realtime_exit() {
    // Cleanup logic for the module
}

pub struct MeetingTranscriber {
    transcript: Vec<String>,
    participants: Vec<String>,
}

impl MeetingTranscriber {
    pub fn new() -> Self {
        MeetingTranscriber {
            transcript: Vec::new(),
            participants: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, name: &str) {
        if !self.participants.contains(&name.to_string()) {
            self.participants.push(name.to_string());
        }
    }

    pub fn remove_participant(&mut self, name: &str) {
        self.participants.retain(|participant| participant != name);
    }

    pub fn add_transcript_entry(&mut self, entry: &str) {
        self.transcript.push(entry.to_string());
    }

    pub fn get_transcript(&self) -> &[String] {
        &self.transcript
    }

    pub fn list_participants(&self) -> &[String] {
        &self.participants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meeting_transcriber() {
        let mut transcriber = MeetingTranscriber::new();
        assert!(transcriber.list_participants().is_empty());
        assert!(transcriber.get_transcript().is_empty());

        transcriber.add_participant("Alice");
        transcriber.add_participant("Bob");
        assert_eq!(transcriber.list_participants(), &["Alice", "Bob"]);

        transcriber.remove_participant("Alice");
        assert_eq!(transcriber.list_participants(), &["Bob"]);

        transcriber.add_transcript_entry("Hello, Bob!");
        assert_eq!(transcriber.get_transcript(), &["Hello, Bob!"]);
    }
}
