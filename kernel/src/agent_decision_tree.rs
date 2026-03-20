extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentDecisionTree {
    root: Node,
}

impl AgentDecisionTree {
    pub fn new() -> Self {
        AgentDecisionTree {
            root: Node::new("root"),
        }
    }

    pub fn add_decision(&mut self, parent_label: &str, decision: Decision) {
        if let Some(parent_node) = self.find_node_by_label(&self.root, parent_label) {
            parent_node.add_child(decision.into());
        }
    }

    pub fn make_decision(&self, label: &str) -> Option<&Decision> {
        self.find_node_by_label(&self.root, label)
            .and_then(|node| node.decision.as_ref())
    }

    pub fn get_all_decisions(&self) -> Vec<&Decision> {
        let mut decisions = Vec::new();
        self.collect_decisions(&self.root, &mut decisions);
        decisions
    }

    pub fn remove_decision(&mut self, label: &str) {
        self.remove_node_by_label(&mut self.root, label);
    }
}

struct Node {
    label: String,
    decision: Option<Decision>,
    children: Vec<Node>,
}

impl Node {
    fn new(label: &str) -> Self {
        Node {
            label: String::from(label),
            decision: None,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn get_child_by_label(&self, label: &str) -> Option<&Node> {
        self.children.iter().find(|child| child.label == label)
    }
}

struct Decision {
    action: String,
    outcome: String,
}

impl From<Decision> for Node {
    fn from(decision: Decision) -> Self {
        let mut node = Node::new(&decision.action);
        node.decision = Some(decision);
        node
    }
}

impl AgentDecisionTree {
    fn find_node_by_label(&self, node: &Node, label: &str) -> Option<&Node> {
        if node.label == label {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found_node) = self.find_node_by_label(child, label) {
                return Some(found_node);
            }
        }
        None
    }

    fn collect_decisions(&self, node: &Node, decisions: &mut Vec<&Decision>) {
        if let Some(decision) = &node.decision {
            decisions.push(decision);
        }
        for child in &node.children {
            self.collect_decisions(child, decisions);
        }
    }

    fn remove_node_by_label(&mut self, node: &mut Node, label: &str) {
        node.children.retain(|child| {
            if child.label == label {
                false
            } else {
                self.remove_node_by_label(child, label);
                true
            }
        });
    }
}
