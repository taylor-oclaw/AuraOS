extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut intro_suggest = RelWarmIntroSuggest::new();
    
    // Example usage of the methods
    intro_suggest.add_user("Alice");
    intro_suggest.add_user("Bob");
    intro_suggest.add_user("Charlie");

    if let Some(suggestion) = intro_suggest.get_introduction_suggestion() {
        println!("Introduction suggestion: {}", suggestion);
    }

    loop {}
}

pub struct RelWarmIntroSuggest {
    users: Vec<String>,
}

impl RelWarmIntroSuggest {
    pub fn new() -> Self {
        RelWarmIntroSuggest {
            users: Vec::new(),
        }
    }

    pub fn add_user(&mut self, name: &str) {
        self.users.push(String::from(name));
    }

    pub fn remove_user(&mut self, name: &str) {
        if let Some(index) = self.users.iter().position(|user| user == name) {
            self.users.remove(index);
        }
    }

    pub fn get_users_count(&self) -> usize {
        self.users.len()
    }

    pub fn get_introduction_suggestion(&self) -> Option<String> {
        if self.users.is_empty() {
            None
        } else {
            Some(format!("Hello, I'm {}!", self.users[0]))
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }
}
