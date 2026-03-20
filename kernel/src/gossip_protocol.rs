extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GossipMessage {
    pub from_agent: u64,
    pub topic: String,
    pub payload: String,
    pub hop_count: u8,
    pub max_hops: u8,
    pub seen_by: Vec<u64>,
    pub timestamp: u64,
}

pub struct GossipNode {
    pub agent_id: u64,
    pub peers: Vec<u64>,
    pub inbox: Vec<GossipMessage>,
    pub outbox: Vec<GossipMessage>,
}

pub struct GossipProtocol {
    pub nodes: Vec<GossipNode>,
    pub fan_out: usize,
    pub max_hops: u8,
    pub messages_total: u64,
}

impl GossipProtocol {
    pub fn new(fan_out: usize) -> Self {
        Self {
            nodes: Vec::new(),
            fan_out,
            max_hops: 5,
            messages_total: 0,
        }
    }

    pub fn register_node(&mut self, agent_id: u64) {
        self.nodes.push(GossipNode {
            agent_id,
            peers: Vec::new(),
            inbox: Vec::new(),
            outbox: Vec::new(),
        });
    }

    pub fn add_peer(&mut self, agent_id: u64, peer_id: u64) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.agent_id == agent_id) {
            if !node.peers.contains(&peer_id) {
                node.peers.push(peer_id);
            }
        }
    }

    pub fn broadcast(&mut self, from: u64, topic: &str, payload: &str) {
        let msg = GossipMessage {
            from_agent: from,
            topic: String::from(topic),
            payload: String::from(payload),
            hop_count: 0,
            max_hops: self.max_hops,
            seen_by: vec![from],
            timestamp: 0,
        };
        if let Some(node) = self.nodes.iter_mut().find(|n| n.agent_id == from) {
            node.outbox.push(msg);
        }
        self.messages_total += 1;
    }

    pub fn tick(&mut self) {
        let fan_out = self.fan_out;
        for node in &mut self.nodes {
            while let Some(msg) = node.outbox.pop() {
                if msg.hop_count < msg.max_hops {
                    for peer_id in node.peers.iter().take(fan_out) {
                        if !msg.seen_by.contains(peer_id) {
                            node.inbox.push(GossipMessage {
                                from_agent: msg.from_agent,
                                topic: msg.topic.clone(),
                                payload: msg.payload.clone(),
                                hop_count: msg.hop_count + 1,
                                max_hops: msg.max_hops,
                                seen_by: msg.seen_by.clone(),
                                timestamp: msg.timestamp,
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
