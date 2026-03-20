extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CompressionModule {
    data: Vec<u8>,
}

impl CompressionModule {
    pub fn new() -> Self {
        CompressionModule { data: Vec::new() }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn compress(&mut self) -> Result<(), String> {
        // Simple compression logic: remove consecutive duplicate bytes
        if self.data.is_empty() {
            return Ok(());
        }

        let mut compressed = Vec::new();
        let mut last_byte = self.data[0];
        compressed.push(last_byte);

        for &byte in &self.data[1..] {
            if byte != last_byte {
                compressed.push(byte);
                last_byte = byte;
            }
        }

        self.data = compressed;
        Ok(())
    }

    pub fn decompress(&mut self) -> Result<(), String> {
        // Simple decompression logic: expand consecutive duplicate bytes
        if self.data.is_empty() {
            return Ok(());
        }

        let mut decompressed = Vec::new();
        for &byte in &self.data {
            decompressed.push(byte);
            decompressed.push(byte); // Duplicate each byte
        }

        self.data = decompressed;
        Ok(())
    }
}
