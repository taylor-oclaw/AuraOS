extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AppUIImageExtract {
    images: Vec<String>,
}

impl AppUIImageExtract {
    pub fn new() -> Self {
        AppUIImageExtract { images: Vec::new() }
    }

    pub fn add_image(&mut self, image_path: &str) {
        self.images.push(image_path.to_string());
    }

    pub fn remove_image(&mut self, index: usize) -> Option<String> {
        if index < self.images.len() {
            Some(self.images.remove(index))
        } else {
            None
        }
    }

    pub fn get_image(&self, index: usize) -> Option<&String> {
        self.images.get(index)
    }

    pub fn list_images(&self) -> Vec<&String> {
        self.images.iter().collect()
    }

    pub fn clear_images(&mut self) {
        self.images.clear();
    }
}
