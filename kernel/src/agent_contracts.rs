extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentContracts {
    contracts: Vec<Contract>,
}

struct Contract {
    agent_id: String,
    terms: Vec<String>,
    active: bool,
}

impl AgentContracts {
    pub fn new() -> Self {
        AgentContracts { contracts: Vec::new() }
    }

    pub fn create_contract(&mut self, agent_id: &str) {
        self.contracts.push(Contract {
            agent_id: String::from(agent_id),
            terms: Vec::new(),
            active: true,
        });
    }

    pub fn add_term(&mut self, agent_id: &str, term: &str) {
        for contract in self.contracts.iter_mut() {
            if contract.agent_id == agent_id {
                contract.terms.push(String::from(term));
                return;
            }
        }
    }

    pub fn deactivate(&mut self, agent_id: &str) {
        for contract in self.contracts.iter_mut() {
            if contract.agent_id == agent_id {
                contract.active = false;
                return;
            }
        }
    }

    pub fn is_active(&self, agent_id: &str) -> bool {
        for contract in &self.contracts {
            if contract.agent_id == agent_id {
                return contract.active;
            }
        }
        false
    }

    pub fn count(&self) -> usize {
        self.contracts.len()
    }
}
