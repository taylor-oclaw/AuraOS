extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut moderation = MarketplaceModeration::new();
    moderation.add_product(String::from("AI Camera"), 299);
    moderation.add_product(String::from("Smartphone"), 799);
    moderation.remove_product(String::from("AI Camera"));
    moderation.update_price(String::from("Smartphone"), 699);
    let products = moderation.list_products();
    for product in products {
        println!("Product: {}, Price: {}", product.name, product.price);
    }
}

pub struct Product {
    name: String,
    price: u32,
}

impl Product {
    pub fn new(name: String, price: u32) -> Self {
        Product { name, price }
    }
}

pub struct MarketplaceModeration {
    products: Vec<Product>,
}

impl MarketplaceModeration {
    pub fn new() -> Self {
        MarketplaceModeration {
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, name: String, price: u32) {
        let product = Product::new(name, price);
        self.products.push(product);
    }

    pub fn remove_product(&mut self, name: String) {
        self.products.retain(|p| p.name != name);
    }

    pub fn update_price(&mut self, name: String, new_price: u32) {
        if let Some(product) = self.products.iter_mut().find(|p| p.name == name) {
            product.price = new_price;
        }
    }

    pub fn list_products(&self) -> Vec<Product> {
        self.products.clone()
    }

    pub fn get_product_price(&self, name: String) -> Option<u32> {
        self.products.iter().find(|p| p.name == name).map(|p| p.price)
    }
}
