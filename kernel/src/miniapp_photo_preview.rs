extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MiniAppPhotoPreview {
    photos: Vec<String>,
    current_index: usize,
}

impl MiniAppPhotoPreview {
    pub fn new() -> Self {
        MiniAppPhotoPreview {
            photos: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_photo(&mut self, photo_path: String) {
        self.photos.push(photo_path);
    }

    pub fn remove_photo(&mut self, index: usize) -> Option<String> {
        if index < self.photos.len() {
            Some(self.photos.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_photo(&self) -> Option<&String> {
        if !self.photos.is_empty() {
            Some(&self.photos[self.current_index])
        } else {
            None
        }
    }

    pub fn next_photo(&mut self) {
        if !self.photos.is_empty() {
            self.current_index = (self.current_index + 1) % self.photos.len();
        }
    }

    pub fn previous_photo(&mut self) {
        if !self.photos.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.photos.len() - 1
            } else {
                self.current_index - 1
            };
        }
    }
}
