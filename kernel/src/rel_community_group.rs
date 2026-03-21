extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CommunityGroup {
    name: String,
    members: Vec<String>,
    max_members: usize,
}

impl CommunityGroup {
    pub fn new(name: &str, max_members: usize) -> Self {
        CommunityGroup {
            name: String::from(name),
            members: Vec::new(),
            max_members,
        }
    }

    pub fn add_member(&mut self, member_name: &str) -> Result<(), &'static str> {
        if self.members.len() >= self.max_members {
            return Err("Maximum number of members reached");
        }
        self.members.push(String::from(member_name));
        Ok(())
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

    pub fn list_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn group_size(&self) -> usize {
        self.members.len()
    }

    pub fn is_full(&self) -> bool {
        self.members.len() >= self.max_members
    }
}
