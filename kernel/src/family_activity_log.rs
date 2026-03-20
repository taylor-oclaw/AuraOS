extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyActivityLog {
    activities: Vec<String>,
}

impl FamilyActivityLog {
    pub fn new() -> Self {
        FamilyActivityLog {
            activities: Vec::new(),
        }
    }

    pub fn add_activity(&mut self, activity: String) {
        self.activities.push(activity);
    }

    pub fn get_activities(&self) -> &Vec<String> {
        &self.activities
    }

    pub fn remove_activity(&mut self, index: usize) -> Option<String> {
        if index < self.activities.len() {
            Some(self.activities.remove(index))
        } else {
            None
        }
    }

    pub fn count_activities(&self) -> usize {
        self.activities.len()
    }

    pub fn find_activity(&self, activity: &str) -> Option<usize> {
        self.activities.iter().position(|a| a == activity)
    }
}
