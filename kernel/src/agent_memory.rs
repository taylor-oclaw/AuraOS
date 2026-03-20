extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MemoryEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub created_at: u64,
    pub last_accessed: u64,
    pub access_count: u64,
    pub persistent: bool,
    pub agent_id: u64,
}

pub struct AgentMemoryStore {
    pub entries: Vec<MemoryEntry>,
    pub max_per_agent: usize,
}

impl AgentMemoryStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_per_agent: 1000,
        }
    }

    pub fn store(&mut self, agent_id: u64, key: &str, value: Vec<u8>, persistent: bool) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.agent_id == agent_id && e.key == key) {
            e.value = value;
            e.last_accessed = 0;
        } else {
            self.entries.push(MemoryEntry {
                key: String::from(key),
                value,
                created_at: 0,
                last_accessed: 0,
                access_count: 0,
                persistent,
                agent_id,
            });
        }
    }

    pub fn recall(&mut self, agent_id: u64, key: &str) -> Option<&[u8]> {
        if let Some(e) = self.entries.iter_mut().find(|e| e.agent_id == agent_id && e.key == key) {
            e.access_count += 1;
            e.last_accessed = 0;
            Some(&e.value)
        } else {
            None
        }
    }

    pub fn forget(&mut self, agent_id: u64, key: &str) {
        self.entries.retain(|e| !(e.agent_id == agent_id && e.key == key));
    }

    pub fn agent_memories(&self, agent_id: u64) -> Vec<&MemoryEntry> {
        self.entries.iter().filter(|e| e.agent_id == agent_id).collect()
    }

    pub fn persist_all(&self) -> Vec<&MemoryEntry> {
        self.entries.iter().filter(|e| e.persistent).collect()
    }

    pub fn cleanup_volatile(&mut self) {
        self.entries.retain(|e| e.persistent);
    }

    pub fn total_size(&self) -> usize {
        self.entries.iter().map(|e| e.value.len()).sum()
    }
}
