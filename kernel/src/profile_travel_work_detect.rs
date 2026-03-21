extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileTravelWorkDetect {
    profiles: Vec<String>,
    travel_history: Vec<String>,
    work_tasks: Vec<String>,
}

impl ProfileTravelWorkDetect {
    pub fn new() -> Self {
        ProfileTravelWorkDetect {
            profiles: Vec::new(),
            travel_history: Vec::new(),
            work_tasks: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        let name = String::from(profile_name);
        if !self.profiles.contains(&name) {
            self.profiles.push(name);
        }
    }

    pub fn remove_profile(&mut self, profile_name: &str) {
        let name = String::from(profile_name);
        self.profiles.retain(|p| *p != name);
    }

    pub fn add_travel_history(&mut self, profile_name: &str, location: &str) {
        let name = String::from(profile_name);
        if self.profiles.contains(&name) {
            let entry = String::from("info");
            self.travel_history.push(entry);
        }
    }

    pub fn add_work_task(&mut self, profile_name: &str, task_description: &str) {
        let name = String::from(profile_name);
        if self.profiles.contains(&name) {
            let entry = String::from("info");
            self.work_tasks.push(entry);
        }
    }

    pub fn get_travel_history(&self, profile_name: &str) -> Vec<String> {
        let name = String::from(profile_name);
        self.travel_history
            .iter()
            .filter(|&entry| entry.starts_with(&name))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_travel_work_detect() {
        let mut detector = ProfileTravelWorkDetect::new();
        detector.add_profile("Alice");
        assert_eq!(detector.profiles.len(), 1);

        detector.add_travel_history("Alice", "New York");
        let history = detector.get_travel_history("Alice");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "Alice - New York");

        detector.remove_profile("Alice");
        assert_eq!(detector.profiles.len(), 0);
    }
}
