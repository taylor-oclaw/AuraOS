extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod family_hub_member {
    use super::*;

    pub struct FamilyHubMemberMgr {
        members: Vec<String>,
    }

    impl FamilyHubMemberMgr {
        pub fn new() -> Self {
            FamilyHubMemberMgr {
                members: Vec::new(),
            }
        }

        pub fn add_member(&mut self, name: &str) {
            if !self.members.contains(&String::from(name)) {
                self.members.push(String::from(name));
            }
        }

        pub fn remove_member(&mut self, name: &str) {
            self.members.retain(|member| member != name);
        }

        pub fn list_members(&self) -> Vec<String> {
            self.members.clone()
        }

        pub fn is_member(&self, name: &str) -> bool {
            self.members.contains(&String::from(name))
        }

        pub fn count_members(&self) -> usize {
            self.members.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_family_hub_member_mgr() {
        let mut mgr = family_hub_member::FamilyHubMemberMgr::new();
        assert_eq!(mgr.count_members(), 0);

        mgr.add_member("Alice");
        assert_eq!(mgr.count_members(), 1);
        assert!(mgr.is_member("Alice"));

        mgr.add_member("Bob");
        assert_eq!(mgr.count_members(), 2);
        assert!(mgr.is_member("Bob"));

        let members = mgr.list_members();
        assert_eq!(members.len(), 2);
        assert!(members.contains(&String::from("Alice")));
        assert!(members.contains(&String::from("Bob")));

        mgr.remove_member("Alice");
        assert_eq!(mgr.count_members(), 1);
        assert!(!mgr.is_member("Alice"));

        mgr.remove_member("Bob");
        assert_eq!(mgr.count_members(), 0);
        assert!(!mgr.is_member("Bob"));
    }
}
