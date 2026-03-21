extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ProfileDataCrossoverAllow {
    allowed_users: Vec<String>,
    max_crossovers: usize,
    current_crossovers: usize,
}

impl ProfileDataCrossoverAllow {
    pub fn new(max_crossovers: usize) -> Self {
        ProfileDataCrossoverAllow {
            allowed_users: Vec::new(),
            max_crossovers,
            current_crossovers: 0,
        }
    }

    pub fn add_allowed_user(&mut self, user: String) {
        if !self.allowed_users.contains(&user) {
            self.allowed_users.push(user);
        }
    }

    pub fn remove_allowed_user(&mut self, user: &str) {
        self.allowed_users.retain(|u| u != user);
    }

    pub fn is_user_allowed(&self, user: &str) -> bool {
        self.allowed_users.contains(&String::from(user))
    }

    pub fn allow_crossover(&mut self, user: &str) -> bool {
        if self.is_user_allowed(user) && self.current_crossovers < self.max_crossovers {
            self.current_crossovers += 1;
            true
        } else {
            false
        }
    }

    pub fn reset_crossovers(&mut self) {
        self.current_crossovers = 0;
    }
}
