extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ActionItemClassifier {
    items: Vec<String>,
}

impl ActionItemClassifier {
    pub fn new() -> Self {
        ActionItemClassifier { items: Vec::new() }
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

    pub fn list_items(&self) -> &Vec<String> {
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
    fn test_action_item_classifier() {
        let mut classifier = ActionItemClassifier::new();

        classifier.add_item(String::from("Task 1"));
        classifier.add_item(String::from("Task 2"));

        assert_eq!(classifier.get_item(0), Some(&String::from("Task 1")));
        assert_eq!(classifier.list_items(), &vec![String::from("Task 1"), String::from("Task 2")]);

        let removed = classifier.remove_item(1);
        assert_eq!(removed, Some(String::from("Task 2")));
        assert_eq!(classifier.list_items(), &vec![String::from("Task 1")]);

        classifier.clear_items();
        assert_eq!(classifier.list_items().len(), 0);
    }
}
