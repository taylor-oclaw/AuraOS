extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct IpcMessage {
    pub id: u64,
    pub from_agent: u64,
    pub to_agent: u64,
    pub channel: String,
    pub payload: Vec<u8>,
    pub encrypted: bool,
    pub timestamp: u64,
    pub acknowledged: bool
}

pub struct IpcChannel {
    pub name: String,
    pub owner: u64,
    pub subscribers: Vec<u64>,
    pub messages: Vec<IpcMessage>,
    pub max_messages: usize
}

pub struct SecureIpc {
    pub channels: Vec<IpcChannel>,
    pub next_msg_id: u64
}

impl SecureIpc {
    pub fn new() -> Self {
        Self { 
            channels: Vec::new(), 
            next_msg_id: 1 
        }
    }

    pub fn create_channel(&mut self, name: &str, owner: u64) {
        self.channels.push(IpcChannel {
            name: String::from(name),
            owner,
            subscribers: Vec::new(),
            messages: Vec::new(),
            max_messages: 1000
        };
    }

    pub fn subscribe(&mut self, channel: &str, agent_id: u64) -> bool {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.name == channel) {
            if !ch.subscribers.contains(&agent_id) {
                ch.subscribers.push(agent_id);
            }
            true
        } else {
            false
        }
    }

    pub fn send(&mut self, channel: &str, from: u64, payload: Vec<u8>) -> Option<u64> {
        let id = self.next_msg_id;
        self.next_msg_id += 1;

        if let Some(ch) = self.channels.iter_mut().find(|c| c.name == channel) {
            for sub in &ch.subscribers {
                ch.messages.push(IpcMessage {
                    id,
                    from_agent: from,
                    to_agent: *sub,
                    channel: String::from(channel),
                    payload: payload.clone(),
                    encrypted: true,
                    timestamp: 0,
                    acknowledged: false
                };
            }

            if ch.messages.len() > ch.max_messages {
                ch.messages.drain(0..ch.messages.len() - ch.max_messages);
            }
            Some(id)
        } else {
            None
        }
    }

    pub fn receive(&mut self, agent_id: u64, channel: &str) -> Vec<&IpcMessage> {
        if let Some(ch) = self.channels.iter().find(|c| c.name == channel) {
            ch.messages.iter().filter(|m| m.to_agent == agent_id && !m.acknowledged).collect()
        } else {
            Vec::new()
        }
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }
))}
