extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NatGateway {
    rules: Vec<Rule>,
}

impl NatGateway {
    pub fn new() -> Self {
        NatGateway { rules: Vec::new() }
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

    pub fn match_packet(&self, packet: Packet) -> Option<&Rule> {
        for rule in &self.rules {
            if rule.matches(&packet) {
                return Some(rule);
            }
        }
        None
    }

    pub fn apply_rule(&self, packet: &mut Packet, rule: &Rule) {
        packet.src_ip = rule.new_src_ip.clone();
        packet.dst_ip = rule.new_dst_ip.clone();
    }
}

pub struct Rule {
    src_ip: String,
    dst_ip: String,
    new_src_ip: String,
    new_dst_ip: String,
}

impl Rule {
    pub fn new(src_ip: String, dst_ip: String, new_src_ip: String, new_dst_ip: String) -> Self {
        Rule { src_ip, dst_ip, new_src_ip, new_dst_ip }
    }

    pub fn matches(&self, packet: &Packet) -> bool {
        packet.src_ip == self.src_ip && packet.dst_ip == self.dst_ip
    }
}

pub struct Packet {
    src_ip: String,
    dst_ip: String,
    // Add other fields as necessary for a packet
}

impl Packet {
    pub fn new(src_ip: String, dst_ip: String) -> Self {
        Packet { src_ip, dst_ip }
    }
}
