extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceVerifiedBadge {
    name: String,
    description: String,
    version: u32,
    is_verified: bool,
    features: Vec<String>,
}

impl MarketplaceVerifiedBadge {
    pub fn new(name: &str, description: &str, version: u32) -> Self {
        MarketplaceVerifiedBadge {
            name: String::from(name),
            description: String::from(description),
            version,
            is_verified: false,
            features: Vec::new(),
        }
    }

    pub fn verify(&mut self) {
        self.is_verified = true;
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) -> bool {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_features(&self) -> &[String] {
        &self.features
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
    }
}
