extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct FileCompression {
    data: Vec<u8>,
}

impl FileCompression {
    pub fn new(data: Vec<u8>) -> Self {
        FileCompression { data }
    }

    pub fn compress(&mut self) -> Result<(), &'static str> {
        // Simple run-length encoding for demonstration
        if self.data.is_empty() {
            return Err("Data is empty");
        }

        let mut compressed = Vec::new();
        let mut count = 1;
        let mut prev = self.data[0];

        for &byte in &self.data[1..] {
            if byte == prev {
                count += 1;
            } else {
                compressed.push(prev);
                compressed.push(count as u8);
                prev = byte;
                count = 1;
            }
        }

        // Push the last run
        compressed.push(prev);
        compressed.push(count as u8);

        self.data = compressed;
        Ok(())
    }

    pub fn decompress(&mut self) -> Result<(), &'static str> {
        if self.data.len() % 2 != 0 {
            return Err("Invalid compressed data");
        }

        let mut decompressed = Vec::new();
        for i in (0..self.data.len()).step_by(2) {
            let byte = self.data[i];
            let count = self.data[i + 1] as usize;
            decompressed.extend(vec![byte; count]);
        }

        self.data = decompressed;
        Ok(())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn is_compressed(&self) -> bool {
        // Simple heuristic to check if data is compressed
        // This is not a reliable method for actual compression formats
        self.data.windows(2).any(|w| w[0] == w[1])
    }
}
