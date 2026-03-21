extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DeviceTree {
    nodes: Vec<Node>,
}

impl DeviceTree {
    pub fn new() -> Self {
        DeviceTree { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn get_node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }

    pub fn remove_node_by_name(&mut self, name: &str) {
        self.nodes.retain(|n| n.name != name);
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.iter().map(|n| n.name.clone()).collect()
    }
}

pub struct Node {
    name: String,
    properties: Vec<Property>,
}

impl Node {
    pub fn new(name: String, properties: Vec<Property>) -> Self {
        Node { name, properties }
    }

    pub fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    pub fn get_property_by_name(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    pub fn remove_property_by_name(&mut self, name: &str) {
        self.properties.retain(|p| p.name != name);
    }
}

pub struct Property {
    name: String,
    value: Vec<u8>,
}

impl Property {
    pub fn new(name: String, value: Vec<u8>) -> Self {
        Property { name, value }
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
}
