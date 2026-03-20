extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct SkillCertification {
    name: String,
    skills: Vec<String>,
    certifications: Vec<String>,
}

impl SkillCertification {
    pub fn new(name: &str) -> Self {
        SkillCertification {
            name: String::from(name),
            skills: Vec::new(),
            certifications: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_certification(&mut self, certification: &str) {
        self.certifications.push(String::from(certification));
    }

    pub fn remove_certification(&mut self, certification: &str) -> bool {
        if let Some(index) = self.certifications.iter().position(|c| c == certification) {
            self.certifications.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn list_certifications(&self) -> Vec<String> {
        self.certifications.clone()
    }
}
