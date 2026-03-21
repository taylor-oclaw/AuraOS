extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut leadership_change = BizClientLeadershipChange::new();
    leadership_change.add_client("Alice");
    leadership_change.add_client("Bob");
    leadership_change.transfer_leadership("Alice", "Bob");
    leadership_change.remove_client("Alice");
    leadership_change.list_clients();
}

pub struct BizClientLeadershipChange {
    clients: Vec<String>,
    leader: Option<String>,
}

impl BizClientLeadershipChange {
    pub fn new() -> Self {
        BizClientLeadershipChange {
            clients: Vec::new(),
            leader: None,
        }
    }

    pub fn add_client(&mut self, client_name: &str) {
        if !self.clients.contains(&String::from(client_name)) {
            self.clients.push(String::from(client_name));
        }
    }

    pub fn remove_client(&mut self, client_name: &str) {
        self.clients.retain(|c| c != client_name);
        if let Some(leader) = &self.leader {
            if leader == client_name {
                self.leader = None;
            }
        }
    }

    pub fn transfer_leadership(&mut self, from: &str, to: &str) {
        if self.clients.contains(&String::from(from)) && self.clients.contains(&String::from(to)) {
            if let Some(leader) = &self.leader {
                if leader == from {
                    self.leader = Some(String::from(to));
                }
            }
        }
    }

    pub fn set_leader(&mut self, client_name: &str) {
        if self.clients.contains(&String::from(client_name)) {
            self.leader = Some(String::from(client_name));
        }
    }

    pub fn list_clients(&self) {
        for client in &self.clients {
            // Simulate printing clients
            unsafe { println!("Client: {}", client); } // This is a placeholder, actual kernel module should handle logging differently
        }
    }
}
