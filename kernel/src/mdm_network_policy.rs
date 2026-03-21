extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct NetworkPolicy {
    rules: Vec<Rule>,
}

impl NetworkPolicy {
    pub fn new() -> Self {
        NetworkPolicy { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<Rule> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn is_allowed(&self, packet: &Packet) -> bool {
        for rule in &self.rules {
            if rule.matches(packet) {
                return rule.action == Action::Allow;
            }
        }
        false
    }

    pub fn update_rule(&mut self, index: usize, new_rule: Rule) -> Option<Rule> {
        if index < self.rules.len() {
            Some(core::mem::replace(&mut self.rules[index], new_rule))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Rule {
    src_ip: String,
    dst_ip: String,
    action: Action,
}

impl Rule {
    pub fn new(src_ip: String, dst_ip: String, action: Action) -> Self {
        Rule { src_ip, dst_ip, action }
    }

    pub fn matches(&self, packet: &Packet) -> bool {
        self.src_ip == packet.src_ip && self.dst_ip == packet.dst_ip
    }
}

#[derive(Clone)]
pub enum Action {
    Allow,
    Deny,
}

#[repr(C)]
pub struct Packet {
    src_ip: String,
    dst_ip: String,
    // Add other fields as necessary
}

impl Packet {
    pub fn new(src_ip: String, dst_ip: String) -> Self {
        Packet { src_ip, dst_ip }
    }
}
