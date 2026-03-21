extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GiftRegistry {
    gifts: Vec<String>,
}

impl GiftRegistry {
    pub fn new() -> Self {
        GiftRegistry { gifts: Vec::new() }
    }

    pub fn add_gift(&mut self, gift: String) {
        self.gifts.push(gift);
    }

    pub fn remove_gift(&mut self, index: usize) -> Option<String> {
        if index < self.gifts.len() {
            Some(self.gifts.remove(index))
        } else {
            None
        }
    }

    pub fn list_gifts(&self) -> &Vec<String> {
        &self.gifts
    }

    pub fn find_gift(&self, gift_name: &str) -> Option<&String> {
        self.gifts.iter().find(|&&gift| gift == gift_name)
    }

    pub fn count_gifts(&self) -> usize {
        self.gifts.len()
    }
}
