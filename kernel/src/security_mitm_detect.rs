extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_mitm_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_mitm_detect_exit() {
    // Cleanup logic for the module
}

pub struct SecurityMitmDetect {
    detected_attacks: Vec<String>,
    allowed_hosts: Vec<String>,
}

impl SecurityMitmDetect {
    pub fn new() -> Self {
        SecurityMitmDetect {
            detected_attacks: Vec::new(),
            allowed_hosts: Vec::new(),
        }
    }

    pub fn add_allowed_host(&mut self, host: &str) {
        self.allowed_hosts.push(host.to_string());
    }

    pub fn remove_allowed_host(&mut self, host: &str) -> bool {
        if let Some(index) = self.allowed_hosts.iter().position(|h| h == host) {
            self.allowed_hosts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_host_allowed(&self, host: &str) -> bool {
        self.allowed_hosts.contains(&host.to_string())
    }

    pub fn log_attack(&mut self, attack_description: &str) {
        self.detected_attacks.push(attack_description.to_string());
    }

    pub fn get_detected_attacks(&self) -> &[String] {
        &self.detected_attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_mitm_detect() {
        let mut detector = SecurityMitmDetect::new();
        assert_eq!(detector.allowed_hosts.len(), 0);
        assert_eq!(detector.get_detected_attacks().len(), 0);

        detector.add_allowed_host("192.168.1.1");
        assert_eq!(detector.allowed_hosts.len(), 1);
        assert!(detector.is_host_allowed("192.168.1.1"));

        detector.remove_allowed_host("192.168.1.1");
        assert_eq!(detector.allowed_hosts.len(), 0);
        assert!(!detector.is_host_allowed("192.168.1.1"));

        detector.log_attack("Man-in-the-middle attack detected on host 192.168.1.2");
        assert_eq!(detector.get_detected_attacks().len(), 1);
    }
}
