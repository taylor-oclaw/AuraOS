extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn gift_expense_report_link_init() {
    // Initialization code for the module
}

pub extern "C" fn gift_expense_report_link_exit() {
    // Cleanup code for the module
}

pub struct GiftExpenseReport {
    items: Vec<(String, u32)>,
    total: u32,
}

impl GiftExpenseReport {
    pub fn new() -> Self {
        GiftExpenseReport {
            items: Vec::new(),
            total: 0,
        }
    }

    pub fn add_item(&mut self, item_name: String, cost: u32) {
        self.items.push((item_name, cost));
        self.total += cost;
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        let mut index = None;
        for (i, (name, _)) in self.items.iter().enumerate() {
            if name == item_name {
                index = Some(i);
                break;
            }
        }
        if let Some(idx) = index {
            let (_, cost) = self.items.remove(idx);
            self.total -= cost;
            true
        } else {
            false
        }
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn list_items(&self) -> Vec<(String, u32)> {
        self.items.clone()
    }

    pub fn clear_report(&mut self) {
        self.items.clear();
        self.total = 0;
    }
}
