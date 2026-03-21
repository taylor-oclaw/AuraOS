extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingRecordingIndex {
    records: Vec<String>,
}

impl MeetingRecordingIndex {
    pub fn new() -> Self {
        MeetingRecordingIndex {
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: String) {
        self.records.push(record);
    }

    pub fn remove_record(&mut self, index: usize) -> Option<String> {
        if index < self.records.len() {
            Some(self.records.remove(index))
        } else {
            None
        }
    }

    pub fn get_record(&self, index: usize) -> Option<&String> {
        self.records.get(index)
    }

    pub fn list_records(&self) -> &[String] {
        &self.records
    }

    pub fn count_records(&self) -> usize {
        self.records.len()
    }
}
