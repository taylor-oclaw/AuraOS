extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct GiftHistory {
    history: Vec<String>,
}

impl GiftHistory {
    pub fn new() -> Self {
        GiftHistory {
            history: Vec::new(),
        }
    }

    pub fn add_gift(&mut self, gift: String) {
        self.history.push(gift);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn has_gift(&self, gift: &str) -> bool {
        self.history.iter().any(|g| g == gift)
    }

    pub fn count_gifts(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gift_history() {
        let mut history = GiftHistory::new();
        assert_eq!(history.count_gifts(), 0);

        history.add_gift(String::from("Book"));
        assert_eq!(history.count_gifts(), 1);
        assert!(history.has_gift("Book"));

        history.add_gift(String::from("Pen"));
        assert_eq!(history.count_gifts(), 2);
        assert!(history.has_gift("Pen"));

        let gifts = history.get_history();
        assert_eq!(gifts.len(), 2);
        assert_eq!(gifts[0], "Book");
        assert_eq!(gifts[1], "Pen");

        history.clear_history();
        assert_eq!(history.count_gifts(), 0);
        assert!(!history.has_gift("Book"));
    }
}
