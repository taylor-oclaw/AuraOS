extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct NotifyBehaviorLearn {
    behaviors: Vec<String>,
}

impl NotifyBehaviorLearn {
    pub fn new() -> Self {
        NotifyBehaviorLearn {
            behaviors: Vec::new(),
        }
    }

    pub fn add_behavior(&mut self, behavior: String) {
        self.behaviors.push(behavior);
    }

    pub fn remove_behavior(&mut self, index: usize) -> Option<String> {
        if index < self.behaviors.len() {
            Some(self.behaviors.remove(index))
        } else {
            None
        }
    }

    pub fn get_behavior(&self, index: usize) -> Option<&String> {
        self.behaviors.get(index)
    }

    pub fn list_behaviors(&self) -> &[String] {
        &self.behaviors
    }

    pub fn clear_behaviors(&mut self) {
        self.behaviors.clear();
    }
}
