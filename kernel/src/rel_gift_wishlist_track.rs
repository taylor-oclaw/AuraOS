extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut wishlist = GiftWishlistTrack::new();
    wishlist.add_gift("AI Book");
    wishlist.add_gift("Smartphone");
    wishlist.remove_gift("AI Book");
    if let Some(gift) = wishlist.get_gift(0) {
    }
    loop {}
}

pub struct GiftWishlistTrack {
    gifts: Vec<String>,
}

impl GiftWishlistTrack {
    pub fn new() -> Self {
        GiftWishlistTrack {
            gifts: Vec::new(),
        }
    }

    pub fn add_gift(&mut self, gift: &str) {
        self.gifts.push(String::from(gift));
    }

    pub fn remove_gift(&mut self, gift: &str) {
        if let Some(index) = self.gifts.iter().position(|g| g == gift) {
            self.gifts.remove(index);
        }
    }

    pub fn total_gifts(&self) -> usize {
        self.gifts.len()
    }

    pub fn get_gift(&self, index: usize) -> Option<&String> {
        self.gifts.get(index)
    }

    pub fn list_gifts(&self) -> Vec<String> {
        self.gifts.clone()
    }
}
