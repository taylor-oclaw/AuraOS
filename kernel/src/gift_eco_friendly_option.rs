extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut gift = GiftEcoFriendlyOption::new();
    gift.add_option(String::from("Bamboo Toothbrush"), 1.99);
    gift.add_option(String::from("Recycled Paper Napkins"), 2.49);
    gift.add_option(String::from("Solar Charger"), 15.99);
    gift.add_option(String::from("Eco-Friendly Cleaning Products"), 8.99);
    gift.add_option(String::from("Reusable Water Bottle"), 12.99);

    let total_cost = gift.calculate_total_cost();
    let cheapest_option = gift.find_cheapest_option().unwrap_or_else(|| String::from("No options available"));

    // Simulate some kernel operations
    loop {}
}

pub struct GiftEcoFriendlyOption {
    options: Vec<(String, f32)>,
}

impl GiftEcoFriendlyOption {
    pub fn new() -> Self {
        GiftEcoFriendlyOption { options: Vec::new() }
    }

    pub fn add_option(&mut self, name: String, price: f32) {
        self.options.push((name, price));
    }

    pub fn remove_option(&mut self, name: &str) {
        self.options.retain(|(option_name, _)| option_name != name);
    }

    pub fn calculate_total_cost(&self) -> f32 {
        self.options.iter().map(|(_, price)| *price).sum()
    }

    pub fn find_cheapest_option(&self) -> Option<String> {
        self.options.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(name, _)| name.clone())
    }

    pub fn list_options(&self) -> Vec<String> {
        self.options.iter().map(|(name, price)| String::from("info")).collect()
    }
}
