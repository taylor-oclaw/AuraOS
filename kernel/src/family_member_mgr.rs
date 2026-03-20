extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod family_member {
    use super::*;

    pub struct FamilyMember {
        name: String,
        age: u8,
        relation: String,
    }

    impl FamilyMember {
        pub fn new(name: &str, age: u8, relation: &str) -> Self {
            FamilyMember {
                name: String::from(name),
                age,
                relation: String::from(relation),
            }
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = String::from(name);
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn set_age(&mut self, age: u8) {
            self.age = age;
        }

        pub fn get_relation(&self) -> &str {
            &self.relation
        }

        pub fn set_relation(&mut self, relation: &str) {
            self.relation = String::from(relation);
        }
    }
}

pub struct FamilyMemberMgr {
    members: Vec<family_member::FamilyMember>,
}

impl FamilyMemberMgr {
    pub fn new() -> Self {
        FamilyMemberMgr {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: family_member::FamilyMember) {
        self.members.push(member);
    }

    pub fn remove_member_by_name(&mut self, name: &str) {
        self.members.retain(|m| m.get_name() != name);
    }

    pub fn get_members(&self) -> &[family_member::FamilyMember] {
        &self.members
    }

    pub fn find_member_by_name(&self, name: &str) -> Option<&family_member::FamilyMember> {
        self.members.iter().find(|m| m.get_name() == name)
    }

    pub fn count_members(&self) -> usize {
        self.members.len()
    }
}
