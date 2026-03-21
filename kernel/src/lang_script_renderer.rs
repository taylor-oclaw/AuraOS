extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangScriptRenderer {
    scripts: Vec<String>,
}

impl LangScriptRenderer {
    pub fn new() -> Self {
        LangScriptRenderer {
            scripts: Vec::new(),
        }
    }

    pub fn add_script(&mut self, script: String) {
        self.scripts.push(script);
    }

    pub fn remove_script(&mut self, index: usize) -> Option<String> {
        if index < self.scripts.len() {
            Some(self.scripts.remove(index))
        } else {
            None
        }
    }

    pub fn get_script(&self, index: usize) -> Option<&String> {
        self.scripts.get(index)
    }

    pub fn list_scripts(&self) -> &[String] {
        &self.scripts
    }

    pub fn clear_scripts(&mut self) {
        self.scripts.clear();
    }
}
