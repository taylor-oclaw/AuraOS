extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SdCardHandler {
    card_present: bool,
    capacity: u64,
    block_size: u32,
    data_buffer: Vec<u8>,
}

impl SdCardHandler {
    pub fn new() -> Self {
        SdCardHandler {
            card_present: false,
            capacity: 0,
            block_size: 512, // Standard SD card block size
            data_buffer: Vec::new(),
        }
    }

    pub fn is_card_present(&self) -> bool {
        self.card_present
    }

    pub fn set_card_present(&mut self, present: bool) {
        self.card_present = present;
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }

    pub fn set_capacity(&mut self, capacity: u64) {
        self.capacity = capacity;
    }

    pub fn read_block(&mut self, block_number: u32) -> Result<(), &'static str> {
        if !self.card_present {
            return Err("SD card not present");
        }
        if block_number as u64 * self.block_size as u64 > self.capacity {
            return Err("Block number out of range");
        }

        // Simulate reading a block from the SD card
        self.data_buffer.resize(self.block_size as usize, 0);
        for i in 0..self.block_size {
            self.data_buffer[i as usize] = (block_number + i) as u8;
        }
        Ok(())
    }

    pub fn write_block(&mut self, block_number: u32, data: &[u8]) -> Result<(), &'static str> {
        if !self.card_present {
            return Err("SD card not present");
        }
        if block_number as u64 * self.block_size as u64 > self.capacity {
            return Err("Block number out of range");
        }
        if data.len() != self.block_size as usize {
            return Err("Data size does not match block size");
        }

        // Simulate writing a block to the SD card
        for i in 0..self.block_size {
            self.data_buffer[i as usize] = data[i as usize];
        }
        Ok(())
    }

    pub fn get_block_size(&self) -> u32 {
        self.block_size
    }
}
