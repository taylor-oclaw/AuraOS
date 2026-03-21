extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("biz_client_cross_sell_detect module loaded");
    0
}

pub struct CrossSellDetector {
    client_data: Vec<ClientData>,
}

impl CrossSellDetector {
    pub fn new() -> Self {
        CrossSellDetector {
            client_data: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client_id: u32, purchase_history: Vec<String>) {
        let client = ClientData {
            id: client_id,
            history: purchase_history,
        };
        self.client_data.push(client);
    }

    pub fn get_client_history(&self, client_id: u32) -> Option<&Vec<String>> {
        self.client_data.iter().find(|c| c.id == client_id).map(|c| &c.history)
    }

    pub fn detect_cross_sell(&self, client_id: u32, new_item: &str) -> bool {
        if let Some(history) = self.get_client_history(client_id) {
            history.contains(new_item)
        } else {
            false
        }
    }

    pub fn remove_client(&mut self, client_id: u32) {
        self.client_data.retain(|c| c.id != client_id);
    }

    pub fn list_all_clients(&self) -> Vec<u32> {
        self.client_data.iter().map(|c| c.id).collect()
    }
}

struct ClientData {
    id: u32,
    history: Vec<String>,
}
