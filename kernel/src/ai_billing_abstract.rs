extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_billing_abstract_init() {
    // Initialization logic for the AI billing abstract module
}

#[no_mangle]
pub extern "C" fn ai_billing_abstract_exit() {
    // Cleanup logic for the AI billing abstract module
}

pub struct BillingRecord {
    customer_id: String,
    amount: u32,
    description: String,
}

impl BillingRecord {
    pub fn new(customer_id: &str, amount: u32, description: &str) -> Self {
        BillingRecord {
            customer_id: String::from(customer_id),
            amount,
            description: String::from(description),
        }
    }

    pub fn get_customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn update_amount(&mut self, new_amount: u32) {
        self.amount = new_amount;
    }

    pub fn update_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }
}

pub struct BillingSystem {
    records: Vec<BillingRecord>,
}

impl BillingSystem {
    pub fn new() -> Self {
        BillingSystem {
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: BillingRecord) {
        self.records.push(record);
    }

    pub fn get_records(&self) -> &Vec<BillingRecord> {
        &self.records
    }

    pub fn total_amount(&self) -> u32 {
        self.records.iter().map(|r| r.get_amount()).sum()
    }

    pub fn find_record_by_customer_id(&self, customer_id: &str) -> Option<&BillingRecord> {
        self.records.iter().find(|r| r.get_customer_id() == customer_id)
    }
}
