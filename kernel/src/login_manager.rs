extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LoginManager {
    users: Vec<String>,
}

impl LoginManager {
    pub fn new() -> Self {
        LoginManager { users: Vec::new() }
    }

    pub fn add_user(&mut self, username: &str) {
        if !self.users.contains(&username.to_string()) {
            self.users.push(username.to_string());
        }
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        let index = self.users.iter().position(|u| u == username);
        if let Some(i) = index {
            self.users.remove(i);
            true
        } else {
            false
        }
    }

    pub fn user_exists(&self, username: &str) -> bool {
        self.users.contains(&username.to_string())
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn count_users(&self) -> usize {
        self.users.len()
    }
}
