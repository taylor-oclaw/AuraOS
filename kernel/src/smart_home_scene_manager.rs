extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct SceneManager {
    scenes: Vec<String>,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            scenes: Vec::new(),
        }
    }

    pub fn add_scene(&mut self, scene_name: &str) {
        self.scenes.push(String::from(scene_name));
    }

    pub fn remove_scene(&mut self, scene_name: &str) {
        if let Some(index) = self.scenes.iter().position(|s| s == scene_name) {
            self.scenes.remove(index);
        }
    }

    pub fn list_scenes(&self) -> Vec<String> {
        self.scenes.clone()
    }

    pub fn has_scene(&self, scene_name: &str) -> bool {
        self.scenes.contains(&String::from(scene_name))
    }

    pub fn clear_scenes(&mut self) {
        self.scenes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_scene() {
        let mut manager = SceneManager::new();
        manager.add_scene("Morning");
        assert!(manager.has_scene("Morning"));
        manager.remove_scene("Morning");
        assert!(!manager.has_scene("Morning"));
    }

    #[test]
    fn test_list_scenes() {
        let mut manager = SceneManager::new();
        manager.add_scene("Evening");
        manager.add_scene("Night");
        let scenes = manager.list_scenes();
        assert_eq!(scenes.len(), 2);
        assert!(scenes.contains(&String::from("Evening")));
        assert!(scenes.contains(&String::from("Night")));
    }

    #[test]
    fn test_clear_scenes() {
        let mut manager = SceneManager::new();
        manager.add_scene("Day");
        manager.clear_scenes();
        assert_eq!(manager.list_scenes().len(), 0);
    }
}
