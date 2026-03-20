extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DmaChannel {
    pub id: u8,
    pub base_addr: u64,
    pub count: u32,
    pub in_use: bool,
    pub direction: DmaDirection,
    pub transfers_completed: u64,
}

pub enum DmaDirection { MemToDevice, DeviceToMem, MemToMem }

pub struct DmaController {
    pub channels: Vec<DmaChannel>,
    pub max_channels: u8,
}

impl DmaController {
    pub fn new(max: u8) -> Self {
        let mut channels = Vec::new();
        for i in 0..max {
            channels.push(DmaChannel {
                id: i, base_addr: 0, count: 0, in_use: false,
                direction: DmaDirection::MemToMem, transfers_completed: 0,
            });
        }
        Self { channels, max_channels: max }
    }

    pub fn allocate(&mut self) -> Option<u8> {
        for ch in &mut self.channels {
            if !ch.in_use { ch.in_use = true; return Some(ch.id); }
        }
        None
    }

    pub fn setup_transfer(&mut self, channel: u8, addr: u64, count: u32, dir: DmaDirection) -> bool {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.id == channel && c.in_use) {
            ch.base_addr = addr;
            ch.count = count;
            ch.direction = dir;
            true
        } else { false }
    }

    pub fn complete_transfer(&mut self, channel: u8) {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.id == channel) {
            ch.transfers_completed += 1;
            ch.count = 0;
        }
    }

    pub fn release(&mut self, channel: u8) {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.id == channel) {
            ch.in_use = false;
            ch.base_addr = 0;
            ch.count = 0;
        }
    }

    pub fn active_channels(&self) -> usize {
        self.channels.iter().filter(|c| c.in_use).count()
    }

    pub fn total_transfers(&self) -> u64 {
        self.channels.iter().map(|c| c.transfers_completed).sum()
    }
}
