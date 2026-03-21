extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipProfessionalTrack {
    name: String,
    skills: Vec<String>,
    experience_years: u32,
    certifications: Vec<String>,
    projects_completed: u32,
}

impl RelationshipProfessionalTrack {
    pub fn new(name: &str) -> Self {
        RelationshipProfessionalTrack {
            name: String::from(name),
            skills: Vec::new(),
            experience_years: 0,
            certifications: Vec::new(),
            projects_completed: 0,
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn set_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }

    pub fn add_certification(&mut self, certification: &str) {
        self.certifications.push(String::from(certification));
    }

    pub fn increment_projects_completed(&mut self) {
        self.projects_completed += 1;
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("info");
        summary.push_str(&String::from("info"));
        summary.push_str("Skills:\n");
        for skill in &self.skills {
            summary.push_str(&String::from("info"));
        }
        summary.push_str("Certifications:\n");
        for certification in &self.certifications {
            summary.push_str(&String::from("info"));
        }
        summary.push_str(&String::from("info"));
        summary
    }
}
