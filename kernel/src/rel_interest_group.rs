extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_interest_group_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_interest_group_exit() {
    // Cleanup logic for the module
}

pub struct InterestGroup {
    name: String,
    members: Vec<String>,
}

impl InterestGroup {
    pub fn new(name: &str) -> Self {
        InterestGroup {
            name: String::from(name),
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: &str) {
        if !self.members.contains(&String::from(member)) {
            self.members.push(String::from(member));
        }
    }

    pub fn remove_member(&mut self, member: &str) {
        self.members.retain(|m| m != member);
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn has_member(&self, member: &str) -> bool {
        self.members.contains(&String::from(member))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_group() {
        let mut group = InterestGroup::new("AI Enthusiasts");
        assert_eq!(group.get_members(), Vec::<String>::new());

        group.add_member("Alice");
        group.add_member("Bob");
        assert_eq!(group.get_members().len(), 2);
        assert!(group.has_member("Alice"));
        assert!(group.has_member("Bob"));

        group.remove_member("Alice");
        assert!(!group.has_member("Alice"));
        assert_eq!(group.get_members().len(), 1);

        group.add_member("Alice"); // Adding again to test idempotency
        assert_eq!(group.get_members().len(), 2);
    }
}
