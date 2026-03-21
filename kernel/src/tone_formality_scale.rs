extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ToneFormalityScale {
    scale: Vec<String>,
}

impl ToneFormalityScale {
    pub fn new() -> Self {
        let mut scale = Vec::new();
        scale.push(String::from("Casual"));
        scale.push(String::from("Friendly"));
        scale.push(String::from("Polite"));
        scale.push(String::from("Formal"));
        scale.push(String::from("Professional"));
        ToneFormalityScale { scale }
    }

    pub fn get_level(&self, index: usize) -> Option<&String> {
        self.scale.get(index)
    }

    pub fn add_level(&mut self, level: String) {
        self.scale.push(level);
    }

    pub fn remove_level(&mut self, index: usize) -> Option<String> {
        if index < self.scale.len() {
            Some(self.scale.remove(index))
        } else {
            None
        }
    }

    pub fn update_level(&mut self, index: usize, new_level: String) -> bool {
        if index < self.scale.len() {
            self.scale[index] = new_level;
            true
        } else {
            false
        }
    }

    pub fn list_levels(&self) -> &Vec<String> {
        &self.scale
    }
}
