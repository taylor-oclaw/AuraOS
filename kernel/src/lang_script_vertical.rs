extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_script_vertical_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_script_vertical_exit() {
    // Cleanup logic for the module
}

pub struct LangScriptVertical {
    scripts: Vec<String>,
}

impl LangScriptVertical {
    pub fn new() -> Self {
        LangScriptVertical {
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

    pub fn list_scripts(&self) -> Vec<&String> {
        self.scripts.iter().collect()
    }

    pub fn clear_scripts(&mut self) {
        self.scripts.clear();
    }
}
