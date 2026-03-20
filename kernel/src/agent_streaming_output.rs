extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentStreamingOutput {
    buffer: Vec<u8>,
    position: usize,
}

impl AgentStreamingOutput {
    pub fn new() -> Self {
        AgentStreamingOutput {
            buffer: Vec::new(),
            position: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn read(&mut self, size: usize) -> Option<Vec<u8>> {
        if self.position + size > self.buffer.len() {
            return None;
        }
        let result = self.buffer[self.position..self.position + size].to_vec();
        self.position += size;
        Some(result)
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
