extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct MarketplaceOrgVolumeLicense {
    product_name: String,
    license_key: String,
    volume_count: u32,
    expiration_date: String,
    is_active: bool,
}

impl MarketplaceOrgVolumeLicense {
    pub fn new(product_name: &str, license_key: &str, volume_count: u32, expiration_date: &str) -> Self {
        MarketplaceOrgVolumeLicense {
            product_name: String::from(product_name),
            license_key: String::from(license_key),
            volume_count,
            expiration_date: String::from(expiration_date),
            is_active: true,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn update_volume_count(&mut self, new_count: u32) {
        if new_count > 0 {
            self.volume_count = new_count;
        }
    }

    pub fn check_license_validity(&self) -> bool {
        // Simple validity check based on expiration date
        let current_date = String::from("2023-10-01"); // Example current date
        self.is_active && self.expiration_date > current_date
    }

    pub fn get_license_info(&self) -> Vec<String> {
        vec![
            String::from("info"),
            String::from("info"),
            String::from("info"),
            String::from("info"),
            String::from("info"),
        ]
    }
}
