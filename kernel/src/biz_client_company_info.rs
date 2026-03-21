extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct BizClientIndustry {
    clients: Vec<String>,
    industries: Vec<String>,
}

impl BizClientIndustry {
    pub fn new() -> Self {
        BizClientIndustry {
            clients: Vec::new(),
            industries: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client_name: &str) {
        if !self.clients.contains(&client_name.to_string()) {
            self.clients.push(client_name.to_string());
        }
    }

    pub fn remove_client(&mut self, client_name: &str) {
        if let Some(index) = self.clients.iter().position(|x| x == client_name) {
            self.clients.remove(index);
        }
    }

    pub fn add_industry(&mut self, industry_name: &str) {
        if !self.industries.contains(&industry_name.to_string()) {
            self.industries.push(industry_name.to_string());
        }
    }

    pub fn remove_industry(&mut self, industry_name: &str) {
        if let Some(index) = self.industries.iter().position(|x| x == industry_name) {
            self.industries.remove(index);
        }
    }

    pub fn list_clients(&self) -> Vec<String> {
        self.clients.clone()
    }

    pub fn list_industries(&self) -> Vec<String> {
        self.industries.clone()
    }
}
