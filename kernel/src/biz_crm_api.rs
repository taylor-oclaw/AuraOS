extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

mod biz_crm_api {
    use super::*;

    pub struct Customer {
        id: u32,
        name: String,
        email: String,
        phone: String,
        address: String,
    }

    impl Customer {
        pub fn new(id: u32, name: &str, email: &str, phone: &str, address: &str) -> Self {
            Customer {
                id,
                name: String::from(name),
                email: String::from(email),
                phone: String::from(phone),
                address: String::from(address),
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

        pub fn get_phone(&self) -> &str {
            &self.phone
        }

        pub fn get_address(&self) -> &str {
            &self.address
        }
    }

    pub struct CRMSystem {
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

        pub fn get_customer_by_id(&self, id: u32) -> Option<&Customer> {
            self.customers.iter().find(|c| c.get_id() == id)
        }

        pub fn remove_customer_by_id(&mut self, id: u32) -> bool {
            let pos = self.customers.iter().position(|c| c.get_id() == id);
            if let Some(index) = pos {
                self.customers.remove(index);
                true
            } else {
                false
            }
        }

        pub fn list_customers(&self) -> Vec<&Customer> {
            self.customers.iter().collect()
        }
    }
}
