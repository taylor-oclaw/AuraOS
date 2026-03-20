extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Belief {
    pub id: u64,
    pub agent_id: u64,
    pub statement: String,
    pub confidence: f32,
    pub evidence: Vec<String>,
    pub version: u32,
    pub superseded_by: Option<u64>,
}

pub struct BeliefRevision {
    pub beliefs: Vec<Belief>,
    pub next_id: u64,
}

impl BeliefRevision {
    pub fn new() -> Self {
        Self { beliefs: Vec::new(), next_id: 1 }
    }

    pub fn assert_belief(&mut self, agent_id: u64, statement: &str, confidence: f32, evidence: Vec<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.beliefs.push(Belief {
            id, agent_id, statement: String::from(statement),
            confidence, evidence, version: 1, superseded_by: None,
        });
        id
    }

    pub fn revise(&mut self, belief_id: u64, new_confidence: f32, new_evidence: Vec<String>) -> Option<u64> {
        let new_id = self.next_id;
        self.next_id += 1;
        if let Some(old) = self.beliefs.iter_mut().find(|b| b.id == belief_id && b.superseded_by.is_none()) {
            let agent_id = old.agent_id;
            let statement = old.statement.clone();
            let version = old.version + 1;
            let mut all_evidence = old.evidence.clone();
            all_evidence.extend(new_evidence);
            old.superseded_by = Some(new_id);
            self.beliefs.push(Belief {
                id: new_id, agent_id, statement, confidence: new_confidence,
                evidence: all_evidence, version, superseded_by: None,
            });
            Some(new_id)
        } else { None }
    }

    pub fn current_beliefs(&self, agent_id: u64) -> Vec<&Belief> {
        self.beliefs.iter().filter(|b| b.agent_id == agent_id && b.superseded_by.is_none()).collect()
    }

    pub fn belief_history(&self, statement: &str) -> Vec<&Belief> {
        self.beliefs.iter().filter(|b| b.statement == statement).collect()
    }

    pub fn high_confidence(&self, threshold: f32) -> Vec<&Belief> {
        self.beliefs.iter().filter(|b| b.superseded_by.is_none() && b.confidence >= threshold).collect()
    }

    pub fn total(&self) -> usize { self.beliefs.len() }
}
