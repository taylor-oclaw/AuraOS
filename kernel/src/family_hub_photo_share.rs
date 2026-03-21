extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut photo_share = FamilyHubPhotoShare::new();
    photo_share.add_user("Alice".into());
    photo_share.add_user("Bob".into());
    photo_share.share_photo("Alice", "Sunset at the beach".into());
    photo_share.share_photo("Bob", "Mountain hike".into());
    photo_share.display_photos("Alice");
    photo_share.display_photos("Bob");

    loop {}
}

pub struct FamilyHubPhotoShare {
    users: Vec<String>,
    photos: Vec<(String, String)>, // (user, photo_description)
}

impl FamilyHubPhotoShare {
    pub fn new() -> Self {
        FamilyHubPhotoShare {
            users: Vec::new(),
            photos: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: String) {
        if !self.users.contains(&username) {
            self.users.push(username);
        }
    }

    pub fn share_photo(&mut self, user: &str, photo_description: String) {
        if self.users.contains(&user.into()) {
            self.photos.push((user.into(), photo_description));
        }
    }

    pub fn get_user_photos(&self, user: &str) -> Vec<&String> {
        self.photos
            .iter()
            .filter(|&&(ref u, _)| u == user)
            .map(|(_, ref desc)| desc)
            .collect()
    }

    pub fn display_photos(&self, user: &str) {
        let photos = self.get_user_photos(user);
        for photo in photos {
            // Simulate displaying a photo
            println!("{} shared: {}", user, photo); // This is just a placeholder for demonstration
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        if let Some(index) = self.users.iter().position(|u| u == username) {
            self.users.remove(index);
            self.photos.retain(|(u, _)| u != username);
        }
    }
}
