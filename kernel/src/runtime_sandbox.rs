extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn runtime_sandbox_init() {
    // Initialization logic for the sandbox module
}

pub extern "C" fn runtime_sandbox_exit() {
    // Cleanup logic for the sandbox module
}

pub struct RuntimeSandbox {
    processes: Vec<String>,
    resources: Vec<String>,
    permissions: Vec<String>,
    logs: Vec<String>,
    status: String,
}

impl RuntimeSandbox {
    pub fn new() -> Self {
        RuntimeSandbox {
            processes: Vec::new(),
            resources: Vec::new(),
            permissions: Vec::new(),
            logs: Vec::new(),
            status: String::from("Initialized"),
        }
    }

    pub fn add_process(&mut self, process_name: &str) {
        self.processes.push(process_name.to_string());
        self.logs.push(String::from("info"));
    }

    pub fn remove_process(&mut self, process_name: &str) -> bool {
        if let Some(index) = self.processes.iter().position(|p| p == process_name) {
            self.processes.remove(index);
            self.logs.push(String::from("info"));
            true
        } else {
            false
        }
    }

    pub fn list_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn add_resource(&mut self, resource_name: &str) {
        self.resources.push(resource_name.to_string());
        self.logs.push(String::from("info"));
    }

    pub fn remove_resource(&mut self, resource_name: &str) -> bool {
        if let Some(index) = self.resources.iter().position(|r| r == resource_name) {
            self.resources.remove(index);
            self.logs.push(String::from("info"));
            true
        } else {
            false
        }
    }

    pub fn list_resources(&self) -> Vec<String> {
        self.resources.clone()
    }

    pub fn add_permission(&mut self, permission: &str) {
        self.permissions.push(permission.to_string());
        self.logs.push(String::from("info"));
    }

    pub fn remove_permission(&mut self, permission: &str) -> bool {
        if let Some(index) = self.permissions.iter().position(|p| p == permission) {
            self.permissions.remove(index);
            self.logs.push(String::from("info"));
            true
        } else {
            false
        }
    }

    pub fn list_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = new_status.to_string();
        self.logs.push(String::from("info"));
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
