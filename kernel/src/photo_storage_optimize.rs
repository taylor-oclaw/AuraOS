extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoStorage {
    photos: Vec<String>,
}

impl PhotoStorage {
    pub fn new() -> Self {
        PhotoStorage {
            photos: Vec::new(),
        }
    }

    pub fn add_photo(&mut self, photo_name: String) {
        self.photos.push(photo_name);
    }

    pub fn remove_photo(&mut self, photo_name: &str) -> bool {
        if let Some(index) = self.photos.iter().position(|p| p == photo_name) {
            self.photos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_photos(&self) -> &[String] {
        &self.photos
    }

    pub fn find_photo(&self, photo_name: &str) -> Option<&String> {
        self.photos.iter().find(|&&p| p == photo_name)
    }

    pub fn count_photos(&self) -> usize {
        self.photos.len()
    }
}
