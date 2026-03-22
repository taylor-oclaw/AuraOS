extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubActivityFeed {
    activities: Vec<String>,
}

impl FamilyHubActivityFeed {
    pub fn new() -> Self {
        FamilyHubActivityFeed { activities: Vec::new() }
    }

    pub fn add_activity(&mut self, activity: String) {
        self.activities.push(activity);
    }

    pub fn get_activities(&self) -> &Vec<String> {
        &self.activities
    }

    pub fn remove_activity(&mut self, index: usize) {
        if index < self.activities.len() {
            self.activities.remove(index);
        }
    }

    pub fn update_activity(&mut self, old_index: usize, new_activity: String) {
        if old_index < self.activities.len() {
            let activity = self.activities.remove(old_index).unwrap();
            self.add_activity(new_activity);
        }
    }

    pub fn get_last_activity(&self) -> Option<&String> {
        self.activities.last()
    }
}