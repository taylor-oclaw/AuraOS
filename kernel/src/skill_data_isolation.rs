extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillData {
    name: String,
    description: String,
    level: u8,
    attributes: Vec<String>,
    effects: Vec<String>,
}

impl SkillData {
    pub fn new(name: &str, description: &str, level: u8) -> Self {
        SkillData {
            name: String::from(name),
            description: String::from(description),
            level,
            attributes: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, attribute: &str) {
        self.attributes.push(String::from(attribute));
    }

    pub fn remove_attribute(&mut self, attribute: &str) -> bool {
        if let Some(index) = self.attributes.iter().position(|x| x == attribute) {
            self.attributes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_effect(&mut self, effect: &str) {
        self.effects.push(String::from(effect));
    }

    pub fn remove_effect(&mut self, effect: &str) -> bool {
        if let Some(index) = self.effects.iter().position(|x| x == effect) {
            self.effects.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_info(&self) -> String {
        let mut info = String::from("info");
        info.push_str(String::from("info").as_str());
        info.push_str(String::from("info").as_str());
        info.push_str("Attributes:\n");
        for attr in &self.attributes {
            info.push_str(String::from("info").as_str());
        }
        info.push_str("Effects:\n");
        for effect in &self.effects {
            info.push_str(String::from("info").as_str());
        }
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_data() {
        let mut skill = SkillData::new("Fireball", "A powerful fire spell", 5);
        skill.add_attribute("Magic");
        skill.add_attribute("Fire");
        skill.add_effect("Burns enemy for 10 damage");
        skill.add_effect("Deals fire elemental damage");

        assert_eq!(skill.name, "Fireball");
        assert_eq!(skill.description, "A powerful fire spell");
        assert_eq!(skill.level, 5);
        assert_eq!(skill.attributes.len(), 2);
        assert_eq!(skill.effects.len(), 2);

        skill.remove_attribute("Magic");
        assert_eq!(skill.attributes.len(), 1);

        skill.remove_effect("Deals fire elemental damage");
        assert_eq!(skill.effects.len(), 1);

        let info = skill.get_info();
        assert!(info.contains("Skill Name: Fireball"));
        assert!(info.contains("Description: A powerful fire spell"));
        assert!(info.contains("Level: 5"));
        assert!(info.contains("- Fire"));
        assert!(info.contains("- Burns enemy for 10 damage"));
    }
}
