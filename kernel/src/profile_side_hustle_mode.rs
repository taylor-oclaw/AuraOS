extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = ProfileSideHustleMode::new(String::from("Alice"), 30);
    profile.add_skill(String::from("Machine Learning"));
    profile.add_skill(String::from("Rust Programming"));
    profile.set_age(31);
    profile.update_status("Active");
    profile.display_profile();
    loop {}
}

pub struct ProfileSideHustleMode {
    name: String,
    age: u8,
    skills: Vec<String>,
    status: String,
}

impl ProfileSideHustleMode {
    pub fn new(name: String, age: u8) -> Self {
        ProfileSideHustleMode {
            name,
            age,
            skills: Vec::new(),
            status: String::from("Inactive"),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn display_profile(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Status: {}", self.status);
        println!("Skills:");
        for skill in &self.skills {
            println!("- {}", skill);
        }
    }

    pub fn get_skills_count(&self) -> usize {
        self.skills.len()
    }
}
