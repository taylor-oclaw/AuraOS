extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingRecordingStore {
    recordings: Vec<String>,
}

impl MeetingRecordingStore {
    pub fn new() -> Self {
        MeetingRecordingStore {
            recordings: Vec::new(),
        }
    }

    pub fn add_recording(&mut self, recording: String) {
        self.recordings.push(recording);
    }

    pub fn get_recordings(&self) -> &Vec<String> {
        &self.recordings
    }

    pub fn remove_recording(&mut self, index: usize) -> Option<String> {
        if index < self.recordings.len() {
            Some(self.recordings.remove(index))
        } else {
            None
        }
    }

    pub fn find_recording(&self, keyword: &str) -> Vec<&String> {
        self.recordings.iter().filter(|rec| rec.contains(keyword)).collect()
    }

    pub fn clear_recordings(&mut self) {
        self.recordings.clear();
    }
}
