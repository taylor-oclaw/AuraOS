extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentStreamProcessor {
    data: Vec<u8>,
    processed_data: Vec<u8>,
    buffer_size: usize,
}

impl AgentStreamProcessor {
    pub fn new(buffer_size: usize) -> Self {
        AgentStreamProcessor {
            data: Vec::with_capacity(buffer_size),
            processed_data: Vec::new(),
            buffer_size,
        }
    }

    pub fn add_data(&mut self, chunk: &[u8]) {
        if self.data.len() + chunk.len() > self.buffer_size {
            panic!("Buffer overflow");
        }
        self.data.extend_from_slice(chunk);
    }

    pub fn process_data(&mut self) {
        // Simple processing: reverse the data
        self.processed_data.clear();
        self.processed_data.extend(self.data.iter().rev());
        self.data.clear();
    }

    pub fn get_processed_data(&self) -> &[u8] {
        &self.processed_data
    }

    pub fn clear_processed_data(&mut self) {
        self.processed_data.clear();
    }

    pub fn buffer_remaining_capacity(&self) -> usize {
        self.buffer_size - self.data.len()
    }
}
