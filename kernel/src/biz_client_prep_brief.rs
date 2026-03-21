extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut client = BizClientPrepBrief::new("Alice".to_string(), 30);
    client.update_age(31);
    client.add_interest(String::from("AI"));
    client.add_interest(String::from("Rust"));
    client.remove_interest(String::from("AI"));
    println!("Client Name: {}", client.get_name());
    println!("Client Age: {}", client.get_age());
    println!("Client Interests: {:?}", client.get_interests());
}

pub struct BizClientPrepBrief {
    name: String,
    age: u32,
    interests: Vec<String>,
}

impl BizClientPrepBrief {
    pub fn new(name: String, age: u32) -> Self {
        BizClientPrepBrief {
            name,
            age,
            interests: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn update_age(&mut self, age: u32) {
        if age > 0 {
            self.age = age;
        }
    }

    pub fn add_interest(&mut self, interest: String) {
        if !self.interests.contains(&interest) {
            self.interests.push(interest);
        }
    }

    pub fn remove_interest(&mut self, interest: String) {
        self.interests.retain(|i| i != &interest);
    }

    pub fn get_interests(&self) -> &Vec<String> {
        &self.interests
    }
}
