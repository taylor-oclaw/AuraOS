extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileCommuteEmailTriage {
    profiles: Vec<String>,
    commute_times: Vec<u32>,
    email_subjects: Vec<String>,
}

impl ProfileCommuteEmailTriage {
    pub fn new() -> Self {
        ProfileCommuteEmailTriage {
            profiles: Vec::new(),
            commute_times: Vec::new(),
            email_subjects: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        self.profiles.push(String::from(profile_name));
    }

    pub fn set_commute_time(&mut self, profile_index: usize, commute_time: u32) {
        if profile_index < self.commute_times.len() {
            self.commute_times[profile_index] = commute_time;
        } else {
            self.commute_times.push(commute_time);
        }
    }

    pub fn add_email_subject(&mut self, subject: &str) {
        self.email_subjects.push(String::from(subject));
    }

    pub fn get_commute_time(&self, profile_index: usize) -> Option<u32> {
        self.commute_times.get(profile_index).cloned()
    }

    pub fn list_email_subjects(&self) -> &[String] {
        &self.email_subjects
    }
}
