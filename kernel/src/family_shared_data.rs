extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct FamilySharedData {
    members: Vec<String>,
    family_name: String,
}

impl FamilySharedData {
    pub fn new(family_name: &str) -> Self {
        FamilySharedData {
            members: Vec::new(),
            family_name: String::from(family_name),
        }
    }

    pub fn add_member(&mut self, member_name: &str) {
        self.members.push(String::from(member_name));
    }

    pub fn remove_member(&mut self, member_name: &str) -> bool {
        if let Some(index) = self.members.iter().position(|m| m == member_name) {
            self.members.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_members(&self) -> &[String] {
        &self.members
    }

    pub fn get_family_name(&self) -> &str {
        &self.family_name
    }

    pub fn has_member(&self, member_name: &str) -> bool {
        self.members.contains(&String::from(member_name))
    }
}
