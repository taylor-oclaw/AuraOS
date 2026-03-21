extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn people_name_pronounce_init() {
    // Initialization code if needed
}

#[no_mangle]
pub extern "C" fn people_name_pronounce_exit() {
    // Cleanup code if needed
}

pub struct PeopleNamePronounce {
    names: Vec<String>,
}

impl PeopleNamePronounce {
    pub fn new() -> Self {
        PeopleNamePronounce {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn remove_name(&mut self, index: usize) -> Option<String> {
        if index < self.names.len() {
            Some(self.names.remove(index))
        } else {
            None
        }
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        self.names.get(index)
    }

    pub fn list_names(&self) -> Vec<&String> {
        self.names.iter().collect()
    }

    pub fn count_names(&self) -> usize {
        self.names.len()
    }
}
