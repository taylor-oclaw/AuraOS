extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PhotoMemory {
    photos: Vec<String>,
}

impl PhotoMemory {
    pub fn new() -> Self {
        PhotoMemory {
            photos: Vec::new(),
        }
    }

    pub fn add_photo(&mut self, photo_name: &str) {
        let name = String::from(photo_name);
        self.photos.push(name);
    }

    pub fn remove_photo(&mut self, photo_name: &str) -> bool {
        if let Some(index) = self.photos.iter().position(|x| x == photo_name) {
            self.photos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_photos(&self) -> &[String] {
        &self.photos
    }

    pub fn has_photo(&self, photo_name: &str) -> bool {
        self.photos.contains(&String::from(photo_name))
    }

    pub fn count_photos(&self) -> usize {
        self.photos.len()
    }
}
