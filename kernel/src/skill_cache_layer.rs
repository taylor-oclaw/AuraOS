extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillCacheLayer {
    cache: Vec<(String, String)>,
}

impl SkillCacheLayer {
    pub fn new() -> Self {
        SkillCacheLayer {
            cache: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str, description: &str) {
        let name = String::from(skill_name);
        let desc = String::from(description);
        self.cache.push((name, desc));
    }

    pub fn get_skill_description(&self, skill_name: &str) -> Option<&String> {
        for (name, desc) in &self.cache {
            if name == skill_name {
                return Some(desc);
            }
        }
        None
    }

    pub fn remove_skill(&mut self, skill_name: &str) {
        self.cache.retain(|(name, _)| name != skill_name);
    }

    pub fn list_skills(&self) -> Vec<&String> {
        self.cache.iter().map(|(name, _)| name).collect()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
