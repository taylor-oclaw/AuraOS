extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct GiftReceipt {
    recipient: String,
    items: Vec<String>,
    total_amount: u32,
}

impl GiftReceipt {
    pub fn new(recipient: &str, items: Vec<&str>, total_amount: u32) -> Self {
        GiftReceipt {
            recipient: String::from(recipient),
            items: items.into_iter().map(String::from).collect(),
            total_amount,
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn remove_item(&mut self, item_index: usize) -> Option<String> {
        if item_index < self.items.len() {
            Some(self.items.remove(item_index))
        } else {
            None
        }
    }

    pub fn get_total_amount(&self) -> u32 {
        self.total_amount
    }

    pub fn set_total_amount(&mut self, amount: u32) {
        self.total_amount = amount;
    }

    pub fn list_items(&self) -> Vec<&str> {
        self.items.iter().map(|item| item.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gift_receipt() {
        let mut receipt = GiftReceipt::new("Alice", vec!["Book", "Pen"], 20);

        assert_eq!(receipt.recipient, "Alice");
        assert_eq!(receipt.list_items(), vec!["Book", "Pen"]);
        assert_eq!(receipt.get_total_amount(), 20);

        receipt.add_item("Notebook");
        assert_eq!(receipt.list_items(), vec!["Book", "Pen", "Notebook"]);

        let removed_item = receipt.remove_item(1);
        assert_eq!(removed_item, Some(String::from("Pen")));
        assert_eq!(receipt.list_items(), vec!["Book", "Notebook"]);

        receipt.set_total_amount(30);
        assert_eq!(receipt.get_total_amount(), 30);
    }
}
