extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DecisionNode {
    label: String,
    children: Vec<DecisionNode>,
    value: Option<String>,
}

pub struct AgentDecisionTree {
    root: Option<DecisionNode>,
}

impl AgentDecisionTree {
    pub fn new() -> Self {
        AgentDecisionTree { root: None }
    }

    pub fn set_root(&mut self, label: &str) {
        self.root = Some(DecisionNode {
            label: String::from(label),
            children: Vec::new(),
            value: None,
        });
    }

    pub fn add_child(&mut self, parent_label: &str, child_label: &str) {
        if let Some(ref mut root) = self.root {
            Self::add_child_recursive(root, parent_label, child_label);
        }
    }

    fn add_child_recursive(node: &mut DecisionNode, parent: &str, child: &str) {
        if node.label == parent {
            node.children.push(DecisionNode {
                label: String::from(child),
                children: Vec::new(),
                value: None,
            });
            return;
        }
        for c in node.children.iter_mut() {
            Self::add_child_recursive(c, parent, child);
        }
    }

    pub fn set_value(&mut self, label: &str, value: &str) {
        if let Some(ref mut root) = self.root {
            Self::set_value_recursive(root, label, value);
        }
    }

    fn set_value_recursive(node: &mut DecisionNode, label: &str, value: &str) {
        if node.label == label {
            node.value = Some(String::from(value));
            return;
        }
        for c in node.children.iter_mut() {
            Self::set_value_recursive(c, label, value);
        }
    }

    pub fn depth(&self) -> usize {
        match &self.root {
            Some(root) => Self::calc_depth(root),
            None => 0,
        }
    }

    fn calc_depth(node: &DecisionNode) -> usize {
        if node.children.is_empty() {
            return 1;
        }
        let mut max_d = 0;
        for c in &node.children {
            let d = Self::calc_depth(c);
            if d > max_d { max_d = d; }
        }
        max_d + 1
    }
}
