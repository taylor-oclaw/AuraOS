extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CoreDump {
    data: Vec<u8>,
    name: String,
}

impl CoreDump {
    pub fn new(name: &str) -> Self {
        CoreDump {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, chunk: &[u8]) {
        self.data.extend_from_slice(chunk);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend_from_slice(self.name.as_bytes());
        serialized.push(0); // Null terminator for the string
        serialized.extend_from_slice(&self.data);
        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_dump() {
        let mut core_dump = CoreDump::new("test_core");
        assert_eq!(core_dump.get_name(), "test_core");
        assert_eq!(core_dump.get_size(), 0);

        let data_chunk = b"some_data";
        core_dump.add_data(data_chunk);
        assert_eq!(core_dump.get_size(), data_chunk.len());

        let serialized = core_dump.serialize();
        assert_eq!(&serialized[..8], b"test_core\0");
        assert_eq!(&serialized[9..], data_chunk);

        core_dump.clear_data();
        assert_eq!(core_dump.get_size(), 0);
    }
}
