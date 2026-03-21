extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut crm = biz_crm_core::CustomerRelationshipManagement::new();
    crm.add_customer("Alice".into(), "alice@example.com".into());
    crm.add_customer("Bob".into(), "bob@example.com".into());
    crm.update_email("Alice", "alice_new@example.com".into());
    let email = crm.get_email("Bob");
    if let Some(email) = email {
        // Do something with the email
    }
    crm.remove_customer("Alice");

    loop {}
}

mod biz_crm_core {
    use super::*;

    pub struct CustomerRelationshipManagement {
        customers: Vec<(String, String)>,
    }

    impl CustomerRelationshipManagement {
        pub fn new() -> Self {
            CustomerRelationshipManagement {
                customers: Vec::new(),
            }
        }

        pub fn add_customer(&mut self, name: String, email: String) {
            self.customers.push((name, email));
        }

        pub fn update_email(&mut self, name: &str, new_email: String) {
            if let Some(customer) = self.customers.iter_mut().find(|(n, _)| n == name) {
                customer.1 = new_email;
            }
        }

        pub fn get_email(&self, name: &str) -> Option<String> {
            self.customers
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, email)| email.clone())
        }

        pub fn remove_customer(&mut self, name: &str) {
            self.customers.retain(|(n, _)| n != name);
        }

        pub fn list_customers(&self) -> Vec<String> {
            self.customers.iter().map(|(name, _)| name.clone()).collect()
        }
    }
}
