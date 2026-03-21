extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_script_complex_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_script_complex_exit() {
    // Cleanup logic for the module
}

pub struct LangScriptComplex {
    data: Vec<String>,
}

impl LangScriptComplex {
    pub fn new() -> Self {
        LangScriptComplex {
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn find_data(&self, item: &str) -> Option<usize> {
        self.data.iter().position(|x| x == item)
    }
}
