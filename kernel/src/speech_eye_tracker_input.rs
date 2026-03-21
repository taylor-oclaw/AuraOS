extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechEyeTrackerInput {
    data: Vec<u8>,
    position: usize,
}

impl SpeechEyeTrackerInput {
    pub fn new() -> Self {
        SpeechEyeTrackerInput {
            data: Vec::new(),
            position: 0,
        }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn has_more_data(&self) -> bool {
        self.position < self.data.len()
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.has_more_data() {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..count {
            if let Some(byte) = self.read_byte() {
                result.push(byte);
            } else {
                break;
            }
        }
        result
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}
