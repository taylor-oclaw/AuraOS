extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut assessment = HealthErgonomicAssessment::new();
    assessment.add_user("Alice");
    assessment.add_user("Bob");
    assessment.record_activity("Alice", "typing");
    assessment.record_activity("Bob", "standing");
    assessment.print_users();
    assessment.print_activities("Alice");

    loop {}
}

pub struct HealthErgonomicAssessment {
    users: Vec<String>,
    activities: Vec<(String, String)>, // (user, activity)
}

impl HealthErgonomicAssessment {
    pub fn new() -> Self {
        HealthErgonomicAssessment {
            users: Vec::new(),
            activities: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: &str) {
        if !self.users.contains(&user.to_string()) {
            self.users.push(user.to_string());
        }
    }

    pub fn remove_user(&mut self, user: &str) {
        self.users.retain(|u| u != user);
        self.activities.retain(|(u, _)| u != user);
    }

    pub fn record_activity(&mut self, user: &str, activity: &str) {
        if self.users.contains(&user.to_string()) {
            self.activities.push((user.to_string(), activity.to_string()));
        }
    }

    pub fn print_users(&self) {
        for user in &self.users {
            // Simulate printing to a log or console
            println!("User: {}", user);
        }
    }

    pub fn print_activities(&self, user: &str) {
        for (u, activity) in &self.activities {
            if u == user {
                // Simulate printing to a log or console
                println!("Activity for {}: {}", user, activity);
            }
        }
    }
}
