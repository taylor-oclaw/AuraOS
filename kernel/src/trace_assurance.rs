extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TraceEntry {
    pub id: u64,
    pub agent_id: u64,
    pub action: String,
    pub input_hash: [u8; 16],
    pub output_hash: [u8; 16],
    pub timestamp: u64,
    pub verified: bool,
    pub parent_trace: Option<u64>,
}

pub struct TraceAssurance {
    pub traces: Vec<TraceEntry>,
    pub next_id: u64,
    pub enabled: bool,
}

impl TraceAssurance {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            next_id: 1,
            enabled: true,
        }
    }

    pub fn record(&mut self, agent_id: u64, action: &str, input: &[u8], output: &[u8]) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut ihash = [0u8; 16];
        let mut ohash = [0u8; 16];
        for (i, b) in input.iter().enumerate().take(16) {
            ihash[i] = *b;
        }
        for (i, b) in output.iter().enumerate().take(16) {
            ohash[i] = *b;
        }
        self.traces.push(TraceEntry {
            id,
            agent_id,
            action: String::from(action),
            input_hash: ihash,
            output_hash: ohash,
            timestamp: 0,
            verified: false,
            parent_trace: None,
        };
        id
    }

    pub fn verify(&mut self, id: u64) -> bool {
        if let Some(t) = self.traces.iter_mut().find(|t| t.id == id) {
            t.verified = true;
            true
        } else {
            false
        }
    }

    pub fn chain(&mut self, parent_id: u64, child_id: u64) {
        if let Some(t) = self.traces.iter_mut().find(|t| t.id == child_id) {
            t.parent_trace = Some(parent_id);
        }
    }

    pub fn agent_trace(&self, agent_id: u64) -> Vec<&TraceEntry> {
        self.traces.iter().filter(|t| t.agent_id == agent_id).collect()
    }

    pub fn unverified(&self) -> Vec<&TraceEntry> {
        self.traces.iter().filter(|t| !t.verified).collect()
    }

    pub fn total(&self) -> usize {
        self.traces.len()
    }
)}
