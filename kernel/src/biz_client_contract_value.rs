extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ContractRenewal {
    client_id: u32,
    contract_duration: u32,
    renewal_date: String,
    services: Vec<String>,
    status: String,
}

impl ContractRenewal {
    pub fn new(client_id: u32, contract_duration: u32, renewal_date: &str, services: &[&str]) -> Self {
        let mut service_vec = Vec::new();
        for service in services {
            service_vec.push(String::from(*service));
        }
        ContractRenewal {
            client_id,
            contract_duration,
            renewal_date: String::from(renewal_date),
            services: service_vec,
            status: String::from("Pending"),
        }
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }

    pub fn set_contract_duration(&mut self, duration: u32) {
        self.contract_duration = duration;
    }

    pub fn add_service(&mut self, service: &str) {
        self.services.push(String::from(service));
    }

    pub fn remove_service(&mut self, service: &str) -> bool {
        if let Some(index) = self.services.iter().position(|s| s == service) {
            self.services.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
    }
}
