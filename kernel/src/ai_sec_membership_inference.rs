extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct MembershipInference {
    members: Vec<String>,
}

impl MembershipInference {
    pub fn new() -> Self {
        MembershipInference {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: String) {
        if !self.members.contains(&member) {
            self.members.push(member);
        }
    }

    pub fn remove_member(&mut self, member: &str) {
        self.members.retain(|m| m != member);
    }

    pub fn is_member(&self, member: &str) -> bool {
        self.members.contains(&String::from(member))
    }

    pub fn list_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn count_members(&self) -> usize {
        self.members.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_membership_inference() {
        let mut inference = MembershipInference::new();
        assert_eq!(inference.count_members(), 0);

        inference.add_member(String::from("Alice"));
        assert_eq!(inference.count_members(), 1);
        assert!(inference.is_member("Alice"));

        inference.add_member(String::from("Bob"));
        assert_eq!(inference.count_members(), 2);
        assert!(inference.is_member("Bob"));

        inference.remove_member("Alice");
        assert_eq!(inference.count_members(), 1);
        assert!(!inference.is_member("Alice"));

        let members = inference.list_members();
        assert_eq!(members, vec![String::from("Bob")]);
    }
}
