extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct RelationshipGiftSuggest {
    gifts: Vec<String>,
}

impl RelationshipGiftSuggest {
    pub fn new() -> Self {
        RelationshipGiftSuggest {
            gifts: vec![
                String::from("Personalized Jewelry"),
                String::from("Experience Gift Card"),
                String::from("Subscription Box"),
                String::from("Handwritten Love Letter"),
                String::from("Custom Artwork"),
            ],
        }
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

    pub fn get_gift(&self, index: usize) -> Option<&String> {
        self.gifts.get(index)
    }

    pub fn list_gifts(&self) -> &Vec<String> {
        &self.gifts
    }

    pub fn suggest_gift(&self, relationship_type: &str) -> Option<&String> {
        match relationship_type {
            "romantic" => Some(&self.gifts[0]),
            "friendship" => Some(&self.gifts[1]),
            "family" => Some(&self.gifts[2]),
            _ => None,
        }
    }
}
