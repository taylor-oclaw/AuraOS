extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut milestone = ParentalAgeMilestone::new();
    milestone.add_milestone(18, String::from("Legal adulthood"));
    milestone.add_milestone(21, String::from("Drinking age in many countries"));
    milestone.add_milestone(30, String::from("Majority in some countries"));
    milestone.add_milestone(65, String::from("Retirement age"));

    let milestones = milestone.get_all_milestones();
    for (age, description) in milestones {
        // Simulate printing to a log or console
        log(String::from("info"));
    }

    loop {}
}

fn log(message: String) {
    // Placeholder for logging mechanism
}

pub struct ParentalAgeMilestone {
    milestones: Vec<(u32, String)>,
}

impl ParentalAgeMilestone {
    pub fn new() -> Self {
        ParentalAgeMilestone {
            milestones: Vec::new(),
        }
    }

    pub fn add_milestone(&mut self, age: u32, description: String) {
        self.milestones.push((age, description));
    }

    pub fn get_milestone_at_age(&self, age: u32) -> Option<&String> {
        for (milestone_age, description) in &self.milestones {
            if *milestone_age == age {
                return Some(description);
            }
        }
        None
    }

    pub fn get_all_milestones(&self) -> Vec<(u32, String)> {
        self.milestones.clone()
    }

    pub fn remove_milestone_at_age(&mut self, age: u32) -> bool {
        let original_len = self.milestones.len();
        self.milestones.retain(|&(milestone_age, _)| milestone_age != age);
        original_len != self.milestones.len()
    }
}
