extern crate alloc;
use alloc::vec::Vec;

pub struct DmaChannel {
    pub id: u8,
    pub base_addr: u64,
    pub count: u32,
    pub in_use: bool,
    pub transfers_completed: u64,
}

pub struct DmaController {
    pub channels: Vec<DmaChannel>,
}

impl DmaController {
    pub fn new(max: u8) -> Self {
        let mut channels = Vec::new();
        for i in 0..max {
            channels.push(DmaChannel { id: i, base_addr: 0, count: 0, in_use: false, transfers_completed: 0 });
        }
        Self { channels }
    }
    pub fn allocate(&mut self) -> Option<u8> {
        for ch in &mut self.channels { if !ch.in_use { ch.in_use = true; return Some(ch.id); } }
        None
    }
    pub fn setup(&mut self, ch: u8, addr: u64, count: u32) -> bool {
        if let Some(c) = self.channels.iter_mut().find(|c| c.id == ch && c.in_use) { c.base_addr = addr; c.count = count; true } else { false }
    }
    pub fn complete(&mut self, ch: u8) {
        if let Some(c) = self.channels.iter_mut().find(|c| c.id == ch) { c.transfers_completed += 1; c.count = 0; }
    }
    pub fn release(&mut self, ch: u8) {
        if let Some(c) = self.channels.iter_mut().find(|c| c.id == ch) { c.in_use = false; c.base_addr = 0; }
    }
    pub fn active(&self) -> usize { self.channels.iter().filter(|c| c.in_use).count() }
}
