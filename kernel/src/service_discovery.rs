extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Service {
    pub name: String,
    pub service_type: String,
    pub host: String,
    pub port: u16,
    pub agent_id: Option<u64>,
    pub metadata: Vec<(String, String)>,
    pub discovered_at: u64
}

pub struct ServiceDiscovery {
    pub services: Vec<Service>,
    pub published: Vec<Service>,
    pub scanning: bool
}

impl ServiceDiscovery {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            published: Vec::new(),
            scanning: false
        }
    }

    pub fn publish(&mut self, name: &str, stype: &str, port: u16, agent: Option<u64>) {
        self.published.push(Service {
            name: String::from(name),
            service_type: String::from(stype),
            host: String::from("localhost"),
            port,
            agent_id: agent,
            metadata: Vec::new(),
            discovered_at: 0
        });
    }

    pub fn scan(&mut self) {
        self.scanning = true;
    }

    pub fn find_by_type(&self, stype: &str) -> Vec<&Service> {
        self.services.iter().filter(|s| s.service_type == stype).collect()
    }

    pub fn find_agent_services(&self) -> Vec<&Service> {
        self.services.iter().filter(|s| s.agent_id.is_some()).collect()
    }

    pub fn unpublish(&mut self, name: &str) {
        self.published.retain(|s| s.name != name);
    }

    pub fn discovered_count(&self) -> usize {
        self.services.len()
    }
}
