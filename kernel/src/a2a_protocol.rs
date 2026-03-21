extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum A2AMessageType {
    Request,
    Response,
    Notify,
    Stream,
    Error,
}

pub struct A2AMessage {
    pub id: u64,
    pub from_agent: u64,
    pub to_agent: u64,
    pub msg_type: A2AMessageType,
    pub method: String,
    pub payload: Vec<u8>,
    pub correlation_id: Option<u64>,
    pub timestamp: u64,
}

pub struct A2AEndpoint {
    pub agent_id: u64,
    pub capabilities: Vec<String>,
    pub version: u32,
}

pub struct A2AProtocol {
    pub endpoints: Vec<A2AEndpoint>,
    pub messages: Vec<A2AMessage>,
    pub next_id: u64,
    pub delivered: u64,
    pub failed: u64,
}

impl A2AProtocol {
    pub fn new() -> Self {
        Self {
            endpoints: Vec::new(),
            messages: Vec::new(),
            next_id: 1,
            delivered: 0,
            failed: 0,
        }
    }

    pub fn register(&mut self, agent_id: u64, capabilities: Vec<String>) {
        self.endpoints.push(A2AEndpoint {
            agent_id,
            capabilities,
            version: 1,
        });
    }

    pub fn send(&mut self, from: u64, to: u64, method: &str, payload: Vec<u8>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        if self.endpoints.iter().any(|e| e.agent_id == to) {
            self.messages.push(A2AMessage {
                id,
                from_agent: from,
                to_agent: to,
                msg_type: A2AMessageType::Request,
                method: String::from(method),
                payload,
                correlation_id: None,
                timestamp: 0,
            });
            self.delivered += 1;
        } else {
            self.failed += 1;
        }
        id
    }

    pub fn reply(&mut self, original_id: u64, from: u64, to: u64, payload: Vec<u8>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.messages.push(A2AMessage {
            id,
            from_agent: from,
            to_agent: to,
            msg_type: A2AMessageType::Response,
            method: String::new(),
            payload,
            correlation_id: Some(original_id),
            timestamp: 0,
        });
        self.delivered += 1;
        id
    }

    pub fn find_capable(&self, capability: &str) -> Vec<u64> {
        self.endpoints.iter()
            .filter(|e| e.capabilities.iter().any(|c| c == capability))
            .map(|e| e.agent_id)
            .collect()
    }

    pub fn pending_for(&self, agent_id: u64) -> Vec<&A2AMessage> {
        self.messages.iter()
            .filter(|m| m.to_agent == agent_id)
            .collect()
    }

    pub fn total_messages(&self) -> usize {
        self.messages.len()
    }
}
