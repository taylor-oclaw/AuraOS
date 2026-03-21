extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneReplyChainAware {
    chain: Vec<String>,
}

impl ToneReplyChainAware {
    pub fn new() -> Self {
        ToneReplyChainAware {
            chain: Vec::new(),
        }
    }

    pub fn add_reply(&mut self, reply: String) {
        self.chain.push(reply);
    }

    pub fn get_last_reply(&self) -> Option<&String> {
        self.chain.last()
    }

    pub fn remove_last_reply(&mut self) -> Option<String> {
        self.chain.pop()
    }

    pub fn clear_chain(&mut self) {
        self.chain.clear();
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }
}
