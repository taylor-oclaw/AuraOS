extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;

pub struct PhotoDuplicateDetect {
    photos: Vec<String>,
}

impl PhotoDuplicateDetect {
    pub fn new() -> Self {
        PhotoDuplicateDetect {
            photos: Vec::new(),
        }
    }

    pub fn add_photo(&mut self, photo_hash: String) {
        if !self.photos.contains(&photo_hash) {
            self.photos.push(photo_hash);
        }
    }

    pub fn remove_photo(&mut self, photo_hash: &str) -> bool {
        let position = self.photos.iter().position(|p| p == photo_hash);
        match position {
            Some(index) => {
                self.photos.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn is_duplicate(&self, photo_hash: &str) -> bool {
        self.photos.contains(photo_hash)
    }

    pub fn count_photos(&self) -> usize {
        self.photos.len()
    }

    pub fn list_photos(&self) -> Vec<String> {
        self.photos.clone()
    }
}
