extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ReputationEntry {
    pub from_agent: u64,
    pub to_agent: u64,
    pub score: i8,
    pub reason: String,
    pub timestamp: u64,
}

pub struct AgentReputation {
    pub entries: Vec<ReputationEntry>,
    pub cache: Vec<(u64, f32)>,
}

impl AgentReputation {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cache: Vec::new(),
        }
    }

    pub fn rate(&mut self, from: u64, to: u64, score: i8, reason: &str) {
        self.entries.push(ReputationEntry {
            from_agent: from,
            to_agent: to,
            score,
            reason: String::from(reason),
            timestamp: 0,
        });
        self.update_cache(to);
    }

    fn update_cache(&mut self, agent_id: u64) {
        let ratings: Vec<i8> = self.entries.iter().filter(|e| e.to_agent == agent_id).map(|e| e.score).collect();
        if ratings.is_empty() {
            return;
        }
        let sum: i32 = ratings.iter().map(|s| *s as i32).sum();
        let avg = sum as f32 / ratings.len() as f32;
        if let Some(c) = self.cache.iter_mut().find(|(id, _)| *id == agent_id) {
            c.1 = avg;
        } else {
            self.cache.push((agent_id, avg));
        }
    }

    pub fn get_reputation(&self, agent_id: u64) -> f32 {
        self.cache.iter().find(|(id, _)| *id == agent_id).map(|(_, score)| *score).unwrap_or(0.0)
    }

    pub fn top_agents(&self, n: usize) -> Vec<(u64, f32)> {
        let mut sorted = self.cache.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
        sorted.truncate(n);
        sorted
    }

    pub fn reviews_for(&self, agent_id: u64) -> Vec<&ReputationEntry> {
        self.entries.iter().filter(|e| e.to_agent == agent_id).collect()
    }

    pub fn total_ratings(&self) -> usize {
        self.entries.len()
    }
}
