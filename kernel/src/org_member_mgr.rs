extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct OrgMemberMgr {
    members: Vec<String>,
}

impl OrgMemberMgr {
    pub fn new() -> Self {
        OrgMemberMgr {
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

    pub fn get_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn is_member(&self, name: &str) -> bool {
        self.members.contains(&String::from(name))
    }

    pub fn count_members(&self) -> usize {
        self.members.len()
    }
}
