extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct FamilyHubInvite {
    members: Vec<String>,
    max_members: usize,
}

impl FamilyHubInvite {
    pub fn new(max_members: usize) -> Self {
        FamilyHubInvite {
            members: Vec::new(),
            max_members,
        }
    }

    pub fn add_member(&mut self, name: String) -> Result<(), String> {
        if self.members.len() >= self.max_members {
            Err(String::from("Maximum number of members reached"))
        } else {
            self.members.push(name);
            Ok(())
        })
    }

    pub fn remove_member(&mut self, name: &str) -> Result<(), String> {
        if let Some(index) = self.members.iter().position(|member| member == name) {
            self.members.remove(index);
            Ok(())
        } else {
            Err(String::from("Member not found"))
        })
    }

    pub fn list_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn is_full(&self) -> bool {
        self.members.len() >= self.max_members
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_member() {
        let mut invite = FamilyHubInvite::new(2);
        assert_eq!(invite.add_member(String::from("Alice")), Ok(()));
        assert_eq!(invite.add_member(String::from("Bob")), Ok(()));
        assert_eq!(
            invite.add_member(String::from("Charlie")),
            Err(String::from("Maximum number of members reached"))
        ;
    }

    #[test]
    fn test_remove_member() {
        let mut invite = FamilyHubInvite::new(2);
        invite.add_member(String::from("Alice")).unwrap();
        assert_eq!(invite.remove_member("Alice"), Ok(()));
        assert_eq!(
            invite.remove_member("Bob"),
            Err(String::from("Member not found"))
        ;
    }

    #[test]
    fn test_list_members() {
        let mut invite = FamilyHubInvite::new(2);
        invite.add_member(String::from("Alice")).unwrap();
        invite.add_member(String::from("Bob")).unwrap();
        assert_eq!(invite.list_members(), vec![String::from("Alice"), String::from("Bob")]);
    }

    #[test]
    fn test_is_full() {
        let mut invite = FamilyHubInvite::new(2);
        assert!(!invite.is_full());
        invite.add_member(String::from("Alice")).unwrap();
        invite.add_member(String::from("Bob")).unwrap();
        assert!(invite.is_full());
    }

    #[test]
    fn test_member_count() {
        let mut invite = FamilyHubInvite::new(2);
        assert_eq!(invite.member_count(), 0);
        invite.add_member(String::from("Alice")).unwrap();
        assert_eq!(invite.member_count(), 1);
        invite.add_member(String::from("Bob")).unwrap();
        assert_eq!(invite.member_count(), 2);
    }
}
