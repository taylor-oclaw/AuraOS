extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FoodOrder {
    order_id: u32,
    customer_name: String,
    items: Vec<String>,
    status: OrderStatus,
}

impl FoodOrder {
    pub fn new(order_id: u32, customer_name: &str) -> Self {
        FoodOrder {
            order_id,
            customer_name: String::from(customer_name),
            items: Vec::new(),
            status: OrderStatus::Received,
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|i| i == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    pub fn get_order_details(&self) -> String {
        let mut details = format!("Order ID: {}\nCustomer Name: {}\nStatus: {}", self.order_id, self.customer_name, self.status);
        if !self.items.is_empty() {
            details.push_str("\nItems:\n");
            for item in &self.items {
                details.push_str(&format!("- {}\n", item));
            }
        } else {
            details.push_str("\nNo items ordered.");
        }
        details
    }

    pub fn get_status(&self) -> OrderStatus {
        self.status
    }
}

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Received,
    Preparing,
    Cooking,
    ReadyForPickup,
    Delivered,
}
