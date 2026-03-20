extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AsfTransportAbstract {
    data: Vec<u8>,
    buffer_size: usize,
    position: usize,
}

impl AsfTransportAbstract {
    pub fn new(buffer_size: usize) -> Self {
        AsfTransportAbstract {
            data: vec![0; buffer_size],
            buffer_size,
            position: 0,
        }
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, &'static str> {
        if self.position + bytes.len() > self.buffer_size {
            return Err("Buffer overflow");
        }
        let end = self.position + bytes.len();
        self.data[self.position..end].copy_from_slice(bytes);
        self.position = end;
        Ok(bytes.len())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if buffer.len() > self.buffer_size {
            return Err("Buffer overflow");
        }
        let bytes_to_read = core::cmp::min(buffer.len(), self.position);
        buffer[..bytes_to_read].copy_from_slice(&self.data[..bytes_to_read]);
        self.position -= bytes_to_read;
        Ok(bytes_to_read)
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
        self.position = 0;
    }

    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn get_position(&self) -> usize {
        self.position
    }
}
