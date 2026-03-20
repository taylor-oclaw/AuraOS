extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillTransformLayer {
    skills: Vec<String>,
}

impl SkillTransformLayer {
    pub fn new() -> Self {
        SkillTransformLayer {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, index: usize) -> Option<String> {
        if index < self.skills.len() {
            Some(self.skills.remove(index))
        } else {
            None
        }
    }

    pub fn get_skill(&self, index: usize) -> Option<&String> {
        self.skills.get(index)
    }

    pub fn list_skills(&self) -> &Vec<String> {
        &self.skills
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_transform_layer() {
        let mut layer = SkillTransformLayer::new();

        assert_eq!(layer.list_skills().len(), 0);

        layer.add_skill(String::from("Rust"));
        layer.add_skill(String::from("AI"));

        assert_eq!(layer.list_skills().len(), 2);
        assert_eq!(layer.get_skill(0), Some(&String::from("Rust")));
        assert_eq!(layer.get_skill(1), Some(&String::from("AI")));

        let removed = layer.remove_skill(0);
        assert_eq!(removed, Some(String::from("Rust")));
        assert_eq!(layer.list_skills().len(), 1);

        layer.clear_skills();
        assert_eq!(layer.list_skills().len(), 0);
    }
}
