extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut source = AISource::new();
    source.add_requirement("AI-123");
    source.add_requirement("AI-456");
    source.remove_requirement("AI-123");
    println!("Current requirements: {:?}", source.get_requirements());
}

pub struct AISource {
    requirements: Vec<String>,
}

impl AISource {
    pub fn new() -> Self {
        AISource {
            requirements: Vec::new(),
        }
    }

    pub fn add_requirement(&mut self, requirement: &str) {
        if !self.requirements.contains(&String::from(requirement)) {
            self.requirements.push(String::from(requirement));
        }
    }

    pub fn remove_requirement(&mut self, requirement: &str) {
        self.requirements.retain(|r| r != requirement);
    }

    pub fn get_requirements(&self) -> Vec<String> {
        self.requirements.clone()
    }

    pub fn has_requirement(&self, requirement: &str) -> bool {
        self.requirements.contains(&String::from(requirement))
    }

    pub fn count_requirements(&self) -> usize {
        self.requirements.len()
    }
}
