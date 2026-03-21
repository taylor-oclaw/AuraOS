extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct EntrepreneurProfile {
    name: String,
    business_name: String,
    industry: String,
    years_of_experience: u32,
    skills: Vec<String>,
}

impl EntrepreneurProfile {
    pub fn new(name: &str, business_name: &str, industry: &str, years_of_experience: u32) -> Self {
        EntrepreneurProfile {
            name: String::from(name),
            business_name: String::from(business_name),
            industry: String::from(industry),
            years_of_experience,
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_business_name(&self) -> &str {
        &self.business_name
    }

    pub fn get_industry(&self) -> &str {
        &self.industry
    }

    pub fn get_years_of_experience(&self) -> u32 {
        self.years_of_experience
    }

    pub fn list_skills(&self) -> &[String] {
        &self.skills
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entrepreneur_profile() {
        let mut profile = EntrepreneurProfile::new("John Doe", "Tech Innovations Inc.", "Technology", 5);
        assert_eq!(profile.get_name(), "John Doe");
        assert_eq!(profile.get_business_name(), "Tech Innovations Inc.");
        assert_eq!(profile.get_industry(), "Technology");
        assert_eq!(profile.get_years_of_experience(), 5);

        profile.add_skill("Rust Programming");
        profile.add_skill("Machine Learning");

        let skills = profile.list_skills();
        assert_eq!(skills.len(), 2);
        assert_eq!(skills[0], "Rust Programming");
        assert_eq!(skills[1], "Machine Learning");
    }
}
