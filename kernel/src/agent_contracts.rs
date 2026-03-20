extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ContractStatus {
    Proposed,
    Accepted,
    Active,
    Completed,
    Breached,
    Expired,
}

pub struct AgentContract {
    pub id: u64,
    pub party_a: u64,
    pub party_b: u64,
    pub terms: String,
    pub status: ContractStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub deliverables: Vec<String>,
    pub completed_deliverables: Vec<u64>,
}

pub struct ContractManager {
    pub contracts: Vec<AgentContract>,
    pub next_id: u64,
}

impl ContractManager {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
            next_id: 1,
        }
    }

    pub fn propose(&mut self, from: u64, to: u64, terms: &str, expires: u64, deliverables: Vec<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.contracts.push(AgentContract {
            id,
            party_a: from,
            party_b: to,
            terms: String::from(terms),
            status: ContractStatus::Proposed,
            created_at: 0,
            expires_at: expires,
            deliverables,
            completed_deliverables: Vec::new(),
        });
        id
    }

    pub fn accept(&mut self, id: u64, agent_id: u64) -> bool {
        if let Some(c) = self.contracts.iter_mut().find(|c| c.id == id && c.party_b == agent_id) {
            c.status = ContractStatus::Active;
            true
        } else {
            false
        }
    }

    pub fn complete_deliverable(&mut self, id: u64, deliverable_idx: u64) {
        if let Some(c) = self.contracts.iter_mut().find(|c| c.id == id) {
            c.completed_deliverables.push(deliverable_idx);
            if c.completed_deliverables.len() >= c.deliverables.len() {
                c.status = ContractStatus::Completed;
            }
        }
    }

    pub fn breach(&mut self, id: u64) {
        if let Some(c) = self.contracts.iter_mut().find(|c| c.id == id) {
            c.status = ContractStatus::Breached;
        }
    }

    pub fn active_contracts(&self, agent_id: u64) -> Vec<&AgentContract> {
        self.contracts
            .iter()
            .filter(|c| matches!(c.status, ContractStatus::Active) && (c.party_a == agent_id || c.party_b == agent_id))
            .collect()
    }

    pub fn total(&self) -> usize {
        self.contracts.len()
    }
}
