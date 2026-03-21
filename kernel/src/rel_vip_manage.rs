extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_vip_manage_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_vip_manage_exit() {
    // Cleanup logic for the module
}

pub struct VIPManager {
    vips: Vec<String>,
}

impl VIPManager {
    pub fn new() -> Self {
        VIPManager { vips: Vec::new() }
    }

    pub fn add_vip(&mut self, vip: String) {
        if !self.vips.contains(&vip) {
            self.vips.push(vip);
        }
    }

    pub fn remove_vip(&mut self, vip: &str) {
        self.vips.retain(|v| v != vip);
    }

    pub fn is_vip(&self, vip: &str) -> bool {
        self.vips.contains(&String::from(vip))
    }

    pub fn list_vips(&self) -> Vec<String> {
        self.vips.clone()
    }

    pub fn count_vips(&self) -> usize {
        self.vips.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_manager() {
        let mut manager = VIPManager::new();
        assert_eq!(manager.count_vips(), 0);

        manager.add_vip(String::from("user1"));
        assert_eq!(manager.count_vips(), 1);
        assert!(manager.is_vip("user1"));

        manager.add_vip(String::from("user2"));
        assert_eq!(manager.count_vips(), 2);
        assert!(manager.is_vip("user2"));

        manager.remove_vip("user1");
        assert_eq!(manager.count_vips(), 1);
        assert!(!manager.is_vip("user1"));

        let vips = manager.list_vips();
        assert_eq!(vips.len(), 1);
        assert_eq!(vips[0], "user2");

        manager.remove_vip("user2");
        assert_eq!(manager.count_vips(), 0);
    }
}
