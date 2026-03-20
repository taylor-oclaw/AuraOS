extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mcp_transport_stdio {
    use super::*;

    pub struct McpTransportStdio {
        buffer: Vec<u8>,
        position: usize,
    }

    impl McpTransportStdio {
        pub fn new() -> Self {
            McpTransportStdio {
                buffer: Vec::new(),
                position: 0,
            }
        }

        pub fn write(&mut self, data: &[u8]) {
            self.buffer.extend_from_slice(data);
        }

        pub fn read(&mut self, size: usize) -> Option<Vec<u8>> {
            if self.position + size > self.buffer.len() {
                None
            } else {
                let result = self.buffer[self.position..self.position + size].to_vec();
                self.position += size;
                Some(result)
            }
        }

        pub fn clear(&mut self) {
            self.buffer.clear();
            self.position = 0;
        }

        pub fn len(&self) -> usize {
            self.buffer.len()
        }

        pub fn is_empty(&self) -> bool {
            self.buffer.is_empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mcp_transport_stdio::McpTransportStdio;

    #[test]
    fn test_mcp_transport_stdio() {
        let mut transport = McpTransportStdio::new();
        assert!(transport.is_empty());
        assert_eq!(transport.len(), 0);

        let data = b"Hello, World!";
        transport.write(data);
        assert!(!transport.is_empty());
        assert_eq!(transport.len(), data.len());

        let read_data = transport.read(5).unwrap();
        assert_eq!(read_data, &data[..5]);

        let remaining_data = transport.read(7).unwrap();
        assert_eq!(remaining_data, &data[5..12]);

        assert!(transport.is_empty());
        assert_eq!(transport.len(), 0);

        transport.write(data);
        transport.clear();
        assert!(transport.is_empty());
        assert_eq!(transport.len(), 0);
    }
}
