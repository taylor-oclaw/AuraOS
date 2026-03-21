extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum FirewallAction {
    Allow,
    Deny,
    Log,
}

pub enum Direction {
    Inbound,
    Outbound,
    Both,
}

pub struct FirewallRule {
    pub id: u64,
    pub name: String,
    pub direction: Direction,
    pub port: Option<u16>,
    pub protocol: String,
    pub action: FirewallAction,
    pub agent_id: Option<u64>,
    pub enabled: bool,
    pub hit_count: u64,
}

pub struct Firewall {
    pub rules: Vec<FirewallRule>,
    pub default_action: FirewallAction,
    pub enabled: bool,
    pub next_id: u64,
    pub blocked_count: u64,
    pub allowed_count: u64,
}

impl Firewall {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            default_action: FirewallAction::Deny,
            enabled: true,
            next_id: 1,
            blocked_count: 0,
            allowed_count: 0,
        }
    }

    pub fn add_rule(&mut self, name: &str, dir: Direction, port: Option<u16>, proto: &str, action: FirewallAction) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.rules.push(FirewallRule {
            id,
            name: String::from(name),
            direction: dir,
            port,
            protocol: String::from(proto),
            action,
            agent_id: None,
            enabled: true,
            hit_count: 0,
        };
        id
    }

    pub fn check(&mut self, port: u16, proto: &str, inbound: bool) -> bool {
        for rule in &mut self.rules {
            if !rule.enabled {
                continue;
            }
            let dir_match = match rule.direction {
                Direction::Inbound => inbound,
                Direction::Outbound => !inbound,
                Direction::Both => true,
            };
            let port_match = rule.port.map(|p| p == port).unwrap_or(true);
            let proto_match = rule.protocol == proto || rule.protocol == "*";
            if dir_match && port_match && proto_match {
                rule.hit_count += 1;
                return matches!(rule.action, FirewallAction::Allow);
            }
        }
        matches!(self.default_action, FirewallAction::Allow)
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
)}
