extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct FavorTrack {
    items: Vec<String>,
}

impl FavorTrack {
    pub fn new() -> Self {
        FavorTrack { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_item(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    pub fn list_items(&self) -> &[String] {
        &self.items
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favor_track() {
        let mut track = FavorTrack::new();
        assert_eq!(track.list_items().len(), 0);

        track.add_item(String::from("item1"));
        track.add_item(String::from("item2"));
        assert_eq!(track.list_items().len(), 2);
        assert_eq!(track.get_item(0), Some(&String::from("item1")));
        assert_eq!(track.get_item(1), Some(&String::from("item2")));

        let removed = track.remove_item(0);
        assert_eq!(removed, Some(String::from("item1")));
        assert_eq!(track.list_items().len(), 1);

        track.clear_items();
        assert_eq!(track.list_items().len(), 0);
    }
}
