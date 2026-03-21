extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FinanceReceipt {
    pub vendor: String,
    pub amount: u32,
    pub date: String,
    pub items: Vec<String>,
}

impl FinanceReceipt {
    pub fn new(vendor: &str, amount: u32, date: &str) -> Self {
        FinanceReceipt {
            vendor: String::from(vendor),
            amount,
            date: String::from(date),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn total_items(&self) -> usize {
        self.items.len()
    }

    pub fn is_over_expense(&self, threshold: u32) -> bool {
        self.amount > threshold
    }

    pub fn get_receipt_summary(&self) -> String {
        let mut summary = format!("Vendor: {}, Date: {}\n", self.vendor, self.date);
        for item in &self.items {
            summary.push_str(&format!("Item: {}\n", item));
        }
        summary.push_str(&format!("Total Amount: ${}\n", self.amount));
        summary
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }
}
