extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_anniversary_milestone_init() {
    println!("rel_anniversary_milestone module initialized");
}

#[no_mangle]
pub extern "C" fn rel_anniversary_milestone_exit() {
    println!("rel_anniversary_milestone module exited");
}

pub struct AnniversaryMilestone {
    milestones: Vec<String>,
}

impl AnniversaryMilestone {
    pub fn new() -> Self {
        AnniversaryMilestone {
            milestones: Vec::new(),
        }
    }

    pub fn add_milestone(&mut self, milestone: String) {
        self.milestones.push(milestone);
    }

    pub fn get_milestone_count(&self) -> usize {
        self.milestones.len()
    }

    pub fn get_all_milestones(&self) -> Vec<String> {
        self.milestones.clone()
    }

    pub fn remove_milestone(&mut self, index: usize) -> Option<String> {
        if index < self.milestones.len() {
            Some(self.milestones.remove(index))
        } else {
            None
        }
    }
}
