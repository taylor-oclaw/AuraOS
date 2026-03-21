extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct EntrepreneurProfile {
    name: String,
    age: u32,
    skills: Vec<String>,
    projects: Vec<String>,
    experience_years: u32,
}

impl EntrepreneurProfile {
    pub fn new(name: &str, age: u32) -> Self {
        EntrepreneurProfile {
            name: String::from(name),
            age,
            skills: Vec::new(),
            projects: Vec::new(),
            experience_years: 0,
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) {
        self.skills.retain(|s| s != skill);
    }

    pub fn add_project(&mut self, project: &str) {
        self.projects.push(String::from(project));
    }

    pub fn remove_project(&mut self, project: &str) {
        self.projects.retain(|p| p != project);
    }

    pub fn set_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }
}
