extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CacheEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub hits: u64,
    pub tier: u8,
    pub created: u64,
    pub last_hit: u64,
    pub size_bytes: usize
}

pub struct TieredCache {
    pub l1_hot: Vec<CacheEntry>,
    pub l2_warm: Vec<CacheEntry>,
    pub l3_cold: Vec<CacheEntry>,
    pub l4_archive: Vec<CacheEntry>,
    pub l5_disk: Vec<CacheEntry>,
    pub l1_max: usize,
    pub l2_max: usize,
    pub l3_max: usize,
    pub total_hits: u64,
    pub total_misses: u64
}

impl TieredCache {
    pub fn new() -> Self {
        Self {
            l1_hot: Vec::new(),
            l2_warm: Vec::new(),
            l3_cold: Vec::new(),
            l4_archive: Vec::new(),
            l5_disk: Vec::new(),
            l1_max: 100,
            l2_max: 500,
            l3_max: 2000,
            total_hits: 0,
            total_misses: 0
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
        for tier in [&mut self.l1_hot, &mut self.l2_warm, &mut self.l3_cold, &mut self.l4_archive, &mut self.l5_disk] {
            if let Some(entry) = tier.iter_mut().find(|e| e.key == key) {
                entry.hits += 1;
                self.total_hits += 1;
                return Some(&entry.value);
            }
        }
        self.total_misses += 1;
        None
    }

    pub fn put(&mut self, key: &str, value: Vec<u8>) {
        let size = value.len();
        self.l1_hot.push(CacheEntry {
            key: String::from(key),
            value,
            hits: 0,
            tier: 1,
            created: 0,
            last_hit: 0,
            size_bytes: size
        };
        if self.l1_hot.len() > self.l1_max {
            if let Some(evicted) = self.l1_hot.pop() {
                self.l2_warm.push(evicted);
            }
        }
    }

    pub fn promote(&mut self, key: &str) {
        if let Some(idx) = self.l2_warm.iter().position(|e| e.key == key) {
            let entry = self.l2_warm.remove(idx);
            self.l1_hot.push(entry);
        }
    }

    pub fn hit_rate(&self) -> f32 {
        let total = self.total_hits + self.total_misses;
        if total == 0 {
            0.0
        } else {
            self.total_hits as f32 / total as f32
        }
    }

    pub fn total_entries(&self) -> usize {
        self.l1_hot.len() + self.l2_warm.len() + self.l3_cold.len() + self.l4_archive.len() + self.l5_disk.len()
    }
)}
