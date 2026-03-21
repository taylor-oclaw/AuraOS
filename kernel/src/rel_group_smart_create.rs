extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_group_smart_create_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_group_smart_create_exit() {
    // Cleanup logic for the module
}

pub struct RelGroupSmartCreate {
    name: String,
    members: Vec<String>,
    max_members: usize,
}

impl RelGroupSmartCreate {
    pub fn new(name: &str, max_members: usize) -> Self {
        RelGroupSmartCreate {
            name: String::from(name),
            members: Vec::new(),
            max_members,
        }
    }

    pub fn add_member(&mut self, member_name: &str) -> bool {
        if self.members.len() < self.max_members {
            self.members.push(String::from(member_name));
            true
        } else {
            false
        }
    }

    pub fn remove_member(&mut self, member_name: &str) -> bool {
        let pos = self.members.iter().position(|m| m == member_name);
        if let Some(index) = pos {
            self.members.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_members(&self) -> &[String] {
        &self.members
    }

    pub fn is_full(&self) -> bool {
        self.members.len() >= self.max_members
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_creation() {
        let mut group = RelGroupSmartCreate::new("TestGroup", 3);
        assert_eq!(group.name(), "TestGroup");
        assert!(!group.is_full());
    }

    #[test]
    fn test_add_member() {
        let mut group = RelGroupSmartCreate::new("TestGroup", 2);
        assert!(group.add_member("Alice"));
        assert!(group.add_member("Bob"));
        assert!(!group.add_member("Charlie")); // Should fail, group is full
        assert_eq!(group.get_members().len(), 2);
    }

    #[test]
    fn test_remove_member() {
        let mut group = RelGroupSmartCreate::new("TestGroup", 3);
        group.add_member("Alice");
        group.add_member("Bob");
        assert!(group.remove_member("Alice"));
        assert_eq!(group.get_members().len(), 1);
        assert!(!group.remove_member("Charlie")); // Should fail, Charlie is not in the group
    }

    #[test]
    fn test_is_full() {
        let mut group = RelGroupSmartCreate::new("TestGroup", 2);
        assert!(!group.is_full());
        group.add_member("Alice");
        group.add_member("Bob");
        assert!(group.is_full());
    }
}
