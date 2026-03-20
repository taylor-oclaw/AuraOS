extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MemoryItem {
    pub id: u64,
    pub content: String,
    pub created_at: u64,
    pub last_accessed: u64,
    pub access_count: u32,
    pub strength: f32,
    pub tags: Vec<String>,
    pub consolidated: bool
}

pub struct MemoryDecay {
    pub memories: Vec<MemoryItem>,
    pub next_id: u64,
    pub decay_rate: f32,
    pub consolidation_threshold: f32,
    pub min_strength: f32
}

impl MemoryDecay {
    pub fn new() -> Self {
        Self {
            memories: Vec::new(),
            next_id: 1,
            decay_rate: 0.05,
            consolidation_threshold: 0.8,
            min_strength: 0.01
        }
    }

    pub fn store(&mut self, content: &str, tags: Vec<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.memories.push(MemoryItem {
            id,
            content: String::from(content),
            created_at: 0,
            last_accessed: 0,
            access_count: 0,
            strength: 1.0,
            tags,
            consolidated: false
        });
        id
    }

    pub fn recall(&mut self, id: u64) -> Option<&MemoryItem> {
        if let Some(mem) = self.memories.iter_mut().find(|m| m.id == id) {
            mem.access_count += 1;
            mem.strength = (mem.strength + 0.2).min(1.0);
            mem.last_accessed = 0;
        }
        self.memories.iter().find(|m| m.id == id)
    }

    pub fn decay_all(&mut self, current_time: u64) {
        let rate = self.decay_rate;
        let min = self.min_strength;
        for mem in &mut self.memories {
            let age = current_time.saturating_sub(mem.last_accessed);
            let decay = rate * (age as f32 / 3600.0);
            mem.strength = (mem.strength - decay).max(min);
        }
    }

    pub fn consolidate(&mut self) {
        let threshold = self.consolidation_threshold;
        for mem in &mut self.memories {
            if mem.strength >= threshold && mem.access_count > 5 {
                mem.consolidated = true;
            }
        }
    }

    pub fn prune(&mut self) {
        let min = self.min_strength * 2.0;
        self.memories.retain(|m| m.strength > min || m.consolidated);
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<&MemoryItem> {
        self.memories.iter().filter(|m| m.tags.iter().any(|t| t == tag)).collect()
    }

    pub fn strong_memories(&self) -> Vec<&MemoryItem> {
        self.memories.iter().filter(|m| m.strength > 0.5).collect()
    }

    pub fn total(&self) -> usize {
        self.memories.len()
    }
}
