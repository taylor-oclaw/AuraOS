extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod checksum_engine {
    use super::*;

    pub struct ChecksumEngine {
        data: Vec<u8>,
        checksum: u32,
    }

    impl ChecksumEngine {
        pub fn new() -> Self {
            ChecksumEngine {
                data: Vec::new(),
                checksum: 0,
            }
        }

        pub fn add_data(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
            self.update_checksum();
        }

        pub fn get_checksum(&self) -> u32 {
            self.checksum
        }

        pub fn clear(&mut self) {
            self.data.clear();
            self.checksum = 0;
        }

        pub fn verify_data(&self, bytes: &[u8]) -> bool {
            let mut temp_engine = ChecksumEngine::new();
            temp_engine.add_data(bytes);
            temp_engine.get_checksum() == self.checksum
        }

        pub fn get_data(&self) -> &Vec<u8> {
            &self.data
        }

        fn update_checksum(&mut self) {
            self.checksum = 0;
            for byte in &self.data {
                self.checksum = self.checksum.wrapping_add(*byte as u32);
            }
        }
    }
}
