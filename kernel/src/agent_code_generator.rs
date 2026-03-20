extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentCodeGenerator {
    code: Vec<u8>,
}

impl AgentCodeGenerator {
    pub fn new() -> Self {
        AgentCodeGenerator { code: Vec::new() }
    }

    pub fn add_code(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn get_code(&self) -> &[u8] {
        &self.code
    }

    pub fn clear_code(&mut self) {
        self.code.clear();
    }

    pub fn code_length(&self) -> usize {
        self.code.len()
    }

    pub fn append_string(&mut self, s: &str) {
        for byte in s.as_bytes() {
            self.add_code(*byte);
        }
    }
}
