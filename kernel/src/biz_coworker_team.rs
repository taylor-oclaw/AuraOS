extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CoworkerTeam {
    members: Vec<String>,
}

impl CoworkerTeam {
    pub fn new() -> Self {
        CoworkerTeam {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: &str) {
        self.members.push(String::from(name));
    }

    pub fn remove_member(&mut self, name: &str) {
        if let Some(index) = self.members.iter().position(|member| member == name) {
            self.members.remove(index);
        }
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn has_member(&self, name: &str) -> bool {
        self.members.contains(&String::from(name))
    }

    pub fn team_size(&self) -> usize {
        self.members.len()
    }
}
