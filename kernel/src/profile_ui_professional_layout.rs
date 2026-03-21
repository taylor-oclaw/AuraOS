extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileUIProfessionalLayout {
    username: String,
    email: String,
    bio: String,
    skills: Vec<String>,
    experience_years: u32,
}

impl ProfileUIProfessionalLayout {
    pub fn new(username: &str, email: &str) -> Self {
        ProfileUIProfessionalLayout {
            username: String::from(username),
            email: String::from(email),
            bio: String::new(),
            skills: Vec::new(),
            experience_years: 0,
        }
    }

    pub fn set_bio(&mut self, bio: &str) {
        self.bio = String::from(bio);
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn set_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_ui_professional_layout() {
        let mut profile = ProfileUIProfessionalLayout::new("JohnDoe", "john@example.com");
        assert_eq!(profile.get_username(), "JohnDoe");
        assert_eq!(profile.get_email(), "john@example.com");

        profile.set_bio("Experienced software developer with a passion for Rust.");
        assert_eq!(profile.bio, "Experienced software developer with a passion for Rust.");

        profile.add_skill("Rust programming");
        profile.add_skill("Kernel development");
        assert_eq!(profile.skills.len(), 2);
        assert_eq!(profile.skills[0], "Rust programming");
        assert_eq!(profile.skills[1], "Kernel development");

        profile.set_experience_years(5);
        assert_eq!(profile.experience_years, 5);
    }
}
