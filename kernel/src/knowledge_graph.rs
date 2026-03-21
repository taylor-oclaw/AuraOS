extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub type NodeId = u64;
pub type AgentId = u64;

pub struct KnowledgeNode {
    pub id: NodeId,
    pub label: String,
    pub data: Vec<u8>,
    pub created_by: AgentId,
    pub trust_zone: u8,
    pub connections: Vec<(NodeId, String)>,
    pub access_count: u64,
    pub created_at: u64,
}

pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub next_id: NodeId,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_knowledge(&mut self, label: &str, data: Vec<u8>, agent: AgentId, zone: u8) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.push(KnowledgeNode {
            id,
            label: String::from(label),
            data,
            created_by: agent,
            trust_zone: zone,
            connections: Vec::new(),
            access_count: 0,
            created_at: 0,
        });
        id
    }

    pub fn connect(&mut self, from: NodeId, to: NodeId, relation: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == from) {
            node.connections.push((to, String::from(relation)));
        }
    }

    pub fn query(&mut self, label: &str, agent: AgentId, max_zone: u8) -> Vec<&KnowledgeNode> {
        self.nodes
            .iter()
            .filter(|n| n.label.contains(label) && n.trust_zone <= max_zone)
            .collect()
    }

    pub fn learn(&mut self, from_agent: AgentId, label: &str, data: Vec<u8>, zone: u8) -> NodeId {
        if let Some(existing) = self.nodes.iter().find(|n| n.label == label) {
            let id = existing.id;
            self.connect(id, id, "updated_by");
            id
        } else {
            self.add_knowledge(label, data, from_agent, zone)
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
