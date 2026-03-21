extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod blake3 {
    pub struct Blake3Hasher {
        state: [u64; 8],
        buffer: Vec<u8>,
        count: u64,
    }

    impl Blake3Hasher {
        pub fn new() -> Self {
            Blake3Hasher {
                state: [
                    0x67e60b20, 0x6a2c51d5, 0x915f04ad, 0xf851a308,
                    0xbe0cc441, 0xd88fb35b, 0x5e76e8ff, 0x8b428fa4,
                ],
                buffer: Vec::new(),
                count: 0,
            }
        }

        pub fn update(&mut self, data: &[u8]) {
            self.buffer.extend_from_slice(data);
            self.count += data.len() as u64;

            while self.buffer.len() >= 16384 {
                let chunk = self.buffer.split_off(16384);
                self.compress(&chunk, false);
            }
        }

        pub fn finalize(mut self) -> [u8; 32] {
            if !self.buffer.is_empty() {
                self.compress(&self.buffer, true);
            }

            let mut output = [0u8; 32];
            for i in 0..8 {
                output[i * 4..(i + 1) * 4].copy_from_slice(&self.state[i].to_le_bytes());
            }
            output
        }

        fn compress(&mut self, block: &[u8], last_block: bool) {
            // Simplified compression function for demonstration purposes
            let mut chunk_state = [0u64; 16];
            chunk_state[..block.len() / 8].copy_from_slice(bytemuck::cast_slice(block));

            if last_block {
                chunk_state[7] ^= self.count;
            }

            for i in 0..8 {
                self.state[i] ^= chunk_state[i];
            }
        }
    }
}

pub use blake3::Blake3Hasher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let mut hasher = Blake3Hasher::new();
        hasher.update(b"hello");
        let hash = hasher.finalize();

        assert_eq!(
            &hash,
            &[0x7a, 0x2e, 0x4b, 0x5c, 0x18, 0x9d, 0x3f, 0x64, 0x8f, 0x9b, 0x2b, 0x7a, 0x8e, 0x8e, 0x5c, 0x18]
        ;
    }
)}
