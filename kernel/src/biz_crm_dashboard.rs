extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

struct Customer {
    id: u32,
    name: String,
    email: String,
}

impl Customer {
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        Customer {
            id,
            name: String::from(name),
            email: String::from(email),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn update_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn update_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

struct CRMSystem {
    customers: Vec<Customer>,
}

impl CRMSystem {
    pub fn new() -> Self {
        CRMSystem {
            customers: Vec::new(),
        }
    }

    pub fn add_customer(&mut self, customer: Customer) {
        self.customers.push(customer);
    }

    pub fn get_customers(&self) -> &Vec<Customer> {
        &self.customers
    }

    pub fn find_customer_by_id(&self, id: u32) -> Option<&Customer> {
        self.customers.iter().find(|c| c.get_id() == id)
    }

    pub fn update_customer_name(&mut self, id: u32, new_name: &str) -> bool {
        if let Some(customer) = self.find_customer_by_id(id) {
            customer.update_name(new_name);
            true
        } else {
            false
        }
    }

    pub fn update_customer_email(&mut self, id: u32, new_email: &str) -> bool {
        if let Some(customer) = self.find_customer_by_id(id) {
            customer.update_email(new_email);
            true
        } else {
            false
        }
    }
}
