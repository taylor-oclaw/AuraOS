extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_visual_search_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn app_visual_search_exit() {
    // Cleanup logic for the module
}

pub struct AppVisualSearch {
    images: Vec<String>,
    descriptions: Vec<String>,
}

impl AppVisualSearch {
    pub fn new() -> Self {
        AppVisualSearch {
            images: Vec::new(),
            descriptions: Vec::new(),
        }
    }

    pub fn add_image(&mut self, image_path: &str) {
        self.images.push(String::from(image_path));
    }

    pub fn add_description(&mut self, description: &str) {
        self.descriptions.push(String::from(description));
    }

    pub fn get_image_count(&self) -> usize {
        self.images.len()
    }

    pub fn get_description_count(&self) -> usize {
        self.descriptions.len()
    }

    pub fn search_by_description(&self, query: &str) -> Vec<usize> {
        let mut results = Vec::new();
        for (index, description) in self.descriptions.iter().enumerate() {
            if description.contains(query) {
                results.push(index);
            }
        }
        results
    }
}
