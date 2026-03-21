extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AccessLargeCursor {
    data: Vec<u8>,
    cursor: usize,
}

impl AccessLargeCursor {
    pub fn new(size: usize) -> Self {
        AccessLargeCursor {
            data: vec![0; size],
            cursor: 0,
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> usize {
        let len = self.data.len();
        let end = core::cmp::min(self.cursor + buffer.len(), len);
        let bytes_to_read = end - self.cursor;
        buffer[..bytes_to_read].copy_from_slice(&self.data[self.cursor..end]);
        bytes_to_read
    }

    pub fn write(&mut self, buffer: &[u8]) -> usize {
        let len = self.data.len();
        let end = core::cmp::min(self.cursor + buffer.len(), len);
        let bytes_to_write = end - self.cursor;
        self.data[self.cursor..end].copy_from_slice(&buffer[..bytes_to_write]);
        self.cursor += bytes_to_write;
        bytes_to_write
    }

    pub fn seek(&mut self, offset: isize) {
        if offset >= 0 {
            self.cursor = core::cmp::min(self.cursor + offset as usize, self.data.len());
        } else {
            let offset = -offset;
            self.cursor = core::cmp::max(self.cursor as isize - offset, 0) as usize;
        }
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_cursor(&mut self, position: usize) {
        self.cursor = core::cmp::min(position, self.data.len());
    }
}
