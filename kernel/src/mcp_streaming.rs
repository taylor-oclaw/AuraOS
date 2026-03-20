extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MCPStreaming {
    buffer: Vec<u8>,
    position: usize,
}

impl MCPStreaming {
    pub fn new(capacity: usize) -> Self {
        MCPStreaming {
            buffer: vec![0; capacity],
            position: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.position + data.len() > self.buffer.len() {
            return Err("Buffer overflow");
        }
        self.buffer[self.position..self.position + data.len()].copy_from_slice(data);
        self.position += data.len();
        Ok(data.len())
    }

    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, &'static str> {
        if size > self.position {
            return Err("Not enough data to read");
        }
        let result = self.buffer[self.position - size..self.position].to_vec();
        self.position -= size;
        Ok(result)
    }

    pub fn available(&self) -> usize {
        self.position
    }

    pub fn clear(&mut self) {
        self.position = 0;
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_streaming() {
        let mut stream = MCPStreaming::new(10);
        assert_eq!(stream.capacity(), 10);
        assert_eq!(stream.available(), 0);

        let write_data = vec![1, 2, 3];
        assert_eq!(stream.write(&write_data).unwrap(), 3);
        assert_eq!(stream.available(), 3);

        let read_data = stream.read(2).unwrap();
        assert_eq!(read_data, vec![1, 2]);
        assert_eq!(stream.available(), 1);

        assert_eq!(stream.clear(), ());
        assert_eq!(stream.available(), 0);
    }
}
