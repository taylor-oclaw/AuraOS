extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn marketplace_certification_init() {
    // Initialization logic for the module
}

pub extern "C" fn marketplace_certification_exit() {
    // Cleanup logic for the module
}

pub struct MarketplaceCertification {
    products: Vec<String>,
    certified_products: Vec<String>,
}

impl MarketplaceCertification {
    pub fn new() -> Self {
        MarketplaceCertification {
            products: Vec::new(),
            certified_products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product_name: &str) {
        self.products.push(String::from(product_name));
    }

    pub fn certify_product(&mut self, product_name: &str) -> bool {
        if let Some(index) = self.products.iter().position(|p| p == product_name) {
            self.certified_products.push(self.products.remove(index));
            true
        } else {
            false
        }
    }

    pub fn is_certified(&self, product_name: &str) -> bool {
        self.certified_products.contains(&String::from(product_name))
    }

    pub fn list_products(&self) -> Vec<String> {
        self.products.clone()
    }

    pub fn list_certified_products(&self) -> Vec<String> {
        self.certified_products.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_certification() {
        let mut certification = MarketplaceCertification::new();
        certification.add_product("AI-100");
        certification.add_product("ML-200");

        assert_eq!(certification.list_products(), vec!["AI-100", "ML-200"]);
        assert_eq!(certification.list_certified_products(), Vec::<String>::new());

        assert!(certification.certify_product("AI-100"));
        assert!(!certification.certify_product("AI-300"));

        assert_eq!(certification.list_products(), vec!["ML-200"]);
        assert_eq!(certification.list_certified_products(), vec!["AI-100"]);

        assert!(certification.is_certified("AI-100"));
        assert!(!certification.is_certified("ML-300"));
    }
}
