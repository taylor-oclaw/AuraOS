extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct Skill {
    name: String,
    description: String,
    level: u8,
}

impl Skill {
    pub fn new(name: &str, description: &str, level: u8) -> Self {
        Skill {
            name: String::from(name),
            description: String::from(description),
            level,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn upgrade_skill(&mut self) {
        if self.level < 10 {
            self.level += 1;
        }
    }
}

struct SkillCatalog {
    skills: Vec<Skill>,
}

impl SkillCatalog {
    pub fn new() -> Self {
        SkillCatalog { skills: Vec::new() }
    }

    pub fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    pub fn get_skills(&self) -> &Vec<Skill> {
        &self.skills
    }

    pub fn find_skill_by_name(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|skill| skill.get_name() == name)
    }

    pub fn remove_skill_by_name(&mut self, name: &str) {
        self.skills.retain(|skill| skill.get_name() != name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new("Rust Programming", "Expertise in Rust language.", 5);
        assert_eq!(skill.get_name(), "Rust Programming");
        assert_eq!(skill.get_description(), "Expertise in Rust language.");
        assert_eq!(skill.get_level(), 5);
    }

    #[test]
    fn test_skill_catalog() {
        let mut catalog = SkillCatalog::new();
        let skill1 = Skill::new("AI Development", "Building AI models.", 7);
        let skill2 = Skill::new("Kernel Programming", "Writing operating system kernels.", 8);

        catalog.add_skill(skill1);
        catalog.add_skill(skill2);

        assert_eq!(catalog.get_skills().len(), 2);

        if let Some(skill) = catalog.find_skill_by_name("AI Development") {
            assert_eq!(skill.get_level(), 7);
        } else {
            panic!("Skill not found");
        }

        catalog.remove_skill_by_name("AI Development");
        assert_eq!(catalog.get_skills().len(), 1);
    }
}
