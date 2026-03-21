extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut feed = FamilyHubActivityFeed::new();
    feed.add_activity("John", "cooked dinner");
    feed.add_activity("Jane", "cleaned the house");
    feed.add_activity("Dad", "fixed the car");
    feed.add_activity("Mom", "bought groceries");
    feed.add_activity("Kid", "played video games");

    let recent_activities = feed.get_recent_activities(3);
    for activity in recent_activities {
        // Simulate printing to a log or console
        unsafe { printk(b"{}\n\0".as_ptr(), activity.as_ptr()) };
    }

    loop {}
}

#[repr(C)]
pub struct FamilyHubActivityFeed {
    activities: Vec<String>,
}

impl FamilyHubActivityFeed {
    pub fn new() -> Self {
        FamilyHubActivityFeed {
            activities: Vec::new(),
        }
    }

    pub fn add_activity(&mut self, person: &str, action: &str) {
        let activity = String::from("info");
        self.activities.push(activity);
    }

    pub fn get_recent_activities(&self, count: usize) -> Vec<String> {
        if count > self.activities.len() {
            self.activities.clone()
        } else {
            self.activities.iter().rev().take(count).cloned().collect()
        }
    }

    pub fn clear_activities(&mut self) {
        self.activities.clear();
    }

    pub fn get_activity_count(&self) -> usize {
        self.activities.len()
    }
}

// Dummy printk function for demonstration purposes
extern "C" {
    fn printk(fmt: *const u8, ...) -> i32;
}
