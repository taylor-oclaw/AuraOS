extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TrustDelegationChain {
    chain: Vec<String>,
}

impl TrustDelegationChain {
    pub fn new() -> Self {
        TrustDelegationChain { chain: Vec::new() }
    }

    pub fn add_trustee(&mut self, trustee: &str) {
        self.chain.push(String::from(trustee));
    }

    pub fn remove_trustee(&mut self, trustee: &str) {
        if let Some(pos) = self.chain.iter().position(|t| t == trustee) {
            self.chain.remove(pos);
        }
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn is_trusted(&self, trustee: &str) -> bool {
        self.chain.contains(&String::from(trustee))
    }

    pub fn list_trustees(&self) -> Vec<String> {
        self.chain.clone()
    }
}
