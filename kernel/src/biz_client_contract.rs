extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct BizClientContract {
    client_id: u32,
    contract_data: Vec<u8>,
    status: String,
    version: u16,
    is_active: bool,
}

impl BizClientContract {
    pub fn new(client_id: u32, initial_data: &[u8]) -> Self {
        BizClientContract {
            client_id,
            contract_data: initial_data.to_vec(),
            status: String::from("Active"),
            version: 1,
            is_active: true,
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        if self.is_active {
            self.status = String::from(new_status);
        }
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }

    pub fn get_contract_data(&self) -> &[u8] {
        &self.contract_data
    }

    pub fn update_contract_data(&mut self, new_data: &[u8]) {
        if self.is_active {
            self.contract_data = new_data.to_vec();
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.status = String::from("Inactive");
    }
}
