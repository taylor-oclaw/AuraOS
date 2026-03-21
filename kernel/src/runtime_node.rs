extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimeNode {
    name: String,
    children: Vec<RuntimeNode>,
    data: Option<String>,
}

impl RuntimeNode {
    pub fn new(name: &str) -> Self {
        RuntimeNode {
            name: String::from(name),
            children: Vec::new(),
            data: None,
        }
    }

    pub fn add_child(&mut self, child: RuntimeNode) {
        self.children.push(child);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }

    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }

    pub fn find_child_by_name(&self, name: &str) -> Option<&RuntimeNode> {
        for child in &self.children {
            if child.name == name {
                return Some(child);
            }
        }
        None
    }
}
