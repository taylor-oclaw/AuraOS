extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ProfileCommutePodcastResume {
    name: String,
    commute_time: u32,
    favorite_podcasts: Vec<String>,
    resume_keywords: Vec<String>,
}

impl ProfileCommutePodcastResume {
    pub fn new(name: &str, commute_time: u32) -> Self {
        ProfileCommutePodcastResume {
            name: String::from(name),
            commute_time,
            favorite_podcasts: Vec::new(),
            resume_keywords: Vec::new(),
        }
    }

    pub fn add_favorite_podcast(&mut self, podcast: &str) {
        self.favorite_podcasts.push(String::from(podcast));
    }

    pub fn remove_favorite_podcast(&mut self, podcast: &str) -> bool {
        if let Some(index) = self.favorite_podcasts.iter().position(|p| p == podcast) {
            self.favorite_podcasts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_resume_keyword(&mut self, keyword: &str) {
        self.resume_keywords.push(String::from(keyword));
    }

    pub fn remove_resume_keyword(&mut self, keyword: &str) -> bool {
        if let Some(index) = self.resume_keywords.iter().position(|k| k == keyword) {
            self.resume_keywords.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("info");
        if !self.favorite_podcasts.is_empty() {
            summary.push_str("Favorite Podcasts:\n");
            for podcast in &self.favorite_podcasts {
                summary.push_str(&String::from("info"));
            }
        }
        if !self.resume_keywords.is_empty() {
            summary.push_str("Resume Keywords:\n");
            for keyword in &self.resume_keywords {
                summary.push_str(&String::from("info"));
            }
        }
        summary
    }
}
