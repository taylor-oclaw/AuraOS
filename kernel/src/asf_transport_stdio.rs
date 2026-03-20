extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn asf_transport_stdio_init() {
    // Initialization logic for the module
}

pub extern "C" fn asf_transport_stdio_exit() {
    // Cleanup logic for the module
}

pub struct StdioTransport {
    buffer: Vec<u8>,
}

impl StdioTransport {
    pub fn new() -> Self {
        StdioTransport {
            buffer: Vec::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let len = data.len();
        self.buffer.extend_from_slice(data);
        len
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let len = buf.len().min(self.buffer.len());
        buf[..len].copy_from_slice(&self.buffer[..len]);
        self.buffer.drain(..len);
        len
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for &byte in &self.buffer {
            if byte.is_ascii() {
                result.push(byte as char);
            }
        }
        result
    }
}
