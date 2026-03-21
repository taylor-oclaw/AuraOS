extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub extern "C" fn grpc_runtime_init() {
    // Initialization logic for the gRPC runtime module
}

pub extern "C" fn grpc_runtime_exit() {
    // Cleanup logic for the gRPC runtime module
}

pub struct GrpcRuntime {
    services: Vec<String>,
    clients: Vec<String>,
}

impl GrpcRuntime {
    pub fn new() -> Self {
        GrpcRuntime {
            services: Vec::new(),
            clients: Vec::new(),
        }
    }

    pub fn register_service(&mut self, service_name: &str) {
        self.services.push(String::from(service_name));
    }

    pub fn unregister_service(&mut self, service_name: &str) {
        if let Some(index) = self.services.iter().position(|s| s == service_name) {
            self.services.remove(index);
        }
    }

    pub fn register_client(&mut self, client_name: &str) {
        self.clients.push(String::from(client_name));
    }

    pub fn unregister_client(&mut self, client_name: &str) {
        if let Some(index) = self.clients.iter().position(|c| c == client_name) {
            self.clients.remove(index);
        }
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.clone()
    }

    pub fn list_clients(&self) -> Vec<String> {
        self.clients.clone()
    }
}
