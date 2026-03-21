extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut vault = ProfileDataWorkVault::new();

    vault.add_profile("Alice", 30, "Engineer");
    vault.add_profile("Bob", 25, "Designer");
    vault.update_age("Alice", 31);

    loop {}
}

pub struct ProfileDataWorkVault {
    profiles: Vec<Profile>,
}

impl ProfileDataWorkVault {
    pub fn new() -> Self {
        ProfileDataWorkVault {
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, name: &str, age: u32, occupation: &str) {
        let profile = Profile {
            name: String::from(name),
            age,
            occupation: String::from(occupation),
        };
        self.profiles.push(profile);
    }

    pub fn update_age(&mut self, name: &str, new_age: u32) -> bool {
        for profile in &mut self.profiles {
            if profile.name == name {
                profile.age = new_age;
                return true;
            }
        }
        false
    }

    pub fn get_age(&self, name: &str) -> Option<u32> {
        for profile in &self.profiles {
            if profile.name == name {
                return Some(profile.age);
            }
        }
        None
    }

    pub fn count_profiles(&self) -> usize {
        self.profiles.len()
    }

    pub fn remove_profile(&mut self, name: &str) -> bool {
        let pos = self.profiles.iter().position(|p| p.name == name);
        if let Some(index) = pos {
            self.profiles.remove(index);
            return true;
        }
        false
    }
}

struct Profile {
    name: String,
    age: u32,
    occupation: String,
}
