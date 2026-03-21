extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextLocationAware {
    location: String,
    history: Vec<String>,
    current_context: String,
}

impl ContextLocationAware {
    pub fn new(location: &str) -> Self {
        ContextLocationAware {
            location: String::from(location),
            history: Vec::new(),
            current_context: String::from("default"),
        }
    }

    pub fn update_location(&mut self, new_location: &str) {
        self.history.push(self.location.clone());
        self.location = String::from(new_location);
    }

    pub fn get_current_location(&self) -> &String {
        &self.location
    }

    pub fn set_context(&mut self, context: &str) {
        self.current_context = String::from(context);
    }

    pub fn get_context(&self) -> &String {
        &self.current_context
    }

    pub fn get_location_history(&self) -> &Vec<String> {
        &self.history
    }
}
