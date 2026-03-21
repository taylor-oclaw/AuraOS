extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessAISceneDescribe {
    scenes: Vec<String>,
}

impl AccessAISceneDescribe {
    pub fn new() -> Self {
        AccessAISceneDescribe {
            scenes: Vec::new(),
        }
    }

    pub fn add_scene(&mut self, scene: String) {
        self.scenes.push(scene);
    }

    pub fn remove_scene(&mut self, index: usize) -> Option<String> {
        if index < self.scenes.len() {
            Some(self.scenes.remove(index))
        } else {
            None
        }
    }

    pub fn get_scene(&self, index: usize) -> Option<&String> {
        self.scenes.get(index)
    }

    pub fn list_scenes(&self) -> &Vec<String> {
        &self.scenes
    }

    pub fn clear_scenes(&mut self) {
        self.scenes.clear();
    }
}
