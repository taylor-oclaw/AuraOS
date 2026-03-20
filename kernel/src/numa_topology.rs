extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn numa_topology_init() {
    // Initialization logic for the NUMA topology module
}

#[no_mangle]
pub extern "C" fn numa_topology_exit() {
    // Cleanup logic for the NUMA topology module
}

pub struct NumaTopology {
    nodes: Vec<Node>,
}

impl NumaTopology {
    pub fn new() -> Self {
        NumaTopology { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node_id: u32, memory_size: u64) {
        let node = Node::new(node_id, memory_size);
        self.nodes.push(node);
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_total_memory(&self) -> u64 {
        self.nodes.iter().map(|node| node.memory_size).sum()
    }

    pub fn find_node_by_id(&self, node_id: u32) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == node_id)
    }
}

pub struct Node {
    id: u32,
    memory_size: u64,
}

impl Node {
    fn new(id: u32, memory_size: u64) -> Self {
        Node { id, memory_size }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_memory_size(&self) -> u64 {
        self.memory_size
    }
}
