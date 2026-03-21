extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechStutterDetect {
    buffer: Vec<u8>,
    threshold: u32,
}

impl SpeechStutterDetect {
    pub fn new(threshold: u32) -> Self {
        SpeechStutterDetect {
            buffer: Vec::new(),
            threshold,
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn get_buffer_size(&self) -> usize {
        self.buffer.len()
    }

    pub fn detect_stutter(&self) -> bool {
        if self.buffer.is_empty() {
            return false;
        }

        let mut count = 0;
        for i in 1..self.buffer.len() {
            if self.buffer[i] == self.buffer[i - 1] {
                count += 1;
                if count >= self.threshold {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        false
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}
