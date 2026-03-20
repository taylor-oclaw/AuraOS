extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn asf_lifecycle_manager_init() {
}

pub extern "C" fn asf_lifecycle_manager_exit() {
}

pub struct ASFLifecycleManager {
    services: Vec<String>,
    state: String,
}

impl ASFLifecycleManager {
    pub fn new() -> Self {
        ASFLifecycleManager {
            services: Vec::new(),
            state: String::from("initialized"),
        }
    }

    pub fn add_service(&mut self, service_name: &str) {
        self.services.push(String::from(service_name));
    }

    pub fn remove_service(&mut self, service_name: &str) {
        if let Some(index) = self.services.iter().position(|s| s == service_name) {
            self.services.remove(index);
        }
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.clone()
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }

    pub fn set_state(&mut self, new_state: &str) {
        self.state = String::from(new_state);
    }
}
