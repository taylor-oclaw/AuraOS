extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TreeNode {
    value: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(value: &str) -> Self {
        TreeNode {
            value: String::from(value),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_children(&self) -> &Vec<TreeNode> {
        &self.children
    }

    pub fn find_node(&self, target_value: &str) -> Option<&TreeNode> {
        if self.value == target_value {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found_node) = child.find_node(target_value) {
                return Some(found_node);
            }
        }
        None
    }

    pub fn traverse(&self, visitor: &mut dyn FnMut(&TreeNode)) {
        visitor(self);
        for child in &self.children {
            child.traverse(visitor);
        }
    }
}
