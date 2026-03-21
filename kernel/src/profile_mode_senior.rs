extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut profile = ProfileModeSenior::new(String::from("AI-Native OS"));
    profile.add_user(String::from("Alice"), 30);
    profile.add_user(String::from("Bob"), 25);
    profile.update_user_age("Alice", 31);
    println!("User count: {}", profile.user_count());
    let user_info = profile.get_user_info("Bob");
    if let Some(info) = user_info {
        println!("User Bob is {} years old.", info.age);
    }
}

pub struct ProfileModeSenior {
    name: String,
    users: Vec<User>,
}

impl ProfileModeSenior {
    pub fn new(name: String) -> Self {
        ProfileModeSenior {
            name,
            users: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: String, age: u32) {
        let user = User { name: username, age };
        self.users.push(user);
    }

    pub fn update_user_age(&mut self, username: &str, new_age: u32) {
        if let Some(user) = self.users.iter_mut().find(|u| u.name == username) {
            user.age = new_age;
        }
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    pub fn get_user_info(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|&u| u.name == username)
    }

    pub fn remove_user(&mut self, username: &str) {
        self.users.retain(|u| u.name != username);
    }
}

struct User {
    name: String,
    age: u32,
}
