extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct VendorProfile {
    name: String,
    products: Vec<String>,
    location: String,
    established_year: u16,
    contact_email: String,
}

impl VendorProfile {
    pub fn new(name: &str, location: &str, established_year: u16, contact_email: &str) -> Self {
        VendorProfile {
            name: String::from(name),
            products: Vec::new(),
            location: String::from(location),
            established_year,
            contact_email: String::from(contact_email),
        }
    }

    pub fn add_product(&mut self, product_name: &str) {
        self.products.push(String::from(product_name));
    }

    pub fn get_products(&self) -> &[String] {
        &self.products
    }

    pub fn get_age(&self) -> u16 {
        2023 - self.established_year
    }

    pub fn update_contact_email(&mut self, new_email: &str) {
        self.contact_email = String::from(new_email);
    }

    pub fn display_profile(&self) -> String {
        let mut profile_info = format!("Vendor Name: {}\n", self.name);
        profile_info.push_str(format!("Location: {}\n", self.location).as_str());
        profile_info.push_str(format!("Established Year: {}\n", self.established_year).as_str());
        profile_info.push_str(format!("Contact Email: {}\n", self.contact_email).as_str());
        profile_info.push_str("Products:\n");
        for product in &self.products {
            profile_info.push_str(format!("- {}\n", product).as_str());
        }
        profile_info
    }
}
