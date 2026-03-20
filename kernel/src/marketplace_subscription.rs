extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceSubscription {
    user_id: String,
    subscription_type: String,
    expiration_date: u64,
    active: bool,
    services: Vec<String>,
}

impl MarketplaceSubscription {
    pub fn new(user_id: &str, subscription_type: &str, expiration_date: u64) -> Self {
        MarketplaceSubscription {
            user_id: String::from(user_id),
            subscription_type: String::from(subscription_type),
            expiration_date,
            active: true,
            services: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn add_service(&mut self, service: &str) {
        if !self.services.contains(&String::from(service)) {
            self.services.push(String::from(service));
        }
    }

    pub fn remove_service(&mut self, service: &str) {
        self.services.retain(|s| s != service);
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_services(&self) -> &[String] {
        &self.services
    }
}
