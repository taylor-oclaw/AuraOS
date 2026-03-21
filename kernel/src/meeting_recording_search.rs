extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingRecordingSearch {
    recordings: Vec<String>,
}

impl MeetingRecordingSearch {
    pub fn new() -> Self {
        MeetingRecordingSearch {
            recordings: Vec::new(),
        }
    }

    pub fn add_recording(&mut self, recording: String) {
        self.recordings.push(recording);
    }

    pub fn search_recordings(&self, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        for recording in &self.recordings {
            if recording.contains(query) {
                results.push(recording.clone());
            }
        }
        results
    }

    pub fn list_all_recordings(&self) -> Vec<String> {
        self.recordings.clone()
    }

    pub fn remove_recording(&mut self, index: usize) -> Option<String> {
        if index < self.recordings.len() {
            Some(self.recordings.remove(index))
        } else {
            None
        }
    }

    pub fn count_recordings(&self) -> usize {
        self.recordings.len()
    }
}
