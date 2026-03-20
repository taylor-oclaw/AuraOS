extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod node {
    use super::*;

    pub struct Node {
        name: String,
        children: Vec<Node>,
        properties: Vec<(String, String)>,
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Node {
                name: String::from(name),
                children: Vec::new(),
                properties: Vec::new(),
            }
        }

        pub fn add_child(&mut self, child: Node) {
            self.children.push(child);
        }

        pub fn remove_child(&mut self, index: usize) -> Option<Node> {
            if index < self.children.len() {
                Some(self.children.remove(index))
            } else {
                None
            }
        }

        pub fn add_property(&mut self, key: &str, value: &str) {
            self.properties.push((String::from(key), String::from(value)));
        }

        pub fn get_property(&self, key: &str) -> Option<&String> {
            self.properties.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
        }

        pub fn find_node_by_name(&self, name: &str) -> Option<&Node> {
            if self.name == name {
                return Some(self);
            }
            for child in &self.children {
                if let Some(node) = child.find_node_by_name(name) {
                    return Some(node);
                }
            }
            None
        }
    }
}

pub use node::Node;
