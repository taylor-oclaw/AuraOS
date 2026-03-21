extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoBackupSmart {
    photos: Vec<String>,
    backup_location: String,
}

impl PhotoBackupSmart {
    pub fn new(backup_location: &str) -> Self {
        PhotoBackupSmart {
            photos: Vec::new(),
            backup_location: String::from(backup_location),
        }
    }

    pub fn add_photo(&mut self, photo_path: &str) {
        self.photos.push(String::from(photo_path));
    }

    pub fn remove_photo(&mut self, photo_path: &str) -> bool {
        if let Some(index) = self.photos.iter().position(|p| p == photo_path) {
            self.photos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_photos(&self) -> Vec<String> {
        self.photos.clone()
    }

    pub fn backup_all(&self) -> bool {
        // Simulate backup process
        for photo in &self.photos {
            // Here you would add logic to copy each photo to the backup location
            // For demonstration, we'll just print a message
        }
        true
    }

    pub fn get_backup_location(&self) -> String {
        self.backup_location.clone()
    }
}
