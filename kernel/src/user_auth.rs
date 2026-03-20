extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct UserAuth {
    users: Vec<User>,
}

impl UserAuth {
    pub fn new() -> Self {
        UserAuth { users: Vec::new() }
    }

    pub fn add_user(&mut self, username: String, password: String) -> bool {
        if self.users.iter().any(|user| user.username == username) {
            false
        } else {
            self.users.push(User { username, password });
            true
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        self.users.iter().any(|user| user.username == username && user.password == password)
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        let pos = self.users.iter().position(|user| user.username == username);
        if let Some(index) = pos {
            self.users.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.iter().map(|user| user.username.clone()).collect()
    }

    pub fn change_password(&mut self, username: &str, new_password: String) -> bool {
        if let Some(user) = self.users.iter_mut().find(|user| user.username == username) {
            user.password = new_password;
            true
        } else {
            false
        }
    }
}

struct User {
    username: String,
    password: String,
}
