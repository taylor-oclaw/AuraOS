extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct NativeContinuousBatching {
    data: Vec<u8>,
    batch_size: usize,
    current_index: usize,
}

impl NativeContinuousBatching {
    pub fn new(batch_size: usize) -> Self {
        NativeContinuousBatching {
            data: Vec::new(),
            batch_size,
            current_index: 0,
        }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn get_next_batch(&mut self) -> Option<Vec<u8>> {
        if self.current_index >= self.data.len() {
            return None;
        }

        let end = (self.current_index + self.batch_size).min(self.data.len());
        let batch = self.data[self.current_index..end].to_vec();
        self.current_index += self.batch_size;

        Some(batch)
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_continuous_batching() {
        let mut batching = NativeContinuousBatching::new(3);
        batching.add_data(b"abcdefg");

        assert_eq!(batching.get_next_batch(), Some(vec![97, 98, 99]));
        assert_eq!(batching.get_next_batch(), Some(vec![100, 101, 102]));
        assert_eq!(batching.get_next_batch(), Some(vec![103]));
        assert_eq!(batching.get_next_batch(), None);

        batching.reset();
        assert_eq!(batching.get_next_batch(), Some(vec![97, 98, 99]));

        assert_eq!(batching.data_len(), 7);
        assert_eq!(batching.batch_size(), 3);
    }
}