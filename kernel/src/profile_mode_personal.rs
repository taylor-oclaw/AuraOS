extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = ProfileModePersonal::new(String::from("Alice"), 30);
    profile.update_age(31);
    profile.add_interest(String::from("reading"));
    profile.remove_interest(String::from("sleeping"));
    profile.set_favorite_color(String::from("blue"));
    profile.log_profile();
    loop {}
}

pub struct ProfileModePersonal {
    name: String,
    age: u8,
    interests: Vec<String>,
    favorite_color: Option<String>,
}

impl ProfileModePersonal {
    pub fn new(name: String, age: u8) -> Self {
        ProfileModePersonal {
            name,
            age,
            interests: Vec::new(),
            favorite_color: None,
        }
    }

    pub fn update_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn add_interest(&mut self, interest: String) {
        if !self.interests.contains(&interest) {
            self.interests.push(interest);
        }
    }

    pub fn remove_interest(&mut self, interest: String) {
        self.interests.retain(|i| i != &interest);
    }

    pub fn set_favorite_color(&mut self, color: String) {
        self.favorite_color = Some(color);
    }

    pub fn log_profile(&self) {
        // Simulate logging the profile
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Interests: {:?}", self.interests);
        if let Some(color) = &self.favorite_color {
            println!("Favorite Color: {}", color);
        } else {
            println!("No favorite color set");
        }
    }
}
