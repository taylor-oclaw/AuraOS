extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FinanceSubscription {
    id: u32,
    customer_id: u32,
    subscription_type: String,
    start_date: u64, // Unix timestamp
    end_date: u64,   // Unix timestamp
    status: String,
}

impl FinanceSubscription {
    pub fn new(id: u32, customer_id: u32, subscription_type: &str, start_date: u64, end_date: u64) -> Self {
        FinanceSubscription {
            id,
            customer_id,
            subscription_type: String::from(subscription_type),
            start_date,
            end_date,
            status: String::from("active"),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_customer_id(&self) -> u32 {
        self.customer_id
    }

    pub fn get_subscription_type(&self) -> &str {
        &self.subscription_type
    }

    pub fn get_start_date(&self) -> u64 {
        self.start_date
    }

    pub fn get_end_date(&self) -> u64 {
        self.end_date
    }

    pub fn cancel(&mut self) {
        if self.status == "active" {
            self.status = String::from("cancelled");
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    pub fn extend(&mut self, additional_days: u64) {
        if self.is_active() {
            self.end_date += additional_days * 86400; // 86400 seconds in a day
        }
    }

    pub fn renew(&mut self, new_start_date: u64, new_end_date: u64) {
        if self.status == "cancelled" {
            self.start_date = new_start_date;
            self.end_date = new_end_date;
            self.status = String::from("active");
        }
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
