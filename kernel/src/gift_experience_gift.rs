extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut gift = GiftExperienceGift::new();
    gift.add_experience("AI Workshop");
    gift.add_experience("Robotics Challenge");
    gift.add_experience("Virtual Reality Tour");
    gift.add_experience("Cybersecurity Seminar");
    gift.add_experience("Blockchain Conference");

    println!("Total experiences: {}", gift.total_experiences());
    println!("Last experience added: {}", gift.last_experience().unwrap());

    loop {}
}

pub struct GiftExperienceGift {
    experiences: Vec<String>,
}

impl GiftExperienceGift {
    pub fn new() -> Self {
        GiftExperienceGift {
            experiences: Vec::new(),
        }
    }

    pub fn add_experience(&mut self, experience: &str) {
        self.experiences.push(String::from(experience));
    }

    pub fn total_experiences(&self) -> usize {
        self.experiences.len()
    }

    pub fn last_experience(&self) -> Option<&String> {
        self.experiences.last()
    }

    pub fn remove_experience(&mut self, index: usize) -> Option<String> {
        if index < self.experiences.len() {
            Some(self.experiences.remove(index))
        } else {
            None
        }
    }

    pub fn list_experiences(&self) -> Vec<&String> {
        self.experiences.iter().collect()
    }
}
