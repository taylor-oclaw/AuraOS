extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct RelContractExpireWarn {
    contracts: Vec<Contract>,
}

impl RelContractExpireWarn {
    pub fn new() -> Self {
        RelContractExpireWarn {
            contracts: Vec::new(),
        }
    }

    pub fn add_contract(&mut self, contract_id: String, expiration_date: u64) {
        let contract = Contract {
            id: contract_id,
            expiration_date,
        };
        self.contracts.push(contract);
    }

    pub fn remove_contract(&mut self, contract_id: &str) -> bool {
        if let Some(pos) = self.contracts.iter().position(|c| c.id == contract_id) {
            self.contracts.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_contracts(&self) -> &[Contract] {
        &self.contracts
    }

    pub fn is_contract_expired(&self, contract_id: &str, current_date: u64) -> bool {
        if let Some(contract) = self.contracts.iter().find(|c| c.id == contract_id) {
            contract.expiration_date < current_date
        } else {
            false
        }
    }

    pub fn get_expired_contracts(&self, current_date: u64) -> Vec<&Contract> {
        self.contracts
            .iter()
            .filter(|c| c.expiration_date < current_date)
            .collect()
    }
}

#[repr(C)]
struct Contract {
    id: String,
    expiration_date: u64,
}
