extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn gift_client_milestone_init() {
    // Initialization logic for the module
}

pub extern "C" fn gift_client_milestone_exit() {
    // Cleanup logic for the module
}

pub struct GiftClient {
    name: String,
    milestones: Vec<String>,
}

impl GiftClient {
    pub fn new(name: &str) -> Self {
        GiftClient {
            name: String::from(name),
            milestones: Vec::new(),
        }
    }

    pub fn add_milestone(&mut self, milestone: &str) {
        self.milestones.push(String::from(milestone));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn list_milestones(&self) -> &[String] {
        &self.milestones
    }

    pub fn remove_milestone(&mut self, milestone: &str) {
        if let Some(index) = self.milestones.iter().position(|m| m == milestone) {
            self.milestones.remove(index);
        }
    }

    pub fn has_completed_milestone(&self, milestone: &str) -> bool {
        self.milestones.contains(&String::from(milestone))
    }
}
