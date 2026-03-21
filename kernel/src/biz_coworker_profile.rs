extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct CoworkerProfile {
    name: String,
    age: u8,
    skills: Vec<String>,
    experience_years: u16,
    is_active: bool,
}

impl CoworkerProfile {
    pub fn new(name: &str, age: u8, skills: &[&str], experience_years: u16) -> Self {
        CoworkerProfile {
            name: String::from(name),
            age,
            skills: skills.iter().map(|&skill| String::from(skill)).collect(),
            experience_years,
            is_active: true,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) {
        self.skills.retain(|s| s != skill);
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coworker_profile() {
        let mut profile = CoworkerProfile::new("Alice", 30, &["Rust", "AI"], 5);
        assert_eq!(profile.get_name(), "Alice");
        assert_eq!(profile.age, 30);
        assert_eq!(profile.skills.len(), 2);
        assert!(profile.is_active);

        profile.set_name("Bob");
        assert_eq!(profile.get_name(), "Bob");

        profile.add_skill("Machine Learning");
        assert_eq!(profile.skills.len(), 3);

        profile.remove_skill("Rust");
        assert_eq!(profile.skills.len(), 2);

        profile.deactivate();
        assert!(!profile.is_active);
    }
}
