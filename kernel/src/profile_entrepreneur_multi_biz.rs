extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct EntrepreneurProfile {
    name: String,
    businesses: Vec<String>,
    experience_years: u32,
    skills: Vec<String>,
    is_active: bool,
}

impl EntrepreneurProfile {
    pub fn new(name: &str, businesses: &[&str], experience_years: u32, skills: &[&str]) -> Self {
        EntrepreneurProfile {
            name: String::from(name),
            businesses: businesses.iter().map(|&b| String::from(b)).collect(),
            experience_years,
            skills: skills.iter().map(|&s| String::from(s)).collect(),
            is_active: true,
        }
    }

    pub fn add_business(&mut self, business_name: &str) {
        self.businesses.push(String::from(business_name));
    }

    pub fn remove_business(&mut self, business_name: &str) {
        self.businesses.retain(|b| b != business_name);
    }

    pub fn update_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
