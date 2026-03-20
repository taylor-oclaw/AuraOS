extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_example() {
    // Example FFI function to demonstrate interaction with C code
}

struct AuraUserSetup {
    users: Vec<String>,
    max_users: usize,
}

impl AuraUserSetup {
    pub fn new(max_users: usize) -> Self {
        AuraUserSetup {
            users: Vec::new(),
            max_users,
        }
    }

    pub fn add_user(&mut self, username: &str) -> Result<(), String> {
        if self.users.len() >= self.max_users {
            return Err(String::from("Maximum number of users reached"));
        }
        if self.users.contains(&String::from(username)) {
            return Err(String::from("User already exists"));
        }
        self.users.push(String::from(username));
        Ok(())
    }

    pub fn remove_user(&mut self, username: &str) -> Result<(), String> {
        let pos = self.users.iter().position(|x| x == username);
        match pos {
            Some(index) => {
                self.users.remove(index);
                Ok(())
            }
            None => Err(String::from("User not found")),
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn user_exists(&self, username: &str) -> bool {
        self.users.contains(&String::from(username))
    }

    pub fn max_user_count(&self) -> usize {
        self.max_users
    }
}
