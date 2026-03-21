extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut profile = ProfileModeProfessional::new("AI-Native OS".to_string());

    profile.add_user("Alice".to_string(), 30);
    profile.add_user("Bob".to_string(), 25);


    if let Some(user) = profile.get_user_by_name("Alice") {
    }

    profile.remove_user("Bob");

}

pub struct ProfileModeProfessional {
    name: String,
    users: Vec<User>,
}

impl ProfileModeProfessional {
    pub fn new(name: String) -> Self {
        ProfileModeProfessional {
            name,
            users: Vec::new(),
        }
    }

    pub fn add_user(&mut self, name: String, age: u32) {
        let user = User { name, age };
        self.users.push(user);
    }

    pub fn remove_user(&mut self, name: &str) {
        self.users.retain(|user| user.name != name);
    }

    pub fn get_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|user| user.name == name)
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

struct User {
    name: String,
    age: u32,
}
