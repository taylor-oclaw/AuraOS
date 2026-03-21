extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PhotoBestShotSelect {
    photos: Vec<String>,
    best_shot_index: usize,
}

impl PhotoBestShotSelect {
    pub fn new(photos: Vec<String>) -> Self {
        let best_shot_index = if photos.is_empty() { 0 } else { 0 };
        PhotoBestShotSelect {
            photos,
            best_shot_index,
        }
    }

    pub fn add_photo(&mut self, photo_name: String) {
        self.photos.push(photo_name);
    }

    pub fn remove_photo(&mut self, index: usize) -> Option<String> {
        if index < self.photos.len() {
            Some(self.photos.remove(index))
        } else {
            None
        }
    }

    pub fn get_best_shot(&self) -> Option<&String> {
        self.photos.get(self.best_shot_index)
    }

    pub fn set_best_shot(&mut self, index: usize) -> bool {
        if index < self.photos.len() {
            self.best_shot_index = index;
            true
        } else {
            false
        }
    }

    pub fn list_photos(&self) -> &[String] {
        &self.photos
    }
}
