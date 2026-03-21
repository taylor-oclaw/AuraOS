extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CommunicationChannel {
    buffer: Vec<u8>,
    read_index: usize,
    write_index: usize,
}

impl CommunicationChannel {
    pub fn new(capacity: usize) -> Self {
        CommunicationChannel {
            buffer: vec![0; capacity],
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.read_index == self.write_index
    }

    pub fn is_full(&self) -> bool {
        (self.write_index + 1) % self.buffer.len() == self.read_index
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn available_data(&self) -> usize {
        if self.write_index >= self.read_index {
            self.write_index - self.read_index
        } else {
            self.buffer.len() - self.read_index + self.write_index
        }
    }

    pub fn available_space(&self) -> usize {
        self.capacity() - self.available_data()
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.is_full() {
            return Err("Buffer is full");
        }

        let mut bytes_written = 0;
        for &byte in data {
            if self.is_full() {
                break;
            }
            self.buffer[self.write_index] = byte;
            self.write_index = (self.write_index + 1) % self.buffer.len();
            bytes_written += 1;
        }

        Ok(bytes_written)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.is_empty() {
            return Err("Buffer is empty");
        }

        let mut bytes_read = 0;
        for byte in buffer.iter_mut() {
            if self.is_empty() {
                break;
            }
            *byte = self.buffer[self.read_index];
            self.read_index = (self.read_index + 1) % self.buffer.len();
            bytes_read += 1;
        }

        Ok(bytes_read)
    }
}
