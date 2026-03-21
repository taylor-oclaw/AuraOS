extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug)]
pub struct GiftGroup {
    name: String,
    gifts: Vec<String>,
}

impl GiftGroup {
    pub fn new(name: &str) -> Self {
        GiftGroup {
            name: String::from(name),
            gifts: Vec::new(),
        }
    }

    pub fn add_gift(&mut self, gift: &str) {
        self.gifts.push(String::from(gift));
    }

    pub fn remove_gift(&mut self, gift: &str) -> bool {
        if let Some(index) = self.gifts.iter().position(|g| g == gift) {
            self.gifts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_gifts(&self) -> Vec<String> {
        self.gifts.clone()
    }

    pub fn has_gift(&self, gift: &str) -> bool {
        self.gifts.contains(&String::from(gift))
    }

    pub fn clear_gifts(&mut self) {
        self.gifts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gift_group() {
        let mut group = GiftGroup::new("Birthday");

        assert_eq!(group.list_gifts(), vec![]);
        assert!(!group.has_gift("Cake"));

        group.add_gift("Cake");
        group.add_gift("Balloon");

        assert_eq!(group.list_gifts(), vec![String::from("Cake"), String::from("Balloon")]);
        assert!(group.has_gift("Cake"));
        assert!(group.has_gift("Balloon"));

        assert!(group.remove_gift("Cake"));
        assert!(!group.has_gift("Cake"));
        assert_eq!(group.list_gifts(), vec![String::from("Balloon")]);

        group.clear_gifts();
        assert_eq!(group.list_gifts(), vec![]);
    }
}
