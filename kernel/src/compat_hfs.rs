extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HfsNode {
    name: String,
    parent: Option<Box<HfsNode>>,
}

impl HfsNode {
    pub fn new(name: &str) -> Self {
        HfsNode {
            name: String::from(name),
            parent: None,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_parent(&mut self, node: Box<HfsNode>) {
        self.parent = Some(node);
    }

    pub fn get_parent(&self) -> Option<&HfsNode> {
        self.parent.as_ref().map(|node| node.as_ref())
    }

    pub fn add_child(&mut self, child: Box<HfsNode>) {
        if let Some(parent) = self.get_parent() {
            parent.set_parent(child);
        }
    }

    pub fn get_children(&self) -> Vec<&HfsNode> {
        match self.parent {
            Some(node) => node.get_children(),
            None => vec![],
        }
    }
}

pub struct HfsFilesystem {
    root: Box<HfsNode>,
}

impl HfsFilesystem {
    pub fn new() -> Self {
        let root = Box::new(HfsNode::new("root"));
        HfsFilesystem { root }
    }

    pub fn get_root(&self) -> &HfsNode {
        self.root.as_ref()
    }
}