extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileFreelanceInvoiceTrigger {
    name: String,
    services: Vec<String>,
    rates: Vec<u32>,
    total_hours: u32,
    total_earned: u32,
}

impl ProfileFreelanceInvoiceTrigger {
    pub fn new(name: &str) -> Self {
        ProfileFreelanceInvoiceTrigger {
            name: String::from(name),
            services: Vec::new(),
            rates: Vec::new(),
            total_hours: 0,
            total_earned: 0,
        }
    }

    pub fn add_service(&mut self, service: &str, rate: u32) {
        self.services.push(String::from(service));
        self.rates.push(rate);
    }

    pub fn log_hours(&mut self, hours: u32) {
        self.total_hours += hours;
    }

    pub fn calculate_earnings(&self) -> u32 {
        if self.services.is_empty() || self.rates.is_empty() {
            return 0;
        }
        let rate = self.rates[0];
        rate * self.total_hours
    }

    pub fn generate_invoice(&self) -> String {
        let mut invoice = String::from("info");
        invoice.push_str("Services:\n");
        for (service, rate) in self.services.iter().zip(self.rates.iter()) {
            invoice.push_str(&String::from("info"));
        }
        invoice.push_str(&String::from("info"));
        invoice.push_str(&String::from("info"));
        invoice
    }

    pub fn reset(&mut self) {
        self.services.clear();
        self.rates.clear();
        self.total_hours = 0;
        self.total_earned = 0;
    }
}
