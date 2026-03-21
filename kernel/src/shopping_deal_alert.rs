extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut alert = ShoppingDealAlert::new();
    alert.add_deal("Laptop", 999);
    alert.add_deal("Smartphone", 499);
    alert.add_deal("Tablet", 299);

    if alert.has_deal_below_price(500) {
        alert.send_alert("Price drop detected!");
    }

    let deals = alert.get_all_deals();
    for deal in deals.iter() {
        println!("Deal: {} - ${}", deal.name, deal.price);
    }
}

pub struct ShoppingDealAlert {
    deals: Vec<Deal>,
}

impl ShoppingDealAlert {
    pub fn new() -> Self {
        ShoppingDealAlert { deals: Vec::new() }
    }

    pub fn add_deal(&mut self, name: &str, price: u32) {
        let deal = Deal {
            name: String::from(name),
            price,
        };
        self.deals.push(deal);
    }

    pub fn remove_deal(&mut self, name: &str) -> bool {
        self.deals.retain(|d| d.name != name);
        !self.deals.iter().any(|d| d.name == name)
    }

    pub fn has_deal_below_price(&self, price: u32) -> bool {
        self.deals.iter().any(|d| d.price < price)
    }

    pub fn get_all_deals(&self) -> Vec<Deal> {
        self.deals.clone()
    }

    pub fn send_alert(&self, message: &str) {
        // Simulate sending an alert
        println!("{}", message);
    }
}

struct Deal {
    name: String,
    price: u32,
}
