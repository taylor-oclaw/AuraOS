extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceRecorder {
    buffer: Vec<u8>,
}

impl VoiceRecorder {
    pub fn new(buffer_size: usize) -> Self {
        let mut buffer = Vec::<u8>::with_capacity(buffer_size);
        buffer.resize(buffer_size, 0);
        VoiceRecorder { buffer }
    }

    pub fn record(&mut self, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.buffer[i] = *byte;
        }
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}