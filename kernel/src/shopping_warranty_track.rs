extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut warranty = WarrantyTracker::new();
    warranty.add_product("Laptop", 12);
    warranty.add_product("Smartphone", 24);
    warranty.check_warranty("Laptop");
    warranty.extend_warranty("Smartphone", 6);
    warranty.remove_product("Laptop");
    loop {}
}

pub struct WarrantyTracker {
    products: Vec<Product>,
}

impl WarrantyTracker {
    pub fn new() -> Self {
        WarrantyTracker {
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, name: &str, months: u32) {
        let product = Product {
            name: String::from(name),
            warranty_months: months,
        };
        self.products.push(product);
    }

    pub fn check_warranty(&self, name: &str) {
        for product in &self.products {
            if product.name == name {
                println!("Product: {}, Warranty Months Remaining: {}", product.name, product.warranty_months);
                return;
            }
        }
        println!("Product not found.");
    }

    pub fn extend_warranty(&mut self, name: &str, additional_months: u32) {
        for product in &mut self.products {
            if product.name == name {
                product.warranty_months += additional_months;
                println!("Warranty extended for {}: {} months", product.name, product.warranty_months);
                return;
            }
        }
        println!("Product not found.");
    }

    pub fn remove_product(&mut self, name: &str) {
        self.products.retain(|product| product.name != name);
        println!("Product removed: {}", name);
    }

    pub fn list_products(&self) {
        for product in &self.products {
            println!("Product: {}, Warranty Months: {}", product.name, product.warranty_months);
        }
    }
}

struct Product {
    name: String,
    warranty_months: u32,
}
