extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn packet_filter_init() -> i32 {
    // Initialization logic for the packet filter module
    0
}

pub extern "C" fn packet_filter_exit() {
    // Cleanup logic for the packet filter module
}

pub struct PacketFilter {
    rules: Vec<String>,
}

impl PacketFilter {
    pub fn new() -> Self {
        PacketFilter { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<String> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn list_rules(&self) -> Vec<&String> {
        self.rules.iter().collect()
    }

    pub fn match_packet(&self, packet: &str) -> bool {
        for rule in &self.rules {
            if packet.contains(rule) {
                return true;
            }
        }
        false
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_filter() {
        let mut filter = PacketFilter::new();
        assert_eq!(filter.list_rules().len(), 0);

        filter.add_rule(String::from("rule1"));
        filter.add_rule(String::from("rule2"));
        assert_eq!(filter.list_rules().len(), 2);

        assert!(filter.match_packet("This packet contains rule1"));
        assert!(!filter.match_packet("This packet does not contain any rules"));

        let removed = filter.remove_rule(0);
        assert_eq!(removed, Some(String::from("rule1")));
        assert_eq!(filter.list_rules().len(), 1);

        filter.clear_rules();
        assert_eq!(filter.list_rules().len(), 0);
    }
}
