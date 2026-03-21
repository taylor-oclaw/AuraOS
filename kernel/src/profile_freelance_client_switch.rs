extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileFreelanceClientSwitch {
    clients: Vec<String>,
    current_client_index: usize,
}

impl ProfileFreelanceClientSwitch {
    pub fn new() -> Self {
        ProfileFreelanceClientSwitch {
            clients: Vec::new(),
            current_client_index: 0,
        }
    }

    pub fn add_client(&mut self, client_name: &str) {
        self.clients.push(String::from(client_name));
    }

    pub fn remove_client(&mut self, client_name: &str) -> bool {
        if let Some(index) = self.clients.iter().position(|c| c == client_name) {
            self.clients.remove(index);
            true
        } else {
            false
        }
    }

    pub fn switch_to_next_client(&mut self) {
        if !self.clients.is_empty() {
            self.current_client_index = (self.current_client_index + 1) % self.clients.len();
        }
    }

    pub fn get_current_client(&self) -> Option<&str> {
        self.clients.get(self.current_client_index).map(|s| s.as_str())
    }

    pub fn list_clients(&self) -> Vec<&str> {
        self.clients.iter().map(|c| c.as_str()).collect()
    }
}
