extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DreamEntry {
    pub id: u64,
    pub agent_id: u64,
    pub pattern: String,
    pub frequency: u32,
    pub insight: Option<String>,
    pub consolidated: bool
}

pub struct DreamCycle {
    pub entries: Vec<DreamEntry>,
    pub cycle_count: u64,
    pub next_id: u64,
    pub is_dreaming: bool,
    pub consolidation_threshold: u32
}

impl DreamCycle {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cycle_count: 0,
            next_id: 1,
            is_dreaming: false,
            consolidation_threshold: 3
        }
    }

    pub fn start_dream(&mut self) {
        self.is_dreaming = true;
        self.cycle_count += 1;
    }

    pub fn end_dream(&mut self) {
        self.is_dreaming = false;
    }

    pub fn record_pattern(&mut self, agent_id: u64, pattern: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.pattern == pattern && e.agent_id == agent_id) {
            entry.frequency += 1;
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.entries.push(DreamEntry {
                id,
                agent_id,
                pattern: String::from(pattern),
                frequency: 1,
                insight: None,
                consolidated: false
            };
        }
    }

    pub fn consolidate(&mut self) -> Vec<&DreamEntry> {
        let threshold = self.consolidation_threshold;
        let mut consolidated: Vec<&DreamEntry> = Vec::new();
        for entry in &mut self.entries {
            if entry.frequency >= threshold && !entry.consolidated {
                entry.consolidated = true;
            }
        }
        self.entries.iter().filter(|e| e.consolidated).collect()
    }

    pub fn add_insight(&mut self, id: u64, insight: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.insight = Some(String::from(insight));
        }
    }

    pub fn patterns_for_agent(&self, agent_id: u64) -> Vec<&DreamEntry> {
        self.entries.iter().filter(|e| e.agent_id == agent_id).collect()
    }

    pub fn total_patterns(&self) -> usize {
        self.entries.len()
    }
)}
