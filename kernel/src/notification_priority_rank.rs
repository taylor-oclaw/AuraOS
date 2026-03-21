extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct NotificationPriorityRank {
    priorities: Vec<(String, u32)>,
}

impl NotificationPriorityRank {
    pub fn new() -> Self {
        NotificationPriorityRank {
            priorities: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, name: String, priority: u32) {
        self.priorities.push((name, priority));
        self.priorities.sort_by_key(|&(_, p)| p);
    }

    pub fn remove_notification(&mut self, name: &str) -> bool {
        let pos = self.priorities.iter().position(|&(ref n, _)| n == name);
        if let Some(index) = pos {
            self.priorities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_priority(&self, name: &str) -> Option<u32> {
        self.priorities.iter().find(|&&(ref n, _)| n == name).map(|&(_, p)| p)
    }

    pub fn list_notifications(&self) -> Vec<(String, u32)> {
        self.priorities.clone()
    }

    pub fn update_priority(&mut self, name: &str, new_priority: u32) -> bool {
        if let Some(index) = self.priorities.iter().position(|&(ref n, _)| n == name) {
            self.priorities[index] = (String::from(name), new_priority);
            self.priorities.sort_by_key(|&(_, p)| p);
            true
        } else {
            false
        }
    }
}
