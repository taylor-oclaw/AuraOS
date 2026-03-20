extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyRoleManager {
    roles: Vec<String>,
}

impl FamilyRoleManager {
    pub fn new() -> Self {
        FamilyRoleManager {
            roles: Vec::new(),
        }
    }

    pub fn add_role(&mut self, role: String) {
        if !self.roles.contains(&role) {
            self.roles.push(role);
        }
    }

    pub fn remove_role(&mut self, role: &str) -> bool {
        let index = self.roles.iter().position(|r| r == role);
        match index {
            Some(i) => {
                self.roles.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&String::from(role))
    }

    pub fn list_roles(&self) -> Vec<String> {
        self.roles.clone()
    }

    pub fn count_roles(&self) -> usize {
        self.roles.len()
    }
}
