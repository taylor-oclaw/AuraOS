extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseRoleManager {
    roles: Vec<String>,
}

impl EnterpriseRoleManager {
    pub fn new() -> Self {
        EnterpriseRoleManager {
            roles: Vec::new(),
        }
    }

    pub fn add_role(&mut self, role_name: &str) {
        if !self.roles.contains(&String::from(role_name)) {
            self.roles.push(String::from(role_name));
        }
    }

    pub fn remove_role(&mut self, role_name: &str) {
        self.roles.retain(|role| role != role_name);
    }

    pub fn has_role(&self, role_name: &str) -> bool {
        self.roles.contains(&String::from(role_name))
    }

    pub fn list_roles(&self) -> Vec<String> {
        self.roles.clone()
    }

    pub fn count_roles(&self) -> usize {
        self.roles.len()
    }
}