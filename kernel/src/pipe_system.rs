extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct Pipe {
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl Pipe {
    pub fn new(capacity: usize) -> Self {
        Pipe {
            buffer: vec![0; capacity],
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if data.len() > self.available_space() {
            return Err("Not enough space in the pipe");
        }

        for byte in data {
            self.buffer[self.write_pos] = *byte;
            self.write_pos = (self.write_pos + 1) % self.buffer.len();
        }

        Ok(data.len())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut bytes_read = 0;
        while bytes_read < buf.len() && self.read_pos != self.write_pos {
            buf[bytes_read] = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.buffer.len();
            bytes_read += 1;
        }

        Ok(bytes_read)
    }

    pub fn available_space(&self) -> usize {
        if self.write_pos >= self.read_pos {
            self.buffer.len() - self.write_pos + self.read_pos
        } else {
            self.read_pos - self.write_pos
        }
    }

    pub fn available_data(&self) -> usize {
        if self.write_pos >= self.read_pos {
            self.write_pos - self.read_pos
        } else {
            self.buffer.len() - self.read_pos + self.write_pos
        }
    }

    pub fn clear(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
    }
}
